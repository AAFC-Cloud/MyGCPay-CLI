use crate::paycheque::ChequeNumber;
use facet::Facet;

pub type PaychequeListResponse = Vec<PaychequeListResponseEntry>;

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct PaychequeListResponseEntry {
    pub department_name: String,
    pub cheque_type: String,
    pub cheque_no: ChequeNumber,
    pub cheque_pay_group: String,
    pub cheque_issued_date_number: String,
    pub cheque_issued_date: String,
    pub cheque_month: String,
    pub cheque_issued_date_short: String,
    pub cheque_start_date: String,
    pub cheque_start_date_unformatted: String,
    pub cheque_end_date: String,
    pub cheque_end_date_unformatted: String,
    pub cheque_total_gross: String,
    pub cheque_total_tax: String,
    pub cheque_total_deduction: String,
    pub cheque_total_net: String,
    pub cheque_total_gross_formated: String,
    pub cheque_total_tax_formated: String,
    pub cheque_total_deduction_formated: String,
    pub cheque_total_net_formated: String,
    pub cheque_account_number: String,
    pub paycheck_option: String,
}
