/*
* encoder.rs
*
* The point of this file:
* encode the wasm binaries using brotli and base64
* return a Base, which means what goes into a <pre> tag inside the html
* 
* base64 is highly optimized for encode/decode
* we will eventually implement base94 if there is a need
* 
* so what do we need to do to encode
* we read the file from a filepath
* 
* we compress with brotli (or not)
* 
* we encode as base64
* 
* we return the id and the utf-8 compatible string
*/

// standard
use std::error::Error;
// external
use base64::prelude::*;
use sha2::{Sha256, Digest};
// local
// none

// why do this? easier to just have the base64 import here
pub fn base64_encode(buffer: &[u8]) -> String {
    BASE64_STANDARD.encode(&buffer)
}

// so we have a buffer of bytes
// lets compress with brotli
// create a buffer for compressed data
pub fn brotli_encode(
    //buffer: &Vec<u8>
    buffer: &[u8]
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut compressed_buffer = Vec::new();
    println!("brotli compression"); 
    brotli::BrotliCompress(
        &mut &buffer[..],           // input buffer as a Read impl
        &mut compressed_buffer,     // output buffer as a Write impl
        &brotli::enc::BrotliEncoderParams {
            //quality: 11,          // quality (0-11)
            quality: 9,             // try 9 as sweet spot
            lgwin: 22,              // Window size (recommended 20-22)
            ..Default::default() 
        }
    )?;
    println!("brotli done"); 
    Ok(compressed_buffer)
}

pub fn hash_encode(buffer: &[u8]) -> Result<String, Box<dyn Error>> {
    let hash = Sha256::digest(&buffer);
    let hash_string = format!("{:x}", hash);
    Ok(hash_string)
}

