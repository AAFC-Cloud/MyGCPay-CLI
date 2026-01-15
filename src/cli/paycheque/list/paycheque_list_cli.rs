use crate::cli::ToArgs;
use crate::paycheque::PaychequeListRequest;
use crate::paycheque::PaychequeListResponseEntry;
use crate::paycheque::PaychequeShowRequest;
use crate::paycheque::PaychequeShowResponse;
use arbitrary::Arbitrary;
use clap::Args;
use eyre::ensure;
use facet::Facet;
use std::io::IsTerminal;
use std::io::Write;
use std::io::stdout;
use std::time::Duration;
use std::time::Instant;
use tracing::info;

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct PaychequeListArgs {
    /// Show all paycheques (including archived)
    #[arg(long)]
    pub all: bool,

    /// Sleep duration between requests (e.g., "500ms", "1s")
    #[arg(long, value_parser = humantime::parse_duration, default_value = "500ms")]
    pub sleep: Duration,

    /// Print the shape of the response without fetching details
    #[arg(long, default_value_t = false)]
    pub shape: bool,
}

#[derive(Facet)]
pub struct AllRtn {
    pub paycheques: Vec<PaychequeListResponseEntry>,
    pub paycheque_details: Vec<PaychequeShowResponse>,
}

impl PaychequeListArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        if self.shape {
            let shape = if self.all {
                AllRtn::SHAPE
            } else {
                PaychequeListResponseEntry::SHAPE
            };
            let mut stdout = stdout().lock();
            let display = if stdout.is_terminal() {
                facet_pretty::format_shape_colored(shape)
            } else {
                facet_pretty::format_shape(shape)
            };
            writeln!(stdout, "{}", display)?;
            return Ok(());
        }

        // Fetch all paycheques
        let resp = PaychequeListRequest.await?;
        info!(count=%resp.len(), "Fetched {} paycheques", resp.len());
        if !self.all {
            println!("{}", facet_json::to_string_pretty(resp.as_ref())?);
            return Ok(());
        }
        resp.respectful_sleep(self.sleep).await;

        let mut rtn = AllRtn {
            paycheques: resp.take(),
            paycheque_details: Vec::new(),
        };
        // Fetch each paycheque's details
        let paycheque_count = rtn.paycheques.len();
        let start = Instant::now();
        for (i, paycheque) in rtn.paycheques.iter().enumerate() {
            let details = PaychequeShowRequest {
                cheque_number: paycheque.cheque_no.clone(),
            }
            .await?;
            ensure!(
                details.single_paycheque.cheque_no == paycheque.cheque_no,
                "Mismatched cheque numbers: list {}, show {}",
                paycheque.cheque_no,
                details.single_paycheque.cheque_no
            );
            let elapsed = Instant::now().duration_since(start);
            let avg_per = elapsed.as_secs_f64() / (i as f64 + 1.0);
            let remaining = avg_per * (paycheque_count as f64 - i as f64 - 1.0);
            info!(
                "Fetched paycheque {}/{} (avg {:.2}s, remaining {:.2}s)",
                i + 1,
                paycheque_count,
                avg_per,
                remaining
            );

            details.respectful_sleep(self.sleep).await;
            rtn.paycheque_details.push(details.take());
        }

        println!("{}", facet_json::to_string_pretty(&rtn)?);

        Ok(())
    }
}
impl ToArgs for PaychequeListArgs {}
