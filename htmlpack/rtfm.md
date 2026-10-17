# histos

Histos packs a web app into one `index.html` file. The file contains the WASM, CSS, JS, and HTML. The output is plain HTML. Put it on any static web server, send it as a file, or open it from `file://`. Histos inlines each asset it packs, including URL assets, at build time. Your app decides what it does at load time.

How histos packs each asset:

- WASM: histos compresses it with brotli, encodes it as base64, and puts it in a `<pre>` tag. A bundled runtime decodes it at load time and caches it in IndexedDB.
- CSS: histos joins all files into one `<style>` tag.
- JS and HTML fragments: histos inlines them as-is.
- Favicon: histos encodes the SVG as base64.

## Install

```bash
cargo install histos
```

You need Rust and Cargo. You need `wasm-pack` only when you set `compile_wasm: true`.

## CLI

```bash
histos                                          # reads ./histos.yaml, writes ./index.html
histos config.yaml                              # explicit config path
histos config.yaml --output ./dist/index.html   # or -o
histos config.yaml --watch                      # or -w: rebuild when a local asset changes
histos rtfm                                     # print this manual (alias: histos guide)
```

Watch mode watches local paths only. Histos fetches URL assets once per build.

## Workflow

1. If `histos.yaml` exists, read it first.
2. Find what you pack:
   - WASM from `wasm-pack`: set `path:` to the project root (the parent of `pkg/`).
   - Pre-built `.wasm` and `.js`: set `binary:` and `glue:`.
   - No WASM: omit `wasm:` and set `runtime.enabled: false`.
   - CSS, JS, HTML fragments, SVG favicon, PNG apple icon, metadata: local paths or `https://` URLs.
3. Write `histos.yaml` from the schema below. Omit optional keys that have no value. Do not write empty values or `null`. Replace each placeholder path with a real path.
4. Check the config before you run histos:
   - WASM with the default runtime: `id` is `"bin-wasm-app"`.
   - `compression: "brotli"`: `runtime.decoder` is `true` or omitted.
   - `path:` set: `wasm-pack build --target no-modules` has run and `pkg/` exists.
   - `compile_wasm: true`: `wasm-pack` is on PATH.
   - Each local path exists, relative to the directory where you run histos.
5. Run `histos`.
6. Check the output. A file on disk does not show that the app loads. Open `index.html` in a browser from `file://`. Open the console. Confirm there are no errors. Confirm the app content is visible. Then report the path and size (`ls -lh index.html`).

## Config schema

The config has one top-level key, `pack`. All sub-keys are optional.

```
pack:
  runtime:    RuntimeConfig             # which runtime assets histos injects
  metadata:   MetadataConfig            # <head> metadata
  favicon:    List[String]              # SVG paths or URLs; histos uses the first only
  apple_icon: List[String]              # PNG paths or URLs; histos uses the first only
  css:        List[String]              # joined in order
  html:       List[String]              # inlined in order
  scripts:    List[String]              # inlined in order
  wasm:       Map[String -> WasmModule] # named WASM modules
```

Histos fetches each string that starts with `http://` or `https://`. Histos reads each other string as a local path. Local paths are relative to the directory where you run histos, not to the config file. `wasm.<name>.path` accepts a local path only.

### `pack.runtime`

| Key       | Type | Default | Description |
|-----------|------|---------|-------------|
| `enabled` | bool | `true`  | Master switch. `false` overrides the other keys. |
| `icon`    | bool | `true`  | Inject the default SVG loading icon. |
| `core`    | bool | `true`  | Inject `core.js`. It finds and runs the WASM module. |
| `decoder` | bool | `true`  | Inject the brotli decoder. Required for `compression: "brotli"`. |

### `pack.metadata`

All keys are optional strings. `title` defaults to `"histos"`.

| Key           | Emits |
|---------------|-------|
| `title`       | `<title>` and `<meta name="title">` |
| `author`      | `<meta name="author">` |
| `description` | `<meta name="description">` |
| `keywords`    | `<meta name="keywords">` |

### `pack.favicon`, `pack.apple_icon`, `pack.css`, `pack.html`, `pack.scripts`

Lists of paths or URLs.

```yaml
favicon:
  - ./assets/icon.svg
apple_icon:
  - ./assets/icon-180.png
css:
  - https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css
  - ./styles/main.css
html:
  - ./fragments/canvas.html
scripts:
  - ./src/init.js
```

`favicon` must be SVG. Histos writes it as `<link rel="icon" type="image/svg+xml">`. `apple_icon` must be PNG. Histos writes it as `<link rel="apple-touch-icon" type="image/png">`. Both use base64 data URLs. Histos ignores extra entries in both lists.

The body order is fixed: WASM `<pre>` blocks, then `scripts`, then `html` fragments. Scripts run before the fragment elements exist. Wrap DOM access in `DOMContentLoaded`, or put the element setup in a `<script>` inside the fragment.

### `pack.wasm.<name>`

The map key is a label for the YAML only. The runtime uses `id`.

| Key            | Type   | Default          | Description |
|----------------|--------|------------------|-------------|
| `path`         | String |                  | wasm-pack project root (the parent of `pkg/`). Histos finds `binary` and `glue` in `<path>/pkg/`. Do not set with `binary` or `glue`. |
| `binary`       | String |                  | Path or URL to the `.wasm`. Used when `path` is not set. |
| `glue`         | String |                  | Path or URL to the JS glue. Used when `path` is not set. |
| `id`           | String | `"bin-wasm-app"` | Must be `"bin-wasm-app"` when `runtime.core` is `true`. `core.js` looks up this id. |
| `compile_wasm` | bool   | `false`          | Run `wasm-pack build --target no-modules` before packing. |
| `compression`  | String | `"brotli"`       | `"brotli"` or `"none"`. Set `"none"` for a binary that is already compressed. |

## Examples

### No WASM

```yaml
pack:
  runtime:
    enabled: false
  metadata:
    title: "My App"
    description: "Static app packed by histos"
  css:
    - ./styles/main.css
  scripts:
    - ./src/app.js
```

### WASM, wasm-pack project directory

```yaml
pack:
  metadata:
    title: "My WASM App"
    author: "Your Name"
  favicon:
    - ./assets/icon.svg
  css:
    - ./styles/app.css
  wasm:
    app:
      path: ./my-wasm-project
      id: "bin-wasm-app"
      compile_wasm: false
      compression: "brotli"
```

### WASM, explicit files

```yaml
pack:
  wasm:
    app:
      binary: ./my-wasm-project/pkg/my_app_bg.wasm
      glue: ./my-wasm-project/pkg/my_app.js
      id: "bin-wasm-app"
      compression: "brotli"
```

## Problems

| Problem | Fix |
|---------|-----|
| WASM does not load | Set `id` to `"bin-wasm-app"` when `runtime.core` is `true`. |
| Brotli decoder missing | Keep `runtime.decoder: true` when `compression: "brotli"`. Or set `compression: "none"`. |
| `wasm-pack` not found | Run `cargo install wasm-pack`. Or set `compile_wasm: false` and use a pre-built `pkg/`. |
| `pkg directory not found: <path>` | Run `wasm-pack build --target no-modules` in the WASM project first. |
| `missing required file in pkg: _bg.wasm` or `.js` | `pkg/` exists but is incomplete. Rebuild with `wasm-pack`. Or set `binary` and `glue` to the files directly. |
| `path` and `binary`/`glue` both set | Set one or the other per module. |
| Key typo | The key is `metadata:`, not `meta:`. |
| Paths do not resolve | Local paths are relative to the directory where you run histos. |
| Double compression | Set `compression: "none"` for WASM that is already compressed. |
| Wrong binary name | The installed binary is `histos`. |
| Config not found with no args | The default config path is `./histos.yaml`. Pass the path if it differs. |
| Favicon does not show | Histos uses the first `favicon` entry only. It must be SVG. `.ico` and PNG are not supported. |
| Script sees `null` elements | Scripts run before `html` fragments. Wait for `DOMContentLoaded`, or move the DOM setup into the fragment's own `<script>`. |
