use http::StatusCode;
use reqwest::Request;
use std::borrow::Cow;
use std::path::Path;

pub struct NetCacheEntry<'a> {
    pub url: Cow<'a, str>,
    pub status: StatusCode,
    pub body: Cow<'a, str>,
}

impl<'a> NetCacheEntry<'a> {
    pub async fn write_to_dir(&self, dir: impl AsRef<Path>) -> eyre::Result<()> {
        tokio::fs::create_dir_all(&dir).await?;

        let url_path = dir.as_ref().join("url.txt");
        tokio::fs::write(url_path, self.url.as_ref()).await?;

        let status_path = dir.as_ref().join("status.txt");
        tokio::fs::write(status_path, self.status.as_u16().to_string()).await?;

        let body_path = dir.as_ref().join("body.json");
        tokio::fs::write(body_path, self.body.as_bytes()).await?;
        Ok(())
    }
    pub async fn read_from_dir(dir: impl AsRef<Path>) -> eyre::Result<Self> {
        let url_path = dir.as_ref().join("url.txt");
        let url = tokio::fs::read_to_string(url_path).await?;

        let status_path = dir.as_ref().join("status.txt");
        let status_str = tokio::fs::read_to_string(status_path).await?;
        let status_code: u16 = status_str.trim().parse()?;

        let body_path = dir.as_ref().join("body.json");
        let status = StatusCode::from_u16(status_code)?;

        let body = tokio::fs::read_to_string(body_path).await?;

        Ok(NetCacheEntry {
            url: Cow::Owned(url),
            status,
            body: Cow::Owned(body),
        })
    }
    pub fn matches(&self, req: &Request) -> bool {
        self.url.as_ref() == req.url().to_string()
    }
}
