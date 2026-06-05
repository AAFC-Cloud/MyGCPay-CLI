use crate::net::CacheKey;
use crate::net::ClientExt;
use crate::net::HasCacheKey;
use crate::net::MaybeCached;
use crate::net::create_client;
use crate::paycheque::PaychequeListResponseEntry;
use crate::paycheque::PaychequeListResponse;
use eyre::Context;
use facet::Facet;
use std::path::PathBuf;
use std::pin::Pin;

fn parse_extra_years(body: &str) -> eyre::Result<Vec<i32>> {
    let mut years = facet_json::from_str::<std::collections::BTreeMap<String, ()>>(body)
        .wrap_err("failed to parse extra pay years")?
        .into_keys()
        .map(|year| {
            year.parse::<i32>()
                .wrap_err_with(|| format!("failed to parse year `{year}` as an integer"))
        })
        .collect::<eyre::Result<Vec<_>>>()?;
    years.sort_unstable();
    Ok(years)
}

pub struct PaychequeListRequest;

pub struct PaychequeExtraYearsRequest;

pub struct PaychequeExtraYearRequest {
    pub year: i32,
}

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct PaychequeExtraYearEnvelope {
    pub data: Vec<PaychequeListResponseEntry>,
    pub all_data_loaded: bool,
}

pub enum PaychequeExtraYearResponse {
    Paycheques(PaychequeListResponse),
    Envelope(PaychequeExtraYearEnvelope),
}

impl PaychequeExtraYearResponse {
    pub fn paycheques(self) -> PaychequeListResponse {
        match self {
            Self::Paycheques(paycheques) => paycheques,
            Self::Envelope(envelope) => envelope.data,
        }
    }

    pub fn all_data_loaded(&self) -> bool {
        match self {
            Self::Paycheques(_) => false,
            Self::Envelope(envelope) => envelope.all_data_loaded,
        }
    }

    pub fn count(&self) -> usize {
        match self {
            Self::Paycheques(paycheques) => paycheques.len(),
            Self::Envelope(envelope) => envelope.data.len(),
        }
    }
}

fn parse_extra_year_response(body: &str) -> eyre::Result<PaychequeExtraYearResponse> {
    match body.trim_start().chars().next() {
        Some('[') => Ok(PaychequeExtraYearResponse::Paycheques(
            facet_json::from_str(body).wrap_err("failed to parse extra paycheque list")?,
        )),
        Some('{') => Ok(PaychequeExtraYearResponse::Envelope(
            facet_json::from_str(body).wrap_err("failed to parse extra paycheque envelope")?,
        )),
        _ => eyre::bail!("unexpected extra-year response shape"),
    }
}

impl HasCacheKey for PaychequeListRequest {
    fn cache_key(&self) -> CacheKey {
        CacheKey::new(PathBuf::from_iter(["paycheque", "list"]))
    }
}

impl IntoFuture for PaychequeListRequest {
    type Output = eyre::Result<MaybeCached<PaychequeListResponse>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paycheque-data/";

            let client = create_client().await?;
            let req = client.post(url).form(&[("type", "list")]).build()?;
            let resp = client.execute_cached(req, &self.cache_key()).await?;
            Ok(resp)
        })
    }
}

impl HasCacheKey for PaychequeExtraYearsRequest {
    fn cache_key(&self) -> CacheKey {
        CacheKey::new(PathBuf::from_iter(["paycheque", "list", "extra-years"]))
    }
}

impl IntoFuture for PaychequeExtraYearsRequest {
    type Output = eyre::Result<MaybeCached<Vec<i32>>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paycheque-data/";

            let client = create_client().await?;
            let req = client.post(url).form(&[("type", "pie"), ("year", "list")]).build()?;
            let resp = client.execute_cached_raw(req, &self.cache_key()).await?;
            let years = parse_extra_years(&resp.body)?;

            Ok(match resp {
                MaybeCached::FromCache(_) => MaybeCached::FromCache(years),
                MaybeCached::FromNetwork(_) => MaybeCached::FromNetwork(years),
            })
        })
    }
}

impl HasCacheKey for PaychequeExtraYearRequest {
    fn cache_key(&self) -> CacheKey {
        CacheKey::new(PathBuf::from_iter([
            "paycheque",
            "list",
            "extra-year",
            &self.year.to_string(),
        ]))
    }
}

impl IntoFuture for PaychequeExtraYearRequest {
    type Output = eyre::Result<MaybeCached<PaychequeExtraYearResponse>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/paycheque-data/";
            let start_date = format!("{}-01-01", self.year);
            let end_date = format!("{}-12-31", self.year);

            let client = create_client().await?;
            let req = client
                .post(url)
                .form(&[
                    ("type", "getExtraYear"),
                    ("startdate", start_date.as_str()),
                    ("enddate", end_date.as_str()),
                ])
                .build()?;
            let resp = client.execute_cached_raw(req, &self.cache_key()).await?;
            let parsed = parse_extra_year_response(&resp.body)?;
            Ok(match resp {
                MaybeCached::FromCache(_) => MaybeCached::FromCache(parsed),
                MaybeCached::FromNetwork(_) => MaybeCached::FromNetwork(parsed),
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::PaychequeExtraYearResponse;
    use super::parse_extra_year_response;
    use super::parse_extra_years;

    #[test]
    fn parse_extra_years_returns_sorted_years() -> eyre::Result<()> {
        let years = parse_extra_years(r#"{"2026":{},"2024":{},"2025":{}}"#)?;
        assert_eq!(years, vec![2024, 2025, 2026]);
        Ok(())
    }

    #[test]
    fn parse_extra_year_response_handles_array_payload() -> eyre::Result<()> {
        let response = parse_extra_year_response(
            r#"[{"departmentName":"Agriculture and Agri-Food","chequeType":"Basic Pay","chequeNo":"123","chequePayGroup":"TX1","chequeIssuedDateNumber":"20250101","chequeIssuedDate":"January 01, 2025","chequeMonth":"January 2025","chequeIssuedDateShort":"01/01/2025","chequeStartDate":"December 19, 2024","chequeStartDateUnformatted":"2024/12/19","chequeEndDate":"January 01, 2025","chequeEndDateUnformatted":"2025/01/01","chequeTotalGross":"1.00","chequeTotalTax":"0.00","chequeTotalDeduction":"0.00","chequeTotalNet":"1.00","chequeTotalGrossFormated":"$1.00","chequeTotalTaxFormated":"$0.00","chequeTotalDeductionFormated":"$0.00","chequeTotalNetFormated":"$1.00","chequeAccountNumber":"0","paycheckOption":"A"}]"#,
        )?;

        assert!(matches!(response, PaychequeExtraYearResponse::Paycheques(_)));
        assert_eq!(response.count(), 1);
        assert!(!response.all_data_loaded());
        Ok(())
    }

    #[test]
    fn parse_extra_year_response_handles_completion_envelope() -> eyre::Result<()> {
        let response = parse_extra_year_response(r#"{"data":[],"allDataLoaded":true}"#)?;

        assert!(matches!(response, PaychequeExtraYearResponse::Envelope(_)));
        assert_eq!(response.count(), 0);
        assert!(response.all_data_loaded());
        Ok(())
    }
}
