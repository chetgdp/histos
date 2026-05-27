/
* packer.rs
*
* the main packing logic
*/

/*
This is the main of the program that packs everything together

we fetch scripts from here
might turn into some type of cli tool that you use with yaml files

local/remote: css, js, text, png, etc
pack it up

compress it with brotli
then encode it in base64
*/


// standard
use std::fs;
use std::path::PathBuf;
// external
use clap::{Parser};
use base64::prelude::*;
use crate::config::{
    AssetSource, 
    WasmModule, 
    CompressionType, 
    PackConfig,
    RuntimeConfig,
    //MetadataConfig, // why this not used?
};
// local
use crate::cli::{YamlRoot, Cli};
use crate::encoder;
use crate::error::{HistosResult, ConfigError, EncodeError};
use crate::wasmbuilder;
use crate::html::{HtmlDoc, EncodedWasm, EncodedIcon};
use crate::fetcher;


// runtime assets on default
const RUNTIME_ICON: &str = include_str!("../core/icon.svg");
const RUNTIME_CORE_JS: &str = include_str!("../core/core.js");
const RUNTIME_DECODER_JS: &str = include_str!("../core/wasm_decoder.js");
const RUNTIME_DECODER_WASM: &[u8] = include_bytes!("../core/wasm_decoder_bg.wasm");
const DEFAULT_DECODER_ID: &str = "bin-wasm-decoder";
const DEFAULT_ICON_MIME_TYPE: &str = "svg+xml";
const DEFAULT_ICON_ENCODING: &str = "base64";

// read yaml from file
// set config from yaml
// need serde structs

/// Main entry point for the program and API
///
/// Reads CLI to get config. Loads config. Builds data sources. Renders html document. Saves to file.
/// # Errors
///
/// - Returns [`HistosResult`] every kind of error as defined.
/// 
/// # Examples
/// ```
/// packer::run().await?;
/// ```
pub async fn run() -> HistosResult<()> {
    // parse CLI
    let cli = Cli::parse();
    eprintln!("Config: {}", cli.config.display());
    eprintln!("Output: {}", cli.output.display());

    // oh yeah this feels good
    load_config(cli.config).await?
        .build().await?
        .render()
        .save_to_file(cli.output.clone())?;

    println!("{}", cli.output.display());
    Ok(())
}

/// Loads config from given path, serde yaml->config magic
///
/// Parses yaml config using serde
///
/// # Errors
///
/// - Returns [`ConfigError::FileNotFound`] if the path does not exist.
/// - Returns [`ConfigError::ReadFailed`] if the file cannot be read.
/// - Returns [`ConfigError::YamlParse`] if the YAML is malformed.
/// - Returns [`ConfigError::PkgDirNotFound`] or [`ConfigError::PkgFileMissing`] if a wasm module's pkg directory is missing or incomplete.
///
/// # Examples
///
/// ```no_run
/// let config = load_config(PathBuf::from("config.yaml")).await?;
/// ```
pub async fn load_config(config_path: PathBuf) -> HistosResult<PackConfig> {
    let yaml_text = fs::read_to_string(&config_path)
        .map_err(|source| match source.kind() {
            std::io::ErrorKind::NotFound => ConfigError::FileNotFound { path: config_path.clone() },
            _ => ConfigError::ReadFailed { path: config_path.clone(), source },
        })?;
    let yaml_root: YamlRoot = serde_yaml::from_str(&yaml_text)
        .map_err(|e| ConfigError::YamlParse {
            path: config_path.clone(),
            source: e,
        })?;
    let config: PackConfig = yaml_root.pack.try_into()?;
    eprintln!("Loaded config");
    Ok(config)
}

// extremely wonky
fn default_runtime(
    runtime: &RuntimeConfig,
    // what to inject into
    favicons: &mut Vec<EncodedIcon>,
    scripts: &mut Vec<String>,
    wasm: &mut Vec<EncodedWasm>,
) -> HistosResult<()> {
    eprintln!("Default runtime enabled.");

    // favicon
    if runtime.icon {
        eprintln!("Adding icon.");
        let encoded_icon_string = 
            //BASE64_STANDARD.encode(RUNTIME_ICON.as_bytes());
            encoder::base64_encode(RUNTIME_ICON.as_bytes());
        let encoded_icon = EncodedIcon {
            mime_type: DEFAULT_ICON_MIME_TYPE.to_string(),
            encoding: DEFAULT_ICON_ENCODING.to_string(),
            text: encoded_icon_string,
        };
        favicons.insert(0, encoded_icon);
    }

    // core script
    if runtime.core {
        eprintln!("Adding core.js");
        scripts.push(RUNTIME_CORE_JS.to_string());
    }
    
    // decoder js and wasm
    if runtime.decoder {
        eprintln!("Adding decoder.");
        scripts.push(RUNTIME_DECODER_JS.to_string());
        // decoder wasm binary
        //let wasm_hash = Sha256::digest(RUNTIME_DECODER_WASM);
        //let wasm_hash_string = format!("{:x}", wasm_hash);
        let wasm_hash_string = encoder::hash_encode(RUNTIME_DECODER_WASM);
        let wasm_encoded_text = BASE64_STANDARD.encode(RUNTIME_DECODER_WASM);
        let decoder_module = EncodedWasm {
            id: DEFAULT_DECODER_ID.to_string(),
            hash: wasm_hash_string,
            text: wasm_encoded_text,
        };
        wasm.push(decoder_module);
    }

    Ok(())
}

impl PackConfig {
    
    /// This is the holy grail function 
    /// 
    /// This is the best we could come up with. Its still quite janky but extending the program is easier now. A each stage could be done in async.
    /// 
    ///
    /// # Errors
    ///
    /// - Returns [`crate::error::CompileError`] if wasm-pack compilation fails.
    /// - Returns [`crate::error::FetchError`] if any asset cannot be fetched from disk or over HTTP.
    /// - Returns [`EncodeError::Brotli`] if brotli compression fails.
    /// - Returns [`EncodeError::InvalidUtf8`] if a script or HTML asset is not valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let doc = config.build().await?;
    /// ```
    pub async fn build(self) -> HistosResult<HtmlDoc> {
        // OPERATION 1: COMPILE
        // make sure to compile our wasm binaries and js glue first
        if !self.wasm.is_empty() {
            wasmbuilder::compile_wasm_modules(&self.wasm).await?;
        }

        // OPERATION 2: FETCH
        // preprocess just wasm sources, #noragrets clone lel
        // this is just not it man
        let wasm_bin_vec: Vec<AssetSource> = self.wasm
            .iter()
            .map(|module| module.binary.clone())
            .collect();
        let wasm_glue_vec: Vec<AssetSource> = self.wasm
            .iter()
            .map(|module| module.glue.clone())
            .collect();

        let favicon_bytes = fetcher::fetch_all_sources(self.favicon).await?;
        let style_bytes = fetcher::fetch_all_sources(self.styles).await?;
        let mut script_bytes =  fetcher::fetch_all_sources(self.scripts).await?;
        let html_bytes = fetcher::fetch_all_sources(self.html).await?;
        let wasm_bytes = fetcher::fetch_all_sources(wasm_bin_vec).await?;
        let wasm_glue_bytes = fetcher::fetch_all_sources(wasm_glue_vec).await?;

        script_bytes.extend(wasm_glue_bytes);

        // OPERATION 3: PROCESS
        let favicons = process_icons(favicon_bytes)?;
        let styles = process_styles(style_bytes)?;
        let scripts = process_text(script_bytes)?;
        let html_shards = process_text(html_bytes)?;
        let mut encoded_wasm = process_wasm(wasm_bytes, &self.wasm)?;
        
        // OPERATION 4: RUNTIME
        // need to enable mutability before injecting runtime
        let mut favicons = favicons;
        let mut scripts = scripts;
        if self.runtime.enabled {
            default_runtime(
                &self.runtime,
                &mut favicons,
                &mut scripts,
                &mut encoded_wasm,
            )?;
        }
        
        // OPERATION 5: PACK
        // create the htmldoc representation
        let htmldoc = HtmlDoc::new(
            // head
            // metadata
            self.metadata.title,
            self.metadata.author,
            self.metadata.description,
            self.metadata.keywords,
            // assets
            favicons,
            styles,
            // body
            encoded_wasm,
            scripts,
            html_shards,
        );

        Ok(htmldoc)
    }
}


// helpers for processing
// each part is processed differently for its HtmlDoc format
// the ideas here is that we want to limit the amount of different work done

fn process_icons(bytes: Vec<Vec<u8>>) -> HistosResult<Vec<EncodedIcon>> {
    bytes
        .into_iter()
        .map(|b| {
            let text = encoder::base64_encode(&b);
            Ok(EncodedIcon {
                mime_type:  DEFAULT_ICON_MIME_TYPE.to_string(),
                encoding:   DEFAULT_ICON_ENCODING.to_string(),
                text
            })
        })
        .collect()
}

fn process_text(bytes: Vec<Vec<u8>>) -> HistosResult<Vec<String>> {
    bytes
        .into_iter()
        .map(|b| String::from_utf8(b)
            .map_err(EncodeError::from)
            .map_err(Into::into))
        .collect()

    /*
    let mut results = Vec::new();
    for b in bytes {
        results.push(&string::from_utf8(b)?);
    }
    Ok(results)
    */
}

fn process_styles(bytes: Vec<Vec<u8>>) -> HistosResult<String> {
    process_text(bytes).map(|strings| strings.join(""))
}

fn process_wasm(bytes: Vec<Vec<u8>>, modules: &[WasmModule]) -> HistosResult<Vec<EncodedWasm>> {
    bytes
        .into_iter()
        .zip(modules.iter())
        .map(|(b, m)| {
            let compressed_buffer = match m.compression {
                CompressionType::Brotli => { encoder::brotli_encode(&b)? },
                _ => { b }
            };
            //let hash = Sha256::digest(&compressed_buffer);
            //let hash_string = format!("{:x}", hash);
            let hash_string = encoder::hash_encode(&compressed_buffer);
            let text = encoder::base64_encode(&compressed_buffer);
            
            // is this my style?
            let encoded_wasm = 
            EncodedWasm::new(
                m.id.clone(),
                hash_string,
                text,
            );

            Ok(encoded_wasm)
        })
        .collect()
}

