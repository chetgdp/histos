/*
* wasmbuilder.rs
*
* functions that will compile rust code in the directories
* works with as many modules as are passed in
*/

// standard
use std::process::Command;
// external
use tokio::task;
// local
use crate::config::WasmModule;
use crate::error::{HistosResult, CompileError};

// pass in all the modules you need to be compiled with wasm_pack
pub async fn compile_wasm_modules(
    modules: &[WasmModule],
) -> HistosResult<()> {

    // make a list of all the modules to compile
    let mut compile_futures = Vec::new();
    for module in modules {
        // do we need this check?
        if module.compile_wasm {
            // get project directory from path
            let module_dir = module.path.display().to_string();
            eprintln!("Queue compilation for {}: {}", module.id, module_dir);
            let future = Box::pin(build_wasm(module_dir));
            compile_futures.push((
                module.id.clone(), 
                future
            ));
        }
    }

    if compile_futures.is_empty() {
        eprintln!("No WASM modules need compilation");
        return Ok(());
    }

    // separate ids and futures
    let (ids, futures): (Vec<_>, Vec<_>) = compile_futures.into_iter().unzip();

    let results = futures::future::join_all(futures).await;

    let mut first_err: Option<CompileError> = None;
    for (id, result) in ids.into_iter().zip(results.into_iter()) {
        match result {
            Ok(_) => eprintln!("✅ {} compiled successfully", id),
            Err(err) => {
                eprintln!("❌ {} build failed: {}", id, err);
                if first_err.is_none() {
                    first_err = Some(err);
                }
            }
        }
    }

    if let Some(err) = first_err {
        return Err(err.into());
    }

    eprintln!("All WASM builds compiled successfully!");

    Ok(())
}

// build script for our wasm modules
//.env("RUSTFLAGS", "--cfg getrandom_backend=\"wasm_js\"")
// this line is for when we add random
// wasm-pack build --target no-modules
async fn build_wasm(dir: String) -> Result<(), CompileError> {
    eprintln!("Building WASM in {}", dir);

    let dir_copy = dir.clone();

    let join_result = task::spawn_blocking(move || {
        Command::new("wasm-pack")
            .current_dir(&dir)
            .args(["build", "--target", "no-modules"])
            .status()
    })
    .await
    .map_err(CompileError::TaskJoin)?;

    let exit_status = join_result.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            CompileError::WasmPackNotFound
        } else {
            CompileError::ProcessSpawn(e)
        }
    })?;

    if !exit_status.success() {
        return Err(CompileError::BuildFailed { dir: dir_copy });
    }

    eprintln!("WASM compiled in {}.", &dir_copy);
    Ok(())
}

