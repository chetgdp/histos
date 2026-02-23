/*
 * error.rs
 *
 * good information on what is going wrong for the user
 * lot of macros here cause of this error
 *
 * hierarchy:
 * Histos Error
 * - Config
 * - Compile
 * - Fetch
 * - Encode
 * - Save
 * - Internal
 */

// standard
use std::path::PathBuf;
// external
use thiserror::Error;
// local

// alias
pub type HistosResult<T> = std::result::Result<T, HistosError>;

// error definitions using thiserror
#[derive(Error, Debug)]
pub enum HistosError {
    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("compilation error: {0}")]
    Compile(#[from] CompileError),
    
    #[error("fetch error: {0}")]
    Fetch(#[from] FetchError),
    
    #[error("encoding error: {0}")]
    Encode(#[from] EncodeError),
    
    #[error("save error: {0}")]
    Save(#[from] SaveError),
    
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("config file not found: {path}")]
    FileNotFound { path: PathBuf },
    
    #[error("failed to read config file {path}")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("invalid YAML in config")]
    YamlParse(#[from] serde_yaml::Error),
    
    #[error("pkg directory not found: {path}")]
    PkgDirNotFound { path: PathBuf },
    
    #[error("missing required file in pkg: {missing} (searched {path})")]
    PkgFileMissing { path: PathBuf, missing: &'static str },
    
    #[error("failed to read pkg directory {path}")]
    PkgDirRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    }
}

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("wasm-pack not found or inaccessible")]
    WasmPackNotFound,
    
    #[error("wasm-pack build failed in {dir}")]
    BuildFailed { dir: String },
    
    #[error("compilation task failed")]
    TaskJoin(#[from] tokio::task::JoinError),
    
    #[error("failed to spawn wasm-pack process")]
    ProcessSpawn(#[source] std::io::Error)
}

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("file not found: {path}")]
    LocalNotFound { path: PathBuf },
    
    #[error("failed to read file {path}")]
    LocalRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("HTTP request failed for {url}")]
    HttpRequest {
        url: String,
        #[source]
        source: reqwest::Error,
    },
    
    #[error("HTTP {status} for {url}")]
    HttpStatus { url: String, status: u16 },
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("Brotli compression failed: {0}")]
    Brotli(#[source] std::io::Error),
    
    #[error("invalid UTF-8 in source file")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

#[derive(Error, Debug)]
pub enum SaveError {
    #[error("failed to create output directory {path}")]
    CreateDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("failed to create output file {path}")]
    CreateFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("failed to write output file {path}")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

