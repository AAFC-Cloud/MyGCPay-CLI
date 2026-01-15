use crate::cli::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct PaychequeShowArgs {
    /// ID of paycheque to show
    #[arg(value_name = "ID")]
    pub id: Option<String>,
}
impl PaychequeShowArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        println!("stub: paycheque show {:?}", self.id);
        Ok(())
    }
}
impl ToArgs for PaychequeShowArgs {}
