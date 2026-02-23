# histos

Pack any web app WASM(Rust/C), HTML, CSS, JS,  into a single self-contained `.html` file.

No server. No CDN. No internet required at runtime. Open directly in a browser.

First class wasm-pack support for **Rust** and `SINGLE_FILE=1` for **Emscripten/C**.

## [repo](https://github.com/?/histos)

---

## How it works

histos reads a YAML config describing your web assets and weaves together one portable `index.html`:

- WASM binaries are brotli-compressed, base64-encoded, and stored in `<pre>` tags
- A bundled runtime decoder unpacks the WASM at load time
- Decoded WASM is cached in IndexedDB — subsequent loads are instant
- CSS files are fetched and inlined as a single `<style>` block
- JS and HTML fragments are inlined verbatim
- Favicons are base64-encoded SVGs

The result is a fully offline, browser-runnable application in one file.

---

## Install

```bash
cargo install histos
```

To compile WASM from source during packing, also install `wasm-pack`:

```bash
cargo install wasm-pack
```

---

## Quick start

```yaml
# config.yaml
pack:
  metadata:
    title: "My App"
  css:
    - ./styles/main.css
  runtime:
    enabled: false
```

```bash
histos config.yaml
histos config.yaml --output ./dist/index.html
```

---

## WASM app example

```yaml
pack:
  metadata:
    title: "My WASM App"
    author: "Jane Doe"
    description: "A Rust/WASM app packed by histos"

  favicon:
    - ./assets/icon.svg

  css:
    - https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css
    - ./styles/app.css

  wasm:
    app:
      path: ./my-wasm-project        # parent of pkg/
      id: "bin-wasm-app"             # must match default runtime expectation
      compile_wasm: false            # set true to run wasm-pack automatically
      compression: "brotli"
```

---

## Config reference

| Key | Description |
|-----|-------------|
| `pack.runtime` | Controls which runtime components are injected (decoder, loader, icon) |
| `pack.metadata` | HTML `<head>` metadata: title, author, description, keywords |
| `pack.favicon` | List of SVG paths or URLs — base64-encoded into `<link rel="icon">` |
| `pack.css` | List of CSS paths or URLs — concatenated into a single `<style>` block |
| `pack.html` | List of HTML fragment paths or URLs — inlined into the body |
| `pack.scripts` | List of JS paths or URLs — inlined inside `<script>` tags |
| `pack.wasm.<name>` | WASM module: `path`, `binary`, `glue`, `id`, `compile_wasm`, `compression` |

Full schema with examples: [`llms.txt`](https://github.com/ironhands/histos/blob/main/llms.txt)

---

## Common pitfalls

- **`id` must be `"bin-wasm-app"`** when using the default runtime — `core.js` looks for this exact ID
- **`compression: "brotli"` requires `runtime.decoder: true`** (the default) — or the WASM can't be decoded
- **`metadata:` not `meta:`** — the YAML key is `metadata`
- **Paths are relative to CWD** where `histos` is invoked, not the config file location

---

## Library API

histos is also usable as a library crate for programmatic HTML packing:

```rust
use histos::{PackConfig, HistosResult};

#[tokio::main]
async fn main() -> HistosResult<()> {
    let mut config = PackConfig::new();
    config.set_metadata("My App", "Jane Doe", "An offline WASM app", "");
    config.add_style("https://example.com/style.css");

    let packed = config.build().await?;
    packed.save_to_file("index.html").await?;
    Ok(())
}
```

---

License: MIT
