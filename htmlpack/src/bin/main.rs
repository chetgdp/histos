/*
* main.rs
*
* tiny bin entry point
*/

use std::error::Error;
use std::process::ExitCode;

use histos::*;

#[tokio::main]
async fn main() -> ExitCode {
    // program runs and and we capture any error
    if let Err(e) = packer::run().await { 
        eprintln!("error: {e}");
        // histoserror message already contains inner error, skip one level
        let mut source = e.source().and_then(|s| s.source());
        while let Some(s) = source {
            eprintln!("  caused by: {s}");
            source = s.source();
        }
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

