/*
* main.rs
*
* tiny bin entry point
*/

use histos::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    packer::run().await?;
    Ok(())
}

