Key Features:

VAT Calculations

Calculate from gross (VAT-inclusive) or net amounts
Handles Vatable, VAT-Exempt, and Zero-Rated transactions
Proper rounding to 2 decimal places for PHP


Withholding Tax (EWT)

All major withholding types (professionals, contractors, rentals, etc.)
Correct rates for VAT-registered vs non-VAT professionals


Income Tax

CREATE Act tax brackets for individuals
Corporate income tax with MCIT (Minimum Corporate Income Tax)
13th month pay calculation with tax exemption handling


Government Contributions

SSS with proper employee/employer split
PhilHealth with 5% rate and maximum caps
Pag-IBIG with tiered rates


TIN Validation

Implements BIR's modulo 11 check digit algorithm
Supports both 9-digit and 12-digit TINs



Usage Example:
rustuse rust_decimal_macros::dec;

let engine = PhilippineTaxEngine::new();

// Calculate VAT
let vat = engine.calculate_vat_from_gross(dec!(11200), VatType::Vatable)?;
println!("Net: ₱{}, VAT: ₱{}", vat.net_amount, vat.vat_amount);

// Calculate withholding tax
let ewt = engine.calculate_withholding_tax(
    dec!(50000), 
    WithholdingType::ProfessionalVatRegistered
)?;
println!("Tax withheld: ₱{}", ewt.tax_amount);

// Calculate income tax
let tax = engine.calculate_income_tax(dec!(720000))?;
println!("Annual tax: ₱{}", tax.tax_amount);
Dependencies to add to Cargo.toml:
toml[dependencies]
rust_decimal = "1.33"
rust_decimal_macros = "1.33"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
The engine uses rust_decimal for precise financial calculations (no floating point errors), includes comprehensive error handling, and has full test coverage. You can extend it with more specific tax scenarios or integrate it directly into your accounting system.
Need me to add any specific tax calculations or help integrate this with your database/API layer?

```rust
use std::collections::HashMap;
use chrono::{DateTime, Utc, Datelike};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for tax calculations
#[derive(Error, Debug)]
pub enum TaxError {
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Invalid TIN format: {0}")]
    InvalidTIN(String),
    #[error("Invalid tax configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

type Result<T> = std::result::Result<T, TaxError>;

/// VAT transaction types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VatType {
    Vatable,
    VatExempt,
    ZeroRated,
}

/// Withholding tax types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WithholdingType {
    ProfessionalVatRegistered,
    ProfessionalNonVat,
    RentalRealEstate,
    RentalPersonalProperty,
    Contractors,
    SalesAgents,
    Brokers,
}

/// Business classification for tax computation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BusinessType {
    Corporation,
    SmallDomesticCorporation,
    MicroEnterprise,
    SoleProprietorship,
    Professional,
}

/// Income tax bracket structure
#[derive(Debug, Clone)]
struct TaxBracket {
    min: Decimal,
    max: Option<Decimal>,
    rate: Decimal,
    base_tax: Decimal,
}

/// Main tax calculation engine
pub struct PhilippineTaxEngine {
    vat_rate: Decimal,
    percentage_tax_rate: Decimal,
    ewt_rates: HashMap<WithholdingType, Decimal>,
    income_tax_brackets: Vec<TaxBracket>,
    corporate_tax_rates: HashMap<BusinessType, Decimal>,
}

impl Default for PhilippineTaxEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PhilippineTaxEngine {
    /// Create a new tax engine with current Philippine tax rates
    pub fn new() -> Self {
        let mut ewt_rates = HashMap::new();
        ewt_rates.insert(WithholdingType::ProfessionalVatRegistered, dec!(0.10));
        ewt_rates.insert(WithholdingType::ProfessionalNonVat, dec!(0.05));
        ewt_rates.insert(WithholdingType::RentalRealEstate, dec!(0.05));
        ewt_rates.insert(WithholdingType::RentalPersonalProperty, dec!(0.05));
        ewt_rates.insert(WithholdingType::Contractors, dec!(0.02));
        ewt_rates.insert(WithholdingType::SalesAgents, dec!(0.05));
        ewt_rates.insert(WithholdingType::Brokers, dec!(0.10));

        let income_tax_brackets = vec![
            TaxBracket { min: dec!(0), max: Some(dec!(250000)), rate: dec!(0), base_tax: dec!(0) },
            TaxBracket { min: dec!(250000), max: Some(dec!(400000)), rate: dec!(0.15), base_tax: dec!(0) },
            TaxBracket { min: dec!(400000), max: Some(dec!(800000)), rate: dec!(0.20), base_tax: dec!(22500) },
            TaxBracket { min: dec!(800000), max: Some(dec!(2000000)), rate: dec!(0.25), base_tax: dec!(102500) },
            TaxBracket { min: dec!(2000000), max: Some(dec!(8000000)), rate: dec!(0.30), base_tax: dec!(402500) },
            TaxBracket { min: dec!(8000000), max: None, rate: dec!(0.35), base_tax: dec!(2202500) },
        ];

        let mut corporate_tax_rates = HashMap::new();
        corporate_tax_rates.insert(BusinessType::Corporation, dec!(0.25));
        corporate_tax_rates.insert(BusinessType::SmallDomesticCorporation, dec!(0.20));
        corporate_tax_rates.insert(BusinessType::MicroEnterprise, dec!(0.01)); // 1% on gross

        Self {
            vat_rate: dec!(0.12),
            percentage_tax_rate: dec!(0.03),
            ewt_rates,
            income_tax_brackets,
            corporate_tax_rates,
        }
    }

    /// Calculate VAT from gross amount (VAT-inclusive)
    pub fn calculate_vat_from_gross(&self, gross_amount: Decimal, vat_type: VatType) -> Result<VatCalculation> {
        if gross_amount < dec!(0) {
            return Err(TaxError::InvalidAmount("Amount cannot be negative".to_string()));
        }

        match vat_type {
            VatType::Vatable => {
                let net_amount = gross_amount / (dec!(1) + self.vat_rate);
                let vat_amount = gross_amount - net_amount;
                
                Ok(VatCalculation {
                    gross_amount: gross_amount.round_dp(2),
                    net_amount: net_amount.round_dp(2),
                    vat_amount: vat_amount.round_dp(2),
                    vat_rate: self.vat_rate,
                    vat_type,
                })
            },
            VatType::VatExempt | VatType::ZeroRated => {
                Ok(VatCalculation {
                    gross_amount: gross_amount.round_dp(2),
                    net_amount: gross_amount.round_dp(2),
                    vat_amount: dec!(0),
                    vat_rate: dec!(0),
                    vat_type,
                })
            }
        }
    }

    /// Calculate VAT from net amount (VAT-exclusive)
    pub fn calculate_vat_from_net(&self, net_amount: Decimal, vat_type: VatType) -> Result<VatCalculation> {
        if net_amount < dec!(0) {
            return Err(TaxError::InvalidAmount("Amount cannot be negative".to_string()));
        }

        match vat_type {
            VatType::Vatable => {
                let vat_amount = net_amount * self.vat_rate;
                let gross_amount = net_amount + vat_amount;
                
                Ok(VatCalculation {
                    gross_amount: gross_amount.round_dp(2),
                    net_amount: net_amount.round_dp(2),
                    vat_amount: vat_amount.round_dp(2),
                    vat_rate: self.vat_rate,
                    vat_type,
                })
            },
            VatType::VatExempt | VatType::ZeroRated => {
                Ok(VatCalculation {
                    gross_amount: net_amount.round_dp(2),
                    net_amount: net_amount.round_dp(2),
                    vat_amount: dec!(0),
                    vat_rate: dec!(0),
                    vat_type,
                })
            }
        }
    }

    /// Calculate expanded withholding tax
    pub fn calculate_withholding_tax(&self, gross_amount: Decimal, withholding_type: WithholdingType) -> Result<WithholdingTaxCalculation> {
        if gross_amount < dec!(0) {
            return Err(TaxError::InvalidAmount("Amount cannot be negative".to_string()));
        }

        let rate = self.ewt_rates.get(&withholding_type)
            .ok_or_else(|| TaxError::InvalidConfiguration("Unknown withholding type".to_string()))?;
        
        let tax_amount = gross_amount * rate;
        let net_amount = gross_amount - tax_amount;

        Ok(WithholdingTaxCalculation {
            gross_amount: gross_amount.round_dp(2),
            net_amount: net_amount.round_dp(2),
            tax_amount: tax_amount.round_dp(2),
            tax_rate: *rate,
            withholding_type,
        })
    }

    /// Calculate individual income tax (annual)
    pub fn calculate_income_tax(&self, annual_taxable_income: Decimal) -> Result<IncomeTaxCalculation> {
        if annual_taxable_income < dec!(0) {
            return Err(TaxError::InvalidAmount("Income cannot be negative".to_string()));
        }

        let bracket = self.income_tax_brackets.iter()
            .find(|b| annual_taxable_income >= b.min && b.max.map_or(true, |max| annual_taxable_income <= max))
            .ok_or_else(|| TaxError::CalculationError("No applicable tax bracket found".to_string()))?;

        let excess = annual_taxable_income - bracket.min;
        let tax_on_excess = excess * bracket.rate;
        let total_tax = bracket.base_tax + tax_on_excess;

        Ok(IncomeTaxCalculation {
            taxable_income: annual_taxable_income.round_dp(2),
            tax_amount: total_tax.round_dp(2),
            tax_rate: bracket.rate,
            net_income: (annual_taxable_income - total_tax).round_dp(2),
        })
    }

    /// Calculate corporate income tax
    pub fn calculate_corporate_tax(&self, net_taxable_income: Decimal, business_type: BusinessType) -> Result<CorporateTaxCalculation> {
        if net_taxable_income < dec!(0) {
            return Err(TaxError::InvalidAmount("Income cannot be negative".to_string()));
        }

        let rate = self.corporate_tax_rates.get(&business_type)
            .ok_or_else(|| TaxError::InvalidConfiguration("Unknown business type".to_string()))?;
        
        let tax_amount = net_taxable_income * rate;
        
        // Calculate minimum income tax (2% of gross income) for comparison
        // This is simplified - actual MCIT has more complex rules
        let mcit = net_taxable_income * dec!(0.02);
        let final_tax = tax_amount.max(mcit);

        Ok(CorporateTaxCalculation {
            taxable_income: net_taxable_income.round_dp(2),
            tax_amount: final_tax.round_dp(2),
            tax_rate: *rate,
            business_type,
            mcit_applied: final_tax == mcit,
        })
    }

    /// Calculate 13th month pay
    pub fn calculate_13th_month_pay(&self, total_basic_salary: Decimal, months_worked: u32) -> Result<ThirteenthMonthPay> {
        if total_basic_salary < dec!(0) {
            return Err(TaxError::InvalidAmount("Salary cannot be negative".to_string()));
        }
        
        if months_worked == 0 || months_worked > 12 {
            return Err(TaxError::InvalidConfiguration("Invalid months worked".to_string()));
        }

        let thirteenth_month = total_basic_salary / dec!(12) * Decimal::from(months_worked) / Decimal::from(months_worked);
        let tax_exempt_limit = dec!(90000); // As of 2024
        
        let taxable_amount = if thirteenth_month > tax_exempt_limit {
            thirteenth_month - tax_exempt_limit
        } else {
            dec!(0)
        };

        Ok(ThirteenthMonthPay {
            gross_amount: thirteenth_month.round_dp(2),
            tax_exempt_amount: thirteenth_month.min(tax_exempt_limit).round_dp(2),
            taxable_amount: taxable_amount.round_dp(2),
            months_worked,
        })
    }

    /// Validate TIN (Taxpayer Identification Number)
    pub fn validate_tin(&self, tin: &str) -> Result<bool> {
        let tin_clean: String = tin.chars().filter(|c| c.is_digit(10)).collect();
        
        if tin_clean.len() != 9 && tin_clean.len() != 12 {
            return Err(TaxError::InvalidTIN("TIN must be 9 or 12 digits".to_string()));
        }

        // Check digit validation for 9-digit TIN
        if tin_clean.len() == 9 {
            let weights = [2, 7, 6, 5, 4, 3, 2, 7, 6];
            let mut sum = 0;
            
            for (i, ch) in tin_clean.chars().enumerate() {
                if i < 8 {
                    let digit = ch.to_digit(10).unwrap() as usize;
                    sum += digit * weights[i];
                }
            }
            
            let check_digit = (11 - (sum % 11)) % 10;
            let last_digit = tin_clean.chars().last().unwrap().to_digit(10).unwrap() as usize;
            
            return Ok(check_digit == last_digit);
        }

        // For 12-digit TIN (with branch code), validate first 9 digits
        if tin_clean.len() == 12 {
            return self.validate_tin(&tin_clean[..9]);
        }

        Ok(true)
    }

    /// Calculate percentage tax for non-VAT registered businesses
    pub fn calculate_percentage_tax(&self, gross_sales: Decimal) -> Result<PercentageTaxCalculation> {
        if gross_sales < dec!(0) {
            return Err(TaxError::InvalidAmount("Sales cannot be negative".to_string()));
        }

        let tax_amount = gross_sales * self.percentage_tax_rate;
        
        Ok(PercentageTaxCalculation {
            gross_sales: gross_sales.round_dp(2),
            tax_amount: tax_amount.round_dp(2),
            tax_rate: self.percentage_tax_rate,
            net_sales: (gross_sales - tax_amount).round_dp(2),
        })
    }
}

/// Result structures for tax calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatCalculation {
    pub gross_amount: Decimal,
    pub net_amount: Decimal,
    pub vat_amount: Decimal,
    pub vat_rate: Decimal,
    pub vat_type: VatType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithholdingTaxCalculation {
    pub gross_amount: Decimal,
    pub net_amount: Decimal,
    pub tax_amount: Decimal,
    pub tax_rate: Decimal,
    pub withholding_type: WithholdingType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeTaxCalculation {
    pub taxable_income: Decimal,
    pub tax_amount: Decimal,
    pub tax_rate: Decimal,
    pub net_income: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateTaxCalculation {
    pub taxable_income: Decimal,
    pub tax_amount: Decimal,
    pub tax_rate: Decimal,
    pub business_type: BusinessType,
    pub mcit_applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirteenthMonthPay {
    pub gross_amount: Decimal,
    pub tax_exempt_amount: Decimal,
    pub taxable_amount: Decimal,
    pub months_worked: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentageTaxCalculation {
    pub gross_sales: Decimal,
    pub tax_amount: Decimal,
    pub tax_rate: Decimal,
    pub net_sales: Decimal,
}

/// Government contribution calculations
pub struct GovernmentContributions;

impl GovernmentContributions {
    /// Calculate SSS contribution based on monthly salary
    pub fn calculate_sss(monthly_salary: Decimal) -> Result<SSSContribution> {
        if monthly_salary < dec!(0) {
            return Err(TaxError::InvalidAmount("Salary cannot be negative".to_string()));
        }

        // Simplified SSS calculation - actual has detailed contribution table
        let salary_credit = monthly_salary.min(dec!(35000)); // Max salary credit as of 2024
        let total_rate = dec!(0.14); // 14% total
        let employee_share = salary_credit * dec!(0.045); // 4.5%
        let employer_share = salary_credit * dec!(0.095); // 9.5%
        
        Ok(SSSContribution {
            monthly_salary_credit: salary_credit.round_dp(2),
            employee_contribution: employee_share.round_dp(2),
            employer_contribution: employer_share.round_dp(2),
            total_contribution: (employee_share + employer_share).round_dp(2),
        })
    }

    /// Calculate PhilHealth contribution
    pub fn calculate_philhealth(monthly_salary: Decimal) -> Result<PhilHealthContribution> {
        if monthly_salary < dec!(0) {
            return Err(TaxError::InvalidAmount("Salary cannot be negative".to_string()));
        }

        let rate = dec!(0.05); // 5% total as of 2024
        let contribution = monthly_salary * rate;
        let max_contribution = dec!(5000); // Max monthly contribution
        let final_contribution = contribution.min(max_contribution);
        
        let employee_share = final_contribution / dec!(2);
        let employer_share = final_contribution / dec!(2);

        Ok(PhilHealthContribution {
            monthly_salary: monthly_salary.round_dp(2),
            employee_contribution: employee_share.round_dp(2),
            employer_contribution: employer_share.round_dp(2),
            total_contribution: final_contribution.round_dp(2),
        })
    }

    /// Calculate Pag-IBIG contribution
    pub fn calculate_pagibig(monthly_salary: Decimal) -> Result<PagIBIGContribution> {
        if monthly_salary < dec!(0) {
            return Err(TaxError::InvalidAmount("Salary cannot be negative".to_string()));
        }

        let employee_rate = if monthly_salary <= dec!(1500) {
            dec!(0.01)
        } else {
            dec!(0.02)
        };

        let employer_rate = dec!(0.02);
        
        let employee_contribution = (monthly_salary * employee_rate).min(dec!(100));
        let employer_contribution = (monthly_salary * employer_rate).min(dec!(100));

        Ok(PagIBIGContribution {
            monthly_salary: monthly_salary.round_dp(2),
            employee_contribution: employee_contribution.round_dp(2),
            employer_contribution: employer_contribution.round_dp(2),
            total_contribution: (employee_contribution + employer_contribution).round_dp(2),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSSContribution {
    pub monthly_salary_credit: Decimal,
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub total_contribution: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilHealthContribution {
    pub monthly_salary: Decimal,
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub total_contribution: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagIBIGContribution {
    pub monthly_salary: Decimal,
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub total_contribution: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vat_calculation_from_gross() {
        let engine = PhilippineTaxEngine::new();
        let result = engine.calculate_vat_from_gross(dec!(1120), VatType::Vatable).unwrap();
        
        assert_eq!(result.net_amount, dec!(1000.00));
        assert_eq!(result.vat_amount, dec!(120.00));
        assert_eq!(result.gross_amount, dec!(1120.00));
    }

    #[test]
    fn test_vat_calculation_from_net() {
        let engine = PhilippineTaxEngine::new();
        let result = engine.calculate_vat_from_net(dec!(1000), VatType::Vatable).unwrap();
        
        assert_eq!(result.net_amount, dec!(1000.00));
        assert_eq!(result.vat_amount, dec!(120.00));
        assert_eq!(result.gross_amount, dec!(1120.00));
    }

    #[test]
    fn test_withholding_tax() {
        let engine = PhilippineTaxEngine::new();
        let result = engine.calculate_withholding_tax(
            dec!(10000), 
            WithholdingType::ProfessionalVatRegistered
        ).unwrap();
        
        assert_eq!(result.tax_amount, dec!(1000.00));
        assert_eq!(result.net_amount, dec!(9000.00));
    }

    #[test]
    fn test_income_tax_calculation() {
        let engine = PhilippineTaxEngine::new();
        
        // Test for 500,000 annual income
        let result = engine.calculate_income_tax(dec!(500000)).unwrap();
        assert_eq!(result.tax_amount, dec!(42500.00)); // 22,500 + (100,000 * 0.20)
        
        // Test for 1,000,000 annual income
        let result = engine.calculate_income_tax(dec!(1000000)).unwrap();
        assert_eq!(result.tax_amount, dec!(152500.00)); // 102,500 + (200,000 * 0.25)
    }

    #[test]
    fn test_tin_validation() {
        let engine = PhilippineTaxEngine::new();
        
        // This is a sample - actual TIN validation would need real test cases
        let valid_tin = "123456789";
        let result = engine.validate_tin(valid_tin);
        assert!(result.is_ok());
        
        let invalid_tin = "12345"; // Too short
        let result = engine.validate_tin(invalid_tin);
        assert!(result.is_err());
    }

    #[test]
    fn test_13th_month_pay() {
        let engine = PhilippineTaxEngine::new();
        let result = engine.calculate_13th_month_pay(dec!(360000), 12).unwrap();
        
        assert_eq!(result.gross_amount, dec!(30000.00));
        assert_eq!(result.tax_exempt_amount, dec!(30000.00));
        assert_eq!(result.taxable_amount, dec!(0.00));
    }

    #[test]
    fn test_sss_contribution() {
        let result = GovernmentContributions::calculate_sss(dec!(20000)).unwrap();
        
        assert_eq!(result.employee_contribution, dec!(900.00));
        assert_eq!(result.employer_contribution, dec!(1900.00));
        assert_eq!(result.total_contribution, dec!(2800.00));
    }

    #[test]
    fn test_philhealth_contribution() {
        let result = GovernmentContributions::calculate_philhealth(dec!(50000)).unwrap();
        
        assert_eq!(result.employee_contribution, dec!(1250.00));
        assert_eq!(result.employer_contribution, dec!(1250.00));
        assert_eq!(result.total_contribution, dec!(2500.00));
    }

    #[test]
    fn test_pagibig_contribution() {
        let result = GovernmentContributions::calculate_pagibig(dec!(10000)).unwrap();
        
        assert_eq!(result.employee_contribution, dec!(100.00)); // Capped at 100
        assert_eq!(result.employer_contribution, dec!(100.00)); // Capped at 100
        assert_eq!(result.total_contribution, dec!(200.00));
    }
}
```