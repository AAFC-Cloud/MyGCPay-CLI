use crate::cli::ToArgs;
use crate::net::MyGcPayCookie;
use crate::paths::CACHE_DIR;
use arbitrary::Arbitrary;
use clap::Args;
use eyre::ensure;
use tokio::io::AsyncReadExt;

/// Set cookie.
///
/// 1. Visit <https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/>
///
/// 2. Open network tab in developer tools
///
/// 3. Refresh the page
///
/// 4. Identify a request to `paycheque-data/`
///
/// 5. Right click the request and select "Copy > Copy request headers"
///
/// 6. `Get-Clipboard | mgcp cookie set` (PowerShell)
#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct CookieSetArgs {
    /// Cookie value to set. If omitted, reads from stdin.
    #[arg(value_name = "VALUE")]
    pub value: Option<String>,
}
impl CookieSetArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        let value = if let Some(v) = self.value {
            v
        } else {
            let mut s = String::new();
            tokio::io::stdin().read_to_string(&mut s).await?;
            let s = s.trim().to_string();
            ensure!(
                !s.is_empty(),
                "cookie should be provided as argument or via stdin"
            );
            s
        };

        // parse value
        // - if multi line, find line starting with `Cookie: {}` and extract
        // - if single line, strip `Cookie: ` prefix if present
        let value = if value.contains('\n') {
            let line = value
                .lines()
                .find(|line| line.trim_start().starts_with("Cookie:"))
                .ok_or_else(|| eyre::eyre!("no line starting with 'Cookie:' found in input"))?;
            line.trim_start()
                .strip_prefix("Cookie:")
                .ok_or_else(|| eyre::eyre!("failed to parse cookie line"))?
                .trim()
                .to_string()
        } else {
            value
                .trim_start()
                .strip_prefix("Cookie:")
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| value.trim().to_string())
        };

        let cache = &*CACHE_DIR;
        MyGcPayCookie(value).write(cache).await?;
        Ok(())
    }
}
impl ToArgs for CookieSetArgs {}
