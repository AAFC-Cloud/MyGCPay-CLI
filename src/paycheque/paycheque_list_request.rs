use crate::net::create_client;
use crate::paycheque::PaychequeListResponse;
use std::pin::Pin;

pub struct PaychequeListRequest;

impl IntoFuture for PaychequeListRequest {
    type Output = eyre::Result<PaychequeListResponse>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paycheque-data/";

            let client = create_client().await?;
            let resp = client
                .post(url)
                .header(reqwest::header::ACCEPT, "application/json, text/javascript, */*; q=0.01")
                .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded; charset=UTF-8")
                .header("Origin", "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca")
                .header("Referer", "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paye-pay")
                .header("Cache-Control", "no-cache")
                .header("Pragma", "no-cache")
                .header("X-Requested-With", "XMLHttpRequest")
                .header("Sec-Fetch-Dest", "empty")
                .header("Sec-Fetch-Mode", "cors")
                .header("Sec-Fetch-Site", "same-origin")
                .header("sec-ch-ua", r#"\"Microsoft Edge\";v=\"143\", \"Chromium\";v=\"143\", \"Not A(Brand\";v=\"24\""#)
                .header("sec-ch-ua-mobile", "?0")
                .header("sec-ch-ua-platform", r#"\"Windows\""#)
                .form(&[("type", "list")])
                .send()
                .await?;
            let text = resp.text().await?;
            match facet_json::from_str(&text) {
                Ok(rtn) => Ok(rtn),
                Err(e) => {
                    eyre::bail!("Failed to parse PaychequeListResponse: {}\nResponse text: {}", e, text);
                }
            }
        })
    }
}
