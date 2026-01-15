use crate::cli::ToArgs;
use crate::paycheque::ChequeNumber;
use crate::paycheque::PaychequeShowRequest;
use arbitrary::Arbitrary;
use clap::Args;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct PaychequeShowArgs {
    /// ID of paycheque to show
    #[arg(value_name = "ID")]
    pub id: String,
}
impl PaychequeShowArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        let resp = PaychequeShowRequest {
            cheque_number: ChequeNumber(self.id),
        }
        .await?;

        println!("{}", facet_json::to_string_pretty(&resp)?);

        Ok(())
    }
}
impl ToArgs for PaychequeShowArgs {}
