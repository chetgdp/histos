/*
* config.rs
*
* where the configuration structs are declared
* also where the config is built
* the config determines how the sources get packed into html
*/

// standard
use std::path::PathBuf;
use std::collections::HashMap;
// external
use url::Url;
// local
use crate::cli::*;

// where else to put this?
const DEFAULT_WASM_ID:  &str = "bin-wasm-app";
const DEFAULT_TITLE:    &str = "histos";

// internal config structs

// enum that distinguishes between local and remote files
//#[derive(Debug, Clone, Deserialize, Serialize)]
//#[serde(untagged)]
#[derive(Debug, Clone)]
pub enum AssetSource {
    Local(PathBuf),
    Remote(Url),
    //Inline(String),
}

impl Default for AssetSource {
    fn default() -> Self {
        AssetSource::Local(PathBuf::new())
    }
}

//#[derive(Default, Debug, Deserialize, Serialize)]
//#[serde(rename_all = "lowercase")]
#[derive(Default, Debug)]
pub enum CompressionType {
    #[default]
    Brotli,
    None,
}

// these are the configuration options
// this defines the source files that will be built
#[derive(Debug)]
pub struct PackConfig {
    pub runtime:        RuntimeConfig,
    pub metadata:       MetadataConfig,
    pub favicon:        Vec<AssetSource>,
    pub styles:         Vec<AssetSource>,
    pub html:           Vec<AssetSource>,
    pub scripts:        Vec<AssetSource>,
    pub wasm:           Vec<WasmModule>,
}

#[derive(Debug)]
pub struct RuntimeConfig {
    pub enabled:        bool,
    pub icon:           bool,
    pub core:           bool,
    pub decoder:        bool,
}

#[derive(Debug)]
pub struct MetadataConfig {
    pub title:          String,
    pub author:         String,
    pub description:    String,
    pub keywords:       String,
}

#[derive(Debug)]
pub struct WasmModule {
    pub path:           PathBuf,
    pub binary:         AssetSource,
    pub glue:           AssetSource,
    pub id:             String,
    pub compile_wasm:   bool,
    pub compression:    CompressionType,
}

// defaults ------------------------------------------------------------------ /

impl Default for PackConfig {
    fn default() -> Self {
        Self {
            runtime:        RuntimeConfig::default(),
            metadata:       MetadataConfig::default(),
            favicon:        Vec::new(),
            styles:         Vec::new(),
            html:           Vec::new(),
            scripts:        Vec::new(),
            wasm:           Vec::new(),
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self { 
            enabled:        true,
            icon:           true,
            core:           true,
            decoder:        true,
        }
    }
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            title:          String::from(DEFAULT_TITLE),
            author:         String::new(),
            description:    String::new(),
            keywords:       String::new(),
        }
    }
}

impl Default for WasmModule {
    fn default() -> Self {
        Self {
            path:           PathBuf::new(),
            binary:         AssetSource::default(),
            glue:           AssetSource::default(),
            id:             String::from(DEFAULT_WASM_ID),
            compile_wasm:   false,
            compression:    CompressionType::Brotli,
        }
    }
}

// tranform from YamlConfig -------------------------------------------------- /

impl From<YamlPack> for PackConfig {
    fn from(yaml: YamlPack) -> Self {
        Self {
            runtime:    yaml.runtime
                            .map(RuntimeConfig::from)
                            .unwrap_or_default(),

            metadata:   yaml.metadata
                            .map(MetadataConfig::from)
                            .unwrap_or_default(),

            favicon:    yaml.favicon
                            .map(convert_yaml_assets)
                            .unwrap_or_default(),

            styles:     yaml.css
                            .map(convert_yaml_assets)
                            .unwrap_or_default(),

            scripts:     yaml.scripts
                            .map(convert_yaml_assets)
                            .unwrap_or_default(),
                            
            html:       yaml.html
                            .map(convert_yaml_assets)
                            .unwrap_or_default(),
            
            wasm:       yaml.wasm
                            .map(convert_yaml_wasm_modules)
                            .unwrap_or_default(),
        }
    }
}

impl From<YamlRuntime> for RuntimeConfig {
    fn from(yaml: YamlRuntime) -> Self {
        // if runtime is disabled, we want to disable all of it
        if yaml.enabled == Some(false) {
            return Self {
                enabled:    false,
                icon:       false,
                core:       false,
                decoder:    false,
            };
        }
        // if runtime is enabled, use defaults or given values
        let defaults = Self::default();
        Self {
            enabled:        yaml.enabled.unwrap_or(defaults.enabled),
            icon:           yaml.icon.unwrap_or(defaults.icon),
            core:           yaml.core.unwrap_or(defaults.core),
            decoder:        yaml.decoder.unwrap_or(defaults.decoder),
        }
    }
}

impl From<YamlMetadata> for MetadataConfig {
    fn from(yaml: YamlMetadata) -> Self {
        let defaults = Self::default();
        Self {
            title:          yaml.title.unwrap_or(defaults.title),
            author:         yaml.author.unwrap_or(defaults.author),
            description:    yaml.description.unwrap_or(defaults.description),
            keywords:       yaml.keywords.unwrap_or(defaults.keywords),
        }
    }
}

impl From<YamlWasmModule> for WasmModule {
    fn from(yaml: YamlWasmModule) -> Self {
        let defaults = Self::default();

        // if path is provided use it
        let (base_path, binary, glue) = 
        if let Some(path_str) = yaml.path {
            let base = PathBuf::from(&path_str);
            let pkg_dir = base.join("pkg");
            // find and get the files using given "directory/pkg/"
            let (wasm, js) = get_pkg_files(&pkg_dir);
            (
                base,
                AssetSource::Local(wasm),
                AssetSource::Local(js),
            )
        // fall back to explicit paths given
        } else {
            let wasm = yaml.binary
                .map(determine_asset_source)
                .unwrap_or(defaults.binary);
            let js = yaml.glue
                .map(determine_asset_source)
                .unwrap_or(defaults.glue);
            (
                PathBuf::new(),
                wasm,
                js,
            )
        };

        Self {
            path: base_path,
            binary,
            glue,
            id: yaml.id.unwrap_or(defaults.id),
            compile_wasm: yaml.compile_wasm.unwrap_or(defaults.compile_wasm),
            compression: yaml.compression
                .map(determine_compression_type)
                .unwrap_or_default(),
        }
    }
}
//path:   AssetSource::Local(PathBuf::from(yaml.path)),
//binary: AssetSource::Local(PathBuf::from(yaml.binary)),
//glue:   AssetSource::Local(PathBuf::from(yaml.glue)),
//path: yaml.path.unwrap_or(defaults.path)

// helpers

fn determine_asset_source(path: String) -> AssetSource {
    // check if url
    if path.starts_with("https://") || path.starts_with("http://") {
        if let Ok(url) = Url::parse(&path) {
            return AssetSource::Remote(url);
        }
    }  
    // inline or local file path?
    AssetSource::Local(PathBuf::from(path))
}

fn determine_compression_type(s: String) -> CompressionType {
    match s.as_str() {
        "brotli"    => CompressionType::Brotli,
        "none"      => CompressionType::None,
        // set none here leads to default
        _           => CompressionType::None,    
    }
}

fn get_pkg_files(pkg_dir: &PathBuf) -> (PathBuf, PathBuf) {
    let file_entries = std::fs::read_dir(pkg_dir)
        .expect("Failed to read pkg directory");

    let mut wasm = None;
    let mut js = None;

    for entry in file_entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with("_bg.wasm") {
                    wasm = Some(path.clone());
                } else if name.ends_with(".js") {
                    js = Some(path.clone());
                }
            }
        }
    }
    
    (
        wasm.expect("could not find _bg.wasm binary in pkg/"),
        js.expect("could not find .js glue in pkg/")
    )

}

// from YamlAsset strings to specific AssetSource
fn convert_yaml_assets(
    //set_of: YamlAssets
    assets: Vec<String>
) -> Vec<AssetSource> {
    assets
        .into_iter()
        .map(determine_asset_source)
        .collect()
}

fn convert_yaml_wasm_modules(
    modules: HashMap<String, YamlWasmModule>
) -> Vec<WasmModule> {
    modules
        .into_iter()
        .map(|(_key, yaml_module)| WasmModule::from(yaml_module))
        .collect()
}

// builder api --------------------------------------------------------------- /

// shouldn't we be able to build everything from string or bool?

impl PackConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_metadata(
        mut self,
        title: String,
        author: String,
        description: String,
        keywords: String,
    ) -> Self {
        self.metadata = MetadataConfig {
            title,
            author,
            description,
            keywords,
        };
        self
    }

    pub fn set_runtime(
        mut self,
        enabled: bool,
        icon: bool,
        core: bool,
        decoder: bool
    ) -> Self {
        self.runtime = RuntimeConfig {
            enabled,
            icon,
            core,
            decoder,
        };
        self
    }

    pub fn add_style(mut self, path: String) -> Self {
        self.styles.push(determine_asset_source(path));
        self
    }

    pub fn add_script(mut self, path: String) -> Self {
        self.scripts.push(determine_asset_source(path));
        self
    }

    pub fn add_html(mut self, path: String) -> Self {
        self.html.push(determine_asset_source(path));
        self
    }

    pub fn add_wasm(mut self, module: WasmModule) -> Self {
        self.wasm.push(module);
        self
    }

    pub fn add_wasm_pkg(mut self, id: String, path: String) -> Self {
        let module = WasmModule::from_pkg(id, path, false, "brotli".into());
        self.wasm.push(module);
        self
    }
}

impl WasmModule {
    pub fn from_pkg(
        id: String, 
        pkg_path: String,
        compile_wasm: bool,
        compression: String,
    ) -> Self {
        let base = PathBuf::from(&pkg_path);
        let pkg_dir = base.join("pkg");
        let (wasm, js) = get_pkg_files(&pkg_dir);
        Self {
            id: id,
            path: base,
            binary: AssetSource::Local(wasm),
            glue: AssetSource::Local(js),
            compile_wasm: compile_wasm,
            compression: determine_compression_type(compression)
        }
    }

    pub fn set_manually(
        id: String,
        binary_path: String,
        glue_path: String,
        compile_wasm: bool,
        compression: String,
    ) -> Self {
        Self {
            id,
            path: PathBuf::new(),
            binary: determine_asset_source(binary_path),
            glue: determine_asset_source(glue_path),
            compile_wasm,
            compression: determine_compression_type(compression)
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn with_compile(mut self, flag: bool) -> Self {
        self.compile_wasm = flag;
        self
    }

    pub fn with_compression(mut self, compression: String) -> Self {
        self.compression = determine_compression_type(compression);
        self
    }
}
