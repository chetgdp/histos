# histos Skill
You are helping the user pack their web app into a single self-contained HTML file using **histos**.

histos reads a YAML config and produces one `index.html` with all assets inlined: CSS concatenated, JS inlined, WASM brotli-compressed and base64-encoded, favicons base64-encoded SVGs. The output runs offline, directly from the filesystem, with no server.

---

## Step 1: Understand What They're Packing

Ask (or infer from context):

1. **Do they have a WASM module?**
   - Built with `wasm-pack`? → use `path:` pointing to the project root (parent of `pkg/`)
   - Pre-built `.wasm` + `.js` files? → use `binary:` and `glue:` explicitly
   - No WASM → skip the `wasm:` section entirely

2. **Do they have custom CSS?** Local files or CDN URLs?

3. **Do they have custom JS?** (non-WASM scripts)

4. **Do they have an SVG favicon?**

5. **Do they want metadata?** (title, author, description)

6. **Do they already have a `config.yaml`?** If yes, read it first before generating anything.

---

## Step 2: Generate `config.yaml`

Use the schema below to write a correct config. Do not include keys with no value — omit optional fields entirely rather than leaving them empty or null.

### Schema Reference

```yaml
pack:
  runtime:
    enabled: true       # bool, default true — master switch for runtime injection
    icon: true          # bool, default true — inject loading icon SVG
    core: true          # bool, default true — inject core.js loader
    decoder: true       # bool, default true — inject brotli WASM decoder

  metadata:
    title: "string"       # optional, default "histos"
    author: "string"      # optional
    description: "string" # optional
    keywords: "string"    # optional

  favicon:              # list of local paths or https:// URLs to SVG files
    - ./path/to/icon.svg

  css:                  # list of local paths or https:// URLs to CSS files
    - ./styles/main.css
    - https://example.com/normalize.css

  html:                 # list of local paths or https:// URLs to HTML fragments
    - ./fragments/canvas.html

  scripts:              # list of local paths or https:// URLs to JS files
    - ./src/init.js

  wasm:
    <name>:             # map key is arbitrary (e.g. "app", "module") — just for YAML
      path: ./my-project      # path to wasm-pack project root (parent of pkg/)
                              # OR use binary: + glue: instead:
      binary: ./pkg/app_bg.wasm
      glue: ./pkg/app.js
      id: "bin-wasm-app"      # MUST be "bin-wasm-app" with default runtime
      compile_wasm: false     # true = run wasm-pack build before packing
      compression: "brotli"   # "brotli" (default) or "none"
```

### WASM Paths: `path` vs `binary`/`glue`

- Use **`path`** when you have a wasm-pack project directory. histos auto-discovers `*_bg.wasm` and `*.js` inside `<path>/pkg/`.
- Use **`binary` + `glue`** when you have specific file paths or remote URLs.
- Do not combine both for the same module.

---

## Templates

### JS/CSS only (no WASM)

```yaml
pack:
  runtime:
    enabled: false    # no runtime needed without WASM

  metadata:
    title: "My App"
    description: "Static app packed by histos"

  css:
    - ./styles/main.css

  scripts:
    - ./src/app.js
```

### WASM app (wasm-pack project directory)

```yaml
pack:
  runtime:
    enabled: true
    icon: true
    core: true
    decoder: true

  metadata:
    title: "My WASM App"
    author: "Your Name"
    description: "Packed by histos"

  favicon:
    - ./assets/icon.svg

  css:
    - ./styles/app.css

  wasm:
    app:
      path: ./my-wasm-project    # parent of pkg/; run wasm-pack build here first
      id: "bin-wasm-app"         # must match exactly when using default runtime
      compile_wasm: false
      compression: "brotli"
```

### WASM app (explicit file paths)

```yaml
pack:
  runtime:
    enabled: true

  wasm:
    app:
      binary: ./my-wasm-project/pkg/my_app_bg.wasm
      glue: ./my-wasm-project/pkg/my_app.js
      id: "bin-wasm-app"
      compression: "brotli"
```

---

## Step 3: Run histos

```bash
# Default output: ./index.html
histos config.yaml

# Custom output path
histos config.yaml --output ./dist/index.html
```

If histos is not installed:
```bash
# Install from crate root (htmlpack/)
cargo install --path .

# Or run without installing (from htmlpack/)
cargo run -- config.yaml
```

If `compile_wasm: true` is set and `wasm-pack` is missing:
```bash
cargo install wasm-pack
```

---

## Step 4: Report Results

After running, show the user:
- The output file path
- The file size (`ls -lh ./index.html`)
- Confirm it's self-contained: "Open this file directly in a browser — no server needed."

---

## Common Gotchas

| Problem | Fix |
|---------|-----|
| WASM module not loading | `id` must be exactly `"bin-wasm-app"` when `runtime.core = true` |
| Brotli decoder missing | Ensure `runtime.decoder: true` (default) when `compression: "brotli"` |
| `wasm-pack` not found | Install: `cargo install wasm-pack`. Or set `compile_wasm: false` and use pre-built `pkg/`. |
| `pkg/` directory not found | Run `wasm-pack build --target no-modules` inside the WASM project first |
| Config key typo | Use `metadata:` not `meta:`. The example-config.yaml in the repo has this wrong. |
| Paths not resolving | Local paths are relative to the CWD where `histos` is invoked, not the config file |
| Already-compressed WASM | Set `compression: "none"` to skip double-compression |
| Binary name | The installed binary is `histos` (not `pithos`, `htmlpack`, or `histos-pack`) |

---

## Config Verification Checklist

Before running, verify the generated config:

- [ ] If WASM + default runtime: `id: "bin-wasm-app"` is present
- [ ] If `compression: "brotli"`: `runtime.decoder` is `true` (or omitted — default is true)
- [ ] If `compile_wasm: true`: `wasm-pack` is installed, `path:` points to project root
- [ ] If `path:` used: `wasm-pack build --target no-modules` has been run, `pkg/` exists
- [ ] All local paths exist relative to where you'll invoke `histos`
- [ ] YAML key is `metadata:` not `meta:`

---

## Full Example Session

**User:** "I have a Rust/WASM app built with wasm-pack at `./my-app` and a CSS file at `./styles/main.css`. Pack it."

**Generated config.yaml:**
```yaml
pack:
  runtime:
    enabled: true

  metadata:
    title: "My App"

  css:
    - ./styles/main.css

  wasm:
    app:
      path: ./my-app
      id: "bin-wasm-app"
      compile_wasm: false
      compression: "brotli"
```

**Run:**
```bash
histos config.yaml
```

**Output:** `./index.html` — open in any browser, works offline.
