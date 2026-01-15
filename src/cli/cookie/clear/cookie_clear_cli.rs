use crate::cli::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use crate::net::MyGcPayCookie;
use crate::paths::CACHE_DIR;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct CookieClearArgs {}
impl CookieClearArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        let cache = &*CACHE_DIR;
        MyGcPayCookie::delete(cache).await?;
        Ok(())
    }
}
impl ToArgs for CookieClearArgs {}
