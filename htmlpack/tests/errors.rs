//! Error suite — one test per error variant to verify each path is reachable.

use std::path::PathBuf;

use histos::config::{PackConfig, WasmModule};
use histos::error::{ConfigError, EncodeError, FetchError, HistosError, SaveError};
use histos::fetcher;
use histos::packer;
use histos::render::PackedHtml;
use url::Url;

// ── ConfigError ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn config_file_not_found() {
    let result = packer::load_config(PathBuf::from("/no/such/config.yaml")).await;
    assert!(matches!(
        result,
        Err(HistosError::Config(ConfigError::FileNotFound { .. }))
    ));
}

#[tokio::test]
async fn config_yaml_parse_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bad.yaml");
    std::fs::write(&path, b": : invalid\n  yaml: [\n").unwrap();
    let result = packer::load_config(path).await;
    assert!(matches!(
        result,
        Err(HistosError::Config(ConfigError::YamlParse(_)))
    ));
}

#[test]
fn config_pkg_dir_not_found() {
    let result = WasmModule::from_pkg(
        "test".into(),
        "/no/such/wasm/project".into(),
        false,
        "brotli".into(),
    );
    assert!(matches!(
        result,
        Err(HistosError::Config(ConfigError::PkgDirNotFound { .. }))
    ));
}

#[test]
fn config_pkg_missing_wasm() {
    let dir = tempfile::tempdir().unwrap();
    let pkg = dir.path().join("pkg");
    std::fs::create_dir(&pkg).unwrap();
    // only a .js, no _bg.wasm
    std::fs::write(pkg.join("glue.js"), b"/* js glue */").unwrap();
    let result = WasmModule::from_pkg(
        "test".into(),
        dir.path().to_str().unwrap().into(),
        false,
        "brotli".into(),
    );
    assert!(matches!(
        result,
        Err(HistosError::Config(ConfigError::PkgFileMissing {
            missing: "_bg.wasm",
            ..
        }))
    ));
}

#[test]
fn config_pkg_missing_js() {
    let dir = tempfile::tempdir().unwrap();
    let pkg = dir.path().join("pkg");
    std::fs::create_dir(&pkg).unwrap();
    // only a _bg.wasm, no .js
    std::fs::write(pkg.join("app_bg.wasm"), b"\x00asm").unwrap();
    let result = WasmModule::from_pkg(
        "test".into(),
        dir.path().to_str().unwrap().into(),
        false,
        "brotli".into(),
    );
    assert!(matches!(
        result,
        Err(HistosError::Config(ConfigError::PkgFileMissing {
            missing: ".js",
            ..
        }))
    ));
}

// ── FetchError ───────────────────────────────────────────────────────────────

#[test]
fn fetch_local_not_found() {
    let result = fetcher::get_local_file(&PathBuf::from("/no/such/file.css"));
    assert!(matches!(
        result,
        Err(HistosError::Fetch(FetchError::LocalNotFound { .. }))
    ));
}

#[tokio::test]
async fn fetch_http_request_error() {
    // Port 1 is privileged — connections are always refused.
    let url = Url::parse("http://127.0.0.1:1/resource.css").unwrap();
    let result = fetcher::get_remote_file(url).await;
    assert!(matches!(
        result,
        Err(HistosError::Fetch(FetchError::HttpRequest { .. }))
    ));
}

#[tokio::test]
async fn fetch_http_status_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/resource.css")
        .with_status(404)
        .create_async()
        .await;
    let url = Url::parse(&format!("{}/resource.css", server.url())).unwrap();
    let result = fetcher::get_remote_file(url).await;
    assert!(matches!(
        result,
        Err(HistosError::Fetch(FetchError::HttpStatus { status: 404, .. }))
    ));
    mock.assert_async().await;
}

// ── EncodeError ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn encode_invalid_utf8() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("binary.js");
    // Invalid UTF-8 sequence
    std::fs::write(&path, b"\xff\xfe not valid utf-8").unwrap();

    // build() fetches the file as a script and tries to decode it as text
    let config = PackConfig::new()
        .set_runtime(false, false, false, false)
        .add_script(path.to_str().unwrap().into());

    let result = config.build().await;
    assert!(matches!(
        result,
        Err(HistosError::Encode(EncodeError::InvalidUtf8(_)))
    ));
}

// ── SaveError ────────────────────────────────────────────────────────────────

#[test]
fn save_create_dir_error() {
    let dir = tempfile::tempdir().unwrap();
    // A regular file standing in for a directory → create_dir_all fails.
    let fake_parent = dir.path().join("not_a_dir");
    std::fs::write(&fake_parent, b"I am a file").unwrap();
    let output = fake_parent.join("output.html");

    let packed = PackedHtml { html: "<html/>".into() };
    let result = packed.save_to_file(output);
    assert!(matches!(
        result,
        Err(HistosError::Save(SaveError::CreateDir { .. }))
    ));
}

#[test]
#[cfg(unix)]
fn save_create_file_error() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    // Remove write permission so File::create fails inside an existing dir.
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o555)).unwrap();
    let output = dir.path().join("output.html");

    let packed = PackedHtml { html: "<html/>".into() };
    let result = packed.save_to_file(output);

    // Restore permissions so tempdir cleanup succeeds.
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755)).unwrap();

    assert!(matches!(
        result,
        Err(HistosError::Save(SaveError::CreateFile { .. }))
    ));
}
