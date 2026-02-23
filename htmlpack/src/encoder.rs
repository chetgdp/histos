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

// external
use base64::prelude::*;
use sha2::{Sha256, Digest};
// local
use crate::error::{HistosResult, EncodeError};

/// base64 crate's implementation of byte array base64 encoding.
///
/// could be optimized to SIMD later
///
/// # Errors
///
/// This function is infallible.
///
/// # Examples
///
/// ```
/// let encoded = base64_encode(b"hello world");
/// ```
pub fn base64_encode(buffer: &[u8]) -> String {
    BASE64_STANDARD.encode(&buffer)
}

/// Encodes a byte array into brotli compression format
///
/// We use harcoded quality 9 and window size 22. Found that it was the optimal compression size.
///
/// # Errors
///
/// - Returns [`EncodeError::Brotli`] if the brotli compressor returns an I/O error.
///
/// # Examples
///
/// ```no_run
/// let compressed = brotli_encode(wasm_bytes)?;
/// ```
pub fn brotli_encode(
    buffer: &[u8]
) -> HistosResult<Vec<u8>> {
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
    ).map_err(EncodeError::Brotli)?;
    println!("brotli done");
    Ok(compressed_buffer)
}

/// Generates a Sha256 hash for IndexDB runtime system.
///
/// We use this hash to compare it to the as the key in the hashtable. If the hash is the same you
/// can use the already stored decompressed and decoded binary. Speeding consecutive load times up
/// while making sure new binaries get loaded on new deployments. 
/// 
/// # Errors
///
/// This function is infallible.
///
/// # Examples
///
/// ```
/// let hex = hash_encode(b"hello");
/// ```
pub fn hash_encode(buffer: &[u8]) -> String {
    let hash = Sha256::digest(&buffer);
    format!("{:x}", hash)
}
