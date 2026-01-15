pub mod check;
pub mod clear;
pub mod set;

use crate::cli::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct CookieArgs {
    #[command(subcommand)]
    pub command: CookieCommand,
}

impl CookieArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        match self.command {
            CookieCommand::Set(args) => args.invoke().await,
            CookieCommand::Clear(args) => args.invoke().await,
            CookieCommand::Check(args) => args.invoke().await,
        }
    }
}

impl ToArgs for CookieArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match &self.command {
            CookieCommand::Set(set) => {
                args.push("set".into());
                args.extend(set.to_args());
            }
            CookieCommand::Clear(clear) => {
                args.push("clear".into());
                args.extend(clear.to_args());
            }
            CookieCommand::Check(check) => {
                args.push("check".into());
                args.extend(check.to_args());
            }
        }
        args
    }
}

#[derive(Subcommand, Debug, PartialEq, Arbitrary)]
pub enum CookieCommand {
    Set(set::CookieSetArgs),
    Clear(clear::CookieClearArgs),
    Check(check::CookieCheckArgs),
}
