/*
* wasmbuilder.rs
*
* functions that will compile rust code in the directories
* works with as many modules as are passed in
*/

// standard
use std::error::Error;
use std::process::Command;
// external
use tokio::task;
// local
use crate::config::{WasmModule};

// pass in all the modules you need to be compiled with wasm_pack
pub async fn compile_wasm_modules(
    modules: &[WasmModule],
) -> Result<(), Box<dyn Error>> {

    // make a list of all the modules to compile
    let mut compile_futures = Vec::new();
    for module in modules {
        // do we need this check?
        if module.compile_wasm {
            // get project directory from path
            let module_dir = module.path.display().to_string();
            println!("Queue compilation for {}: {}", module.id, module_dir);
            let future = Box::pin(build_wasm(module_dir));
            compile_futures.push((
                module.id.clone(), 
                future
            ));
        }
    }

    if compile_futures.is_empty() {
        println!("No WASM modules need compilation");
        return Ok(());
    }

    // separate ids and futures
    let (ids, futures): (Vec<_>, Vec<_>) = compile_futures.into_iter().unzip();

    let results = futures::future::join_all(futures).await;

    let mut failed = false;
    for (id, result) in ids.iter().zip(results.iter()) {
        match result {
            Ok(_) => println!("✅ {} compiled successfully", id),
            Err(err) => {
                eprintln!("❌ {} build failed: {}", id, err);
                failed = true;
            }
        }
    }

    if failed {
        return Err("One or more WASM builds failed".into());
    }

    println!("All WASM builds compiled successfully!");

    Ok(())
}

// build script for our wasm modules
//.env("RUSTFLAGS", "--cfg getrandom_backend=\"wasm_js\"")
// this line is for when we add random
// wasm-pack build --target no-modules
async fn build_wasm(
    dir: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Building WASM in {}", dir);

    let dir_copy = dir.clone();

    // spawn command in blocking task
    //let dir_owned = dir.to_string();
    let status = task::spawn_blocking(move || {
        Command::new("wasm-pack")
            //.current_dir(&dir_owned)
            .current_dir(&dir)
            .args([
                "build",
                "--target",
                "no-modules",
            ])
            .status()
    })
    .await?;

    if !status?.success() {
        //println!("HELO THERE!");
        return Err(format!("Failed to compiled WASM in {}", &dir_copy).into());
    }

    println!("WASM compiled in {}.", &dir_copy);
    Ok(())
}

