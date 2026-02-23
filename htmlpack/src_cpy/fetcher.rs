/*
* fetcher.rs
*
* fetches source files whether they are local or external
*/

// standard
//use std::fs::File;
use std::path::{PathBuf};
use std::error::Error;
// external
use url::Url;
// local
use crate::config::{AssetSource};

// easy mode
pub fn get_local_file(path: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(std::fs::read(path)?)
}

// easy mode
// could refactor out reqwest later
pub async fn get_remote_file(url: Url) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(reqwest::get(url).await?.bytes().await?.to_vec())
}

// basic asset fetch 
pub async fn fetch_source(
    source: AssetSource
) -> Result<Vec<u8>, Box<dyn Error>> {
    let r = match source {
        AssetSource::Local(path) => { get_local_file(&path)? },
        AssetSource::Remote(url) => { get_remote_file(url).await? },
    };
    Ok(r)
}

// fetch multiple assets as vec
pub async fn fetch_all_sources(
    sources: Vec<AssetSource>
) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
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

