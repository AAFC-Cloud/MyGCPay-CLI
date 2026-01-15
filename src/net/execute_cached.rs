use crate::net::CacheKey;
use crate::net::MaybeCached;
use crate::net::NetCacheEntry;
use crate::paths::CACHE_DIR;
use chrono::Local;
use eyre::bail;
use facet::Facet;
use reqwest::Request;
use tracing::debug;
use tracing::error;

pub trait ClientExt {
    #[expect(async_fn_in_trait)]
    async fn execute_cached_raw(
        &self,
        req: Request,
        cache_key: &CacheKey,
    ) -> eyre::Result<MaybeCached<NetCacheEntry<'static>>>;

    #[expect(async_fn_in_trait)]
    async fn execute_cached<T: Facet<'static>>(
        &self,
        req: Request,
        cache_key: &CacheKey,
    ) -> eyre::Result<MaybeCached<T>>;
}

impl ClientExt for reqwest::Client {
    async fn execute_cached_raw(
        &self,
        req: Request,
        cache_key: &CacheKey,
    ) -> eyre::Result<MaybeCached<NetCacheEntry<'static>>> {
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
                    return Ok(MaybeCached::FromCache(cache_entry));
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
            url: std::borrow::Cow::Owned(url.clone()),
            status,
            body: std::borrow::Cow::Owned(body),
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
            let fail_dir = request_cache_dir.join("failures");
            tokio::fs::create_dir_all(&fail_dir).await?;
            let dir = tempfile::Builder::new()
                .prefix(Local::now().format("%Y%m%d_%H%M%S_").to_string().as_str())
                .tempdir_in(&fail_dir)?
                .keep();
            cache_entry.write_to_dir(&dir).await?;
            error!(
                dir=%dir.display(),
                "Wrote failed response to failure cache"
            );
            bail!(
                "Request failed with status {}: {}\n{}\nCheck the cache for details: {}",
                status,
                url,
                cache_entry.body,
                dir.display()
            );
        }

        Ok(MaybeCached::FromNetwork(cache_entry))
    }

    async fn execute_cached<T: Facet<'static>>(
        &self,
        req: Request,
        cache_key: &CacheKey,
    ) -> eyre::Result<MaybeCached<T>> {
        let raw_resp = self.execute_cached_raw(req, cache_key).await?;
        // Parse and return the response
        match facet_json::from_str(&raw_resp.body) {
            Ok(rtn) => {
                return Ok(match raw_resp {
                    MaybeCached::FromCache(_) => MaybeCached::FromCache(rtn),
                    MaybeCached::FromNetwork(_) => MaybeCached::FromNetwork(rtn),
                });
            }
            Err(e) => {
                let request_cache_dir = CACHE_DIR.join(cache_key);
                eyre::bail!(
                    "Failed to parse response: {}\nCheck the cache for details: {}\nHave you updated your cookie recently?\nRun `mgcp cookie set --help` for more info.",
                    e,
                    request_cache_dir.display()
                );
            }
        }
    }
}
