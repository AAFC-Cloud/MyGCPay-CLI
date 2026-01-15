use crate::cli::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct PaychequeListArgs {
    /// Show all paycheques (including archived)
    #[arg(long)]
    pub all: bool,
}
impl PaychequeListArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        println!("stub: paycheque list --all={} ", self.all);
        Ok(())
    }
}
impl ToArgs for PaychequeListArgs {}
