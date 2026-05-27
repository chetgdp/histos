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

## examples
for v0.0.2
- [] hello world examples of code
- Rust
    - basic
    - [blogpost](https://chetgdp.github.io/notes#/TECH/README.md)
    - [paleomap 3d - bevy]
- C
    - basic
    - [game](https://afnleaf.github.io/mecha)
- Vanilla 
    - three.js
        - [pixel.ai](https://afnleaf.github.io/pixel.ai) 
        - [three.wasm demo]()
    - calculator

### [config reference](llms.txt)
### [SKILL.md](SKILL.md)
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

## implemenations
With these current implemented features, we have a solid backbone.
- [x] base64 encoding
- [x] brotli compression
- [x] wasm simple
- [x] wasm-bindgen
- [x] wasm canvas
- [x] favicon svg
- [x] custom html
- [x] css
- [x] text
- [x] png
- [x] metadata
- [x] default runtime
- [x] loading screen
- [x] indexedDB during first time load, cache wasm_modules as u8
- [x] basic cli tool
- [x] single wasm_modules path

### paleomap3d specific
Paleomap3d is the main demo for this project. Ensuring Bevy ECS with WebGL and WebGPU render backends is able to work.
- [x] wasm bevy engine
- [x] webgl
- [x] webgpu
- [x] encode textures
- [x] encode models/meshes/3d
- [x] fps counter
- [x] instanced mesh custom render shader pipeline

## todo: roadmap

### priority
- [x] more ergonomic default runtime
- [x] basic errors and propagation
- [x] library api
- [x] test suite
- [x] LLM integration and tool calling
- [x] verbose description of what is happening
- [x] basic documentation
- [] license (who is releasing this software?)
- **v0.1.0 gap**
- [ ] proper errors and propagation
- [ ] working examples
- [ ] build cache
- [ ] signing for security
- [ ] watch/dev hot reload mode
- [ ] base122 (in rust)
- [ ] mini http server variation
- [ ] reproducible builds/deterministic output
- [ ] json manifest block
- [ ] check signing
- [] backwards compatability with browsers

### nice to have
- [ ] support multiple favicon types
- [ ] lazy loading
- [ ] wasm advanced
- [ ] advanced cli options
- [ ] auto webgpu turn on -> webgl2 fallback (storage buffer = bad)
- [ ] error on webgpu not found
- [ ] single wasm-binary self loading
- [ ] output profiles
- [ ] template html?
- [ ] simple asset loader for textures
- [ ] asset loader for models
- [ ] basis universal 
- [ ] encode fonts
- [ ] big png
- [ ] new animation
- [ ] basic game

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
pithos --help
pithos config.yaml

// check version
which pithos // unix/macos
where pithos // windows

// uninstall
cargo uninstall pithos

// run without install
cargo run -- config.yaml
```
---

