use crate::net::CacheKey;
use crate::net::ClientExt;
use crate::net::HasCacheKey;
use crate::net::MaybeCached;
use crate::net::create_client;
use crate::paycheque::ChequeNumber;
use crate::paycheque::PaychequeShowResponse;
use eyre::Context;
use eyre::eyre;
use http::header;
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
            let client = create_client().await?;

            // Phase 1: request the webpage, required to warm the server for the next request
            let url = format!(
                "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/chequepaiedetaille-detailedpaycheque/{}",
                self.cheque_number
            );
            let req = client.get(&url).build()?;
            client
                .execute_cached_raw(req, &self.cache_key().join("webpage"))
                .await
                .wrap_err_with(|| {
                    eyre!(
                        "Failed to execute warming request for cheque number {}",
                        self.cheque_number
                    )
                })?;

            // Phase 2: request the data
            let url = "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paycheque-data/";

            let req = client
                .post(url)
                .header(header::REFERER, format!("https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/chequepaiedetaille-detailedpaycheque/{}", self.cheque_number))
                .form(&[
                    ("type", "buildDetailedTables"),
                    ("chkNumber", &self.cheque_number),
                ])
                .build()?;
            let resp = client
                .execute_cached(req, &self.cache_key())
                .await
                .wrap_err_with(|| {
                    eyre!(
                        "Failed to execute request for cheque number {}",
                        self.cheque_number
                    )
                })?;
            Ok(resp)
        })
    }
}
