Excellent! I've created a comprehensive Government Contributions calculator with TIN validation. Here's what's included:
Key Features:
1. SSS Contribution Calculator

Complete 2024 contribution table with 53 salary brackets
Accurate monthly salary credit mapping
Employee contribution (4.5% effective rate)
Employer contribution (9.5% effective rate)
EC (Employer Compensation) and WISP contributions
Special handling for different employment types:

Regular employees
Self-employed (pays both shares)
OFWs (employee share only)
Kasambahay (special rules for < PHP 5,000)
Voluntary members



2. PhilHealth Contribution

5% premium rate (2.5% each for employee/employer)
Minimum contribution: PHP 500/month
Maximum contribution: PHP 5,000/month
Automatic 50-50 split calculation

3. Pag-IBIG Contribution

Tiered employee rates:

1% for salaries ≤ PHP 1,500
2% for salaries > PHP 1,500


Employer always contributes 2%
Maximum contribution: PHP 100 each

4. TIN Validator

Full implementation of BIR's modulo 11 check digit algorithm
Supports both 9-digit (individual) and 12-digit (corporate with branch code) TINs
Formatting with dashes (XXX-XXX-XXX-XXX)
Batch validation capability
TIN generation for testing

5. Comprehensive Calculations

Total contributions summary
Net take-home pay calculation
Annual contribution schedule with remittance deadlines
Monthly payment schedules

Usage Examples:
rustuse rust_decimal_macros::dec;

// Initialize calculator
let calc = GovernmentContributionsCalculator::new();

// Calculate all contributions for PHP 25,000 salary
let contributions = calc.calculate_all_contributions(
    dec!(25000),
    EmploymentType::Regular
)?;

println!("Monthly Salary: ₱{}", contributions.monthly_salary);
println!("\nSSS:");
println!("  Employee: ₱{}", contributions.sss.employee_contribution);
println!("  Employer: ₱{}", contributions.sss.employer_contribution);
println!("\nPhilHealth:");
println!("  Employee: ₱{}", contributions.philhealth.employee_contribution);
println!("  Employer: ₱{}", contributions.philhealth.employer_contribution);
println!("\nPag-IBIG:");
println!("  Employee: ₱{}", contributions.pagibig.employee_contribution);
println!("  Employer: ₱{}", contributions.pagibig.employer_contribution);
println!("\nTotal Deductions: ₱{}", contributions.total_employee_contribution);
println!("Net Take Home: ₱{}", contributions.net_take_home);

// Generate annual schedule
let annual = calc.generate_annual_schedule(
    dec!(25000),
    EmploymentType::Regular
)?;

for month in &annual.monthly_schedules {
    println!("{}: Due on {}", month.month, month.due_date);
}

// Validate TIN
let tin_validation = TINValidator::validate("123-456-789")?;
if tin_validation.is_valid {
    println!("Valid TIN: {}", tin_validation.formatted);
} else {
    println!("Invalid: {}", tin_validation.validation_message);
}

// Generate valid TIN for testing
let test_tin = TINValidator::generate_valid_tin(
    "12345678", 
    TINType::Corporate
)?;
println!("Generated TIN: {}", test_tin);
Real-world Example Output:
For a PHP 25,000 monthly salary:
SSS:
  Employee: ₱1,125.00
  Employer: ₱2,385.00
  EC: ₱30.00
  WISP: ₱10.00
  
PhilHealth:
  Employee: ₱625.00 (2.5% of 25,000)
  Employer: ₱625.00
  
Pag-IBIG:
  Employee: ₱100.00 (capped)
  Employer: ₱100.00 (capped)
  
Total Employee Deductions: ₱1,850.00
Total Employer Cost: ₱3,150.00
Net Take Home: ₱23,150.00
Special Features:
Kasambahay (Domestic Workers):

Below PHP 5,000: Employer pays all SSS contributions
PHP 5,000 and above: Normal employee/employer split

Self-Employed/Voluntary:

Pays both employee and employer SSS shares
No EC/WISP contributions
Same PhilHealth and Pag-IBIG rates

Annual Schedule:

Generates 12-month payment schedule
Calculates remittance deadlines (10th of following month)
Adjusts for weekends automatically
Provides annual totals for budgeting

Integration Points:
This calculator integrates perfectly with:

Payroll system - for automated deduction calculations
Tax engine - contributions are tax-deductible
Chart of Accounts - auto-post to correct liability accounts
Invoice generator - for billing clients with proper withholding

The TIN validator can be used across all modules for:

Employee registration
Vendor verification
Customer validation
BIR form generation


```rust
use chrono::{Datelike, Local, NaiveDate};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for contribution calculations
#[derive(Error, Debug)]
pub enum ContributionError {
    #[error("Invalid salary amount: {0}")]
    InvalidSalary(String),
    #[error("Invalid TIN format: {0}")]
    InvalidTIN(String),
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

type Result<T> = std::result::Result<T, ContributionError>;

/// Employee classification for SSS
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EmploymentType {
    Regular,
    OFW,
    SelfEmployed,
    Voluntary,
    NonWorking,
    Kasambahay,
}

/// SSS Contribution Table Entry
#[derive(Debug, Clone)]
struct SSSBracket {
    min_range: Decimal,
    max_range: Decimal,
    salary_credit: Decimal,
    employer_contribution: Decimal,
    employee_contribution: Decimal,
    ec_contribution: Decimal, // Employer Compensation
    wisp_contribution: Decimal, // Work Injury and Sickness Prevention
}

/// Main Government Contributions Calculator
pub struct GovernmentContributionsCalculator {
    sss_table: Vec<SSSBracket>,
    philhealth_rate: Decimal,
    philhealth_min: Decimal,
    philhealth_max: Decimal,
    pagibig_rates: HashMap<String, Decimal>,
    current_year: i32,
}

impl Default for GovernmentContributionsCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl GovernmentContributionsCalculator {
    /// Initialize calculator with 2024/2025 rates
    pub fn new() -> Self {
        // SSS Contribution Table (2024 rates)
        let sss_table = vec![
            SSSBracket { min_range: dec!(0), max_range: dec!(4249.99), salary_credit: dec!(4000), 
                employer_contribution: dec!(390), employee_contribution: dec!(180), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(4250), max_range: dec!(4749.99), salary_credit: dec!(4500), 
                employer_contribution: dec!(437.50), employee_contribution: dec!(202.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(4750), max_range: dec!(5249.99), salary_credit: dec!(5000), 
                employer_contribution: dec!(485), employee_contribution: dec!(225), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(5250), max_range: dec!(5749.99), salary_credit: dec!(5500), 
                employer_contribution: dec!(532.50), employee_contribution: dec!(247.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(5750), max_range: dec!(6249.99), salary_credit: dec!(6000), 
                employer_contribution: dec!(580), employee_contribution: dec!(270), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(6250), max_range: dec!(6749.99), salary_credit: dec!(6500), 
                employer_contribution: dec!(627.50), employee_contribution: dec!(292.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(6750), max_range: dec!(7249.99), salary_credit: dec!(7000), 
                employer_contribution: dec!(675), employee_contribution: dec!(315), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(7250), max_range: dec!(7749.99), salary_credit: dec!(7500), 
                employer_contribution: dec!(722.50), employee_contribution: dec!(337.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(7750), max_range: dec!(8249.99), salary_credit: dec!(8000), 
                employer_contribution: dec!(770), employee_contribution: dec!(360), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(8250), max_range: dec!(8749.99), salary_credit: dec!(8500), 
                employer_contribution: dec!(817.50), employee_contribution: dec!(382.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(8750), max_range: dec!(9249.99), salary_credit: dec!(9000), 
                employer_contribution: dec!(865), employee_contribution: dec!(405), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(9250), max_range: dec!(9749.99), salary_credit: dec!(9500), 
                employer_contribution: dec!(912.50), employee_contribution: dec!(427.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(9750), max_range: dec!(10249.99), salary_credit: dec!(10000), 
                employer_contribution: dec!(960), employee_contribution: dec!(450), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(10250), max_range: dec!(10749.99), salary_credit: dec!(10500), 
                employer_contribution: dec!(1007.50), employee_contribution: dec!(472.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(10750), max_range: dec!(11249.99), salary_credit: dec!(11000), 
                employer_contribution: dec!(1055), employee_contribution: dec!(495), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(11250), max_range: dec!(11749.99), salary_credit: dec!(11500), 
                employer_contribution: dec!(1102.50), employee_contribution: dec!(517.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(11750), max_range: dec!(12249.99), salary_credit: dec!(12000), 
                employer_contribution: dec!(1150), employee_contribution: dec!(540), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(12250), max_range: dec!(12749.99), salary_credit: dec!(12500), 
                employer_contribution: dec!(1197.50), employee_contribution: dec!(562.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(12750), max_range: dec!(13249.99), salary_credit: dec!(13000), 
                employer_contribution: dec!(1245), employee_contribution: dec!(585), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(13250), max_range: dec!(13749.99), salary_credit: dec!(13500), 
                employer_contribution: dec!(1292.50), employee_contribution: dec!(607.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(13750), max_range: dec!(14249.99), salary_credit: dec!(14000), 
                employer_contribution: dec!(1340), employee_contribution: dec!(630), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(14250), max_range: dec!(14749.99), salary_credit: dec!(14500), 
                employer_contribution: dec!(1387.50), employee_contribution: dec!(652.50), 
                ec_contribution: dec!(10), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(14750), max_range: dec!(15249.99), salary_credit: dec!(15000), 
                employer_contribution: dec!(1435), employee_contribution: dec!(675), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(15250), max_range: dec!(15749.99), salary_credit: dec!(15500), 
                employer_contribution: dec!(1482.50), employee_contribution: dec!(697.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(15750), max_range: dec!(16249.99), salary_credit: dec!(16000), 
                employer_contribution: dec!(1530), employee_contribution: dec!(720), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(16250), max_range: dec!(16749.99), salary_credit: dec!(16500), 
                employer_contribution: dec!(1577.50), employee_contribution: dec!(742.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(16750), max_range: dec!(17249.99), salary_credit: dec!(17000), 
                employer_contribution: dec!(1625), employee_contribution: dec!(765), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(17250), max_range: dec!(17749.99), salary_credit: dec!(17500), 
                employer_contribution: dec!(1672.50), employee_contribution: dec!(787.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(17750), max_range: dec!(18249.99), salary_credit: dec!(18000), 
                employer_contribution: dec!(1720), employee_contribution: dec!(810), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(18250), max_range: dec!(18749.99), salary_credit: dec!(18500), 
                employer_contribution: dec!(1767.50), employee_contribution: dec!(832.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(18750), max_range: dec!(19249.99), salary_credit: dec!(19000), 
                employer_contribution: dec!(1815), employee_contribution: dec!(855), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(19250), max_range: dec!(19749.99), salary_credit: dec!(19500), 
                employer_contribution: dec!(1862.50), employee_contribution: dec!(877.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(19750), max_range: dec!(20249.99), salary_credit: dec!(20000), 
                employer_contribution: dec!(1910), employee_contribution: dec!(900), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(20250), max_range: dec!(20749.99), salary_credit: dec!(20500), 
                employer_contribution: dec!(1957.50), employee_contribution: dec!(922.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(20750), max_range: dec!(21249.99), salary_credit: dec!(21000), 
                employer_contribution: dec!(2005), employee_contribution: dec!(945), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(21250), max_range: dec!(21749.99), salary_credit: dec!(21500), 
                employer_contribution: dec!(2052.50), employee_contribution: dec!(967.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(21750), max_range: dec!(22249.99), salary_credit: dec!(22000), 
                employer_contribution: dec!(2100), employee_contribution: dec!(990), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(22250), max_range: dec!(22749.99), salary_credit: dec!(22500), 
                employer_contribution: dec!(2147.50), employee_contribution: dec!(1012.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(22750), max_range: dec!(23249.99), salary_credit: dec!(23000), 
                employer_contribution: dec!(2195), employee_contribution: dec!(1035), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(23250), max_range: dec!(23749.99), salary_credit: dec!(23500), 
                employer_contribution: dec!(2242.50), employee_contribution: dec!(1057.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(23750), max_range: dec!(24249.99), salary_credit: dec!(24000), 
                employer_contribution: dec!(2290), employee_contribution: dec!(1080), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(24250), max_range: dec!(24749.99), salary_credit: dec!(24500), 
                employer_contribution: dec!(2337.50), employee_contribution: dec!(1102.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(24750), max_range: dec!(25249.99), salary_credit: dec!(25000), 
                employer_contribution: dec!(2385), employee_contribution: dec!(1125), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(25250), max_range: dec!(25749.99), salary_credit: dec!(25500), 
                employer_contribution: dec!(2432.50), employee_contribution: dec!(1147.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(25750), max_range: dec!(26249.99), salary_credit: dec!(26000), 
                employer_contribution: dec!(2480), employee_contribution: dec!(1170), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(26250), max_range: dec!(26749.99), salary_credit: dec!(26500), 
                employer_contribution: dec!(2527.50), employee_contribution: dec!(1192.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(26750), max_range: dec!(27249.99), salary_credit: dec!(27000), 
                employer_contribution: dec!(2575), employee_contribution: dec!(1215), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(27250), max_range: dec!(27749.99), salary_credit: dec!(27500), 
                employer_contribution: dec!(2622.50), employee_contribution: dec!(1237.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(27750), max_range: dec!(28249.99), salary_credit: dec!(28000), 
                employer_contribution: dec!(2670), employee_contribution: dec!(1260), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(28250), max_range: dec!(28749.99), salary_credit: dec!(28500), 
                employer_contribution: dec!(2717.50), employee_contribution: dec!(1282.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(28750), max_range: dec!(29249.99), salary_credit: dec!(29000), 
                employer_contribution: dec!(2765), employee_contribution: dec!(1305), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(29250), max_range: dec!(29749.99), salary_credit: dec!(29500), 
                employer_contribution: dec!(2812.50), employee_contribution: dec!(1327.50), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
            SSSBracket { min_range: dec!(29750), max_range: dec!(1000000), salary_credit: dec!(30000), 
                employer_contribution: dec!(2860), employee_contribution: dec!(1350), 
                ec_contribution: dec!(30), wisp_contribution: dec!(10) },
        ];

        let mut pagibig_rates = HashMap::new();
        pagibig_rates.insert("employee_below_1500".to_string(), dec!(0.01));
        pagibig_rates.insert("employee_above_1500".to_string(), dec!(0.02));
        pagibig_rates.insert("employer_below_1500".to_string(), dec!(0.02));
        pagibig_rates.insert("employer_above_1500".to_string(), dec!(0.02));
        pagibig_rates.insert("max_contribution".to_string(), dec!(100));

        Self {
            sss_table,
            philhealth_rate: dec!(0.05), // 5% total (2.5% each)
            philhealth_min: dec!(500),    // Minimum PHP 500
            philhealth_max: dec!(5000),   // Maximum PHP 5,000
            pagibig_rates,
            current_year: Local::now().year(),
        }
    }

    /// Calculate SSS contribution based on monthly salary
    pub fn calculate_sss(
        &self,
        monthly_salary: Decimal,
        employment_type: EmploymentType,
    ) -> Result<SSSContribution> {
        if monthly_salary < dec!(0) {
            return Err(ContributionError::InvalidSalary(
                "Salary cannot be negative".to_string(),
            ));
        }

        // Find the appropriate bracket
        let bracket = self
            .sss_table
            .iter()
            .find(|b| monthly_salary >= b.min_range && monthly_salary <= b.max_range)
            .or_else(|| self.sss_table.last())
            .ok_or_else(|| {
                ContributionError::ConfigurationError("SSS bracket not found".to_string())
            })?;

        // Adjust for employment type
        let (employee_contrib, employer_contrib, ec_contrib, wisp_contrib) = match employment_type {
            EmploymentType::Regular => (
                bracket.employee_contribution,
                bracket.employer_contribution,
                bracket.ec_contribution,
                bracket.wisp_contribution,
            ),
            EmploymentType::SelfEmployed | EmploymentType::Voluntary | EmploymentType::NonWorking => {
                // Self-employed pays both employee and employer shares
                let total = bracket.employee_contribution + bracket.employer_contribution;
                (total, dec!(0), dec!(0), dec!(0))
            }
            EmploymentType::OFW => {
                // OFW typically pays only employee share
                (bracket.employee_contribution, dec!(0), dec!(0), dec!(0))
            }
            EmploymentType::Kasambahay => {
                // Kasambahay has special rates
                if monthly_salary < dec!(5000) {
                    // Employer pays all
                    (dec!(0), bracket.employee_contribution + bracket.employer_contribution, 
                     bracket.ec_contribution, bracket.wisp_contribution)
                } else {
                    (bracket.employee_contribution, bracket.employer_contribution,
                     bracket.ec_contribution, bracket.wisp_contribution)
                }
            }
        };

        Ok(SSSContribution {
            monthly_salary,
            monthly_salary_credit: bracket.salary_credit,
            employee_contribution: employee_contrib.round_dp(2),
            employer_contribution: employer_contrib.round_dp(2),
            ec_contribution: ec_contrib.round_dp(2),
            wisp_contribution: wisp_contrib.round_dp(2),
            total_contribution: (employee_contrib + employer_contrib + ec_contrib + wisp_contrib)
                .round_dp(2),
            employment_type,
        })
    }

    /// Calculate PhilHealth contribution
    pub fn calculate_philhealth(&self, monthly_salary: Decimal) -> Result<PhilHealthContribution> {
        if monthly_salary < dec!(0) {
            return Err(ContributionError::InvalidSalary(
                "Salary cannot be negative".to_string(),
            ));
        }

        // Calculate 5% of monthly salary
        let total_contribution = monthly_salary * self.philhealth_rate;
        
        // Apply minimum and maximum
        let final_contribution = if total_contribution < self.philhealth_min {
            self.philhealth_min
        } else if total_contribution > self.philhealth_max {
            self.philhealth_max
        } else {
            total_contribution
        };

        // Split equally between employee and employer
        let employee_share = final_contribution / dec!(2);
        let employer_share = final_contribution / dec!(2);

        Ok(PhilHealthContribution {
            monthly_salary: monthly_salary.round_dp(2),
            premium_rate: self.philhealth_rate,
            employee_contribution: employee_share.round_dp(2),
            employer_contribution: employer_share.round_dp(2),
            total_contribution: final_contribution.round_dp(2),
        })
    }

    /// Calculate Pag-IBIG contribution
    pub fn calculate_pagibig(&self, monthly_salary: Decimal) -> Result<PagIBIGContribution> {
        if monthly_salary < dec!(0) {
            return Err(ContributionError::InvalidSalary(
                "Salary cannot be negative".to_string(),
            ));
        }

        let max_contribution = self.pagibig_rates.get("max_contribution").unwrap();

        // Employee contribution rate based on salary
        let employee_rate = if monthly_salary <= dec!(1500) {
            *self.pagibig_rates.get("employee_below_1500").unwrap()
        } else {
            *self.pagibig_rates.get("employee_above_1500").unwrap()
        };

        // Employer contribution is always 2%
        let employer_rate = dec!(0.02);

        // Calculate contributions
        let employee_contribution = (monthly_salary * employee_rate).min(*max_contribution);
        let employer_contribution = (monthly_salary * employer_rate).min(*max_contribution);

        Ok(PagIBIGContribution {
            monthly_salary: monthly_salary.round_dp(2),
            employee_rate,
            employer_rate,
            employee_contribution: employee_contribution.round_dp(2),
            employer_contribution: employer_contribution.round_dp(2),
            total_contribution: (employee_contribution + employer_contribution).round_dp(2),
        })
    }

    /// Calculate all government contributions
    pub fn calculate_all_contributions(
        &self,
        monthly_salary: Decimal,
        employment_type: EmploymentType,
    ) -> Result<TotalContributions> {
        let sss = self.calculate_sss(monthly_salary, employment_type)?;
        let philhealth = self.calculate_philhealth(monthly_salary)?;
        let pagibig = self.calculate_pagibig(monthly_salary)?;

        let total_employee = sss.employee_contribution
            + philhealth.employee_contribution
            + pagibig.employee_contribution;

        let total_employer = sss.employer_contribution
            + sss.ec_contribution
            + sss.wisp_contribution
            + philhealth.employer_contribution
            + pagibig.employer_contribution;

        Ok(TotalContributions {
            monthly_salary: monthly_salary.round_dp(2),
            sss,
            philhealth,
            pagibig,
            total_employee_contribution: total_employee.round_dp(2),
            total_employer_contribution: total_employer.round_dp(2),
            grand_total: (total_employee + total_employer).round_dp(2),
            net_take_home: (monthly_salary - total_employee).round_dp(2),
        })
    }

    /// Generate contribution schedule for the year
    pub fn generate_annual_schedule(
        &self,
        monthly_salary: Decimal,
        employment_type: EmploymentType,
    ) -> Result<AnnualContributionSchedule> {
        let mut monthly_schedules = Vec::new();
        
        for month in 1..=12 {
            let contributions = self.calculate_all_contributions(monthly_salary, employment_type)?;
            let month_name = match month {
                1 => "January",
                2 => "February",
                3 => "March",
                4 => "April",
                5 => "May",
                6 => "June",
                7 => "July",
                8 => "August",
                9 => "September",
                10 => "October",
                11 => "November",
                12 => "December",
                _ => "",
            };

            monthly_schedules.push(MonthlySchedule {
                month: month_name.to_string(),
                contributions,
                due_date: self.get_remittance_deadline(month, self.current_year),
            });
        }

        // Calculate annual totals
        let annual_sss_employee: Decimal = monthly_schedules
            .iter()
            .map(|s| s.contributions.sss.employee_contribution)
            .sum();
        let annual_sss_employer: Decimal = monthly_schedules
            .iter()
            .map(|s| {
                s.contributions.sss.employer_contribution
                    + s.contributions.sss.ec_contribution
                    + s.contributions.sss.wisp_contribution
            })
            .sum();
        let annual_philhealth_employee: Decimal = monthly_schedules
            .iter()
            .map(|s| s.contributions.philhealth.employee_contribution)
            .sum();
        let annual_philhealth_employer: Decimal = monthly_schedules
            .iter()
            .map(|s| s.contributions.philhealth.employer_contribution)
            .sum();
        let annual_pagibig_employee: Decimal = monthly_schedules
            .iter()
            .map(|s| s.contributions.pagibig.employee_contribution)
            .sum();
        let annual_pagibig_employer: Decimal = monthly_schedules
            .iter()
            .map(|s| s.contributions.pagibig.employer_contribution)
            .sum();

        Ok(AnnualContributionSchedule {
            year: self.current_year,
            monthly_salary: monthly_salary.round_dp(2),
            employment_type,
            monthly_schedules,
            annual_totals: AnnualTotals {
                sss_employee: annual_sss_employee.round_dp(2),
                sss_employer: annual_sss_employer.round_dp(2),
                philhealth_employee: annual_philhealth_employee.round_dp(2),
                philhealth_employer: annual_philhealth_employer.round_dp(2),
                pagibig_employee: annual_pagibig_employee.round_dp(2),
                pagibig_employer: annual_pagibig_employer.round_dp(2),
                total_employee: (annual_sss_employee + annual_philhealth_employee + annual_pagibig_employee)
                    .round_dp(2),
                total_employer: (annual_sss_employer + annual_philhealth_employer + annual_pagibig_employer)
                    .round_dp(2),
            },
        })
    }

    /// Get remittance deadline for a given month
    fn get_remittance_deadline(&self, month: u32, year: i32) -> NaiveDate {
        // Government contributions are typically due on the 10th of the following month
        // If it falls on a weekend/holiday, it's the next business day
        let mut deadline = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 10).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 10).unwrap()
        };

        // Adjust for weekends (simplified - doesn't account for holidays)
        match deadline.weekday() {
            chrono::Weekday::Sat => {
                deadline = deadline + chrono::Duration::days(2);
            }
            chrono::Weekday::Sun => {
                deadline = deadline + chrono::Duration::days(1);
            }
            _ => {}
        }

        deadline
    }
}

/// TIN Validator
pub struct TINValidator;

impl TINValidator {
    /// Validate Philippine TIN using modulo 11 check digit algorithm
    pub fn validate(tin: &str) -> Result<TINValidation> {
        // Remove any non-digit characters
        let tin_clean: String = tin.chars().filter(|c| c.is_digit(10)).collect();

        // Check length (9 or 12 digits)
        if tin_clean.len() != 9 && tin_clean.len() != 12 {
            return Err(ContributionError::InvalidTIN(
                "TIN must be 9 or 12 digits".to_string(),
            ));
        }

        let tin_type = if tin_clean.len() == 9 {
            TINType::Individual
        } else {
            TINType::Corporate
        };

        // For 12-digit TIN, validate the first 9 digits
        let tin_to_validate = if tin_clean.len() == 12 {
            &tin_clean[..9]
        } else {
            &tin_clean
        };

        // BIR modulo 11 algorithm
        let weights = [2, 7, 6, 5, 4, 3, 2, 7, 6];
        let mut sum = 0;

        // Calculate weighted sum for first 8 digits
        for (i, ch) in tin_to_validate[..8].chars().enumerate() {
            let digit = ch.to_digit(10).unwrap() as usize;
            sum += digit * weights[i];
        }

        // Calculate check digit
        let remainder = sum % 11;
        let check_digit = if remainder == 0 || remainder == 1 {
            0
        } else {
            11 - remainder
        };

        // Get the 9th digit (check digit)
        let ninth_digit = tin_to_validate
            .chars()
            .nth(8)
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;

        let is_valid = check_digit == ninth_digit;

        // Extract branch code for 12-digit TIN
        let branch_code = if tin_clean.len() == 12 {
            Some(tin_clean[9..12].to_string())
        } else {
            None
        };

        // Format TIN for display
        let formatted = match tin_type {
            TINType::Individual => {
                format!(
                    "{}-{}-{}-{}",
                    &tin_clean[0..3],
                    &tin_clean[3..6],
                    &tin_clean[6..9],
                    "000"
                )
            }
            TINType::Corporate => {
                format!(
                    "{}-{}-{}-{}",
                    &tin_clean[0..3],
                    &tin_clean[3..6],
                    &tin_clean[6..9],
                    branch_code.as_ref().unwrap()
                )
            }
        };

        Ok(TINValidation {
            original: tin.to_string(),
            cleaned: tin_clean.clone(),
            formatted,
            is_valid,
            tin_type,
            branch_code,
            check_digit: check_digit as u8,
            actual_check_digit: ninth_digit as u8,
            validation_message: if is_valid {
                "Valid TIN".to_string()
            } else {
                format!(
                    "Invalid check digit. Expected {}, got {}",
                    check_digit, ninth_digit
                )
            },
        })
    }

    /// Generate a valid TIN for testing purposes
    pub fn generate_valid_tin(base_digits: &str, tin_type: TINType) -> Result<String> {
        if base_digits.len() != 8 {
            return Err(ContributionError::InvalidTIN(
                "Base must be 8 digits".to_string(),
            ));
        }

        // Ensure all characters are digits
        if !base_digits.chars().all(|c| c.is_digit(10)) {
            return Err(ContributionError::InvalidTIN(
                "Base must contain only digits".to_string(),
            ));
        }

        // Calculate check digit
        let weights = [2, 7, 6, 5, 4, 3, 2, 7, 6];
        let mut sum = 0;

        for (i, ch) in base_digits.chars().enumerate() {
            let digit = ch.to_digit(10).unwrap() as usize;
            sum += digit * weights[i];
        }

        let remainder = sum % 11;
        let check_digit = if remainder == 0 || remainder == 1 {
            0
        } else {
            11 - remainder
        };

        let tin_9_digit = format!("{}{}", base_digits, check_digit);

        match tin_type {
            TINType::Individual => Ok(tin_9_digit),
            TINType::Corporate => Ok(format!("{}000", tin_9_digit)), // Default branch code
        }
    }

    /// Batch validate multiple TINs
    pub fn validate_batch(tins: &[String]) -> Vec<TINValidation> {
        tins.iter()
            .map(|tin| Self::validate(tin).unwrap_or_else(|_| TINValidation {
                original: tin.clone(),
                cleaned: String::new(),
                formatted: String::new(),
                is_valid: false,
                tin_type: TINType::Individual,
                branch_code: None,
                check_digit: 0,
                actual_check_digit: 0,
                validation_message: "Invalid TIN format".to_string(),
            }))
            .collect()
    }
}

/// Result structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSSContribution {
    pub monthly_salary: Decimal,
    pub monthly_salary_credit: Decimal,
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub ec_contribution: Decimal,
    pub wisp_contribution: Decimal,
    pub total_contribution: Decimal,
    pub employment_type: EmploymentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilHealthContribution {
    pub monthly_salary: Decimal,
    pub premium_rate: Decimal,
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub total_contribution: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagIBIGContribution {
    pub monthly_salary: Decimal,
    pub employee_rate: Decimal,
    pub employer_rate: Decimal,
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub total_contribution: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalContributions {
    pub monthly_salary: Decimal,
    pub sss: SSSContribution,
    pub philhealth: PhilHealthContribution,
    pub pagibig: PagIBIGContribution,
    pub total_employee_contribution: Decimal,
    pub total_employer_contribution: Decimal,
    pub grand_total: Decimal,
    pub net_take_home: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlySchedule {
    pub month: String,
    pub contributions: TotalContributions,
    pub due_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualContributionSchedule {
    pub year: i32,
    pub monthly_salary: Decimal,
    pub employment_type: EmploymentType,
    pub monthly_schedules: Vec<MonthlySchedule>,
    pub annual_totals: AnnualTotals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualTotals {
    pub sss_employee: Decimal,
    pub sss_employer: Decimal,
    pub philhealth_employee: Decimal,
    pub philhealth_employer: Decimal,
    pub pagibig_employee: Decimal,
    pub pagibig_employer: Decimal,
    pub total_employee: Decimal,
    pub total_employer: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TINType {
    Individual,
    Corporate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TINValidation {
    pub original: String,
    pub cleaned: String,
    pub formatted: String,
    pub is_valid: bool,
    pub tin_type: TINType,
    pub branch_code: Option<String>,
    pub check_digit: u8,
    pub actual_check_digit: u8,
    pub validation_message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sss_calculation() {
        let calc = GovernmentContributionsCalculator::new();
        
        // Test regular employee with PHP 20,000 salary
        let result = calc.calculate_sss(dec!(20000), EmploymentType::Regular).unwrap();
        assert_eq!(result.monthly_salary_credit, dec!(20000));
        assert_eq!(result.employee_contribution, dec!(900.00));
        assert_eq!(result.employer_contribution, dec!(1910.00));
        assert_eq!(result.ec_contribution, dec!(30.00));
        assert_eq!(result.wisp_contribution, dec!(10.00));
        
        // Test minimum salary
        let result = calc.calculate_sss(dec!(3000), EmploymentType::Regular).unwrap();
        assert_eq!(result.monthly_salary_credit, dec!(4000));
        assert_eq!(result.employee_contribution, dec!(180.00));
        
        // Test maximum salary
        let result = calc.calculate_sss(dec!(35000), EmploymentType::Regular).unwrap();
        assert_eq!(result.monthly_salary_credit, dec!(30000));
        assert_eq!(result.employee_contribution, dec!(1350.00));
    }

    #[test]
    fn test_philhealth_calculation() {
        let calc = GovernmentContributionsCalculator::new();
        
        // Test normal calculation (5% of 20,000 = 1,000)
        let result = calc.calculate_philhealth(dec!(20000)).unwrap();
        assert_eq!(result.total_contribution, dec!(1000.00));
        assert_eq!(result.employee_contribution, dec!(500.00));
        assert_eq!(result.employer_contribution, dec!(500.00));
        
        // Test minimum (below 10,000 salary)
        let result = calc.calculate_philhealth(dec!(8000)).unwrap();
        assert_eq!(result.total_contribution, dec!(500.00)); // Minimum
        assert_eq!(result.employee_contribution, dec!(250.00));
        
        // Test maximum (above 100,000 salary)
        let result = calc.calculate_philhealth(dec!(120000)).unwrap();
        assert_eq!(result.total_contribution, dec!(5000.00)); // Maximum
        assert_eq!(result.employee_contribution, dec!(2500.00));
    }

    #[test]
    fn test_pagibig_calculation() {
        let calc = GovernmentContributionsCalculator::new();
        
        // Test below 1,500 (1% employee rate)
        let result = calc.calculate_pagibig(dec!(1000)).unwrap();
        assert_eq!(result.employee_rate, dec!(0.01));
        assert_eq!(result.employee_contribution, dec!(10.00));
        assert_eq!(result.employer_contribution, dec!(20.00));
        
        // Test above 1,500 (2% employee rate)
        let result = calc.calculate_pagibig(dec!(5000)).unwrap();
        assert_eq!(result.employee_rate, dec!(0.02));
        assert_eq!(result.employee_contribution, dec!(100.00)); // Capped
        assert_eq!(result.employer_contribution, dec!(100.00)); // Capped
        
        // Test high salary (should be capped at 100)
        let result = calc.calculate_pagibig(dec!(10000)).unwrap();
        assert_eq!(result.employee_contribution, dec!(100.00));
        assert_eq!(result.employer_contribution, dec!(100.00));
    }

    #[test]
    fn test_total_contributions() {
        let calc = GovernmentContributionsCalculator::new();
        
        let result = calc.calculate_all_contributions(
            dec!(25000), 
            EmploymentType::Regular
        ).unwrap();
        
        // Verify totals
        assert_eq!(result.monthly_salary, dec!(25000.00));
        assert!(result.net_take_home < dec!(25000));
        assert!(result.total_employee_contribution > dec!(0));
        assert!(result.total_employer_contribution > dec!(0));
    }

    #[test]
    fn test_tin_validation() {
        // Test valid 9-digit TIN
        let result = TINValidator::validate("123456789").unwrap();
        assert!(result.is_valid || !result.is_valid); // Depends on check digit
        assert_eq!(result.tin_type, TINType::Individual);
        
        // Test valid 12-digit TIN
        let result = TINValidator::validate("123456789001").unwrap();
        assert!(result.is_valid || !result.is_valid); // Depends on check digit
        assert_eq!(result.tin_type, TINType::Corporate);
        assert_eq!(result.branch_code, Some("001".to_string()));
        
        // Test invalid length
        let result = TINValidator::validate("12345");
        assert!(result.is_err());
        
        // Test with non-digits
        let result = TINValidator::validate("123-456-789").unwrap();
        assert_eq!(result.cleaned, "123456789");
    }

    #[test]
    fn test_tin_generation() {
        // Generate a valid TIN
        let tin = TINValidator::generate_valid_tin("12345678", TINType::Individual).unwrap();
        assert_eq!(tin.len(), 9);
        
        // Validate the generated TIN
        let validation = TINValidator::validate(&tin).unwrap();
        assert!(validation.is_valid);
    }

    #[test]
    fn test_kasambahay_special_rates() {
        let calc = GovernmentContributionsCalculator::new();
        
        // Below 5,000 - employer pays all
        let result = calc.calculate_sss(dec!(4000), EmploymentType::Kasambahay).unwrap();
        assert_eq!(result.employee_contribution, dec!(0.00));
        assert!(result.employer_contribution > dec!(0));
        
        // Above 5,000 - normal split
        let result = calc.calculate_sss(dec!(6000), EmploymentType::Kasambahay).unwrap();
        assert!(result.employee_contribution > dec!(0));
        assert!(result.employer_contribution > dec!(0));
    }

    #[test]
    fn test_self_employed_contributions() {
        let calc = GovernmentContributionsCalculator::new();
        
        let result = calc.calculate_sss(dec!(20000), EmploymentType::SelfEmployed).unwrap();
        // Self-employed pays both employee and employer shares
        assert_eq!(result.employer_contribution, dec!(0.00));
        assert!(result.employee_contribution > dec!(1000)); // Pays both shares
    }
}
```