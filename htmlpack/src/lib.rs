/*
* lib.rs
*
* not sure why or what im doing here
*/


// private modules
mod cli;
mod wasmbuilder;

// public modules
pub mod config;
pub mod encoder;
pub mod error;
pub mod fetcher;
pub mod html;
pub mod packer;
pub mod render;

pub use config::{PackConfig, WasmModule, CompressionType, AssetSource};
pub use error::{HistosError, HistosResult};
pub use packer::{load_config};
pub use render::{PackedHtml, render_to_packed};
