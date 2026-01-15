use crate::paycheque::PaychequeListResponseEntry;
use facet::Facet;

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct PaychequeShowResponse {
    pub full_name: String,
    pub pri: String,

    #[facet(rename = "single_paycheque")]
    pub single_paycheque: PaychequeListResponseEntry,

    pub earning_balances: Vec<EarningBalance>,
    pub tax_balances: Vec<TaxBalance>,
    pub deduction_balances: Vec<DeductionBalance>,
    pub paycheque_earnings: Vec<PaychequeEarning>,
    pub paycheque_other_earnings: Vec<PaychequeOtherEarning>,
    pub paycheque_taxes: Vec<PaychequeTax>,
    pub paycheque_deductions: Vec<PaychequeDeduction>,
}

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct EarningBalance {
    pub company: String,
    pub balance_id: String,
    pub balance_year: String,
    pub wage_loss_replacement_plan: String,
    pub province: String,
    pub balance_quarter: String,
    pub balance_period: String,
    pub record_id: String,
    pub special_earning_balance: String,
    pub earning_code: String,
    pub earning_code_en: String,
    pub earning_code_fr: String,
    pub hours_year_to_date: String,
    pub hours_quarter_to_date: String,
    pub hours_month_to_date: String,
    pub gross_earning_year_to_date: String,
    pub gross_earning_quarter_to_date: String,
    pub gross_earning_month_to_date: String,
    #[facet(rename = "subjectCIT")]
    pub subject_cit: String,
    pub add_to_gross: String,
}

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct TaxBalance {
    pub company: String,
    pub balance_id: String,
    pub balance_year: String,
    pub wage_loss_plan: String,
    pub province: String,
    pub balance_quarter: String,
    pub balance_period: String,
    pub canadian_tax_class: String,
    pub canadian_tax_class_en: String,
    pub canadian_tax_class_fr: String,
    pub no_limit_gross_year_to_date: String,
    pub no_limit_gross_quarter_to_date: String,
    pub no_limit_gross_month_to_date: String,
    pub taxable_gross_year_to_date: String,
    pub taxable_gross_quarter_to_date: String,
    pub taxable_gross_month_to_date: String,
    pub tax_year_to_date: String,
    pub tax_quarter_to_date: String,
    pub tax_month_to_date: String,
}
#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct DeductionBalance {
    pub balance_id: String,
    pub balance_year: String,
    pub wage_loss_replacement_plan: String,
    pub province: String,
    pub balance_quarter: String,
    pub balance_period: String,
    pub benefit_record_number: String,
    pub plan_type: String,
    pub benefit_plan: String,
    pub deduction_code: String,
    pub deduction_code_en: String,
    pub deduction_code_fr: String,
    pub deduction_classification: String,
    pub sales_tax_type: String,
    pub company: String,
    pub deduction_balance_year_to_date: String,
    pub deduction_balance_quarter_to_date: String,
    pub deduction_balance_month_to_date: String,
}

#[derive(Facet)]
pub struct PaychequeEarning {
    #[facet(rename = "recordId")]
    pub record_id: String,
    #[facet(rename = "earningsBeginDate")]
    pub earnings_begin_date: String,
    #[facet(rename = "earningsEndDate")]
    pub earnings_end_date: String,
    #[facet(rename = "earningsBeginDateNumber")]
    pub earnings_begin_date_number: String,
    #[facet(rename = "earningsBeginDateUnformatted")]
    pub earnings_begin_date_unformatted: String,
    #[facet(rename = "earningsEndDateUnformatted")]
    pub earnings_end_date_unformatted: String,
    #[facet(rename = "okToPay")]
    pub ok_to_pay: String,
    #[facet(rename = "earningCodeRegularHours")]
    pub earning_code_regular_hours: String,
    #[facet(rename = "earningCodeOvertimeHours")]
    pub earning_code_overtime_hours: String,
    #[facet(rename = "regularPayHours")]
    pub regular_pay_hours: String,
    #[facet(rename = "regularHours")]
    pub regular_hours: String,
    #[facet(rename = "overtimeHours")]
    pub overtime_hours: String,
    #[facet(rename = "regularHourlyEarnings")]
    pub regular_hourly_earnings: String,
    #[facet(rename = "overtimeHourlyEarnings")]
    pub overtime_hourly_earnings: String,
    #[facet(rename = "earningCode")]
    pub earning_code: String,
    #[facet(rename = "regularPay")]
    pub regular_pay: String,
    #[facet(rename = "regularEarnings")]
    pub regular_earnings: String,
    #[facet(rename = "regularEarningHours")]
    pub regular_earning_hours: String,
    #[facet(rename = "deductionTaken")]
    pub deduction_taken: String,
    #[facet(rename = "deductionSubsetId")]
    pub deduction_subset_id: String,
    #[facet(rename = "seperateCheckNumber")]
    pub seperate_check_number: String,
    #[facet(rename = "benefitRecordNumber")]
    pub benefit_record_number: String,
    #[facet(rename = "additionalPayReason")]
    pub additional_pay_reason: String,
    #[facet(rename = "disableDirectDeposit")]
    pub disable_direct_deposit: String,
    #[facet(rename = "grossUp")]
    pub gross_up: String,
    #[facet(rename = "payLineCalculationStatus")]
    pub pay_line_calculation_status: String,
    #[facet(rename = "jobPay")]
    pub job_pay: String,
    #[facet(rename = "singleChequeUse")]
    pub single_cheque_use: String,
    #[facet(rename = "combinationCode")]
    pub combination_code: String,
    #[facet(rename = "generalLedgerPayType")]
    pub general_ledger_pay_type: String,
    #[facet(rename = "departmentId")]
    pub department_id: String,
    #[facet(rename = "jobCode")]
    pub job_code: String,
    #[facet(rename = "positionNumber")]
    pub position_number: String,
    #[facet(rename = "regularShift")]
    pub regular_shift: String,
    #[facet(rename = "shiftDifferentialRate")]
    pub shift_differential_rate: String,
    #[facet(rename = "flsaRate")]
    pub flsa_rate: String,
    #[facet(rename = "rateUsed")]
    pub rate_used: String,
    #[facet(rename = "rateUsedFormattedEn")]
    pub rate_used_formatted_en: String,
    #[facet(rename = "rateUsedFormattedFr")]
    pub rate_used_formatted_fr: String,
    #[facet(rename = "flsaRequired")]
    pub flsa_required: String,
    #[facet(rename = "generalDeductionTaken")]
    pub general_deduction_taken: String,
    #[facet(rename = "generalDeductionSubset")]
    pub general_deduction_subset: String,
    #[facet(rename = "state")]
    pub state: String,
    #[facet(rename = "locality")]
    pub locality: String,
    #[facet(rename = "payFrequency")]
    pub pay_frequency: String,
    #[facet(rename = "taxPeriods")]
    pub tax_periods: String,
    #[facet(rename = "additionalTaxes")]
    pub additional_taxes: String,
    #[facet(rename = "overrideHourlyRateIndicator")]
    pub override_hourly_rate_indicator: String,
    #[facet(rename = "timeAndLaborSource")]
    pub time_and_labor_source: String,
    #[facet(rename = "paySheetSource")]
    pub pay_sheet_source: String,
    #[facet(rename = "businessUnit")]
    pub business_unit: String,
    #[facet(rename = "eiPriorPeriodCorrection")]
    pub ei_prior_period_correction: String,
    #[facet(rename = "compensationRateRegular")]
    pub compensation_rate_regular: String,
    #[facet(rename = "compensationRateOvertime")]
    pub compensation_rate_overtime: String,
    #[facet(rename = "compensationRateRegularUsed")]
    pub compensation_rate_regular_used: String,
    #[facet(rename = "compensationRateOvertimeUsed")]
    pub compensation_rate_overtime_used: String,
    #[facet(rename = "ficaStatusEmployee")]
    pub fica_status_employee: String,
    #[facet(rename = "annualTaxPeriods")]
    pub annual_tax_periods: String,
    #[facet(rename = "flsaEndDate")]
    pub flsa_end_date: String,
    #[facet(rename = "originalPaygroup")]
    pub original_paygroup: String,
    #[facet(rename = "flsaStatus")]
    pub flsa_status: String,
    #[facet(rename = "xrefNumber")]
    pub xref_number: String,
    #[facet(flatten)] // https://github.com/facet-rs/facet/issues/1791
    pub more: PaychequeEarningMore,
}

#[derive(Facet)]
pub struct PaychequeEarningMore {
    #[facet(rename = "unionCode")]
    pub union_code: String,
    #[facet(rename = "benefitDeductionStatus")]
    pub benefit_deduction_status: String,
    #[facet(rename = "generalDeductionStatus")]
    pub general_deduction_status: String,
    #[facet(rename = "contractNumber")]
    pub contract_number: String,
    #[facet(rename = "contractSequence")]
    pub contract_sequence: String,
    #[facet(rename = "payGroup")]
    pub pay_group: String,
    #[facet(rename = "payPeriodEndDate")]
    pub pay_period_end_date: String,
    #[facet(rename = "offCycle")]
    pub off_cycle: String,
    #[facet(rename = "pageNumber")]
    pub page_number: String,
    #[facet(rename = "lineNumber")]
    pub line_number: String,
    #[facet(rename = "taxMethod")]
    pub tax_method: String,
    #[facet(rename = "netClaimAmount")]
    pub net_claim_amount: String,
    #[facet(rename = "specialLetters")]
    pub special_letters: String,
    #[facet(rename = "additionalAmount")]
    pub additional_amount: String,
    #[facet(rename = "additionalPercentage")]
    pub additional_percentage: String,
    #[facet(rename = "QITNetClaim")]
    pub qit_net_claim: String,
    #[facet(rename = "QITSpecialLetters")]
    pub qit_special_letters: String,
    #[facet(rename = "QITAdditionalPercentage")]
    pub qit_additional_percentage: String,
    #[facet(rename = "QITAdditionalAmount")]
    pub qit_additional_amount: String,
    #[facet(rename = "CITClaimAmount")]
    pub cit_claim_amount: String,
    #[facet(rename = "subjectCIT")]
    pub subject_cit: String,
    #[facet(rename = "OTsubjectCIT")]
    pub o_tsubject_cit: String,
    #[facet(rename = "OTearningCodeEn")]
    pub o_tearning_code_en: String,
    #[facet(rename = "OTearningCodeFr")]
    pub o_tearning_code_fr: String,
    #[facet(rename = "earningCodeEn")]
    pub earning_code_en: String,
    #[facet(rename = "earningCodeFr")]
    pub earning_code_fr: String,
    #[facet(rename = "AddToGross")]
    pub add_to_gross: String,
}

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct PaychequeOtherEarning {
    pub earning_code: String,
    pub earning_code_en: String,
    pub earning_code_fr: String,
    pub rate_used: String,
    pub rate_used_formatted_en: String,
    pub rate_used_formatted_fr: String,
    pub job_pay: String,
    pub regular_earning_hours: String,
    pub regular_earnings: String,
    pub other_earnings: String,
    pub add_to_gross: Option<String>,
    pub tax_method: Option<String>,
    pub additional_pay_line_number: String,
    pub seperate_check_number: String,
    pub additional_pay_sequence_number: String,
    pub time_and_labor_source: Option<String>,
    pub compensation_rate_code: Option<String>,
    pub compensation_rate: String,
    pub compensation_rate_regular_used: String,
    pub hours_distribution: Option<String>,
    pub xref_number: String,
    pub expense_document_id: Option<String>,
    pub expense_document_type: Option<String>,
    pub expense_line_number: String,
    pub currency_code: Option<String>,
    pub variable_compensation_plan: Option<String>,
    pub payout_period_id: Option<String>,
    pub group_id: Option<String>,
    pub applicant_id: Option<String>,
    pub award_date: Option<String>,
    pub e_recruit_status: String,
    pub sim_sequence_number: Option<String>,
    pub paysheet_update_source: Option<String>,
    pub retro_pay_sequence_number: Option<String>,
    pub earnings_end_date: String,
    pub earnings_begin_date: String,
    pub earnings_end_date_unformatted: String,
    pub earnings_begin_date_unformatted: String,
    pub earnings_begin_date_number: String,
    pub pay_group: String,
    pub pay_period_end_date: String,
    pub off_cycle: String,
    pub page_number: String,
    pub line_number: String,
    #[facet(rename = "emplID")]
    pub empl_id: String,
    #[facet(rename = "recordID")]
    pub record_id: String,
    #[facet(rename = "subjectCIT")]
    pub subject_cit: String,
}

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct PaychequeTax {
    pub pay_group: String,
    pub pay_period_end_date: String,
    pub off_cycle: String,
    pub page_number: String,
    pub line_number: String,
    pub seperate_check_number: String,
    pub paycheck_number: String,
    #[facet(rename = "emplID")]
    pub empl_id: String,
    pub empl_record: String,
    pub province: String,
    pub canadian_tax_class: String,
    pub canadian_tax_class_fr: String,
    pub canadian_tax_class_en: String,
    pub wage_loss_replacement_plan: String,
    pub current_no_limit_gross: String,
    pub current_taxable_gross: String,
    pub current_tax: String,
    pub tax_not_taken: String,
    pub el_employer_rate: String,
    pub ap_status: String,
}

#[derive(Facet)]
#[facet(rename_all = "camelCase")]
pub struct PaychequeDeduction {
    pub record_id: String,
    pub cheque_number: String,
    pub seperate_check_number: String,
    pub pay_period_end_date: String,
    pub pay_group: String,
    pub off_cycle: String,
    pub page_number: String,
    pub line_number: String,
    pub benefit_record_number: String,
    pub plan_type: String,
    pub benefit_plan: String,
    pub deduction_code: String,
    pub deduction_code_en: String,
    pub deduction_code_fr: String,
    pub deduction_classification: String,
    pub sales_tax_type: String,
    pub current_deduction: String,
    pub current_deduction_pay_back: String,
    pub current_deduction_refund: String,
    pub deduction_not_taken: String,
    pub reason_not_taken: String,
    pub calculated_base: String,
    pub ap_status: String,
    #[facet(rename = "emplID")]
    pub empl_id: String,
}
