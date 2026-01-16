pub mod show;

pub use show::*;

use crate::cli::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct CalendarArgs {
    #[command(subcommand)]
    pub command: CalendarCommand,
}

impl CalendarArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        match self.command {
            CalendarCommand::Show(args) => args.invoke().await,
        }
    }
}

impl ToArgs for CalendarArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match &self.command {
            CalendarCommand::Show(show) => {
                args.push("show".into());
                args.extend(show.to_args());
            }
        }
        args
    }
}

#[derive(Subcommand, Debug, PartialEq, Arbitrary)]
pub enum CalendarCommand {
    /// Show dates for the given year
    Show(show::CalendarShowArgs),
}