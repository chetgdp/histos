# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run without installing (from htmlpack/)
cargo run -- config.yaml
cargo run -- config.yaml --output ./dist/index.html

# Install binary locally
cargo install --path .          # installs to ~/.cargo/bin/histos
cargo install --path . --force  # update

# Run installed binary
histos config.yaml
histos config.yaml --output ./dist/index.html

# Build wasm decoder (from decoder/)
wasm-pack build --target no-modules

# Tests (from htmlpack/)
cargo test
cargo test <test_name>   # run a single test by name
```

The binary name is `histos`. The package/crate name is also `histos` (see `htmlpack/Cargo.toml`).

## Architecture

The project is split into two crates:
- **`htmlpack/`** — the main Rust library + CLI binary (`src/`)
- **`decoder/`** — a separate wasm-pack crate that produces the runtime brotli decoder (compiled output is checked in at `htmlpack/core/`)

`src_cpy/` is a stale backup copy — ignore it.

### Data pipeline

```
CLI args (Cli) → YAML file → YamlRoot/YamlPack (serde structs in cli.rs)
    → PackConfig (via From impls in config.rs)
    → PackConfig::build() in packer.rs
        → wasmbuilder: compile wasm with wasm-pack (optional)
        → fetcher: fetch all AssetSources (local file or HTTP)
        → encoder: brotli compress + base64 encode wasm; base64 encode icons
        → inject default runtime (core.js, wasm_decoder.js, wasm_decoder_bg.wasm)
    → HtmlDoc (in html.rs)
    → HtmlDoc::render() → PackedHtml (in render.rs, using maud templates)
    → PackedHtml::save_to_file()
```

### Two-level config pattern

There are two config representations:
1. **Yaml structs** (`cli.rs`): `YamlRoot`, `YamlPack`, `YamlRuntime`, etc. — all fields `Option<>`, serde-deserialized from YAML
2. **Working config** (`config.rs`): `PackConfig`, `RuntimeConfig`, `MetadataConfig`, `WasmModule` — concrete values with defaults, converted via `impl TryFrom<YamlX> for ConfigX` (fallible because `pkg` directory resolution can fail)

`AssetSource` (in `config.rs`) is the key enum distinguishing local paths from remote URLs — determined at config-load time from string prefix.

### Runtime assets

`packer.rs` embeds the default runtime via `include_str!`/`include_bytes!` from `htmlpack/core/`: `icon.svg`, `core.js`, `wasm_decoder.js`, `wasm_decoder_bg.wasm`. These get injected into the output HTML when `runtime.enabled = true`.

### Error handling

`src/error.rs` defines a structured error hierarchy using `thiserror`:
- `HistosError` — top-level enum with `From` impls for each sub-error
- `ConfigError`, `CompileError`, `FetchError`, `EncodeError`, `SaveError` — typed sub-errors
- `HistosResult<T>` — alias for `Result<T, HistosError>`

All public functions return `HistosResult<T>`. Sub-errors convert into `HistosError` via `?`. `CompileError` variants (`WasmPackNotFound`, `BuildFailed`) are defined but not covered by the test suite since they require spawning wasm-pack processes.

### Public API

`PackConfig` has a builder API (`new()`, `set_metadata()`, `add_style()`, `add_wasm_pkg()`, etc.) allowing programmatic use without YAML. `WasmModule` also has builder methods (`from_pkg()`, `set_manually()`, `with_id()`, etc.).

### Config gotchas

- YAML key for metadata is **`metadata:`**, not `meta:` — the serde field name matches the Rust struct.
- `pack.wasm` is a `HashMap<String, YamlWasmModule>` — the map key (e.g. `module`, `app`) is arbitrary and ignored at runtime; only `id` matters.
- `id` defaults to `"bin-wasm-app"` (`DEFAULT_WASM_ID` in `config.rs`). The default runtime's `core.js` looks for this exact ID — changing it breaks loading unless the user supplies a custom runtime.
- `compression` accepts `"brotli"` or `"none"`; any other string also maps to `None` (see `determine_compression_type` in `config.rs`).
- Local asset paths in the config are resolved relative to the **CWD where `histos` is invoked**, not relative to the config file.

### Docs layout

- `README.md` — project overview and user-facing intro
- `DEVLOG.md` — dev journal / project history, not authoritative for config syntax or CLI usage
- `llms.txt` — canonical config reference (schema, examples, pitfalls); also machine-readable for LLMs (llmstxt.org format)
- `htmlpack/CRATES-IO.md` — crates.io page (`readme = "CRATES-IO.md"` in `htmlpack/Cargo.toml`)
- `SKILL.md` — Claude Code skill invoked via `/histos`; guides generating a correct `config.yaml` and running the CLI

### Publish gotcha

`htmlpack/Cargo.toml` has an explicit `include` array. `CRATES-IO.md` is not in it — add it before publishing or crates.io will show a blank readme.
