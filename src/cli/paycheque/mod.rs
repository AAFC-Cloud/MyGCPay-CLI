pub mod list;
pub mod show;

use crate::cli::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct PaychequeArgs {
    #[command(subcommand)]
    pub command: PaychequeCommand,
}

impl PaychequeArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        match self.command {
            PaychequeCommand::List(args) => args.invoke().await,
            PaychequeCommand::Show(args) => args.invoke().await,
        }
    }
}

impl ToArgs for PaychequeArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match &self.command {
            PaychequeCommand::List(list) => {
                args.push("list".into());
                args.extend(list.to_args());
            }
            PaychequeCommand::Show(show) => {
                args.push("show".into());
                args.extend(show.to_args());
            }
        }
        args
    }
}

#[derive(Subcommand, Debug, PartialEq, Arbitrary)]
pub enum PaychequeCommand {
    /// List paycheques
    List(list::PaychequeListArgs),
    /// Show a specific paycheque
    Show(show::PaychequeShowArgs),
}
