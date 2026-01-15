use crate::net::MyGcPayCookie;
use crate::paths::CACHE_DIR;
use eyre::bail;
use reqwest::header::HeaderValue;
use reqwest::header::{self};

pub async fn create_client() -> eyre::Result<reqwest::Client> {
    let mut headers = header::HeaderMap::new();

    let git_rev = option_env!("GIT_REVISION").unwrap_or("unknown");
    let user_agent = format!(
        "MyGCPay-CLI/{} (rev {}) (+https://github.com/AAFC-Cloud/MyGCPay-CLI)",
        env!("CARGO_PKG_VERSION"),
        git_rev
    );
    headers.insert(header::USER_AGENT, HeaderValue::from_str(&user_agent)?);

    headers.insert(
        header::ACCEPT,
        HeaderValue::from_str("application/json, text/javascript, */*; q=0.01")?,
    );
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/x-www-form-urlencoded; charset=UTF-8")?,
    );
    headers.insert(
        "Origin",
        HeaderValue::from_str("https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca")?,
    );
    headers.insert(
        "Referer",
        HeaderValue::from_str("https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paye-pay")?,
    );
    headers.insert("Cache-Control", HeaderValue::from_str("no-cache")?);
    headers.insert("Pragma", HeaderValue::from_str("no-cache")?);
    headers.insert("X-Requested-With", HeaderValue::from_str("XMLHttpRequest")?);
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_str("empty")?);
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_str("cors")?);
    headers.insert("Sec-Fetch-Site", HeaderValue::from_str("same-origin")?);
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_str(
            r#"\"Microsoft Edge\";v=\"143\", \"Chromium\";v=\"143\", \"Not A(Brand\";v=\"24\""#,
        )?,
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_str("?0")?);
    headers.insert(
        "sec-ch-ua-platform",
        HeaderValue::from_str(r#"\"Windows\""#)?,
    );

    let Some(cookie) = MyGcPayCookie::read(&CACHE_DIR).await? else {
        bail!("cookie is not set, see `cookie` command for details");
    };

    headers.insert(header::COOKIE, HeaderValue::from_str(&cookie)?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
