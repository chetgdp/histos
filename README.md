# Histos
weave all web dependencies into a single html file

a new form of offline, OS agnostic, browser based application

> *put the wasm in the base64 lil bro*

enabling high performance, portable wasm applications 

inlines brotli compressed + base64 encoded wasm inside `<pre>` tags

uses a default runtime decoder to unpack itself

first class wasm-pack support for **Rust** and `SINGLE_FILE=1` for **Emscripten/C**.

## demos
- [paleomap3d](https://afnleaf.github.io/planet) *requires webgpu*
- [nousbase](https://chetgdp.github.io/notes) *rust markdown ssg*
- [pokepack](https://afnleaf.github.io/pokepack) *bitpacking format*
- [a 2d game](https://afnleaf.github.io/mecha) *raylib*
- [MCXWord](https://chetgdp.github.io/crossword) *js*
- [DOOM.html](https://chetgdp.github.io/doom) *C+js*

## examples
for v0.0.2
- hello world examples of code
- Rust
    - basic
    - [blogpost](https://chetgdp.github.io/notes#/TECH/README.md)
    - [paleomap 3d: bevy](https://chetgdp.github.io/planet)
- C
    - basic
    - [game](https://afnleaf.github.io/mecha)
- Vanilla 
    - three.js
        - [pixel.ai](https://chetgdp.github.io/pixel.ai) 
    - [Minecraft Crossword](https://chetgdp.github.io/crossword)
    - calculator

### [RTFM](htmlpack/rtfm.md)
`histos rtfm` prints the same manual.
### [DEVLOG](DEVLOG.md)

## resources
What helped build the data transformation pipeline
- [utf-8](https://en.wikipedia.org/wiki/UTF-8)
- [unicode table](https://www.utf8-chartable.de/)
- [base64](https://en.wikipedia.org/wiki/Base64)
- [base94](https://vorakl.com/articles/base94/)
- [binary conversion](https://vorakl.com/articles/stream-encoding/)
- ~~[base94 - py](https://github.com/vorakl/base94)~~
- ~~[base94 - C](https://gist.github.com/iso2022jp/4054241) 👀~~
- [base122](https://github.com/kevinAlbs/Base122)
- [brotli](https://github.com/google/brotli)

## todo
Roadmap, open work, and implemented features: [todo](todo)

## compile
wasm: `wasm-pack build --target no-modules`

## notes
what computer you compile on will leak bevy crate stuff, how to prevent?

```js
const wasmModule = await WebAssembly.compile(wasmBytes);
wasmBytes = wasmModule;

//Error:  DataCloneError: Failed to execute 'put' on 'IDBObjectStore': A WebAssembly.Module can not be serialized for storage.
```
- browser protects against caching compiled output
- would cause errors if wasm runtime and cached output mismatch

## local install
```
// this installs the binary to ~/.cargo/bin
cargo install --path .

// update
cargo install --path . --force

// allows you to run with 
histos --help
histos config.yaml

// check version
which histos // unix/macos
where histos // windows

// uninstall
cargo uninstall histos

// run without install
cargo run -- config.yaml
```
---

