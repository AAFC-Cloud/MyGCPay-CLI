use crate::cli::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use color_eyre::owo_colors::OwoColorize;
use crate::cookie::MyGcPayCookie;
use crate::paths::CACHE_DIR;

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
