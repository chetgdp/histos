# TASK.md

llm sloppa

This file describes the current work. It is not a todo list. `todo` is the
only checklist: open work, roadmap, and done items. This file gives the
next session the context it needs.

Working mode: human-led pair programming under Data-Oriented Engineering
(DOE). The human writes core code and owns the system model. The assistant
maps code, reviews changes, drafts docs, and gives snippets to the human.

## Emscripten loader (core.js seam)

State: design not decided. This blocks the `runtime` and `wasm.<name>`
sections of `htmlpack/rtfm.md`. Those sections describe wasm-pack behavior
only.

`loadApp` (`htmlpack/core/core.js:116-127`) hardcodes
`elementId: 'bin-wasm-app'` and `initFunction: wasm_bindgen`. The seam is
the step after decode: which function receives the bytes. wasm-bindgen
needs `wasm_bindgen(bytes)`. Emscripten needs `Module.wasmBinary = bytes`
before the glue runs, or an `instantiateWasm` hook.

Precedent: compression already crosses from Rust to core.js as an
attribute. `render.rs` writes `compression="brotli|none"` on each wasm
`<pre>`. `core.js:88` loads the decoder only when the app is brotli.
`core.js:194` reads the attribute to decide decompression. A `glue`
attribute on the `<pre>`, set from a field on `wasm.<name>`, would select
`initFunction` the same way. Not decided.

`README.md:14` still claims first-class `SINGLE_FILE=1` Emscripten support.

## Compression and runtime state

- Omitted `compression:` defaults to `brotli` from `WasmModule::default()`
  (`config.rs`). The enum has no `Default`.
- An invalid value prints
  `warning: invalid compression type "<value>" detected, defaulting to "none"`
  and packs uncompressed.
- `brotli` with `runtime.decoder: false` (runtime enabled, core on) fails
  in `PackConfig::build()` with `ConfigError::DecoderRequired`.
- `compression: none` with `decoder: true` works but ships an unused
  decoder.
- Body order is html, scripts, wasm `<pre>`.
- The CLI prints errors with Display and a `caused by:` chain, exit code 1.

Style rules for all docs: active voice, one instruction per sentence,
capital H only at the start of a sentence, no em dashes, `histos` in
lowercase in code and filenames.
