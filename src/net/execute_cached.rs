use crate::net::CacheKey;
use crate::net::NetCacheEntry;
use crate::paths::CACHE_DIR;
use chrono::Local;
use facet::Facet;
use reqwest::Request;
use tracing::debug;
use tracing::info;

pub trait ClientExt {
    #[expect(async_fn_in_trait)]
    async fn execute_cached<T: Facet<'static>>(
        &self,
        req: Request,
        cache_key: &CacheKey,
    ) -> eyre::Result<T>;
}
impl ClientExt for reqwest::Client {
    async fn execute_cached<T: Facet<'static>>(
        &self,
        req: Request,
        cache_key: &CacheKey,
    ) -> eyre::Result<T> {
        let request_cache_dir = CACHE_DIR.join(cache_key);
        let url = req.url().to_string();

        if let Ok(true) = tokio::fs::try_exists(&request_cache_dir).await {
            // Read from cache
            debug!(
                method = %req.method(),
                url = %req.url(),
                dir=%request_cache_dir.display(),
                "Cache hit, reading from cache"
            );
            if let Ok(cache_entry) = NetCacheEntry::read_from_dir(&request_cache_dir).await {
                if cache_entry.matches(&req) {
                    // Parse and return the response
                    match facet_json::from_str(&cache_entry.body) {
                        Ok(rtn) => return Ok(rtn),
                        Err(e) => {
                            eyre::bail!(
                                "Failed to parse cached response: {}\nCheck the cache for details: {}",
                                e,
                                request_cache_dir.display()
                            );
                        }
                    }
                } else {
                    debug!(
                        dir=%request_cache_dir.display(),
                        "Cached entry does not match request, performing network request"
                    );
                }
            } else {
                debug!(
                    dir=%request_cache_dir.display(),
                    "Failed to read cache entry, performing network request"
                );
            }
        }

        // Perform the request
        debug!(
            method = %req.method(),
            url = %req.url(),
            "Cache miss, performing network request"
        );
        let resp = self.execute(req).await?;

        // Read the body
        let status = resp.status();
        let body = resp.text().await?;

        // Create serializable entry
        let cache_entry = NetCacheEntry {
            url: std::borrow::Cow::Borrowed(&url),
            status,
            body: std::borrow::Cow::Borrowed(&body),
        };

        if status.is_success() {
            // Write entry to cache
            cache_entry.write_to_dir(&request_cache_dir).await?;
            debug!(
                dir=%request_cache_dir.display(),
                "Wrote successful response to cache"
            );
        } else {
            // Write entry to failure dir for inspection
            let dir = tempfile::Builder::new()
                .prefix(Local::now().format("%Y%m%d_%H%M%S").to_string().as_str())
                .tempdir_in(&request_cache_dir.join("failures"))?
                .keep();
            cache_entry.write_to_dir(&dir).await?;
            info!(
                dir=%dir.display(),
                "Wrote failed response to failure cache"
            );
        }

        // Parse and return the response
        match facet_json::from_str(&body) {
            Ok(rtn) => return Ok(rtn),
            Err(e) => {
                eyre::bail!(
                    "Failed to parse cached response: {}\nCheck the cache for details: {}",
                    e,
                    request_cache_dir.display()
                );
            }
        }
    }
}
