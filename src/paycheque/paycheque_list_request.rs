use crate::net::CacheKey;
use crate::net::ClientExt;
use crate::net::HasCacheKey;
use crate::net::MaybeCached;
use crate::net::create_client;
use crate::paycheque::PaychequeListResponse;
use std::path::PathBuf;
use std::pin::Pin;

pub struct PaychequeListRequest;

impl HasCacheKey for PaychequeListRequest {
    fn cache_key(&self) -> CacheKey {
        CacheKey::new(PathBuf::from_iter(["paycheque", "list"]))
    }
}

impl IntoFuture for PaychequeListRequest {
    type Output = eyre::Result<MaybeCached<PaychequeListResponse>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paycheque-data/";

            let client = create_client().await?;
            let req = client.post(url).form(&[("type", "list")]).build()?;
            let resp = client.execute_cached(req, &self.cache_key()).await?;
            Ok(resp)
        })
    }
}
