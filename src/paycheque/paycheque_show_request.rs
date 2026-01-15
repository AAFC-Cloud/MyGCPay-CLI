use crate::net::CacheKey;
use crate::net::ClientExt;
use crate::net::HasCacheKey;
use crate::net::MaybeCached;
use crate::net::create_client;
use crate::paycheque::ChequeNumber;
use crate::paycheque::PaychequeShowResponse;
use std::path::PathBuf;
use std::pin::Pin;

pub struct PaychequeShowRequest {
    pub cheque_number: ChequeNumber,
}

impl HasCacheKey for PaychequeShowRequest {
    fn cache_key(&self) -> CacheKey {
        CacheKey::new(PathBuf::from_iter([
            "paycheque",
            "show",
            self.cheque_number.as_ref(),
        ]))
    }
}

impl IntoFuture for PaychequeShowRequest {
    type Output = eyre::Result<MaybeCached<PaychequeShowResponse>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paycheque-data/";

            let client = create_client().await?;
            let req = client
                .post(url)
                .form(&[
                    ("type", "buildDetailedTables"),
                    ("chkNumber", &self.cheque_number),
                ])
                .build()?;
            let resp = client.execute_cached(req, &self.cache_key()).await?;
            Ok(resp)
        })
    }
}
