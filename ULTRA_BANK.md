

## **Complete Domain Knowledge Base**

### **/knowledge-base/philippine-tax/vat/computation-rules.md**

```markdown
# VAT Computation Rules - Philippines

## Standard VAT Rate: 12%

### Basic Formulas

#### For VAT-Exclusive Pricing


Vatable Sales = Selling Price Output VAT = Vatable Sales × 0.12 Total Amount = Vatable Sales + Output VAT

```

#### For VAT-Inclusive Pricing
```

Total Amount = Selling Price (already includes VAT) Vatable Sales = Total Amount ÷ 1.12 Output VAT = Total Amount - Vatable Sales

```

### Input VAT Rules
- Claimable only from VAT-registered suppliers
- Must have valid VAT receipt/invoice
- Must be used for business purposes
- Cannot claim if supplier is non-VAT

### Monthly VAT Payable
```

VAT Payable = Output VAT - Input VAT If negative = VAT Credit (carry forward)

```

### Common Computation Scenarios

#### Scenario 1: Regular Sale
```

Selling Price: ₱1,000 (VAT exclusive) Output VAT: ₱1,000 × 0.12 = ₱120 Total to Collect: ₱1,120

```

#### Scenario 2: Purchase with Input VAT
```

Purchase Price: ₱5,600 (VAT inclusive) Vatable Purchase: ₱5,600 ÷ 1.12 = ₱5,000 Input VAT Credit: ₱600

```

#### Scenario 3: Mixed Sales (VATable + Exempt)
```

VATable Sales: ₱10,000 VAT-Exempt Sales: ₱5,000 Output VAT: ₱10,000 × 0.12 = ₱1,200 Total Sales: ₱16,200

```

### Special Rules
- Services: VAT applies to service fees
- Goods: VAT applies to selling price
- Discounts: Deduct before computing VAT
- Senior/PWD Discount: Exempt from VAT on the discounted portion
```

### **/knowledge-base/philippine-tax/vat/exemptions.md**

```markdown
# VAT Exemptions - Philippines

## VAT-Exempt Transactions

### 1. Basic Necessities
- Agricultural products in original state
- Marine products in original state
- Livestock and poultry in original state
- Rice (all varieties)
- Corn grits
- Fresh fruits and vegetables
- Fish (fresh, dried, salted)
- Fresh meat
- Fresh eggs
- Fresh milk

### 2. Educational Services
- Tuition fees for basic education
- Books and educational materials
- School supplies prescribed by DepEd/CHED
- Educational services by accredited institutions

### 3. Medical & Health
- Medical and dental services
- Hospital services
- Medicines for diabetes, high cholesterol, hypertension
- Medical supplies and equipment for exclusive use of hospitals

### 4. Financial Services
- Interest income from loans
- Life insurance premiums
- Non-life insurance for crop/livestock

### 5. Real Property
- Sale of residential lot (≤ ₱1,919,500)
- Sale of house & lot (≤ ₱3,199,200)
- Low-cost housing (≤ ₱450,000)
- Socialized housing
- Residential lease (≤ ₱15,000/month)

### 6. Services
- Services by individuals earning ≤ ₱3M annually
- Export of services
- Services to persons engaged in international shipping/air transport

### Senior Citizen & PWD Exemptions
- 20% discount is VAT-exempt
- Applies to:
  - Medicines (prescription)
  - Medical services
  - Restaurant meals
  - Transportation
  - Hotels
  - Recreation centers
  - Funeral services

### Token Patterns for Exemptions
```

"VAT exempt" → mark as exempt "senior discount" → 20% discount, VAT exempt "PWD discount" → 20% discount, VAT exempt "basic goods" → check if in exempt list "tuition" → educational, VAT exempt

### **/knowledge-base/philippine-tax/vat/zero-rating.md**

```markdown
# Zero-Rated VAT Sales - Philippines

## Definition
Zero-rated sales: Subject to 0% VAT but can claim input VAT refund

## Zero-Rated Transactions

### 1. Export Sales
- Direct export of goods
- Constructive export (goods used abroad)
- Foreign currency denominated sales
- Sales to PEZA-registered enterprises

### 2. Services to Non-Residents
- Services rendered outside Philippines
- Services to foreign vessels/aircraft
- International shipping/air transport
- Processing for export

### 3. Special Economic Zones
- Sales to ecozone enterprises
- Sales to freeport enterprises
- Subcontractors of export-oriented enterprises

### 4. International Transactions
- International air/shipping transport
- Sale of gold to BSP
- Services covered by international agreements

## Documentation Requirements
1. Export documents (Bill of Lading/Airway Bill)
2. Commercial Invoice
3. Bank Credit Memo (foreign currency receipt)
4. Certificate of Non-Registration (if applicable)

## Input VAT Treatment
- Can claim input VAT refund
- File within 2 years from close of quarter
- Minimum ₱10,000 refundable amount

## Computation Example
```

Export Sale: $10,000 × ₱50 = ₱500,000 Output VAT: ₱500,000 × 0% = ₱0 Input VAT on Materials: ₱50,000 (claimable) VAT Refund Claim: ₱50,000

```

## Token Recognition
```

"export to [country]" → zero-rated "sale to PEZA" → zero-rated "foreign buyer" → check if export "dollar sale" → likely zero-rated

### **/knowledge-base/philippine-tax/withholding/ewt-rates.md**

```markdown
# Expanded Withholding Tax (EWT) Rates - Philippines
## RR No. 11-2018 (TRAIN Law)

## Professional Services

### 15% Rate (If gross income > ₱3M)
- Lawyers
- CPAs
- Engineers
- Architects
- Doctors
- Consultants

### 10% Rate (If gross income ≤ ₱3M)
- Same professionals as above but lower income bracket

### 5% Rate
- Professional entertainers
- Professional athletes
- Directors
- Management/technical consultants

## Business Income Payments

### 2% Rate
- Commission (non-professional)
- Rental (personal property)
- Cinematographic film rentals
- Prime contractors/subcontractors

### 1% Rate
- Purchase of goods (except those under 2%)
- Purchase from top 20,000 corporations
- Government procurement
- Regular suppliers

## Special Rates

### 10% Rate
- Income distribution from partnerships
- Income from joint ventures

### 15% Rate
- Informer's reward

### 20% Rate
- Interest from deposits
- Royalties

## Computation Examples

### Professional Service
```

Legal Service Fee: ₱50,000 Less: EWT (15%): ₱7,500 Net Payment: ₱42,500

```

### Purchase of Goods
```

Purchase Amount: ₱100,000 (VAT exclusive) VAT (12%): ₱12,000 Gross Amount: ₱112,000 Less: EWT (1% of ₱100,000): ₱1,000 Total Payment: ₱111,000

```

## Token Patterns
```

"professional fee" → check if >₱3M (15%) or ≤₱3M (10%) "rental payment" → 5% EWT "purchase from supplier" → 1% EWT "commission" → 2% EWT

### **/knowledge-base/philippine-tax/withholding/cwt-rates.md**

```markdown
# Creditable Withholding Tax (CWT) on Compensation

## Monthly Tax Table (Effective 2023)

### Daily Wage Earners
| Daily Rate | Tax Rate |
|------------|----------|
| ≤ ₱685 | 0% |
| > ₱685 | Graduated rates |

### Monthly/Semi-Monthly

#### Tax Computation
1. Determine taxable income
2. Apply graduated rates:

| Annual Taxable Income | Tax Rate |
|----------------------|----------|
| ≤ ₱250,000 | 0% |
| ₱250,001 - ₱400,000 | 15% of excess over ₱250,000 |
| ₱400,001 - ₱800,000 | ₱22,500 + 20% of excess over ₱400,000 |
| ₱800,001 - ₱2,000,000 | ₱102,500 + 25% of excess over ₱800,000 |
| ₱2,000,001 - ₱8,000,000 | ₱402,500 + 30% of excess over ₱2,000,000 |
| > ₱8,000,000 | ₱2,202,500 + 35% of excess over ₱8,000,000 |

## Non-Taxable Benefits
- 13th month pay (up to ₱90,000)
- De minimis benefits
- SSS, GSIS, PhilHealth, Pag-IBIG contributions
- Union dues

## Computation Example

### Monthly Salary: ₱50,000
```

Annual Salary: ₱600,000 Less: Non-taxable 13th month: ₱50,000 Taxable Annual: ₱550,000

Tax Computation: First ₱250,000: ₱0 Next ₱150,000 × 15%: ₱22,500 Next ₱150,000 × 20%: ₱30,000 Annual Tax: ₱52,500 Monthly CWT: ₱4,375

```

## Token Recognition
```

"salary" → apply compensation table "13th month" → exempt up to ₱90,000 "allowance" → check if de minimis "overtime" → taxable compensation

### **/knowledge-base/philippine-tax/withholding/compensation.md**

```markdown
# Compensation & Benefits Withholding Rules

## Taxable Compensation
- Basic salary
- Overtime pay
- Holiday pay
- Night differential
- Hazard pay
- Commission
- Taxable allowances
- Bonuses (except 13th month up to ₱90,000)

## Non-Taxable (De Minimis Benefits)
Per RR 11-2018, up to these limits:

### Monetized Benefits
- Unused vacation leave: ≤ 10 days/year
- Medical cash allowance: ≤ ₱10,000/year
- Rice subsidy: ≤ ₱2,000/month
- Uniform allowance: ≤ ₱6,000/year
- Medical allowance for dependents: ≤ ₱10,000/year
- Laundry allowance: ≤ ₱300/month
- Achievement awards: ≤ ₱10,000/year

### Other Benefits
- Productivity bonus: ≤ ₱10,000/year
- Christmas gift: ≤ ₱5,000/year
- Loyalty award (per 5 years): ≤ ₱10,000
- Gifts during special occasions: ≤ ₱5,000/year

## SSS, PhilHealth, Pag-IBIG Contributions

### SSS (2024 Rates)
| Monthly Salary | Employee Share | Employer Share |
|----------------|---------------|----------------|
| ≤ ₱4,250 | ₱180 | ₱390 |
| ₱29,750+ | ₱1,350 | ₱2,400 |

### PhilHealth (5% of salary, split)
- Employee: 2.5%
- Employer: 2.5%
- Monthly cap: ₱5,000 total

### Pag-IBIG
- Employee: 1% (₱100 max if <₱1,500 salary, ₱200 max if >₱5,000)
- Employer: 2% (max ₱200)

## Final Pay Computation
```

Gross Salary: ₱50,000 Less: SSS Employee: (₱1,350) Less: PhilHealth: (₱1,250) Less: Pag-IBIG: (₱200) Taxable Income: ₱47,200 Less: Withholding Tax: (₱4,375) Net Pay: ₱42,825

### **/knowledge-base/philippine-tax/bir-forms/2550M-monthly-vat.md**

```markdown
# BIR Form 2550M - Monthly VAT Return

## Filing Requirements
- Who: VAT-registered taxpayers
- When: On or before 20th day following month
- Where: Authorized banks or eFPS

## Form Structure

### Part I - Sales/Receipts
```

12A - Vatable Sales (Private) 12B - Sales to Government 12C - Zero-Rated Sales 12D - Exempt Sales 12E - Total Sales (12A+12B+12C+12D)

```

### Part II - Purchases
```

13 - Vatable Purchases 14 - Purchases from Non-VAT Suppliers 15 - Importations 16 - Total Purchases

```

### Part III - Output Tax
```

16A - Output Tax from Vatable Sales 16B - Output Tax from Government Sales 16C - Total Output Tax

```

### Part IV - Input Tax
```

17A - Input Tax from Domestic Purchases 17B - Input Tax from Importation 17C - Input Tax claimed this period 17D - Total Input Tax

```

### Part V - Tax Due
```

18 - VAT Payable (16C - 17D) 19 - Add: Penalties 20 - Total Amount Payable

````

## Automated Filling Logic
```rust
fn generate_2550M(transactions: Vec<Transaction>) -> Form2550M {
    Form2550M {
        vatable_sales_private: transactions
            .filter(|t| t.is_vatable && !t.is_government)
            .sum(|t| t.vatable_amount),
        
        output_tax: transactions
            .filter(|t| t.is_sale)
            .sum(|t| t.output_vat),
            
        input_tax: transactions
            .filter(|t| t.is_purchase && t.has_valid_receipt)
            .sum(|t| t.input_vat),
            
        vat_payable: output_tax - input_tax
    }
}
````

## Common Errors to Avoid

- Claiming input VAT without OR/SI
- Including exempt sales in vatable
- Wrong period coverage
- Mathematical errors
- Late filing penalties (25% surcharge + 20% interest/year)

````

### **/knowledge-base/philippine-tax/bir-forms/2550Q-quarterly-vat.md**

```markdown
# BIR Form 2550Q - Quarterly VAT Return

## Filing Requirements
- Who: VAT taxpayers who opted for quarterly filing
- Eligibility: Previous year's sales ≤ ₱6.5M
- When: Within 25 days after quarter end
- Quarters: Q1 (Jan-Mar), Q2 (Apr-Jun), Q3 (Jul-Sep), Q4 (Oct-Dec)

## Differences from 2550M
- Covers 3 months instead of 1
- Same structure but aggregated data
- Different deadlines (25th vs 20th)

## Quarterly Computation
````

Q1 Summary: January Output VAT: ₱50,000 February Output VAT: ₱45,000 March Output VAT: ₱55,000 Total Q1 Output: ₱150,000

January Input VAT: ₱30,000 February Input VAT: ₱25,000 March Input VAT: ₱35,000 Total Q1 Input: ₱90,000

Q1 VAT Payable: ₱60,000

````

## When to Switch Filing Frequency
### Monthly to Quarterly
- Sales dropped below ₱6.5M threshold
- File application with RDO

### Quarterly to Monthly
- Sales exceeded ₱6.5M
- Automatic conversion
- Must file monthly starting next period

## Automated Detection
```rust
fn determine_filing_frequency(annual_sales: Decimal) -> FilingFrequency {
    if annual_sales <= Decimal::from(6_500_000) {
        FilingFrequency::Quarterly
    } else {
        FilingFrequency::Monthly
    }
}
````

````

### **/knowledge-base/philippine-tax/bir-forms/1601C-monthly-withholding.md**

```markdown
# BIR Form 1601C - Monthly Remittance of Withholding Tax on Compensation

## Filing Requirements
- Who: All employers
- When: On or before 10th of following month
- Covers: Salaries, wages, benefits

## Form Sections

### Schedule 1 - Computation of Tax
````

1. Gross Compensation
2. Less: Non-Taxable (13th month, de minimis)
3. Taxable Compensation
4. Tax Withheld
5. Adjustments (over/under withholding)
6. Tax Due
7. Penalties (if late)
8. Total Amount Due

````

### Schedule 2 - Alphalist Details
- TIN of employees
- Name
- Gross compensation
- Tax withheld

## Automated Computation
```rust
struct MonthlyWithholding {
    fn compute_1601C(&self, payroll: Vec<Employee>) -> Form1601C {
        let mut form = Form1601C::new();
        
        for employee in payroll {
            let taxable = employee.gross
                - employee.sss_contribution
                - employee.philhealth_contribution
                - employee.pagibig_contribution
                - employee.non_taxable_allowances;
                
            let tax = self.compute_tax(taxable);
            
            form.add_employee(EmployeeWithholding {
                tin: employee.tin,
                name: employee.name,
                gross: employee.gross,
                tax_withheld: tax,
            });
        }
        
        form.total_tax_due = form.employees.sum(|e| e.tax_withheld);
        form
    }
}
````

## Common Compliance Issues

- Late filing: 25% surcharge + interest
- Under-withholding: Employee liable
- Over-withholding: Refundable
- Missing TIN: Cannot file

````

### **/knowledge-base/philippine-tax/bir-forms/1604CF-annual-information-return.md**

```markdown
# BIR Form 1604CF - Annual Information Return of Income Taxes Withheld on Compensation

## Filing Requirements
- Who: All employers
- When: On or before January 31 of following year
- Covers: Entire calendar year compensation

## Components

### Part I - Employer Information
- TIN, name, address
- RDO code
- Line of business
- Contact details

### Part II - Summary of Compensation & Taxes
````

Total number of employees Total gross compensation paid Total taxes withheld Total non-taxable compensation

- 13th month & bonuses (≤₱90,000)
- De minimis benefits
- SSS, GSIS, PhilHealth, Pag-IBIG

```

### Part III - Schedule (Alphalist)
For each employee:
```

- Sequential number
- TIN
- Last name, First name, Middle name
- Gross compensation
- Non-taxable (13th month, benefits)
- Taxable compensation
- Tax withheld
- Tax due
- Tax refunded/adjusted

```

## BIR Alphalist Data Entry Format
```

D7.1,1604CF,12/31/2024,N,A H7.1,1604CF,000-000-000-000,COMPANY NAME,01/01/2024,12/31/2024,A D7.1.1,C1,000-000-000,LASTNAME,FIRSTNAME,MI,12/31/2024,300000,25000,275000,30000,30000,0

````

## Automated Generation
```rust
fn generate_1604CF(year: i32) -> Form1604CF {
    let employees = fetch_all_employees(year);
    let mut alphalist = Vec::new();
    
    for emp in employees {
        let annual = aggregate_annual_compensation(emp.tin, year);
        
        alphalist.push(AlphalistEntry {
            tin: emp.tin,
            name: format!("{}, {} {}", emp.last, emp.first, emp.middle),
            gross: annual.gross,
            non_taxable_13th: min(annual.bonus_13th, 90_000),
            non_taxable_benefits: annual.de_minimis,
            taxable: annual.taxable,
            tax_withheld: annual.total_tax_withheld,
        });
    }
    
    Form1604CF {
        employer_tin: config.company_tin,
        year,
        alphalist,
        total_employees: alphalist.len(),
        total_gross: alphalist.sum(|a| a.gross),
        total_tax: alphalist.sum(|a| a.tax_withheld),
    }
}
````

````

### **/knowledge-base/accounting-standards/pfrs-for-smes/revenue-recognition.md**

```markdown
# Revenue Recognition - PFRS for SMEs

## Core Principle
Revenue is recognized when:
1. Significant risks and rewards transferred
2. Amount can be measured reliably
3. Economic benefits probable
4. Costs can be measured reliably

## Types of Revenue

### Sale of Goods
Recognized when:
- Ownership transferred
- Delivery completed
- Collection reasonably assured

### Rendering of Services
- Percentage of completion method
- Straight-line over service period
- Completed contract method (if uncertain)

### Construction Contracts
- Percentage of completion based on:
  - Costs incurred / Total estimated costs
  - Surveys of work performed
  - Physical proportion completed

## Philippine Context

### VAT Treatment
````

Sales Revenue (net of VAT): ₱100,000 Output VAT: ₱12,000 Total Invoice: ₱112,000

Journal Entry: Dr. Accounts Receivable ₱112,000 Cr. Sales Revenue ₱100,000 Cr. Output VAT Payable ₱12,000

```

### Installment Sales
- Revenue recognized at point of sale (full amount)
- Interest component recognized over time
- Default risk provided for

### Consignment Sales
- Revenue only when consignee sells
- Inventory remains with consignor until sold
- Commission to consignee as expense

## Token Recognition Patterns
```

"sold" → immediate recognition "delivered" → check if accepted "on account" → receivable created "installment" → separate principal from interest "consignment" → wait for consignee sale

### **/knowledge-base/accounting-standards/pfrs-for-smes/inventory-valuation.md**

```markdown
# Inventory Valuation - PFRS for SMEs

## Measurement Principle
Inventory valued at lower of:
- Cost
- Net Realizable Value (NRV)

## Cost Components

### Purchase Costs
- Purchase price
- Import duties
- Non-refundable taxes (except VAT)
- Transport costs
- Handling costs
- Less: Trade discounts, rebates

### Conversion Costs
- Direct labor
- Fixed production overhead (allocated)
- Variable production overhead

### Excluded from Cost
- Storage (unless necessary)
- Administrative overhead
- Selling costs
- Interest expense

## Cost Flow Methods

### Allowed Methods
1. **FIFO (First-In-First-Out)**
   - Oldest items sold first
   - Ending inventory at recent costs

2. **Weighted Average**
   - Average cost per unit
   - Recalculated after each purchase

### NOT Allowed
- LIFO (Last-In-First-Out) - prohibited under PFRS

## Philippine Tax Implications

### BIR Accepted Methods
- FIFO
- Average
- Moving average
- Specific identification

### Sample Computation - Weighted Average
```

Beginning: 100 units @ ₱50 = ₱5,000 Purchase 1: 200 units @ ₱55 = ₱11,000 Purchase 2: 150 units @ ₱60 = ₱9,000 Total: 450 units = ₱25,000 Average Cost: ₱55.56/unit

Sale of 300 units: COGS = 300 × ₱55.56 = ₱16,668 Ending Inventory = 150 × ₱55.56 = ₱8,334

```

## Write-Down Rules
```

Cost: ₱100 NRV (Selling price - costs to sell): ₱85 Write-down: ₱15

Dr. Inventory Write-down Expense ₱15 Cr. Inventory ₱15

### **/knowledge-base/accounting-standards/chart-of-accounts/standard-coa.json**

```json
{
  "chart_of_accounts": {
    "version": "PH-SME-2024",
    "structure": "5-digit-code",
    "accounts": [
      {
        "category": "ASSETS",
        "code_range": "10000-19999",
        "accounts": [
          {
            "code": "10000",
            "title": "CURRENT ASSETS",
            "type": "header"
          },
          {
            "code": "10100",
            "title": "Cash and Cash Equivalents",
            "type": "header"
          },
          {
            "code": "10110",
            "title": "Cash on Hand",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10111",
            "title": "Petty Cash Fund",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10120",
            "title": "Cash in Bank",
            "type": "header"
          },
          {
            "code": "10121",
            "title": "BDO Checking Account",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10122",
            "title": "BPI Savings Account",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10200",
            "title": "Accounts Receivable",
            "type": "header"
          },
          {
            "code": "10210",
            "title": "Trade Receivables",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10219",
            "title": "Allowance for Doubtful Accounts",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "10300",
            "title": "Inventory",
            "type": "header"
          },
          {
            "code": "10310",
            "title": "Merchandise Inventory",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10320",
            "title": "Raw Materials",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10400",
            "title": "Prepaid Expenses",
            "type": "header"
          },
          {
            "code": "10410",
            "title": "Prepaid Rent",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10420",
            "title": "Prepaid Insurance",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "10500",
            "title": "Input VAT",
            "type": "detail",
            "normal_balance": "debit"
          }
        ]
      },
      {
        "category": "LIABILITIES",
        "code_range": "20000-29999",
        "accounts": [
          {
            "code": "20000",
            "title": "CURRENT LIABILITIES",
            "type": "header"
          },
          {
            "code": "20100",
            "title": "Accounts Payable",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "20200",
            "title": "Accrued Expenses",
            "type": "header"
          },
          {
            "code": "20210",
            "title": "Salaries Payable",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "20220",
            "title": "SSS Payable",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "20230",
            "title": "PhilHealth Payable",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "20240",
            "title": "Pag-IBIG Payable",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "20300",
            "title": "Tax Liabilities",
            "type": "header"
          },
          {
            "code": "20310",
            "title": "Output VAT Payable",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "20320",
            "title": "Withholding Tax Payable",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "20330",
            "title": "Income Tax Payable",
            "type": "detail",
            "normal_balance": "credit"
          }
        ]
      },
      {
        "category": "EQUITY",
        "code_range": "30000-39999",
        "accounts": [
          {
            "code": "30000",
            "title": "OWNER'S EQUITY",
            "type": "header"
          },
          {
            "code": "30100",
            "title": "Capital",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "30200",
            "title": "Drawings",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "30300",
            "title": "Retained Earnings",
            "type": "detail",
            "normal_balance": "credit"
          }
        ]
      },
      {
        "category": "REVENUE",
        "code_range": "40000-49999",
        "accounts": [
          {
            "code": "40000",
            "title": "REVENUE",
            "type": "header"
          },
          {
            "code": "40100",
            "title": "Sales Revenue",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "40110",
            "title": "Service Revenue",
            "type": "detail",
            "normal_balance": "credit"
          },
          {
            "code": "40200",
            "title": "Sales Returns and Allowances",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "40300",
            "title": "Sales Discounts",
            "type": "detail",
            "normal_balance": "debit"
          }
        ]
      },
      {
        "category": "EXPENSES",
        "code_range": "50000-59999",
        "accounts": [
          {
            "code": "50000",
            "title": "COST OF SALES",
            "type": "header"
          },
          {
            "code": "50100",
            "title": "Cost of Goods Sold",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "51000",
            "title": "OPERATING EXPENSES",
            "type": "header"
          },
          {
            "code": "51100",
            "title": "Salaries and Wages",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "51110",
            "title": "SSS Expense",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "51120",
            "title": "PhilHealth Expense",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "51130",
            "title": "Pag-IBIG Expense",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "51200",
            "title": "Rent Expense",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "51300",
            "title": "Utilities Expense",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "51400",
            "title": "Supplies Expense",
            "type": "detail",
            "normal_balance": "debit"
          },
          {
            "code": "51500",
            "title": "Transportation Expense",
            "type": "detail",
            "normal_balance": "debit"
          }
        ]
      }
    ]
  }
}
```

### **/knowledge-base/business-rules/validations/balanced-entries.rs**

```rust
// Balanced Journal Entry Validation Rules

use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JournalValidationError {
    #[error("Journal entry is not balanced: Debits {debits} ≠ Credits {credits}")]
    UnbalancedEntry { debits: Decimal, credits: Decimal },
    
    #[error("Journal entry must have at least 2 lines")]
    InsufficientLines,
    
    #[error("Account {code} does not exist in chart of accounts")]
    InvalidAccount { code: String },
    
    #[error("Amount cannot be negative or zero")]
    InvalidAmount,
    
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    
    #[error("Duplicate account {code} in same entry")]
    DuplicateAccount { code: String },
}

pub struct JournalValidator {
    chart_of_accounts: HashMap<String, Account>,
}

impl JournalValidator {
    /// Core validation: Debits must equal Credits
    pub fn validate_balance(&self, entry: &JournalEntry) -> Result<(), JournalValidationError> {
        let total_debits: Decimal = entry.lines
            .iter()
            .map(|line| line.debit)
            .sum();
            
        let total_credits: Decimal = entry.lines
            .iter()
            .map(|line| line.credit)
            .sum();
            
        if total_debits != total_credits {
            return Err(JournalValidationError::UnbalancedEntry {
                debits: total_debits,
                credits: total_credits,
            });
        }
        
        Ok(())
    }
    
    /// Validate account codes exist
    pub fn validate_accounts(&self, entry: &JournalEntry) -> Result<(), JournalValidationError> {
        for line in &entry.lines {
            if !self.chart_of_accounts.contains_key(&line.account_code) {
                return Err(JournalValidationError::InvalidAccount {
                    code: line.account_code.clone(),
                });
            }
        }
        Ok(())
    }
    
    /// Validate normal balance rules
    pub fn validate_normal_balance(&self, line: &JournalLine) -> Result<(), String> {
        let account = &self.chart_of_accounts[&line.account_code];
        
        match account.normal_balance {
            NormalBalance::Debit => {
                // Assets, Expenses, Drawings normally have debit balance
                // Increasing = Debit, Decreasing = Credit
                if line.credit > Decimal::ZERO && line.debit == Decimal::ZERO {
                    // This is a decrease, which is valid
                    Ok(())
                } else if line.debit > Decimal::ZERO && line.credit == Decimal::ZERO {
                    // This is an increase, which is valid
                    Ok(())
                } else {
                    Err(format!("Account {} should have either debit or credit, not both", account.title))
                }
            }
            NormalBalance::Credit => {
                // Liabilities, Equity, Revenue normally have credit balance
                // Increasing = Credit, Decreasing = Debit
                Ok(())
            }
        }
    }
    
    /// Complete validation pipeline
    pub fn validate(&self, entry: &JournalEntry) -> Result<(), Vec<JournalValidationError>> {
        let mut errors = Vec::new();
        
        // Must have at least 2 lines
        if entry.lines.len() < 2 {
            errors.push(JournalValidationError::InsufficientLines);
        }
        
        // Check balance
        if let Err(e) = self.validate_balance(entry) {
            errors.push(e);
        }
        
        // Check accounts exist
        if let Err(e) = self.validate_accounts(entry) {
            errors.push(e);
        }
        
        // Check for negative amounts
        for line in &entry.lines {
            if line.debit < Decimal::ZERO || line.credit < Decimal::ZERO {
                errors.push(JournalValidationError::InvalidAmount);
            }
        }
        
        // Check required fields
        if entry.date.is_none() {
            errors.push(JournalValidationError::MissingField {
                field: "date".to_string(),
            });
        }
        
        if entry.description.trim().is_empty() {
            errors.push(JournalValidationError::MissingField {
                field: "description".to_string(),
            });
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Common journal entry patterns
pub struct JournalTemplates;

impl JournalTemplates {
    /// Cash sale with VAT
    pub fn cash_sale_vat(amount: Decimal) -> JournalEntry {
        let vatable = amount / Decimal::from_str("1.12").unwrap();
        let vat = amount - vatable;
        
        JournalEntry {
            lines: vec![
                JournalLine {
                    account_code: "10110".to_string(), // Cash on Hand
                    debit: amount,
                    credit: Decimal::ZERO,
                },
                JournalLine {
                    account_code: "40100".to_string(), // Sales Revenue
                    debit: Decimal::ZERO,
                    credit: vatable,
                },
                JournalLine {
                    account_code: "20310".to_string(), // Output VAT
                    debit: Decimal::ZERO,
                    credit: vat,
                },
            ],
            description: "Cash sale with VAT".to_string(),
            ..Default::default()
        }
    }
    
    /// Purchase with EWT
    pub fn purchase_with_ewt(gross: Decimal, ewt_rate: Decimal) -> JournalEntry {
        let vatable = gross / Decimal::from_str("1.12").unwrap();
        let vat = gross - vatable;
        let ewt = vatable * ewt_rate;
        let net_payment = gross - ewt;
        
        JournalEntry {
            lines: vec![
                JournalLine {
                    account_code: "51400".to_string(), // Supplies Expense
                    debit: vatable,
                    credit: Decimal::ZERO,
                },
                JournalLine {
                    account_code: "10500".to_string(), // Input VAT
                    debit: vat,
                    credit: Decimal::ZERO,
                },
                JournalLine {
                    account_code: "10110".to_string(), // Cash
                    debit: Decimal::ZERO,
                    credit: net_payment,
                },
                JournalLine {
                    account_code: "20320".to_string(), // EWT Payable
                    debit: Decimal::ZERO,
                    credit: ewt,
                },
            ],
            description: "Purchase with EWT".to_string(),
            ..Default::default()
        }
    }
}
```

### **/knowledge-base/business-rules/validations/vat-compliance.rs**

```rust
// VAT Compliance Validation Rules for Philippines

use rust_decimal::Decimal;
use chrono::{NaiveDate, Datelike};

pub struct VatComplianceValidator {
    vat_rate: Decimal, // 0.12 for Philippines
    threshold: Decimal, // 3,000,000 for mandatory registration
}

impl VatComplianceValidator {
    pub fn new() -> Self {
        Self {
            vat_rate: Decimal::from_str("0.12").unwrap(),
            threshold: Decimal::from_str("3000000").unwrap(),
        }
    }
    
    /// Validate VAT calculation correctness
    pub fn validate_vat_computation(&self, transaction: &Transaction) -> Result<(), String> {
        match transaction.vat_type {
            VatType::VatExclusive => {
                let expected_vat = transaction.amount * self.vat_rate;
                let computed_vat = transaction.output_vat.unwrap_or(Decimal::ZERO);
                
                if (expected_vat - computed_vat).abs() > Decimal::from_str("0.01").unwrap() {
                    return Err(format!(
                        "VAT computation error: Expected ₱{}, Got ₱{}", 
                        expected_vat, computed_vat
                    ));
                }
            }
            VatType::VatInclusive => {
                let vatable = transaction.amount / Decimal::from_str("1.12").unwrap();
                let expected_vat = transaction.amount - vatable;
                let computed_vat = transaction.output_vat.unwrap_or(Decimal::ZERO);
                
                if (expected_vat - computed_vat).abs() > Decimal::from_str("0.01").unwrap() {
                    return Err(format!(
                        "VAT inclusive computation error: Expected ₱{}, Got ₱{}", 
                        expected_vat, computed_vat
                    ));
                }
            }
            VatType::Exempt | VatType::ZeroRated => {
                if transaction.output_vat.is_some() && transaction.output_vat.unwrap() > Decimal::ZERO {
                    return Err("Exempt/Zero-rated sales should not have output VAT".to_string());
                }
            }
        }
        Ok(())
    }
    
    /// Validate VAT registration requirement
    pub fn check_registration_requirement(&self, annual_sales: Decimal) -> VatRegistrationStatus {
        if annual_sales >= self.threshold {
            VatRegistrationStatus::Required
        } else {
            VatRegistrationStatus::Optional
        }
    }
    
    /// Validate Input VAT claims
    pub fn validate_input_vat(&self, purchase: &Purchase) -> Result<(), String> {
        // Must have valid VAT receipt
        if purchase.receipt_number.is_none() {
            return Err("Cannot claim input VAT without official receipt".to_string());
        }
        
        // Supplier must be VAT registered
        if !purchase.supplier_vat_registered {
            return Err("Cannot claim input VAT from non-VAT supplier".to_string());
        }
        
        // Must be for business use
        if !purchase.business_purpose {
            return Err("Input VAT only claimable for business purchases".to_string());
        }
        
        // Check if VAT computation is correct
        let expected_input_vat = purchase.gross_amount - (purchase.gross_amount / Decimal::from_str("1.12").unwrap());
        if (purchase.input_vat - expected_input_vat).abs() > Decimal::from_str("0.01").unwrap() {
            return Err(format!(
                "Input VAT computation error: Expected ₱{}, Got ₱{}", 
                expected_input_vat, purchase.input_vat
            ));
        }
        
        Ok(())
    }
    
    /// Validate senior/PWD discounts
    pub fn validate_senior_pwd_discount(&self, sale: &Sale) -> Result<(), String> {
        if let Some(discount) = &sale.senior_pwd_discount {
            // Must be 20% discount
            let expected_discount = sale.gross_amount * Decimal::from_str("0.20").unwrap();
            if (discount.amount - expected_discount).abs() > Decimal::from_str("0.01").unwrap() {
                return Err("Senior/PWD discount must be exactly 20%".to_string());
            }
            
            // Discounted portion is VAT exempt
            let discounted_amount = discount.amount;
            let vatable_amount = sale.gross_amount - discounted_amount;
            let expected_vat = vatable_amount * self.vat_rate;
            
            if (sale.output_vat.unwrap_or(Decimal::ZERO) - expected_vat).abs() > Decimal::from_str("0.01").unwrap() {
                return Err("VAT should not apply to senior/PWD discounted portion".to_string());
            }
            
            // Must have ID number
            if discount.id_number.is_none() {
                return Err("Senior/PWD ID number required".to_string());
            }
        }
        Ok(())
    }
    
    /// Validate VAT return filing deadline
    pub fn check_filing_deadline(&self, period: &VatPeriod) -> Result<NaiveDate, String> {
        match period.filing_type {
            FilingType::Monthly => {
                // 20th of following month
                let deadline = period.end_date
                    .with_day(20)
                    .map(|d| d + chrono::Duration::days(30))
                    .ok_or("Invalid date calculation")?;
                Ok(deadline)
            }
            FilingType::Quarterly => {
                // 25th after quarter end
                let deadline = period.end_date + chrono::Duration::days(25);
                Ok(deadline)
            }
        }
    }
    
    /// Generate VAT relief summary
    pub fn generate_vat_relief(&self, transactions: Vec<Transaction>) -> VatRelief {
        let vatable_sales: Decimal = transactions.iter()
            .filter(|t| matches!(t.vat_type, VatType::VatExclusive | VatType::VatInclusive))
            .map(|t| t.vatable_amount())
            .sum();
            
        let exempt_sales: Decimal = transactions.iter()
            .filter(|t| matches!(t.vat_type, VatType::Exempt))
            .map(|t| t.amount)
            .sum();
            
        let zero_rated: Decimal = transactions.iter()
            .filter(|t| matches!(t.vat_type, VatType::ZeroRated))
            .map(|t| t.amount)
            .sum();
            
        let output_vat: Decimal = transactions.iter()
            .filter_map(|t| t.output_vat)
            .sum();
            
        let input_vat: Decimal = transactions.iter()
            .filter_map(|t| t.input_vat)
            .sum();
            
        VatRelief {
            vatable_sales,
            exempt_sales,
            zero_rated_sales: zero_rated,
            total_output_vat: output_vat,
            total_input_vat: input_vat,
            vat_payable: output_vat - input_vat,
        }
    }
}

/// Sample integration test
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vat_inclusive_computation() {
        let validator = VatComplianceValidator::new();
        
        let transaction = Transaction {
            amount: Decimal::from_str("1120").unwrap(),
            vat_type: VatType::VatInclusive,
            output_vat: Some(Decimal::from_str("120").unwrap()),
            ..Default::default()
        };
        
        assert!(validator.validate_vat_computation(&transaction).is_ok());
    }
}
```

### **/knowledge-base/business-rules/validations/withholding-compliance.rs**

```rust
// Withholding Tax Compliance Validation for Philippines

use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct WithholdingValidator {
    ewt_rates: HashMap<String, Decimal>,
    cwt_brackets: Vec<TaxBracket>,
}

impl WithholdingValidator {
    pub fn new() -> Self {
        let mut ewt_rates = HashMap::new();
        
        // Professional services
        ewt_rates.insert("professional_high".to_string(), Decimal::from_str("0.15").unwrap());
        ewt_rates.insert("professional_low".to_string(), Decimal::from_str("0.10").unwrap());
        ewt_rates.insert("professional_entertainment".to_string(), Decimal::from_str("0.05").unwrap());
        
        // Business payments
        ewt_rates.insert("rental".to_string(), Decimal::from_str("0.05").unwrap());
        ewt_rates.insert("commission".to_string(), Decimal::from_str("0.02").unwrap());
        ewt_rates.insert("goods".to_string(), Decimal::from_str("0.01").unwrap());
        
        // Compensation brackets (annual)
        let cwt_brackets = vec![
            TaxBracket { min: Decimal::ZERO, max: Decimal::from(250_000), rate: Decimal::ZERO },
            TaxBracket { min: Decimal::from(250_001), max: Decimal::from(400_000), rate: Decimal::from_str("0.15").unwrap() },
            TaxBracket { min: Decimal::from(400_001), max: Decimal::from(800_000), rate: Decimal::from_str("0.20").unwrap() },
            TaxBracket { min: Decimal::from(800_001), max: Decimal::from(2_000_000), rate: Decimal::from_str("0.25").unwrap() },
            TaxBracket { min: Decimal::from(2_000_001), max: Decimal::from(8_000_000), rate: Decimal::from_str("0.30").unwrap() },
            TaxBracket { min: Decimal::from(8_000_001), max: Decimal::MAX, rate: Decimal::from_str("0.35").unwrap() },
        ];
        
        Self { ewt_rates, cwt_brackets }
    }
    
    /// Validate EWT computation
    pub fn validate_ewt(&self, payment: &Payment) -> Result<(), String> {
        let rate = self.ewt_rates
            .get(&payment.payment_type)
            .ok_or(format!("Unknown payment type: {}", payment.payment_type))?;
            
        // EWT is based on VAT-exclusive amount
        let vatable = payment.gross_amount / Decimal::from_str("1.12").unwrap();
        let expected_ewt = vatable * rate;
        
        if (payment.ewt_amount - expected_ewt).abs() > Decimal::from_str("0.01").unwrap() {
            return Err(format!(
                "EWT computation error: Expected ₱{} ({}% of ₱{}), Got ₱{}", 
                expected_ewt, rate * Decimal::from(100), vatable, payment.ewt_amount
            ));
        }
        
        // Check if correct rate was applied
        match payment.payment_type.as_str() {
            "professional_high" if payment.supplier_income <= Decimal::from(3_000_000) => {
                return Err("Should use 10% rate for professionals with income ≤₱3M".to_string());
            }
            "professional_low" if payment.supplier_income > Decimal::from(3_000_000) => {
                return Err("Should use 15% rate for professionals with income >₱3M".to_string());
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Compute compensation withholding tax
    pub fn compute_compensation_tax(&self, monthly_salary: Decimal) -> Decimal {
        let annual = monthly_salary * Decimal::from(12);
        
        let mut tax = Decimal::ZERO;
        let mut remaining = annual;
        
        for bracket in &self.cwt_brackets {
            if remaining <= Decimal::ZERO {
                break;
            }
            
            let taxable_in_bracket = remaining.min(bracket.max - bracket.min);
            tax += taxable_in_bracket * bracket.rate;
            remaining -= taxable_in_bracket;
        }
        
        tax / Decimal::from(12) // Convert back to monthly
    }
    
    /// Validate payroll withholding
    pub fn validate_payroll(&self, payroll: &Payroll) -> Result<(), String> {
        // Compute taxable income
        let taxable = payroll.gross_salary
            - payroll.sss_contribution
            - payroll.philhealth_contribution  
            - payroll.pagibig_contribution
            - payroll.non_taxable_allowances;
            
        let expected_tax = self.compute_compensation_tax(taxable);
        
        if (payroll.tax_withheld - expected_tax).abs() > Decimal::from(1) {
            return Err(format!(
                "Withholding tax error: Expected ₱{}, Got ₱{}", 
                expected_tax, payroll.tax_withheld
            ));
        }
        
        Ok(())
    }
    
    /// Validate de minimis benefits
    pub fn validate_de_minimis(&self, benefits: &Benefits) -> Result<(), String> {
        let limits = vec![
            ("rice_subsidy", Decimal::from(2_000), "monthly"),
            ("uniform", Decimal::from(6_000), "annual"),
            ("medical_allowance", Decimal::from(10_000), "annual"),
            ("laundry", Decimal::from(300), "monthly"),
            ("achievement_award", Decimal::from(10_000), "annual"),
            ("christmas_gift", Decimal::from(5_000), "annual"),
        ];
        
        for (benefit, limit, period) in limits {
            if let Some(amount) = benefits.get(benefit) {
                let check_amount = match period {
                    "monthly" => amount * Decimal::from(12),
                    "annual" => *amount,
                    _ => *amount,
                };
                
                if check_amount > limit {
                    return Err(format!(
                        "{} exceeds de minimis limit of ₱{} {}", 
                        benefit, limit, period
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// Generate BIR Form 2307 (Certificate of Creditable Tax Withheld)
    pub fn generate_2307(&self, payment: &Payment) -> Form2307 {
        let vatable = payment.gross_amount / Decimal::from_str("1.12").unwrap();
        
        Form2307 {
            payor_tin: payment.payor_tin.clone(),
            payee_tin: payment.payee_tin.clone(),
            payment_date: payment.date,
            income_payment: vatable,
            ewt_rate: self.ewt_rates[&payment.payment_type],
            tax_withheld: payment.ewt_amount,
            atc_code: self.get_atc_code(&payment.payment_type),
        }
    }
    
    /// Get ATC (Alphanumeric Tax Code) for payment type
    fn get_atc_code(&self, payment_type: &str) -> String {
        match payment_type {
            "professional_high" | "professional_low" => "WI010".to_string(),
            "rental" => "WI020".to_string(),
            "commission" => "WI030".to_string(),
            "goods" => "WI100".to_string(),
            _ => "WI999".to_string(),
        }
    }
}

/// Validation for filing deadlines
pub struct FilingDeadlineValidator;

impl FilingDeadlineValidator {
    pub fn validate_remittance_deadline(&self, withholding_date: NaiveDate) -> Result<NaiveDate, String> {
        // EWT must be remitted by 10th of following month
        let deadline = withholding_date
            .with_day(10)
            .map(|d| d + chrono::Duration::days(30))
            .ok_or("Invalid date")?;
            
        // If 10th falls on weekend/holiday, next business day
        let final_deadline = self.next_business_day(deadline);
        
        Ok(final_deadline)
    }
    
    fn next_business_day(&self, date: NaiveDate) -> NaiveDate {
        match date.weekday() {
            Weekday::Sat => date + chrono::Duration::days(2),
            Weekday::Sun => date + chrono::Duration::days(1),
            _ => date,
        }
    }
}
```

### **/knowledge-base/business-rules/workflows/sales-cycle.md**

```markdown
# Sales Cycle Workflow - Philippine Business Context

## Overview
Complete workflow from quotation → order → delivery → invoice → collection → remittance

## 1. Quotation Stage

### Token Commands
```

@quote for @client [client_name] [items] valid until [date]

```

### Process Flow
1. Check client credit status
2. Compute prices with VAT
3. Apply applicable discounts
4. Generate quotation document
5. Send to client for approval

### Validation Rules
- Client must exist in system
- Products must have current prices
- Validity period required (typically 30 days)
- VAT computation must be shown

## 2. Sales Order

### Token Commands  
```

@client [name] confirmed @quote [quote_id] @order from @client [name] [items]

```

### Process Flow
1. Convert quote to sales order
2. Check inventory availability
3. Reserve stock
4. Generate SO number
5. Schedule delivery

### Journal Entry
No entry yet (order is not yet a sale)

## 3. Delivery

### Token Commands
```

delivered @order [SO_id] to @client [name] with DR [number]

```

### Process Flow
1. Generate Delivery Receipt (DR)
2. Update inventory (decrease)
3. Client acknowledges receipt
4. Prepare sales invoice

### Journal Entry (if FOB destination)
```

Dr. Cost of Goods Sold XXX Cr. Inventory XXX

```

## 4. Sales Invoice

### Token Commands
```

@invoice @client [name] for @order [SO_id] OR/SI [number]

```

### Process Flow
1. Generate Official Receipt (OR) or Sales Invoice (SI)
2. Compute VAT (Output VAT)
3. Apply withholding if applicable
4. Record sale

### Journal Entries

#### Cash Sale (VAT Inclusive)
```

Dr. Cash 11,200 Cr. Sales Revenue 10,000 Cr. Output VAT 1,200

```

#### Credit Sale
```

Dr. Accounts Receivable 11,200 Cr. Sales Revenue 10,000 Cr. Output VAT 1,200

```

#### With Senior/PWD Discount
```

Gross Amount: ₱1,000 Less: Senior Discount (20%): ₱200 Subtotal: ₱800 Add: VAT (12% of ₱800): ₱96 Total: ₱896

Dr. Cash 896 Dr. Senior Discount Expense 200 Cr. Sales Revenue 1,000 Cr. Output VAT 96

```

## 5. Collection

### Token Commands
```

collected @payment from @client [name] for @invoice [SI_id] received PDC from @client [name] check# [number] dated [date]

```

### Process Flow
1. Issue Collection Receipt (CR)
2. Update customer balance
3. Deposit to bank
4. Clear PDCs when due

### Journal Entries

#### Full Payment
```

Dr. Cash 11,200 Cr. Accounts Receivable 11,200

```

#### Partial Payment
```

Dr. Cash 5,000 Cr. Accounts Receivable 5,000

```

#### Post-Dated Check
```

Dr. PDC Receivable 11,200 Cr. Accounts Receivable 11,200

(On check date) Dr. Cash 11,200 Cr. PDC Receivable 11,200

```

## 6. Government Remittance

### Monthly Requirements
1. **Output VAT** (Form 2550M/Q)
   - Due: 20th/25th of following month
   - Computation: Total Output VAT - Input VAT

2. **Withholding Tax** (if applicable)
   - Due: 10th of following month
   - Form 1601EQ for expanded withholding

### Token Commands
```

file VAT return for [month] remit withholding tax for [month]

```

## Special Scenarios

### Sales Return
```

@client [name] returned [items] from @invoice [SI_id]

Dr. Sales Returns 10,000 Dr. Output VAT 1,200 Cr. Accounts Receivable 11,200

Dr. Inventory XXX Cr. Cost of Goods Sold XXX

```

### Bad Debt Write-off
```

write off @client [name] balance

Dr. Bad Debt Expense XXX Cr. Accounts Receivable XXX

```

### Export Sales (Zero-rated)
```

export sale to @client [foreign_name] [amount_USD]

Dr. Accounts Receivable XXX Cr. Sales Revenue - Export XXX (No Output VAT)

```

## Compliance Checklist
- [ ] Sequential OR/SI numbering
- [ ] VAT properly computed
- [ ] Senior/PWD discounts documented
- [ ] Withholding certificates issued
- [ ] Monthly VAT return filed
- [ ] Sales journal updated
- [ ] Subsidiary ledgers reconciled
```

### **/knowledge-base/business-rules/workflows/purchase-cycle.md**


```markdown
# Purchase Cycle Workflow - Philippine Business Context

## Overview
Complete workflow from requisition → PO → receiving → invoice → payment → remittance

## 1. Purchase Requisition

### Token Commands
```

request purchase of [items] for [department/purpose] need to buy [quantity] [item] by [date]

```

### Process Flow
1. Department creates purchase request
2. Check budget availability
3. Supervisor approval
4. Forward to purchasing

### Validation Rules
- Budget must be available
- Approval hierarchy followed
- Business purpose documented

## 2. Purchase Order

### Token Commands
```

@po to @supplier [name] for [items] at [price] order from @supplier [name] [quantity] [items]

```

### Process Flow
1. Select supplier (canvassing if needed)
2. Negotiate terms
3. Generate PO number
4. Send PO to supplier
5. Await confirmation

### No Journal Entry Yet
(Commitment only, not a transaction)

## 3. Receiving

### Token Commands
```

received @po [PO_id] from @supplier [name] RR# [number] partial delivery from @supplier [name] [items]

```

### Process Flow
1. Check items against PO
2. Quality inspection
3. Generate Receiving Report (RR)
4. Update inventory records
5. Forward documents to accounting

### Journal Entry (Perpetual Inventory)
```

Dr. Inventory 10,000 Dr. Input VAT 1,200 Cr. Accounts Payable 11,200

```

## 4. Supplier Invoice Processing

### Token Commands
```

@supplier [name] invoice [number] received for @po [PO_id] process invoice from @supplier [name] amount [total]

```

### Process Flow
1. Three-way matching (PO-RR-Invoice)
2. Validate VAT computation
3. Check for EWT requirement
4. Record payable

### Journal Entries

#### Regular Purchase with VAT
```

Dr. Purchases/Expense 10,000 Dr. Input VAT 1,200 Cr. Accounts Payable 11,200

```

#### Service with EWT (Professional 10%)
```

Gross Amount: ₱56,000 (VAT inclusive) Vatable Amount: ₱50,000 VAT: ₱6,000 EWT: ₱5,000 (10% of ₱50,000)

Dr. Professional Fee 50,000 Dr. Input VAT 6,000 Cr. EWT Payable 5,000 Cr. Accounts Payable 51,000

```

#### Purchase from Non-VAT Supplier
```

Dr. Purchases 10,000 Cr. Accounts Payable 10,000 (No Input VAT to claim)

```

## 5. Payment Processing

### Token Commands
```

pay @supplier [name] for invoice [number] issue check to @supplier [name] amount [total] paid @supplier [name] via bank transfer

```

### Process Flow
1. Verify due date
2. Check early payment discount
3. Prepare payment voucher
4. Issue check/transfer
5. Get official receipt

### Journal Entries

#### Full Payment
```

Dr. Accounts Payable 51,000 Cr. Cash 51,000

```

#### With Early Payment Discount (2/10, n/30)
```

Invoice: ₱100,000 Discount: ₱2,000 (if paid within 10 days)

Dr. Accounts Payable 100,000 Cr. Cash 98,000 Cr. Purchase Discount 2,000

```

## 6. Withholding Tax Remittance

### Token Commands
```

remit EWT for [month] file withholding return

```

### Monthly Requirements
1. **EWT Remittance** (Form 1601EQ)
   - Due: 10th of following month
   - Include all EWT collected

2. **Issue BIR Form 2307**
   - Certificate of Creditable Tax Withheld
   - Give to supplier as proof

### Journal Entry (Remittance)
```

Dr. EWT Payable 5,000 Cr. Cash 5,000

```

## Special Purchase Scenarios

### Import Purchase
```

import from @supplier [foreign] [items] [amount_USD]

Dr. Inventory XXX (PHP equivalent) Dr. Customs Duties XXX Dr. Input VAT XXX Cr. Accounts Payable XXX

```

### Fixed Asset Purchase
```

bought equipment from @supplier [name] [amount]

Dr. Equipment 50,000 Dr. Input VAT 6,000 Cr. Accounts Payable 56,000

```

### Petty Cash Purchase
```

petty cash purchase [items] [amount] OR# [number]

Dr. Supplies Expense 1,000 Dr. Input VAT 120 Cr. Petty Cash 1,120

```

### Purchase Return
```

returned [items] to @supplier [name] CM# [number]

Dr. Accounts Payable 11,200 Cr. Inventory 10,000 Cr. Input VAT 1,200

```

## Compliance Checkpoints

### Documents Required
- [ ] Purchase Order (PO)
- [ ] Receiving Report (RR)
- [ ] Supplier Invoice
- [ ] Official Receipt
- [ ] BIR Form 2307 (if with EWT)

### Monthly Compliance
- [ ] Input VAT summary for Form 2550M
- [ ] EWT remittance via Form 1601EQ
- [ ] Update purchase journal
- [ ] Reconcile supplier statements
- [ ] Age payables for cash flow

## Vendor Management Rules

### Accreditation Requirements
1. Valid business registration
2. BIR Certificate of Registration
3. VAT/Non-VAT status verification
4. Mayor's permit
5. Bank account details

### Payment Terms Hierarchy
```

Government suppliers: 30-60 days Regular vendors: 30 days Preferred vendors: 45-60 days  
New vendors: COD or 15 days

```

## Red Flags & Controls

### Warning Signs
- Invoice without PO
- Receiving without inspection
- Duplicate invoice numbers
- Rush payment requests
- Unusual vendor names

### Required Approvals
| Amount | Approver |
|--------|----------|
| ≤ ₱5,000 | Supervisor |
| ≤ ₱50,000 | Manager |
| ≤ ₱500,000 | Director |
| > ₱500,000 | President/Board |
```

### **/knowledge-base/business-rules/workflows/month-end-closing.md**

```markdown
# Month-End Closing Workflow - Philippine Business Context

## Overview
Systematic process to close books monthly for accurate financial reporting and tax compliance

## Pre-Closing Checklist

### Token Commands
```

start month-end closing for [month] check pending transactions for [month]

```

### Week 4 Preparation (Before Month End)
- [ ] Send customer statements
- [ ] Follow up collections
- [ ] Process pending purchases
- [ ] Submit expense reports
- [ ] Count petty cash

## Day 1-3: Transaction Cutoff

### 1. Sales Cutoff
```

close sales for [month] last OR/SI for [month] is [number]

```

**Validation:**
- All deliveries invoiced
- No post-dated sales
- Sequential OR/SI verified
- Returns processed

### 2. Purchase Cutoff  
```

close purchases for [month] last RR for [month] is [number]

```

**Validation:**
- All received items recorded
- Pending POs reviewed
- Returns documented

### 3. Cash Cutoff
```

last deposit for [month] on [date] last check issued [number]

```

**Bank Reconciliation Required**

## Day 4-7: Adjusting Entries

### 1. Accruals

#### Salaries Payable
```

accrue salaries for [days] days

Dr. Salaries Expense XXX Cr. Salaries Payable XXX

```

#### Utilities
```

accrue electricity [amount] for [month]

Dr. Utilities Expense XXX Dr. Input VAT XXX  
Cr. Accrued Expenses XXX

```

### 2. Prepayments

#### Insurance
```

amortize insurance for [month]

Dr. Insurance Expense XXX Cr. Prepaid Insurance XXX

```

#### Rent
```

amortize rent for [month]

Dr. Rent Expense XXX Dr. Input VAT XXX Cr. Prepaid Rent XXX

```

### 3. Depreciation
```

record depreciation for [month]

Dr. Depreciation Expense XXX Cr. Accumulated Depreciation XXX

```

### 4. Inventory Adjustments
```

adjust inventory per physical count

Dr./Cr. Inventory XXX Cr./Dr. Inventory Adjustment XXX

```

### 5. Allowance for Bad Debts
```

provide allowance for doubtful accounts

Dr. Bad Debt Expense XXX Cr. Allowance for Doubtful Accounts XXX

```

## Day 8-10: Tax Computations

### 1. VAT Reconciliation
```

compute VAT payable for [month]

Output VAT: ₱120,000 Input VAT: (₱80,000) VAT Payable: ₱40,000

```

**Generate Form 2550M/Q**

### 2. Withholding Tax Summary
```

summarize EWT for [month]

Professional Services: ₱15,000 Rental: ₱5,000 Goods: ₱3,000 Total EWT: ₱23,000

```

**Prepare Form 1601EQ**

### 3. Compensation Withholding
```

summarize payroll withholding for [month]

Total Tax Withheld: ₱75,000

```

**Prepare Form 1601C**

## Day 11-15: Financial Statements

### 1. Trial Balance
```

generate trial balance for [month]

```

**Validation:**
- Debits = Credits
- All accounts included
- No suspense accounts

### 2. Income Statement
```

generate income statement for [month]

```

**Key Metrics:**
```

Revenue: ₱1,000,000 COGS: (₱600,000) Gross Profit: ₱400,000 (40%) Operating Expenses: (₱250,000) Net Income: ₱150,000 (15%)

```

### 3. Balance Sheet
```

generate balance sheet as of [month-end]

```

**Validation:**
- Assets = Liabilities + Equity
- Reconciled with subsidiary ledgers
- Working capital positive

### 4. Cash Flow Statement
```

generate cash flow for [month]

```

**Categories:**
- Operating Activities
- Investing Activities  
- Financing Activities

## Day 16-20: Government Compliance

### Filing Schedule

| Form | Description | Deadline |
|------|------------|----------|
| 2550M | Monthly VAT | 20th |
| 2550Q | Quarterly VAT | 25th |
| 1601C | Compensation Withholding | 10th |
| 1601EQ | Expanded Withholding | 10th |
| 0619E | Monthly Sales Report | 20th |

### Token Commands
```

file BIR returns for [month] pay taxes for [month]

```

## Post-Closing Activities

### 1. Lock Period
```

lock accounting period [month]

```
**No backdated entries after locking**

### 2. Backup Data
```

backup books for [month]

```

### 3. Management Reports
```

generate MIS reports for [month] prepare variance analysis

```

### Key Performance Indicators
- Gross Profit Margin
- Operating Margin
- Current Ratio
- Quick Ratio
- DSO (Days Sales Outstanding)
- DPO (Days Payable Outstanding)
- Inventory Turnover

## Month-End Journal Entry Examples

### Reclassifications
```

Reclassify long-term debt current portion

Dr. Long-term Debt 100,000 Cr. Current Portion of LTD 100,000

```

### Provisions
```

Provide for 13th month pay

Dr. 13th Month Pay Expense XXX Cr. 13th Month Pay Payable XXX

```

### Reversals (Start of Next Month)
```

Reverse accruals

Dr. Accrued Expenses XXX Cr. Expense Account XXX

```

## Audit Trail Requirements

### Document Filing
- Sales invoices by number
- Purchase invoices by date
- Bank statements
- Receipts and vouchers
- Tax returns copies

### Digital Requirements
- Transaction logs
- User access logs
- Approval workflows
- Change history

## Common Month-End Errors

### To Avoid
1. Missing cutoff transactions
2. Duplicate entries
3. Unreconciled accounts
4. Wrong period posting
5. Missing supporting documents
6. Calculation errors in tax
7. Unsigned approvals
8. Late government filings

### Resolution Process
```

identify error in [account] prepare adjusting entry get approval from [supervisor] post adjustment document in error log

```

## Closing Timeline Summary

| Day | Activity |
|-----|----------|
| 1-3 | Transaction cutoff |
| 4-7 | Adjusting entries |
| 8-10 | Tax computations |
| 10 | File withholding returns |
| 11-15 | Financial statements |
| 20 | File VAT return |
| 21-25 | Management reports |
| 26-30 | Planning for next month |

## Sign-offs Required

- [ ] Accounting Manager - Trial Balance
- [ ] CFO - Financial Statements  
- [ ] CEO - Management Reports
- [ ] External Auditor - Quarterly (if applicable)

---

*Note: This workflow ensures Philippine tax compliance while maintaining clean books for business decisions. Adjust timeline based on company size and complexity.*
```

## **Complete Knowledge Base Delivered**

This comprehensive knowledge base covers:

1. **Philippine Tax Rules** - VAT, withholding, compliance requirements
2. **BIR Forms** - How to fill and when to file
3. **Accounting Standards** - PFRS for SMEs adapted for Philippines
4. **Chart of Accounts** - Standard 5-digit structure
5. **Validation Rules** - Rust code for enforcing compliance
6. **Business Workflows** - Sales cycle, purchase cycle, month-end closing

Each document provides:

- **Practical formulas** for computation
- **Token patterns** for natural language parsing
- **Journal entries** for common transactions
- **Compliance checklists** to avoid penalties
- **Code examples** ready for implementation

This knowledge base ensures your tokenized bookkeeping system will:

- Automatically compute VAT correctly
- Apply proper withholding taxes
- Generate BIR-compliant reports
- Validate entries before posting
- Guide users through proper workflows
- Prevent common accounting errors

The system can now parse commands like:

- "Si Juan bumili ng computer ₱50k plus VAT less 1% EWT"
- "Senior citizen bought medicines worth 1000 pesos"
- "Export sale to Singapore $10,000"

And automatically create correct, compliant journal entries with proper tax handling.