use crate::net::MyGcPayCookie;
use crate::paths::CACHE_DIR;
use eyre::bail;

pub async fn create_client() -> eyre::Result<reqwest::Client> {
    let mut headers = reqwest::header::HeaderMap::new();

    let git_rev = option_env!("GIT_REVISION").unwrap_or("unknown");
    let user_agent = format!(
        "MyGCPay-CLI/{} (rev {}) (+https://github.com/AAFC-Cloud/MyGCPay-CLI)",
        env!("CARGO_PKG_VERSION"),
        git_rev
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_str(&user_agent)?,
    );

    let Some(cookie) = MyGcPayCookie::read(&CACHE_DIR).await? else {
        bail!("cookie is not set, see `cookie` command for details");
    };

    headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_str(&cookie)?,
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
