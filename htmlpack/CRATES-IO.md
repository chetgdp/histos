# histos

Histos packs a web app into one `index.html` file. The file contains the WASM, CSS, JS, and HTML. The output is plain HTML. Put it on any static web server, send it as a file, or open it from `file://`.

Histos has first-class support for Rust via `wasm-pack`.

Repo: <https://github.com/chetgdp/histos>

## How it works

Histos reads a YAML config and weaves the listed assets into one file:

- WASM: histos compresses it with brotli, encodes it as base64, and puts it in a `<pre>` tag. A bundled runtime decodes it at load time and caches it in IndexedDB.
- CSS: histos joins all files into one `<style>` tag.
- JS and HTML fragments: histos inlines them as-is.
- Favicon: histos encodes the SVG as base64.

Histos inlines each asset it packs, including URL assets, at build time.

## Install

```bash
cargo install histos
```

You need `wasm-pack` only when you set `compile_wasm: true`:

```bash
cargo install wasm-pack
```

## Quick start

```yaml
# histos.yaml
pack:
  runtime:
    enabled: false
  metadata:
    title: "My App"
  css:
    - ./styles/main.css
```

```bash
histos                                  # reads ./histos.yaml, writes ./index.html
histos config.yaml -o ./dist/index.html # explicit paths
histos -w                               # rebuild when a local asset changes
histos rtfm                             # print the full manual
```

## WASM app example

```yaml
pack:
  metadata:
    title: "My WASM App"
    author: "Jane Doe"
  favicon:
    - ./assets/icon.svg
  css:
    - https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css
    - ./styles/app.css
  wasm:
    app:
      path: ./my-wasm-project   # parent of pkg/
      id: "bin-wasm-app"        # the default runtime looks up this id
      compile_wasm: false       # true runs wasm-pack for you
      compression: "brotli"
```

## Config reference

| Key | Description |
|-----|-------------|
| `pack.runtime` | Which runtime parts histos injects: loader, decoder, loading icon |
| `pack.metadata` | `<head>` metadata: title, author, description, keywords |
| `pack.favicon` | SVG paths or URLs. Histos uses the first as `<link rel="icon">` |
| `pack.apple_icon` | PNG paths or URLs. Histos uses the first as `<link rel="apple-touch-icon">` |
| `pack.css` | CSS paths or URLs. Joined into one `<style>` block |
| `pack.html` | HTML fragment paths or URLs. Inlined into the body |
| `pack.scripts` | JS paths or URLs. Inlined in `<script>` tags |
| `pack.wasm.<name>` | WASM module: `path`, `binary`, `glue`, `id`, `compile_wasm`, `compression` |

Full schema, workflow, and problem table: run `histos rtfm`, or read [rtfm.md](https://github.com/chetgdp/histos/blob/main/htmlpack/rtfm.md).

## Common problems

- Set `id` to `"bin-wasm-app"` when you use the default runtime. `core.js` looks up this exact id.
- Keep `runtime.decoder: true` (the default) when `compression: "brotli"`. Without it the WASM cannot decode.
- The YAML key is `metadata:`, not `meta:`.
- Local paths are relative to the directory where you run histos, not to the config file.

## Library API

Histos is also a library crate.

```rust
use std::path::Path;
use histos::{PackConfig, HistosResult, render_to_packed};

#[tokio::main]
async fn main() -> HistosResult<()> {
    let config = PackConfig::new()
        .set_metadata("My App".into(), "Jane Doe".into(), "An offline WASM app".into(), "".into())
        .add_style("https://example.com/style.css".into());

    let doc = config.build().await?;
    render_to_packed(doc).save_to_file(Path::new("index.html"))?;
    Ok(())
}
```

`histos::load_config(path)` reads a YAML file into a `PackConfig`.

## License

MIT OR Apache-2.0
