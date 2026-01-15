use crate::cli::ToArgs;
use crate::net::MyGcPayCookie;
use crate::paths::CACHE_DIR;
use arbitrary::Arbitrary;
use clap::Args;
use color_eyre::owo_colors::OwoColorize;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct CookieCheckArgs {}
impl CookieCheckArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        let cache = &*CACHE_DIR;
        if let Some(_cookie) = MyGcPayCookie::read(cache).await? {
            println!("cookie is {}", "set".green());
            Ok(())
        } else {
            eprintln!("cookie is {}", "not set".red());
            std::process::exit(1);
        }
    }
}
impl ToArgs for CookieCheckArgs {}
