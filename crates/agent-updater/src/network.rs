use std::path::Path;

use tokio::io::AsyncWriteExt;

use crate::error::UpdaterError;

pub async fn fetch_text(url: &str) -> Result<String, UpdaterError> {
    let response = reqwest::get(url).await?.error_for_status()?;
    Ok(response.text().await?)
}

pub async fn download_file(url: &str, path: &Path) -> Result<(), UpdaterError> {
    let bytes = reqwest::get(url).await?.error_for_status()?.bytes().await?;
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(&bytes).await?;
    file.flush().await?;
    Ok(())
}
