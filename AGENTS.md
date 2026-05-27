# AGENTS.md

This file provides guidance to agentic coding agents when working with this Rust codebase.

## Build Commands

```bash
# From htmlpack/ directory
cargo build                  # Build project
cargo build --release        # Release build
cargo test                   # Run all tests
cargo test <test_name>       # Run single test by name
```

## Run Commands

```bash
# Run CLI with config file
cargo run -- config.yaml
cargo run -- config.yaml --output ./dist/index.html

# Or install binary locally
cargo install --path .       # Installs to ~/.cargo/bin/histos
histos config.yaml           # Run installed binary
```

## Code Style Guidelines

### Imports
- Standard library first: `use std::path::PathBuf;`
- External crates second: `use clap::Parser;`
- Local modules last: `use crate::config::PackConfig;`
- Use `// standard`, `// external`, `// local` comments to separate sections

### Formatting
- 4-space indentation
- Line length: max 100 characters (preferably shorter)
- Blank lines between logical sections (imports, impl blocks, functions)

### Naming Conventions
- Modules: `mod packer`, `mod render` (lowercase)
- Structs: `PackConfig`, `RuntimeConfig`, `WasmModule` (PascalCase)
- Enums: `AssetSource`, `CompressionType` (PascalCase)
- Functions: `load_config()`, `process_text()` (snake_case)
- Constants: `DEFAULT_WASM_ID` (UPPER_SNAKE_CASE)
- Files: `cli.rs`, `config.rs`, `error.rs` (lowercase)

### Error Handling
- Use `thiserror` for structured error types
- Define error hierarchy: `HistosError` → `ConfigError`, `CompileError`, `FetchError`, `EncodeError`, `SaveError`
- Public functions return `HistosResult<T>` (alias for `Result<T, HistosError>`)
- Use `?` operator for error propagation
- Document error cases in doc comments with `# Errors` section

### Comments
- Use `//` for single-line comments
- Use `/* */` for multi-line comments at file/module level
- Add doc comments (`///`) for public API
- Include `# Errors` and `# Examples` sections in function docs
- Avoid unnecessary comments; code should be self-explanatory

### Type Annotations
- Use explicit types on function parameters and return values
- Use `Option<T>` for nullable fields in YAML structs
- Use `Result<T, E>` for fallible operations
- Avoid `impl Trait` in public API; prefer concrete types

### Documentation
- Every public function should have `///` doc comments
- Include `# Examples` section with code examples (use ` ``` ` or ` ```no_run `)
- Document when functions are infallible vs fallible
- Reference related types and error types in docs

### Testing
- Tests go in module files or `tests/` directory
- Use `#[test]` attribute for unit tests
- Test error cases explicitly
- Use `#[ignore]` for integration tests that require external setup

### Special Notes
- `CLAUDE.md` contains architecture docs; ignore `src_cpy/` (stale backup)
- YAML config uses `metadata:` (not `meta:`)
- Local asset paths resolve relative to CWD, not config file location
- wasm-pack must be installed to compile wasm modules