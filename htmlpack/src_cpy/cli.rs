/*
* cli.rs
* 
* where cli commands are parsed
* yaml config declaration
*/

// standard
//use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;
// external
use clap::{Parser};
use serde::{Deserialize, Serialize};
// local
// none

// yaml structs
// not sure if this is correct
#[derive(Debug, Serialize, Deserialize)]
pub struct YamlRoot {
    pub pack: YamlPack,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlPack {
    pub runtime:        Option<YamlRuntime>,
    pub metadata:       Option<YamlMetadata>,
    pub favicon:        Option<Vec<String>>,      //Option<YamlAssets>,
    pub css:            Option<Vec<String>>,      //Option<YamlAssets>,
    pub html:           Option<Vec<String>>,      //Option<YamlAssets>,
    pub scripts:        Option<Vec<String>>,      //Option<YamlAssets>,
    pub wasm:           Option<HashMap<String, YamlWasmModule>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlRuntime {
    pub enabled:        Option<bool>,
    pub icon:           Option<bool>,
    pub core:           Option<bool>,
    pub decoder:        Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlMetadata {
    pub title:          Option<String>,
    pub author:         Option<String>,
    pub description:    Option<String>,
    pub keywords:       Option<String>,
}


//#[derive(Debug, Serialize, Deserialize)]
//pub struct YamlAssets {
//    pub assets:          Vec<String>,
//}
//#[derive(Debug, Serialize, Deserialize)]
//pub struct YamlAssets {
//    pub local:          Option<Vec<String>>,
//    pub remote:         Option<Vec<String>>,
//}

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlWasmModule {
    pub path:           Option<String>, // local only      
    pub binary:         Option<String>, // can be url
    pub glue:           Option<String>, // can be url
    pub id:             Option<String>,
    pub compile_wasm:   Option<bool>,
    pub compression:    Option<String>,
}

// clap ---------------------------------------------------------------------- /

// this is the name of the main command
#[derive(Parser)]
#[command(name = "histos")]
#[command(about = "Weave web assets into a single HTML file")]
pub struct Cli {
    /// path to the YAML configuration file
    pub config: PathBuf,
    
    /// output file path with default
    #[arg(short, long, default_value = "./index.html")]
    pub output: PathBuf,
}

