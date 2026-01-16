pub mod cache;
pub mod cookie;
mod global_args;
pub mod home;
pub mod paycheque;
pub mod calendar;

use arbitrary::Arbitrary;
use clap::Parser;
use clap::Subcommand;
use eyre::Context;
pub use global_args::*;
use std::ffi::OsString;

#[derive(Parser, Arbitrary, PartialEq, Debug)]
#[clap(version)]
pub struct Cli {
    #[clap(flatten)]
    pub global_args: GlobalArgs,
    #[clap(subcommand)]
    pub command: Command,
}

impl Cli {
    /// # Errors
    ///
    /// This function will return an error if the tokio runtime cannot be built or if the command fails.
    pub fn invoke(self) -> eyre::Result<()> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .wrap_err("Failed to build tokio runtime")?;
        runtime.block_on(async move { self.command.invoke().await })?;
        Ok(())
    }
}

impl ToArgs for Cli {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        args.extend(self.global_args.to_args());
        args.extend(self.command.to_args());
        args
    }
}

/// Top-level commands
#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum Command {
    /// Cookie-related commands
    Cookie(cookie::CookieArgs),
    /// Paycheque-related commands
    Paycheque(paycheque::PaychequeArgs),
    /// Cache-related commands
    Cache(cache::CacheArgs),
    /// Home-related commands
    Home(home::HomeArgs),
    /// Calendar-related commands
    Calendar(calendar::CalendarArgs),
}

impl Command {
    /// # Errors
    ///
    /// This function will return an error if the subcommand fails.
    pub async fn invoke(self) -> eyre::Result<()> {
        match self {
            Command::Cookie(args) => args.invoke().await,
            Command::Paycheque(args) => args.invoke().await,
            Command::Cache(args) => args.invoke().await,
            Command::Home(args) => args.invoke().await,
            Command::Calendar(args) => args.invoke().await,
        }
    }
}

impl ToArgs for Command {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            Command::Cookie(cookie_args) => {
                args.push("cookie".into());
                args.extend(cookie_args.to_args());
            }
            Command::Paycheque(pc_args) => {
                args.push("paycheque".into());
                args.extend(pc_args.to_args());
            }
            Command::Cache(cache_args) => {
                args.push("cache".into());
                args.extend(cache_args.to_args());
            }
            Command::Home(home_args) => {
                args.push("home".into());
                args.extend(home_args.to_args());
            }
            Command::Calendar(calendar_args) => {
                args.push("calendar".into());
                args.extend(calendar_args.to_args());
            }
        }
        args
    }
}

/// Trait for converting CLI structures to command line arguments
pub trait ToArgs {
    fn to_args(&self) -> Vec<OsString> {
        Vec::new()
    }
}

// Blanket implementation for references
impl<T: ToArgs> ToArgs for &T {
    fn to_args(&self) -> Vec<OsString> {
        (*self).to_args()
    }
}
