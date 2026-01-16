use crate::cli::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use chrono::Local;
use chrono::Datelike;
use facet::Facet;
use crate::calendar::pipsc::{PIPSC_CALENDAR_DAY_ANNOTATIONS, PipscCalendarDateType};

#[derive(Args, Debug, PartialEq, Arbitrary)]
pub struct CalendarShowArgs {
    /// Year to show (defaults to current year)
    #[arg(value_parser = clap::value_parser!(i32))]
    pub year: Option<i32>,
}

impl CalendarShowArgs {
    pub async fn invoke(self) -> eyre::Result<()> {
        let year = self.year.unwrap_or_else(|| Local::now().year());
        let year_i16: i16 = year as i16;

        // Collect and sort dates for the given year
        let mut dates: Vec<jiff::civil::Date> = PIPSC_CALENDAR_DAY_ANNOTATIONS
            .iter()
            .filter_map(|(d, _)| if d.year() == year_i16 { Some(*d) } else { None })
            .collect();
        dates.sort();

        #[derive(Facet)]
        struct DateEntry {
            date: String,
            attributes: Vec<String>,
        }

        let output: Vec<DateEntry> = dates
            .into_iter()
            .map(|d| {
                let mut attrs: Vec<String> = PIPSC_CALENDAR_DAY_ANNOTATIONS
                    .get(&d)
                    .map(|set| {
                        let mut v: Vec<String> = set
                            .iter()
                            .map(|a| match a {
                                PipscCalendarDateType::Holiday => "Holiday".to_string(),
                                PipscCalendarDateType::PayDate => "PayDate".to_string(),
                                PipscCalendarDateType::CivicHoliday => "CivicHoliday".to_string(),
                                PipscCalendarDateType::NationalQuebecHoliday => {
                                    "NationalQuebecHoliday".to_string()
                                }
                            })
                            .collect();
                        v.sort();
                        v
                    })
                    .unwrap_or_default();

                DateEntry {
                    date: d.to_string(),
                    attributes: attrs,
                }
            })
            .collect();

        println!("{}", facet_json::to_string_pretty(&output)?);

        Ok(())
    }
}

impl ToArgs for CalendarShowArgs {}
