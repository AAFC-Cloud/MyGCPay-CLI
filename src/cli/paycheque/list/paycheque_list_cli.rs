use crate::cli::ToArgs;
use crate::paycheque::PaychequeExtraYearRequest;
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

fn sort_and_dedup_paycheques(paycheques: &mut Vec<PaychequeListResponseEntry>) {
    paycheques.sort_by(|left, right| {
        right
            .cheque_issued_date_number
            .cmp(&left.cheque_issued_date_number)
            .then_with(|| right.cheque_no.cmp(&left.cheque_no))
    });
    paycheques.dedup_by(|left, right| left.cheque_no == right.cheque_no);
}

fn earliest_paycheque_year(paycheques: &[PaychequeListResponseEntry]) -> eyre::Result<i32> {
    let mut earliest_year: Option<i32> = None;

    for date in paycheques.iter().flat_map(|paycheque| {
        [
            paycheque.cheque_start_date_unformatted.as_str(),
            paycheque.cheque_end_date_unformatted.as_str(),
        ]
    }) {
        let year = date
            .get(0..4)
            .ok_or_else(|| eyre::eyre!("missing year in date `{date}`"))?
            .parse::<i32>()?;
        earliest_year = Some(match earliest_year {
            Some(current) => current.min(year),
            None => year,
        });
    }

    earliest_year.ok_or_else(|| eyre::eyre!("no paycheques returned"))
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

        let mut next_year = earliest_paycheque_year(&rtn.paycheques)? - 1;
        loop {
            let extra_paycheques = PaychequeExtraYearRequest { year: next_year }.await?;
            let count = extra_paycheques.count();
            let all_data_loaded = extra_paycheques.all_data_loaded();
            info!(year = next_year, count, all_data_loaded, "Fetched extra paycheque batch");
            extra_paycheques.respectful_sleep(self.sleep).await;

            let previous_earliest_year = earliest_paycheque_year(&rtn.paycheques)?;
            let paycheques = extra_paycheques.take().paycheques();
            if paycheques.is_empty() {
                break;
            }

            let new_earliest_year = earliest_paycheque_year(&paycheques)?;
            rtn.paycheques = paycheques;

            if all_data_loaded || new_earliest_year >= previous_earliest_year {
                break;
            }
            next_year = new_earliest_year - 1;
        }

        sort_and_dedup_paycheques(&mut rtn.paycheques);

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

#[cfg(test)]
mod test {
    use super::earliest_paycheque_year;
    use super::sort_and_dedup_paycheques;
    use crate::paycheque::ChequeNumber;
    use crate::paycheque::PaychequeListResponseEntry;

    fn entry(cheque_no: &str, issued_date_number: &str) -> PaychequeListResponseEntry {
        PaychequeListResponseEntry {
            department_name: "Agriculture and Agri-Food".to_string(),
            cheque_type: "Basic Pay".to_string(),
            cheque_no: ChequeNumber(cheque_no.to_string()),
            cheque_pay_group: "TX1".to_string(),
            cheque_issued_date_number: issued_date_number.to_string(),
            cheque_issued_date: issued_date_number.to_string(),
            cheque_month: "January 2025".to_string(),
            cheque_issued_date_short: "01/01/2025".to_string(),
            cheque_start_date: "January 01, 2025".to_string(),
            cheque_start_date_unformatted: "2025/01/01".to_string(),
            cheque_end_date: "January 14, 2025".to_string(),
            cheque_end_date_unformatted: "2025/01/14".to_string(),
            cheque_total_gross: "0.00".to_string(),
            cheque_total_tax: "0.00".to_string(),
            cheque_total_deduction: "0.00".to_string(),
            cheque_total_net: "0.00".to_string(),
            cheque_total_gross_formated: "$0.00".to_string(),
            cheque_total_tax_formated: "$0.00".to_string(),
            cheque_total_deduction_formated: "$0.00".to_string(),
            cheque_total_net_formated: "$0.00".to_string(),
            cheque_account_number: "0".to_string(),
            paycheck_option: "A".to_string(),
        }
    }

    #[test]
    fn sort_and_dedup_paycheques_keeps_latest_order() {
        let mut paycheques = vec![
            entry("100", "20250101"),
            entry("200", "20250115"),
            entry("100", "20250101"),
        ];

        sort_and_dedup_paycheques(&mut paycheques);

        assert_eq!(paycheques.len(), 2);
        assert_eq!(paycheques[0].cheque_no.as_ref(), "200");
        assert_eq!(paycheques[1].cheque_no.as_ref(), "100");
    }

    #[test]
    fn earliest_paycheque_year_uses_visible_date_range() -> eyre::Result<()> {
        let mut first = entry("100", "20250101");
        first.cheque_start_date_unformatted = "2025/01/01".to_string();
        first.cheque_end_date_unformatted = "2025/01/14".to_string();

        let mut second = entry("200", "20260101");
        second.cheque_start_date_unformatted = "2026/01/01".to_string();
        second.cheque_end_date_unformatted = "2026/01/14".to_string();

        let paycheques = vec![first, second];

        assert_eq!(earliest_paycheque_year(&paycheques)?, 2025);
        Ok(())
    }
}
