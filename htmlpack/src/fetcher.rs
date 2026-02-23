/*
* fetcher.rs
*
* fetches source files whether they are local or external
*/

// standard
use std::path::PathBuf;
// external
use url::Url;
// local
use crate::config::AssetSource;
use crate::error::{HistosResult, FetchError};

/// Reads the a file from a given filepath buffer.
///
/// This is enables local source files as input.
///
/// # Errors
///
/// - Returns [`FetchError::LocalNotFound`] if no file exists at the path.
/// - Returns [`FetchError::LocalRead`] if the file exists but cannot be read.
///
/// # Examples
///
/// ```no_run
/// let bytes = get_local_file(&PathBuf::from("style.css"))?;
/// ```
pub fn get_local_file(path: &PathBuf) -> HistosResult<Vec<u8>> {
    std::fs::read(path).map_err(|source| match source.kind() {
        std::io::ErrorKind::NotFound => FetchError::LocalNotFound { path: path.clone() }.into(),
        _ => FetchError::LocalRead { path: path.clone(), source }.into(),
    })
}

/// Http Get Request that returns a raw byte vector.
/// 
/// This enables fetching remote files as input via url.
/// # Errors
///
/// - Returns [`FetchError::HttpRequest`] if the request fails to send or the response cannot be read.
/// - Returns [`FetchError::HttpStatus`] if the server responds with a non-2xx status code.
///
/// # Examples
///
/// ```no_run
/// let bytes = get_remote_file(url).await?;
/// ```
pub async fn get_remote_file(url: Url) -> HistosResult<Vec<u8>> {
    let response = reqwest::get(url.clone())
        .await
        .map_err(|source| FetchError::HttpRequest { url: url.to_string(), source })?;

    if !response.status().is_success() {
        return Err(FetchError::HttpStatus {
            url: url.to_string(),
            status: response.status().as_u16(),
        }.into());
    }

    response.bytes()
        .await
        .map(|b| b.to_vec())
        .map_err(|source| FetchError::HttpRequest { url: url.to_string(), source }.into())
}

/// Uses the AssetSource enum to match local vs remote files.
///
/// # Errors
///
/// - Returns [`FetchError::LocalNotFound`] or [`FetchError::LocalRead`] for local assets.
/// - Returns [`FetchError::HttpRequest`] or [`FetchError::HttpStatus`] for remote assets.
///
/// # Examples
///
/// ```no_run
/// let bytes = fetch_source(AssetSource::Local("style.css".into())).await?;
/// ```
pub async fn fetch_source(source: AssetSource) -> HistosResult<Vec<u8>> {
    match source {
        AssetSource::Local(path) => get_local_file(&path),
        AssetSource::Remote(url) => get_remote_file(url).await,
    }
}


// fetch multiple assets as vec
/// Fetches all asset sources sequentially, returning raw bytes for each.
///
/// Sources are resolved in order, local paths are read from disk,
/// remote URLs are fetched over HTTP. Stops and returns an error on
/// the first failure.
///
/// # Errors
///
/// - Returns [`FetchError::LocalNotFound`] if a local path doesn't exist.
/// - Returns [`FetchError::HttpStatus`] if a remote URL returns a non-2xx response.
///
/// # Examples
///
/// ```
/// let sources = vec![AssetSource::Local("style.css".into())];
/// let bytes = fetch_all_sources(sources).await?;
/// ```
pub async fn fetch_all_sources(sources: Vec<AssetSource>) -> HistosResult<Vec<Vec<u8>>> {
    let mut results = Vec::new();
    for source in sources {
        results.push(fetch_source(source).await?);
    }
    Ok(results)
    // can't do this without parallel
    //sources
    //    .into_iter()
    //    .map(|source| fetch_source(source).await?)
    //    .collect()
}

