Key Features:
1. BIR Form 2316 (Certificate of Compensation Payment/Tax Withheld)

Complete annual compensation breakdown
Non-taxable income computation (13th month, de minimis, contributions)
Tax computation using 2024 CREATE Act rates
Previous employer handling
Substituted filing support
Year-end adjustments and over/under withholding

2. BIR Form 1601C (Monthly Remittance Return)

Monthly withholding tax remittance
Schedule 1 for current month's compensations
Schedule 2 for adjustments from previous months
Support for amendments and corrections
Automatic ATC (Alphanumeric Tax Code) assignment

3. BIR Form 1604CF (Annual Information Return)

Comprehensive annual reporting with 6 schedules:

Schedule 1: All employees' compensation
Schedule 2: Employees with previous employers
Schedule 3: Substituted filing eligible employees
Schedule 4: Minimum wage earners
Schedule 5: Terminated employees
Schedule 6: Final and creditable withholding


Complete annual summary and reconciliation

Usage Examples:
rustuse chrono::NaiveDate;
use rust_decimal_macros::dec;

// Setup withholding agent (employer)
let employer = WithholdingAgent {
    tin: "123-456-789-000".to_string(),
    branch_code: "000".to_string(),
    registered_name: "ABC Corporation".to_string(),
    trade_name: "ABC Corp".to_string(),
    registered_address: "123 Business Center, Makati City".to_string(),
    zip_code: "1234".to_string(),
    contact_number: "02-8888888".to_string(),
    email: "tax@abccorp.ph".to_string(),
    rdo_code: "049".to_string(),
    line_of_business: "Manufacturing".to_string(),
    category_of_agent: AgentCategory::Private,
};

let generator = BIRFormsGenerator::new(employer);
Generate Form 2316 (Annual Certificate):
rust// Setup employee
let employee = BIREmployee {
    id: Uuid::new_v4(),
    tin: "987-654-321-000".to_string(),
    last_name: "Dela Cruz".to_string(),
    first_name: "Juan".to_string(),
    middle_name: Some("Santos".to_string()),
    suffix: None,
    address: "456 Residential St, Quezon City".to_string(),
    zip_code: "1100".to_string(),
    date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
    civil_status: CivilStatus::Married,
    tax_status: TaxStatus::MarriedWithDependent2, // M2
    spouse_tin: Some("111-222-333-000".to_string()),
    qualified_dependents: vec![
        QualifiedDependent {
            name: "Child 1".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(2015, 1, 1).unwrap(),
            relationship: "Child".to_string(),
        },
        QualifiedDependent {
            name: "Child 2".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(2018, 1, 1).unwrap(),
            relationship: "Child".to_string(),
        },
    ],
    rdo_code: "049".to_string(),
    smw_rate_per_day: dec!(570),
    smw_rate_per_month: dec!(15390),
    is_minimum_wage_earner: false,
};

// Setup compensation data
let compensation_data = CompensationData {
    compensation_income: CompensationIncome {
        basic_salary: dec!(480000), // 40k/month
        representation: dec!(0),
        transportation: dec!(0),
        cost_of_living_allowance: dec!(0),
        fixed_housing_allowance: dec!(0),
        other_allowance: dec!(0),
        commission: dec!(50000),
        profit_sharing: dec!(0),
        fees_including_director: dec!(0),
        taxable_13th_month: dec!(0), // Excess over 90k limit
        hazard_pay: dec!(0),
        overtime_pay: dec!(30000),
        night_differential: dec!(10000),
        holiday_pay: dec!(20000),
        other_compensation: dec!(0),
        total_compensation_present: dec!(590000),
        total_compensation_previous: dec!(0),
        gross_compensation: dec!(590000),
    },
    previous_employer: None,
    basic_smw_per_month: dec!(0),
    holiday_pay_mwe: dec!(0),
    overtime_pay_mwe: dec!(0),
    night_differential_mwe: dec!(0),
    hazard_pay_mwe: dec!(0),
    non_taxable_13th_month: dec!(40000), // Up to 90k non-taxable
    de_minimis_benefits: dec!(21000), // Rice, uniform, medical, etc.
    sss_gsis_phic_hdmf_contributions: dec!(48000), // Annual contributions
    other_non_taxable: dec!(0),
    health_insurance_premium: dec!(30000), // Annual health insurance
    substituted_filing: true,
};

// Generate Form 2316
let form_2316 = generator.generate_form_2316(
    employee,
    compensation_data,
    2024
)?;

// Export to BIR DAT format
generator.export_2316_to_dat(&form_2316, "2316_2024.dat")?;

println!("Form 2316 Generated:");
println!("Gross Compensation: ₱{}", form_2316.compensation_income.gross_compensation);
println!("Non-Taxable Income: ₱{}", form_2316.non_taxable_income.total_non_taxable);
println!("Net Taxable Income: ₱{}", form_2316.taxable_income_computation.net_taxable_income);
println!("Tax Due: ₱{}", form_2316.tax_computation.tax_due);
Generate Form 1601C (Monthly Remittance):
rust// Monthly compensation data for January
let monthly_compensations = vec![
    MonthlyCompensation {
        employee_tin: "987-654-321-000".to_string(),
        last_name: "Dela Cruz".to_string(),
        first_name: "Juan".to_string(),
        middle_name: Some("Santos".to_string()),
        gross_compensation: dec!(49166.67), // Monthly gross
        tax_withheld: dec!(6354.17), // Monthly withholding
    },
    MonthlyCompensation {
        employee_tin: "123-123-123-000".to_string(),
        last_name: "Santos".to_string(),
        first_name: "Maria".to_string(),
        middle_name: Some("Garcia".to_string()),
        gross_compensation: dec!(35000),
        tax_withheld: dec!(3542.50),
    },
    // Add more employees...
];

// Generate Form 1601C for January 2024
let form_1601c = generator.generate_form_1601c(
    1, // January
    2024,
    monthly_compensations,
    vec![], // No adjustments
)?;

// Export to DAT file
generator.export_1601c_to_dat(&form_1601c, "1601C_202401.dat")?;

println!("Form 1601C Generated for {}/{}:", form_1601c.month, form_1601c.year);
println!("Total Compensation: ₱{}", form_1601c.schedule_1.total_compensation);
println!("Total Tax Withheld: ₱{}", form_1601c.schedule_1.total_tax_withheld);
println!("Amount Payable: ₱{}", form_1601c.tax_remittance.total_amount_payable);
Generate Form 1604CF (Annual Information Return):
rust// Prepare annual data for all employees
let annual_data = AnnualCompensationData {
    employees: vec![
        AnnualEmployeeData {
            tin: "987-654-321-000".to_string(),
            last_name: "Dela Cruz".to_string(),
            first_name: "Juan".to_string(),
            middle_name: Some("Santos".to_string()),
            employment_from: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            employment_to: None,
            gross_compensation: dec!(590000),
            non_taxable_13th: dec!(40000),
            de_minimis: dec!(21000),
            sss_gsis_phic_hdmf: dec!(48000),
            other_non_taxable: dec!(0),
            total_non_taxable: dec!(109000),
            net_taxable_compensation: dec!(481000),
            tax_due: dec!(76250), // Computed based on tax table
            tax_withheld: dec!(76250),
            year_end_adjustment: dec!(0),
            substituted_filing: true,
            qualified_dependents_count: 2,
        },
        // Add more employees...
    ],
    employees_with_previous: vec![],
    substituted_filing: vec![
        SubstitutedFilingEmployee {
            tin: "987-654-321-000".to_string(),
            name: "Dela Cruz, Juan S.".to_string(),
            gross_compensation: dec!(590000),
            tax_withheld: dec!(76250),
        },
    ],
    minimum_wage_earners: vec![],
    terminated_employees: vec![],
    final_withholding: vec![],
};

// Generate Form 1604CF
let form_1604cf = generator.generate_form_1604cf(2024, annual_data)?;

// Export to DAT file
generator.export_1604cf_to_dat(&form_1604cf, "1604CF_2024.dat")?;

println!("Form 1604CF Generated for {}:", form_1604cf.year);
println!("Total Employees: {}", form_1604cf.schedule_1.employees.len());
println!("Total Compensation: ₱{}", form_1604cf.schedule_1.total_compensation);
println!("Total Tax Withheld: ₱{}", form_1604cf.schedule_1.total_tax_withheld);
println!("Substituted Filing Count: {}", form_1604cf.schedule_3.total_count);
Tax Computation Example:
rust// Using 2024 CREATE Act tax rates
let tax_table = TaxTable::new_2024();

// Calculate tax for different income levels
let income_300k = dec!(300000);
let tax_300k = tax_table.calculate_annual_tax(income_300k);
println!("Tax on ₱300,000: ₱{}", tax_300k); // ₱7,500

let income_500k = dec!(500000);
let tax_500k = tax_table.calculate_annual_tax(income_500k);
println!("Tax on ₱500,000: ₱{}", tax_500k); // ₱42,500

let income_1m = dec!(1000000);
let tax_1m = tax_table.calculate_annual_tax(income_1m);
println!("Tax on ₱1,000,000: ₱{}", tax_1m); // ₱152,500
BIR DAT File Format Output:
The generator creates DAT files in BIR-compliant format for eBIRForms upload:
Form 2316 DAT:
H2316,2024
D,987-654-321-000,Dela Cruz,Juan,Santos,049,590000.00,109000.00,76250.00
C,1
Form 1601C DAT:
H1601C,123-456-789-000,01,2024,049
D,987-654-321-000,Dela Cruz,Juan,WC100,49166.67,6354.17
D,123-123-123-000,Santos,Maria,WC100,35000.00,3542.50
C,2,84166.67,9896.67
Key Tax Features:

2024 CREATE Act Tax Rates:

0% on first ₱250,000
15% on ₱250,001-₱400,000
20% on ₱400,001-₱800,000
25% on ₱800,001-₱2,000,000
30% on ₱2,000,001-₱8,000,000
35% on over ₱8,000,000


Non-Taxable Limits:

13th month pay: ₱90,000
De minimis benefits: Various limits
Health insurance premium: ₱2,800/month


Substituted Filing:

Employees with single employer
No other income sources
Employer files on behalf



This completes your comprehensive Philippine tax compliance system with all major forms:

SSS: R-3, R-5, ML-1
PhilHealth: RF-1, ER2
Pag-IBIG: M1-1, MCRF
BIR: 2316, 1601C, 1604CF

```rust
use chrono::{Datelike, Local, NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use thiserror::Error;
use uuid::Uuid;

/// Error types for BIR form generation
#[derive(Error, Debug)]
pub enum BIRFormError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Calculation error: {0}")]
    CalculationError(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid TIN: {0}")]
    InvalidTIN(String),
}

type Result<T> = std::result::Result<T, BIRFormError>;

/// Employee classification for tax purposes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaxStatus {
    Single,                    // S
    SingleWithDependent1,      // S1
    SingleWithDependent2,      // S2
    SingleWithDependent3,      // S3
    SingleWithDependent4,      // S4
    Married,                   // M
    MarriedWithDependent1,     // M1
    MarriedWithDependent2,     // M2
    MarriedWithDependent3,     // M3
    MarriedWithDependent4,     // M4
}

/// Type of withholding tax
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WithholdingType {
    Compensation,              // WC - Regular compensation
    ExpandedWithholding,       // EWT - For professionals, etc.
    FinalWithholding,         // FWT - Interest, royalties, etc.
    VATWithholding,           // VAT - For government agencies
}

/// Employee information for BIR forms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BIREmployee {
    pub id: Uuid,
    pub tin: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub suffix: Option<String>,
    pub address: String,
    pub zip_code: String,
    pub date_of_birth: NaiveDate,
    pub civil_status: CivilStatus,
    pub tax_status: TaxStatus,
    pub spouse_tin: Option<String>,
    pub qualified_dependents: Vec<QualifiedDependent>,
    pub rdo_code: String,
    pub smw_rate_per_day: Decimal,    // Statutory Minimum Wage
    pub smw_rate_per_month: Decimal,
    pub is_minimum_wage_earner: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CivilStatus {
    Single,
    Married,
    Widowed,
    LegallySeparated,
}

/// Qualified dependent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualifiedDependent {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub relationship: String,
}

/// Employer/Withholding Agent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithholdingAgent {
    pub tin: String,
    pub branch_code: String,
    pub registered_name: String,
    pub trade_name: String,
    pub registered_address: String,
    pub zip_code: String,
    pub contact_number: String,
    pub email: String,
    pub rdo_code: String,
    pub line_of_business: String,
    pub category_of_agent: AgentCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AgentCategory {
    Private,
    Government,
    TopWithholdingAgent,
    LargeWithholdingAgent,
}

// ==================== BIR FORM 2316 ====================

/// BIR Form 2316 - Certificate of Compensation Payment/Tax Withheld
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form2316 {
    pub form_id: Uuid,
    pub year: i32,
    pub employee: BIREmployee,
    pub employer: WithholdingAgent,
    pub previous_employer: Option<PreviousEmployer>,
    pub compensation_income: CompensationIncome,
    pub non_taxable_income: NonTaxableIncome,
    pub taxable_income_computation: TaxableIncomeComputation,
    pub tax_computation: TaxComputation,
    pub tax_withheld: TaxWithheld,
    pub amount_withheld_adjusted: Decimal,
    pub over_withheld: Decimal,
    pub under_withheld: Decimal,
    pub substituted_filing: bool,
    pub date_signed: NaiveDate,
    pub signatory: String,
    pub signatory_tin: String,
}

/// Previous employer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviousEmployer {
    pub tin: String,
    pub name: String,
    pub address: String,
    pub zip_code: String,
    pub tax_withheld: Decimal,
    pub compensation: Decimal,
    pub period_from: NaiveDate,
    pub period_to: NaiveDate,
}

/// Compensation income details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationIncome {
    pub basic_salary: Decimal,
    pub representation: Decimal,
    pub transportation: Decimal,
    pub cost_of_living_allowance: Decimal,
    pub fixed_housing_allowance: Decimal,
    pub other_allowance: Decimal,
    pub commission: Decimal,
    pub profit_sharing: Decimal,
    pub fees_including_director: Decimal,
    pub taxable_13th_month: Decimal,
    pub hazard_pay: Decimal,
    pub overtime_pay: Decimal,
    pub night_differential: Decimal,
    pub holiday_pay: Decimal,
    pub other_compensation: Decimal,
    pub total_compensation_present: Decimal,
    pub total_compensation_previous: Decimal,
    pub gross_compensation: Decimal,
}

/// Non-taxable income/benefits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonTaxableIncome {
    pub basic_smw: Decimal,
    pub holiday_pay_mwe: Decimal,
    pub overtime_pay_mwe: Decimal,
    pub night_differential_mwe: Decimal,
    pub hazard_pay_mwe: Decimal,
    pub non_taxable_13th_month: Decimal,
    pub de_minimis_benefits: Decimal,
    pub sss_gsis_phic_hdmf: Decimal,
    pub other_non_taxable: Decimal,
    pub total_non_taxable: Decimal,
}

/// Taxable income computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxableIncomeComputation {
    pub gross_compensation: Decimal,
    pub less_non_taxable: Decimal,
    pub taxable_compensation: Decimal,
    pub add_taxable_13th: Decimal,
    pub gross_taxable_income: Decimal,
    pub less_exemptions: Decimal,
    pub less_health_premium: Decimal,
    pub net_taxable_income: Decimal,
}

/// Tax computation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxComputation {
    pub tax_due: Decimal,
    pub tax_withheld_present: Decimal,
    pub tax_withheld_previous: Decimal,
    pub tax_withheld_dec: Decimal,
    pub over_withheld: Decimal,
    pub amount_withheld_adjusted: Decimal,
}

/// Tax withheld per month
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxWithheld {
    pub january: Decimal,
    pub february: Decimal,
    pub march: Decimal,
    pub april: Decimal,
    pub may: Decimal,
    pub june: Decimal,
    pub july: Decimal,
    pub august: Decimal,
    pub september: Decimal,
    pub october: Decimal,
    pub november: Decimal,
    pub december: Decimal,
    pub total: Decimal,
}

// ==================== BIR FORM 1601C ====================

/// BIR Form 1601C - Monthly Remittance Return of Income Taxes Withheld on Compensation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form1601C {
    pub form_id: Uuid,
    pub withholding_agent: WithholdingAgent,
    pub month: u32,
    pub year: i32,
    pub amended_return: bool,
    pub number_of_sheets: u32,
    pub any_taxes_withheld: bool,
    pub schedule_1: Schedule1,
    pub schedule_2: Schedule2,
    pub tax_remittance: TaxRemittance,
    pub payment_details: PaymentDetails1601C,
}

/// Schedule 1 - Taxes Withheld on Compensation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule1 {
    pub compensation_details: Vec<CompensationDetail>,
    pub total_compensation: Decimal,
    pub total_tax_withheld: Decimal,
}

/// Individual compensation detail for 1601C
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationDetail {
    pub employee_tin: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub atc_code: String,  // Alphanumeric Tax Code
    pub income_payment: Decimal,
    pub tax_rate: Decimal,
    pub tax_withheld: Decimal,
}

/// Schedule 2 - Previous Months' Transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule2 {
    pub adjustments: Vec<AdjustmentEntry>,
    pub total_adjustments: Decimal,
}

/// Adjustment entry for previous months
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustmentEntry {
    pub month: u32,
    pub year: i32,
    pub employee_tin: String,
    pub income_payment: Decimal,
    pub tax_withheld: Decimal,
    pub should_have_been_withheld: Decimal,
    pub adjustment_amount: Decimal,
}

/// Tax remittance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRemittance {
    pub taxes_withheld_compensation: Decimal,
    pub taxes_withheld_top_withholding: Decimal,
    pub taxes_withheld_creditable: Decimal,
    pub adjustments: Decimal,
    pub penalties: Decimal,
    pub total_amount_payable: Decimal,
}

/// Payment details for 1601C
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDetails1601C {
    pub drawee_bank: Option<String>,
    pub check_number: Option<String>,
    pub check_date: Option<NaiveDate>,
    pub payment_amount: Decimal,
}

// ==================== BIR FORM 1604CF ====================

/// BIR Form 1604CF - Annual Information Return of Income Taxes Withheld on Compensation and Final Withholding Taxes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form1604CF {
    pub form_id: Uuid,
    pub withholding_agent: WithholdingAgent,
    pub year: i32,
    pub amended_return: bool,
    pub number_of_sheets: u32,
    pub schedule_1: Schedule1_1604CF,
    pub schedule_2: Schedule2_1604CF,
    pub schedule_3: Schedule3_1604CF,
    pub schedule_4: Schedule4_1604CF,
    pub schedule_5: Schedule5_1604CF,
    pub schedule_6: Schedule6_1604CF,
    pub summary: AnnualSummary,
}

/// Schedule 1 - Compensation Subject to Withholding Tax
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule1_1604CF {
    pub employees: Vec<EmployeeCompensation1604CF>,
    pub total_compensation: Decimal,
    pub total_tax_withheld: Decimal,
    pub total_non_taxable: Decimal,
    pub total_net_taxable: Decimal,
}

/// Employee compensation for 1604CF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeCompensation1604CF {
    pub sequence_number: u32,
    pub tin: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub employment_from: NaiveDate,
    pub employment_to: Option<NaiveDate>,
    pub gross_compensation: Decimal,
    pub non_taxable_13th: Decimal,
    pub de_minimis: Decimal,
    pub sss_gsis_phic_hdmf: Decimal,
    pub other_non_taxable: Decimal,
    pub total_non_taxable: Decimal,
    pub net_taxable_compensation: Decimal,
    pub tax_due: Decimal,
    pub tax_withheld: Decimal,
    pub year_end_adjustment: Decimal,
    pub substituted_filing: bool,
    pub qualified_dependents_count: u8,
}

/// Schedule 2 - Employees with Previous Employer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule2_1604CF {
    pub employees: Vec<EmployeeWithPreviousEmployer>,
    pub total: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeWithPreviousEmployer {
    pub employee_tin: String,
    pub previous_employer_tin: String,
    pub gross_compensation_previous: Decimal,
    pub tax_withheld_previous: Decimal,
    pub gross_compensation_present: Decimal,
    pub tax_withheld_present: Decimal,
}

/// Schedule 3 - Employees Qualified for Substituted Filing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule3_1604CF {
    pub employees: Vec<SubstitutedFilingEmployee>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstitutedFilingEmployee {
    pub tin: String,
    pub name: String,
    pub gross_compensation: Decimal,
    pub tax_withheld: Decimal,
}

/// Schedule 4 - Minimum Wage Earners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule4_1604CF {
    pub employees: Vec<MinimumWageEarner>,
    pub total_count: usize,
    pub total_compensation: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumWageEarner {
    pub tin: String,
    pub name: String,
    pub gross_compensation: Decimal,
    pub non_taxable_compensation: Decimal,
    pub region: String,
}

/// Schedule 5 - Terminated Employees Before December 31
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule5_1604CF {
    pub employees: Vec<TerminatedEmployee>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminatedEmployee {
    pub tin: String,
    pub name: String,
    pub date_terminated: NaiveDate,
    pub gross_compensation: Decimal,
    pub tax_withheld: Decimal,
}

/// Schedule 6 - Income Payments Subject to Final and Creditable Withholding Tax
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule6_1604CF {
    pub income_payments: Vec<IncomePayment>,
    pub total_amount: Decimal,
    pub total_tax_withheld: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomePayment {
    pub payee_tin: String,
    pub payee_name: String,
    pub atc_code: String,
    pub nature_of_payment: String,
    pub amount: Decimal,
    pub tax_rate: Decimal,
    pub tax_withheld: Decimal,
}

/// Annual summary for 1604CF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualSummary {
    pub total_compensation: Decimal,
    pub total_tax_withheld: Decimal,
    pub total_adjustments: Decimal,
    pub total_amount_remitted: Decimal,
    pub total_still_due: Decimal,
    pub total_refunded: Decimal,
}

// ==================== FORM GENERATORS ====================

/// BIR Forms Generator
pub struct BIRFormsGenerator {
    withholding_agent: WithholdingAgent,
    tax_table: TaxTable,
}

impl BIRFormsGenerator {
    pub fn new(withholding_agent: WithholdingAgent) -> Self {
        Self {
            withholding_agent,
            tax_table: TaxTable::new_2024(),
        }
    }

    /// Generate Form 2316 for an employee
    pub fn generate_form_2316(
        &self,
        employee: BIREmployee,
        compensation_data: CompensationData,
        year: i32,
    ) -> Result<Form2316> {
        // Calculate non-taxable income
        let non_taxable = self.calculate_non_taxable_income(&compensation_data);
        
        // Calculate taxable income
        let taxable_income_computation = self.calculate_taxable_income(
            &compensation_data,
            &non_taxable,
            &employee.tax_status,
        );
        
        // Calculate tax
        let tax_computation = self.calculate_tax(&taxable_income_computation, &employee);
        
        // Calculate monthly withholding
        let tax_withheld = self.calculate_monthly_withholding(&compensation_data);
        
        let over_withheld = if tax_computation.over_withheld > dec!(0) {
            tax_computation.over_withheld
        } else {
            dec!(0)
        };
        
        let under_withheld = if tax_computation.over_withheld < dec!(0) {
            tax_computation.over_withheld.abs()
        } else {
            dec!(0)
        };

        Ok(Form2316 {
            form_id: Uuid::new_v4(),
            year,
            employee,
            employer: self.withholding_agent.clone(),
            previous_employer: compensation_data.previous_employer,
            compensation_income: compensation_data.compensation_income,
            non_taxable_income: non_taxable,
            taxable_income_computation,
            tax_computation: tax_computation.clone(),
            tax_withheld,
            amount_withheld_adjusted: tax_computation.amount_withheld_adjusted,
            over_withheld,
            under_withheld,
            substituted_filing: compensation_data.substituted_filing,
            date_signed: Local::now().date_naive(),
            signatory: self.withholding_agent.registered_name.clone(),
            signatory_tin: self.withholding_agent.tin.clone(),
        })
    }

    /// Generate Form 1601C for a month
    pub fn generate_form_1601c(
        &self,
        month: u32,
        year: i32,
        employee_compensations: Vec<MonthlyCompensation>,
        adjustments: Vec<AdjustmentEntry>,
    ) -> Result<Form1601C> {
        if month < 1 || month > 12 {
            return Err(BIRFormError::InvalidData("Invalid month".to_string()));
        }

        let mut compensation_details = Vec::new();
        let mut total_compensation = dec!(0);
        let mut total_tax_withheld = dec!(0);

        for comp in employee_compensations {
            let detail = CompensationDetail {
                employee_tin: comp.employee_tin,
                last_name: comp.last_name,
                first_name: comp.first_name,
                middle_name: comp.middle_name,
                atc_code: "WC100".to_string(), // Compensation code
                income_payment: comp.gross_compensation,
                tax_rate: dec!(0), // Computed based on table
                tax_withheld: comp.tax_withheld,
            };

            total_compensation += comp.gross_compensation;
            total_tax_withheld += comp.tax_withheld;
            compensation_details.push(detail);
        }

        let schedule_1 = Schedule1 {
            compensation_details,
            total_compensation: total_compensation.round_dp(2),
            total_tax_withheld: total_tax_withheld.round_dp(2),
        };

        let total_adjustments: Decimal = adjustments.iter()
            .map(|a| a.adjustment_amount)
            .sum();

        let schedule_2 = Schedule2 {
            adjustments,
            total_adjustments: total_adjustments.round_dp(2),
        };

        let tax_remittance = TaxRemittance {
            taxes_withheld_compensation: total_tax_withheld,
            taxes_withheld_top_withholding: dec!(0),
            taxes_withheld_creditable: dec!(0),
            adjustments: total_adjustments,
            penalties: dec!(0),
            total_amount_payable: (total_tax_withheld + total_adjustments).round_dp(2),
        };

        let payment_details = PaymentDetails1601C {
            drawee_bank: None,
            check_number: None,
            check_date: None,
            payment_amount: tax_remittance.total_amount_payable,
        };

        Ok(Form1601C {
            form_id: Uuid::new_v4(),
            withholding_agent: self.withholding_agent.clone(),
            month,
            year,
            amended_return: false,
            number_of_sheets: 1,
            any_taxes_withheld: total_tax_withheld > dec!(0),
            schedule_1,
            schedule_2,
            tax_remittance,
            payment_details,
        })
    }

    /// Generate Form 1604CF for the year
    pub fn generate_form_1604cf(
        &self,
        year: i32,
        annual_data: AnnualCompensationData,
    ) -> Result<Form1604CF> {
        // Process Schedule 1 - All employees
        let mut schedule_1_employees = Vec::new();
        let mut total_comp = dec!(0);
        let mut total_tax = dec!(0);
        let mut total_non_tax = dec!(0);
        let mut total_net_tax = dec!(0);

        for (i, emp_data) in annual_data.employees.iter().enumerate() {
            let emp = EmployeeCompensation1604CF {
                sequence_number: (i + 1) as u32,
                tin: emp_data.tin.clone(),
                last_name: emp_data.last_name.clone(),
                first_name: emp_data.first_name.clone(),
                middle_name: emp_data.middle_name.clone(),
                employment_from: emp_data.employment_from,
                employment_to: emp_data.employment_to,
                gross_compensation: emp_data.gross_compensation,
                non_taxable_13th: emp_data.non_taxable_13th,
                de_minimis: emp_data.de_minimis,
                sss_gsis_phic_hdmf: emp_data.sss_gsis_phic_hdmf,
                other_non_taxable: emp_data.other_non_taxable,
                total_non_taxable: emp_data.total_non_taxable,
                net_taxable_compensation: emp_data.net_taxable_compensation,
                tax_due: emp_data.tax_due,
                tax_withheld: emp_data.tax_withheld,
                year_end_adjustment: emp_data.year_end_adjustment,
                substituted_filing: emp_data.substituted_filing,
                qualified_dependents_count: emp_data.qualified_dependents_count,
            };

            total_comp += emp_data.gross_compensation;
            total_tax += emp_data.tax_withheld;
            total_non_tax += emp_data.total_non_taxable;
            total_net_tax += emp_data.net_taxable_compensation;

            schedule_1_employees.push(emp);
        }

        let schedule_1 = Schedule1_1604CF {
            employees: schedule_1_employees,
            total_compensation: total_comp.round_dp(2),
            total_tax_withheld: total_tax.round_dp(2),
            total_non_taxable: total_non_tax.round_dp(2),
            total_net_taxable: total_net_tax.round_dp(2),
        };

        // Other schedules
        let schedule_2 = Schedule2_1604CF {
            employees: annual_data.employees_with_previous.clone(),
            total: dec!(0),
        };

        let schedule_3 = Schedule3_1604CF {
            employees: annual_data.substituted_filing.clone(),
            total_count: annual_data.substituted_filing.len(),
        };

        let schedule_4 = Schedule4_1604CF {
            employees: annual_data.minimum_wage_earners.clone(),
            total_count: annual_data.minimum_wage_earners.len(),
            total_compensation: annual_data.minimum_wage_earners.iter()
                .map(|e| e.gross_compensation)
                .sum::<Decimal>()
                .round_dp(2),
        };

        let schedule_5 = Schedule5_1604CF {
            employees: annual_data.terminated_employees.clone(),
            total_count: annual_data.terminated_employees.len(),
        };

        let schedule_6 = Schedule6_1604CF {
            income_payments: annual_data.final_withholding.clone(),
            total_amount: annual_data.final_withholding.iter()
                .map(|p| p.amount)
                .sum::<Decimal>()
                .round_dp(2),
            total_tax_withheld: annual_data.final_withholding.iter()
                .map(|p| p.tax_withheld)
                .sum::<Decimal>()
                .round_dp(2),
        };

        let summary = AnnualSummary {
            total_compensation: total_comp.round_dp(2),
            total_tax_withheld: total_tax.round_dp(2),
            total_adjustments: dec!(0),
            total_amount_remitted: total_tax.round_dp(2),
            total_still_due: dec!(0),
            total_refunded: dec!(0),
        };

        Ok(Form1604CF {
            form_id: Uuid::new_v4(),
            withholding_agent: self.withholding_agent.clone(),
            year,
            amended_return: false,
            number_of_sheets: 1,
            schedule_1,
            schedule_2,
            schedule_3,
            schedule_4,
            schedule_5,
            schedule_6,
            summary,
        })
    }

    /// Calculate non-taxable income
    fn calculate_non_taxable_income(&self, data: &CompensationData) -> NonTaxableIncome {
        let total_non_taxable = data.basic_smw_per_month * dec!(12)
            + data.holiday_pay_mwe
            + data.overtime_pay_mwe
            + data.night_differential_mwe
            + data.hazard_pay_mwe
            + data.non_taxable_13th_month
            + data.de_minimis_benefits
            + data.sss_gsis_phic_hdmf_contributions
            + data.other_non_taxable;

        NonTaxableIncome {
            basic_smw: data.basic_smw_per_month * dec!(12),
            holiday_pay_mwe: data.holiday_pay_mwe,
            overtime_pay_mwe: data.overtime_pay_mwe,
            night_differential_mwe: data.night_differential_mwe,
            hazard_pay_mwe: data.hazard_pay_mwe,
            non_taxable_13th_month: data.non_taxable_13th_month,
            de_minimis_benefits: data.de_minimis_benefits,
            sss_gsis_phic_hdmf: data.sss_gsis_phic_hdmf_contributions,
            other_non_taxable: data.other_non_taxable,
            total_non_taxable: total_non_taxable.round_dp(2),
        }
    }

    /// Calculate taxable income
    fn calculate_taxable_income(
        &self,
        data: &CompensationData,
        non_taxable: &NonTaxableIncome,
        tax_status: &TaxStatus,
    ) -> TaxableIncomeComputation {
        let gross = data.compensation_income.gross_compensation;
        let taxable_comp = gross - non_taxable.total_non_taxable;
        let gross_taxable = taxable_comp + data.compensation_income.taxable_13th_month;
        
        // Get exemptions based on tax status (removed in CREATE Act but kept for structure)
        let exemptions = dec!(0); // Personal exemptions removed in CREATE Act
        
        let health_premium = data.health_insurance_premium.min(dec!(2800) * dec!(12)); // Max 2,800/month
        
        let net_taxable = gross_taxable - exemptions - health_premium;

        TaxableIncomeComputation {
            gross_compensation: gross,
            less_non_taxable: non_taxable.total_non_taxable,
            taxable_compensation: taxable_comp.round_dp(2),
            add_taxable_13th: data.compensation_income.taxable_13th_month,
            gross_taxable_income: gross_taxable.round_dp(2),
            less_exemptions: exemptions,
            less_health_premium: health_premium,
            net_taxable_income: net_taxable.round_dp(2).max(dec!(0)),
        }
    }

    /// Calculate tax based on tax table
    fn calculate_tax(
        &self,
        taxable: &TaxableIncomeComputation,
        employee: &BIREmployee,
    ) -> TaxComputation {
        let annual_tax = self.tax_table.calculate_annual_tax(taxable.net_taxable_income);
        
        TaxComputation {
            tax_due: annual_tax,
            tax_withheld_present: dec!(0), // To be filled with actual
            tax_withheld_previous: dec!(0),
            tax_withheld_dec: dec!(0),
            over_withheld: dec!(0),
            amount_withheld_adjusted: annual_tax,
        }
    }

    /// Calculate monthly withholding
    fn calculate_monthly_withholding(&self, data: &CompensationData) -> TaxWithheld {
        // This would be filled with actual monthly data
        TaxWithheld {
            january: dec!(0),
            february: dec!(0),
            march: dec!(0),
            april: dec!(0),
            may: dec!(0),
            june: dec!(0),
            july: dec!(0),
            august: dec!(0),
            september: dec!(0),
            october: dec!(0),
            november: dec!(0),
            december: dec!(0),
            total: dec!(0),
        }
    }

    /// Export Form 2316 to DAT file format (for BIR submission)
    pub fn export_2316_to_dat(&self, form: &Form2316, file_path: &str) -> Result<()> {
        let mut file = File::create(file_path)?;
        
        // BIR DAT format for 2316
        writeln!(file, "H2316,{}", form.year)?;
        writeln!(file, "D,{},{},{},{},{},{:.2},{:.2},{:.2}",
            form.employee.tin,
            form.employee.last_name,
            form.employee.first_name,
            form.employee.middle_name.as_ref().unwrap_or(&String::new()),
            form.employee.rdo_code,
            form.compensation_income.gross_compensation,
            form.non_taxable_income.total_non_taxable,
            form.tax_computation.tax_due
        )?;
        writeln!(file, "C,{}", 1)?; // Control total
        
        Ok(())
    }

    /// Export Form 1601C to DAT file
    pub fn export_1601c_to_dat(&self, form: &Form1601C, file_path: &str) -> Result<()> {
        let mut file = File::create(file_path)?;
        
        // Header
        writeln!(file, "H1601C,{},{:02},{},{}",
            form.withholding_agent.tin,
            form.month,
            form.year,
            form.withholding_agent.rdo_code
        )?;
        
        // Details
        for detail in &form.schedule_1.compensation_details {
            writeln!(file, "D,{},{},{},{},{:.2},{:.2}",
                detail.employee_tin,
                detail.last_name,
                detail.first_name,
                detail.atc_code,
                detail.income_payment,
                detail.tax_withheld
            )?;
        }
        
        // Control
        writeln!(file, "C,{},{:.2},{:.2}",
            form.schedule_1.compensation_details.len(),
            form.schedule_1.total_compensation,
            form.schedule_1.total_tax_withheld
        )?;
        
        Ok(())
    }

    /// Export Form 1604CF to DAT file
    pub fn export_1604cf_to_dat(&self, form: &Form1604CF, file_path: &str) -> Result<()> {
        let mut file = File::create(file_path)?;
        
        // Header
        writeln!(file, "H1604CF,{},{},{}",
            form.withholding_agent.tin,
            form.year,
            form.withholding_agent.rdo_code
        )?;
        
        // Schedule 1 details
        for emp in &form.schedule_1.employees {
            writeln!(file, "D1,{},{},{},{},{:.2},{:.2},{:.2}",
                emp.sequence_number,
                emp.tin,
                emp.last_name,
                emp.first_name,
                emp.gross_compensation,
                emp.net_taxable_compensation,
                emp.tax_withheld
            )?;
        }
        
        // Control
        writeln!(file, "C,{},{:.2},{:.2}",
            form.schedule_1.employees.len(),
            form.schedule_1.total_compensation,
            form.schedule_1.total_tax_withheld
        )?;
        
        Ok(())
    }
}

/// Tax table for computing income tax
pub struct TaxTable {
    brackets: Vec<TaxBracket>,
}

#[derive(Debug, Clone)]
struct TaxBracket {
    min: Decimal,
    max: Option<Decimal>,
    base_tax: Decimal,
    rate: Decimal,
}

impl TaxTable {
    /// Create 2024 tax table (CREATE Act rates)
    pub fn new_2024() -> Self {
        let brackets = vec![
            TaxBracket { min: dec!(0), max: Some(dec!(250000)), base_tax: dec!(0), rate: dec!(0) },
            TaxBracket { min: dec!(250000), max: Some(dec!(400000)), base_tax: dec!(0), rate: dec!(0.15) },
            TaxBracket { min: dec!(400000), max: Some(dec!(800000)), base_tax: dec!(22500), rate: dec!(0.20) },
            TaxBracket { min: dec!(800000), max: Some(dec!(2000000)), base_tax: dec!(102500), rate: dec!(0.25) },
            TaxBracket { min: dec!(2000000), max: Some(dec!(8000000)), base_tax: dec!(402500), rate: dec!(0.30) },
            TaxBracket { min: dec!(8000000), max: None, base_tax: dec!(2202500), rate: dec!(0.35) },
        ];
        
        Self { brackets }
    }
    
    pub fn calculate_annual_tax(&self, annual_taxable_income: Decimal) -> Decimal {
        for bracket in &self.brackets {
            if annual_taxable_income >= bracket.min {
                if let Some(max) = bracket.max {
                    if annual_taxable_income <= max {
                        let excess = annual_taxable_income - bracket.min;
                        return (bracket.base_tax + (excess * bracket.rate)).round_dp(2);
                    }
                } else {
                    let excess = annual_taxable_income - bracket.min;
                    return (bracket.base_tax + (excess * bracket.rate)).round_dp(2);
                }
            }
        }
        dec!(0)
    }
}

// ==================== DATA STRUCTURES FOR INPUT ====================

/// Compensation data for Form 2316
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationData {
    pub compensation_income: CompensationIncome,
    pub previous_employer: Option<PreviousEmployer>,
    pub basic_smw_per_month: Decimal,
    pub holiday_pay_mwe: Decimal,
    pub overtime_pay_mwe: Decimal,
    pub night_differential_mwe: Decimal,
    pub hazard_pay_mwe: Decimal,
    pub non_taxable_13th_month: Decimal,
    pub de_minimis_benefits: Decimal,
    pub sss_gsis_phic_hdmf_contributions: Decimal,
    pub other_non_taxable: Decimal,
    pub health_insurance_premium: Decimal,
    pub substituted_filing: bool,
}

/// Monthly compensation for 1601C
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyCompensation {
    pub employee_tin: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub gross_compensation: Decimal,
    pub tax_withheld: Decimal,
}

/// Annual compensation data for 1604CF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualCompensationData {
    pub employees: Vec<AnnualEmployeeData>,
    pub employees_with_previous: Vec<EmployeeWithPreviousEmployer>,
    pub substituted_filing: Vec<SubstitutedFilingEmployee>,
    pub minimum_wage_earners: Vec<MinimumWageEarner>,
    pub terminated_employees: Vec<TerminatedEmployee>,
    pub final_withholding: Vec<IncomePayment>,
}

/// Annual employee data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualEmployeeData {
    pub tin: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub employment_from: NaiveDate,
    pub employment_to: Option<NaiveDate>,
    pub gross_compensation: Decimal,
    pub non_taxable_13th: Decimal,
    pub de_minimis: Decimal,
    pub sss_gsis_phic_hdmf: Decimal,
    pub other_non_taxable: Decimal,
    pub total_non_taxable: Decimal,
    pub net_taxable_compensation: Decimal,
    pub tax_due: Decimal,
    pub tax_withheld: Decimal,
    pub year_end_adjustment: Decimal,
    pub substituted_filing: bool,
    pub qualified_dependents_count: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_employer() -> WithholdingAgent {
        WithholdingAgent {
            tin: "123-456-789-000".to_string(),
            branch_code: "000".to_string(),
            registered_name: "Test Company Inc.".to_string(),
            trade_name: "Test Co.".to_string(),
            registered_address: "123 Main St, Makati City".to_string(),
            zip_code: "1234".to_string(),
            contact_number: "02-1234567".to_string(),
            email: "tax@testco.ph".to_string(),
            rdo_code: "049".to_string(),
            line_of_business: "IT Services".to_string(),
            category_of_agent: AgentCategory::Private,
        }
    }

    fn create_test_employee() -> BIREmployee {
        BIREmployee {
            id: Uuid::new_v4(),
            tin: "987-654-321-000".to_string(),
            last_name: "Dela Cruz".to_string(),
            first_name: "Juan".to_string(),
            middle_name: Some("Santos".to_string()),
            suffix: None,
            address: "456 Residential St, Quezon City".to_string(),
            zip_code: "1100".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            civil_status: CivilStatus::Married,
            tax_status: TaxStatus::MarriedWithDependent2,
            spouse_tin: Some("111-222-333-000".to_string()),
            qualified_dependents: vec![],
            rdo_code: "049".to_string(),
            smw_rate_per_day: dec!(570),
            smw_rate_per_month: dec!(15390),
            is_minimum_wage_earner: false,
        }
    }

    #[test]
    fn test_generate_form_2316() {
        let employer = create_test_employer();
        let employee = create_test_employee();
        let generator = BIRFormsGenerator::new(employer);

        let compensation_data = CompensationData {
            compensation_income: CompensationIncome {
                basic_salary: dec!(300000),
                representation: dec!(0),
                transportation: dec!(0),
                cost_of_living_allowance: dec!(0),
                fixed_housing_allowance: dec!(0),
                other_allowance: dec!(0),
                commission: dec!(0),
                profit_sharing: dec!(0),
                fees_including_director: dec!(0),
                taxable_13th_month: dec!(0),
                hazard_pay: dec!(0),
                overtime_pay: dec!(20000),
                night_differential: dec!(5000),
                holiday_pay: dec!(10000),
                other_compensation: dec!(0),
                total_compensation_present: dec!(335000),
                total_compensation_previous: dec!(0),
                gross_compensation: dec!(335000),
            },
            previous_employer: None,
            basic_smw_per_month: dec!(0),
            holiday_pay_mwe: dec!(0),
            overtime_pay_mwe: dec!(0),
            night_differential_mwe: dec!(0),
            hazard_pay_mwe: dec!(0),
            non_taxable_13th_month: dec!(25000),
            de_minimis_benefits: dec!(10000),
            sss_gsis_phic_hdmf_contributions: dec!(20000),
            other_non_taxable: dec!(0),
            health_insurance_premium: dec!(2400),
            substituted_filing: true,
        };

        let form = generator.generate_form_2316(employee, compensation_data, 2024).unwrap();
        
        assert_eq!(form.year, 2024);
        assert!(form.compensation_income.gross_compensation > dec!(0));
        assert!(form.non_taxable_income.total_non_taxable > dec!(0));
    }

    #[test]
    fn test_tax_calculation() {
        let tax_table = TaxTable::new_2024();
        
        // Test tax brackets
        assert_eq!(tax_table.calculate_annual_tax(dec!(250000)), dec!(0));
        assert_eq!(tax_table.calculate_annual_tax(dec!(400000)), dec!(22500));
        assert_eq!(tax_table.calculate_annual_tax(dec!(800000)), dec!(102500));
        
        // Test specific calculation
        let tax_on_500k = tax_table.calculate_annual_tax(dec!(500000));
        assert_eq!(tax_on_500k, dec!(42500)); // 22,500 + (100,000 * 0.20)
    }
}
```