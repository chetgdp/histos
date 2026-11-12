//! `pack.head` suite: histos always emits charset; the shard replaces viewport.

use histos::packer;

const CHARSET: &str = "<meta charset=\"utf-8\">";
const VIEWPORT: &str = "<meta name=\"viewport\"";

async fn pack(yaml: &str, dir: &std::path::Path) -> String {
    let path = dir.join("histos.yaml");
    std::fs::write(&path, yaml).unwrap();
    let config = packer::load_config(&path).await.unwrap();
    config.build().await.unwrap().render().html
}

#[tokio::test]
async fn head_absent_emits_default_charset_and_viewport() {
    let dir = tempfile::tempdir().unwrap();
    let html = pack("pack:\n  runtime:\n    enabled: false\n", dir.path()).await;

    assert_eq!(html.matches(CHARSET).count(), 1);
    assert_eq!(html.matches(VIEWPORT).count(), 1);
}

#[tokio::test]
async fn head_shard_replaces_viewport_and_keeps_charset_and_metadata() {
    let dir = tempfile::tempdir().unwrap();
    let shard = dir.path().join("head.html");
    std::fs::write(&shard, "<meta name=\"theme-color\" content=\"#111\">").unwrap();
    let yaml = format!(
        "pack:\n  head: {}\n  metadata:\n    title: Shard Test\n  runtime:\n    enabled: false\n",
        shard.display()
    );
    let html = pack(&yaml, dir.path()).await;

    let head = &html[html.find("<head>").unwrap()..html.find("</head>").unwrap()];
    assert!(head.trim_start_matches("<head>").trim_start().starts_with(CHARSET));
    assert_eq!(head.matches(CHARSET).count(), 1);
    assert_eq!(head.matches(VIEWPORT).count(), 0);
    assert!(head.contains("<meta name=\"theme-color\" content=\"#111\">"));
    assert!(head.contains("<title>Shard Test</title>"));
}

#[tokio::test]
async fn head_shard_leading_bom_is_removed() {
    let dir = tempfile::tempdir().unwrap();
    let shard = dir.path().join("head.html");
    std::fs::write(&shard, "\u{FEFF}<meta name=\"theme-color\" content=\"#111\">").unwrap();
    let yaml = format!("pack:\n  head: {}\n  runtime:\n    enabled: false\n", shard.display());
    let html = pack(&yaml, dir.path()).await;

    assert!(!html.contains('\u{FEFF}'));
    assert!(html.contains("<meta name=\"theme-color\" content=\"#111\">"));
}
