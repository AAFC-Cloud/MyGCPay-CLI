use facet::Facet;

use crate::paycheque::ChequeNumber;

pub type PaychequeListResponse = Vec<PaychequeListResponseEntry>;

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct PaychequeListResponseEntry {
    department_name: String,
    cheque_type: String,
    cheque_no: ChequeNumber,
    cheque_pay_group: String,
    cheque_issued_date_number: String,
    cheque_issued_date: String,
    cheque_month: String,
    cheque_issued_date_short: String,
    cheque_start_date: String,
    cheque_start_date_unformatted: String,
    cheque_end_date: String,
    cheque_end_date_unformatted: String,
    cheque_total_gross: String,
    cheque_total_tax: String,
    cheque_total_deduction: String,
    cheque_total_net: String,
    cheque_total_gross_formated: String,
    cheque_total_tax_formated: String,
    cheque_total_deduction_formated: String,
    cheque_total_net_formated: String,
    cheque_account_number: String,
    paycheck_option: String,
}
