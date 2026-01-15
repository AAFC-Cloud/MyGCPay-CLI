use crate::cli::ToArgs;
use crate::paycheque::PaychequeListRequest;
use arbitrary::Arbitrary;
use clap::Args;
use eyre::ensure;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct PaychequeListArgs {
    /// Show all paycheques (including archived)
    #[arg(long)]
    pub all: bool,
}
impl PaychequeListArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        ensure!(!self.all, "--all is not implemented yet");

        let resp = PaychequeListRequest.await?;

        println!("{}", facet_json::to_string_pretty(&resp)?);

        Ok(())
    }
}
impl ToArgs for PaychequeListArgs {}
