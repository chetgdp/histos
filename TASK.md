# TASK.md

llm sloppa

This file describes the current work. It is not a todo list. `todo` is the
only checklist: open work, roadmap, and done items. This file gives the
next session the context it needs.

Working mode: human-led pair programming under Data-Oriented Engineering
(DOE). The human writes core code and owns the system model. The assistant
maps code, reviews changes, drafts docs, and gives snippets to the human.

## Compression fallback warning

State: not started. The human writes this change.

`determine_compression_type` (`htmlpack/src/config.rs:278-286`) maps an
unrecognized `compression:` string to `CompressionType::None`. It prints
nothing. Decision: keep the `None` fallback. Print a warning to stderr that
shows the bad value and the fallback. There are two valid values:
`"brotli"` and `"none"`.

An omitted key takes a different path. `config.rs:254-256` calls
`.unwrap_or_default()`. The enum `#[default]` (`config.rs:44`) is `Brotli`.
The `WasmModule` default at `config.rs:137` is also `Brotli`. One default
has two sources of truth. They agree today.

Proposed match arm:

```rust
        other => {
            eprintln!("warning: unknown compression \"{other}\", using \"none\"");
            CompressionType::None
        }
```

After the change lands: add a row to the Problems table in
`htmlpack/rtfm.md`. Use the exact stderr text.

## Emscripten loader (core.js seam)

State: design not decided. This blocks the `runtime` and `wasm.<name>`
sections of `htmlpack/rtfm.md`. Those sections describe wasm-pack behavior
only.

The wasm-pack assumption is in `htmlpack/core/core.js`, not in the Rust
config. `loadApp` (`core.js:114-125`) hardcodes three values:

- `elementId: 'bin-wasm-app'`
- `shouldDecompress: true`
- `initFunction: wasm_bindgen`

`loadDecoder` (`core.js:100-110`, element `bin-wasm-decoder`) does not
depend on the glue. It takes bytes and returns bytes. Brotli does not
depend on the glue. The seam is the step after decode: which function
receives the bytes. wasm-bindgen needs `wasm_bindgen(bytes)`. Emscripten
needs `Module.wasmBinary = bytes` before the glue runs, or an
`instantiateWasm` hook.

Open decision: how `core.js` selects the init path. Candidates:

- a key in `runtime`
- a glue-type field on `wasm.<name>`
- one core file per target

None is chosen.

Related coupling: the compression default (`Brotli`) depends on
`runtime.decoder` defaulting to `true`. A bare config ships the decoder. If
compression defaults to `none` later, `decoder` must default to `false`
too. If not, the decoder is dead weight on first load.

`README.md:14` and the old `CRATES-IO.md` claimed first-class
`SINGLE_FILE=1` Emscripten support. `CRATES-IO.md` no longer claims it.
`README.md:14` still does.

## Docs state

Done this session:

- Moved `rtfm.md` to `htmlpack/rtfm.md` so `cargo publish` works.
  `packer.rs:57` reads it with `include_str!("../rtfm.md")`. Added it to
  the `Cargo.toml` include list. `histos rtfm` and `histos guide` print it.
- Rewrote the stable sections of `htmlpack/rtfm.md` in Simplified
  Technical English: CLI (`histos.yaml` default, `-o`, `-w`, `rtfm`),
  workflow, schema (`apple_icon`, first icon only), Problems table keyed
  to `error.rs` text.
- Rewrote `htmlpack/CRATES-IO.md`: correct repo link, license
  `MIT OR Apache-2.0`, library example that compiles against the current
  re-exports.
- Added `AssetSource` and `load_config` to the `htmlpack/src/lib.rs`
  re-exports.
- Added `examples.git-backup-20260818/` and `repo-meta-backup.json` to
  `.gitignore`.

Still stale:

- `README.md:14` Emscripten claim.
- `htmlpack/rtfm.md` sections `pack.runtime` and `pack.wasm.<name>`. Wait
  for the loader decision. Then rewrite them.

Style rules for all docs: active voice, one instruction per sentence,
capital H only at the start of a sentence, no em dashes, `histos` in
lowercase in code and filenames.
