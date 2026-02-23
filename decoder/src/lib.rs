/*
so what is the point of this file?

get compiled by wasmpack as nomodules
get base64d and embedded into html
get decoded by js atob
get loaded by instantiate WASM

then unpack the rest of the system
so we need web-sys here too?
no, we just do the js in js
this will serve exclusively as:
    - brolti decompressor
    - base94 decoder
so that we can have multiple building blocks
this is just the unpacker

how much control should it have?
js in js or just wasm it all?

*/

use brotli;
//use base94;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = decompress)]
pub fn decompress(buf: Box<[u8]>) -> Result<Box<[u8]>, JsValue> {
    //set_panic_hook();
    let mut out = Vec::<u8>::new();
    match brotli::BrotliDecompress(&mut buf.as_ref(), &mut out) {
        Ok(_) => (),
        Err(e) => return Err(JsValue::from_str(&format!(
            "Brotli decompress failed: {:?}", e
        ))),
    }
    Ok(out.into_boxed_slice())
}

/*
pub fn set_panic_hook() {
    #[cfg(feature="console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
*/

/*
#[wasm_bindgen(js_name = decodeBase94)]
pub fn decode_base94(buf: Box<[u8]>) -> Result<Box<[u8]>, JsValue> {
    set_panic_hook();
    // Convert the byte array to a UTF-8 string
    let encoded_str = match std::str::from_utf8(&buf) {
        Ok(s) => s,
        Err(e) => return Err(JsValue::from_str(&format!(
            "Invalid UTF-8 sequence: {:?}", e
        ))),
    };
    
    // Decode using base94
    match base94::decode(encoded_str, 94) {
        Ok(decoded) => Ok(decoded.into_boxed_slice()),
        Err(e) => Err(JsValue::from_str(&format!(
            "Base94 decode failed: {:?}", e
        ))),
    }

    /*
    let mut out = Vec::<u8>::new();
    match base94::decode(&buf, 94) {
        Ok(decoded) => Ok(decoded.into_boxed_slice()),
        Err(e) => return Err(JsValue::from_str(&format!(
            "Base94 decode failed: {:?}", e
        ))),
    }
    */
}
*/
