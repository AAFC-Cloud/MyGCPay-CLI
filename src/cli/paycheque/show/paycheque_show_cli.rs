use crate::cli::ToArgs;
use crate::paycheque::ChequeNumber;
use crate::paycheque::PaychequeShowRequest;
use crate::paycheque::PaychequeShowResponse;
use arbitrary::Arbitrary;
use clap::Args;
use facet::Facet;
use std::io::IsTerminal;
use std::io::Write;
use std::io::stdout;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct PaychequeShowArgs {
    /// ID of paycheque to show
    #[arg(value_name = "ID")]
    pub id: String,

    /// Print the shape of the response without fetching details
    #[arg(long, default_value_t = false)]
    pub shape: bool,
}
impl PaychequeShowArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        if self.shape {
            let mut stdout = stdout().lock();
            let display = if stdout.is_terminal() {
                facet_pretty::format_shape_colored(PaychequeShowResponse::SHAPE)
            } else {
                facet_pretty::format_shape(PaychequeShowResponse::SHAPE)
            };
            writeln!(stdout, "{}", display)?;
            return Ok(());
        }

        let resp = PaychequeShowRequest {
            cheque_number: ChequeNumber(self.id),
        }
        .await?;

        println!("{}", facet_json::to_string_pretty(resp.as_ref())?);

        Ok(())
    }
}
impl ToArgs for PaychequeShowArgs {}
