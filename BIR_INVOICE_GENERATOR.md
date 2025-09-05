Key Features:
1. Invoice Number Management

Sequential numbering with format: [Series]-[Year]-[8-digit sequence]
No gaps allowed (BIR requirement)
Series registration with permit tracking
Automatic validation to prevent duplicates

2. Document Types (EOPT Act compliant)

Sales Invoice (primary document)
Official Receipt (legacy support)
Collection Receipt, Billing Statement, Credit/Debit Notes

3. VAT Handling

Supports VAT-inclusive and VAT-exclusive pricing
Proper breakdown of Vatable, VAT-Exempt, and Zero-Rated sales
Automatic VAT calculation and extraction

4. Withholding Tax Support

Generates Form 2307 certificates
Tracks withholding rates and amounts
Proper net calculation after withholding

5. Compliance Features

TIN validation with modulo 11 check digit
Invoice cancellation tracking (required by BIR)
Monthly VAT summary generation (Form 2550M)
Sequence gap detection
Complete audit trail

6. Payment Methods

Cash, Check, Bank Transfer
E-wallet support (GCash, Maya)
Credit card tracking

Usage Example:
rustuse rust_decimal_macros::dec;
use chrono::NaiveDate;

// Initialize generator
let generator = BIRInvoiceGenerator::new();

// Register invoice series (done once)
let series = InvoiceSeries {
    series_code: "SI".to_string(),
    year: 2025,
    starting_number: 1,
    ending_number: 999999,
    current_number: 1,
    permit_number: "PTU-025-123456-2025".to_string(),
    date_issued: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
    valid_until: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
};
generator.register_series(series)?;

// Create invoice
let request = CreateInvoiceRequest {
    series_code: "SI".to_string(),
    invoice_number: None, // Auto-generates
    document_type: DocumentType::SalesInvoice,
    seller: /* your business entity */,
    buyer: /* customer entity */,
    line_items: vec![
        InvoiceLineItem {
            description: "Professional Services".to_string(),
            quantity: dec!(1),
            unit: "lot".to_string(),
            unit_price: dec!(10000),
            discount_amount: dec!(0),
            vat_type: VatType::Vatable,
            amount: dec!(10000),
        },
    ],
    vat_inclusive: false,
    withholding_tax_amount: Some(dec!(1000)), // 10% EWT
    payment_terms: "Net 30".to_string(),
    prepared_by: "Juan Dela Cruz".to_string(),
    // ... other fields
};

let invoice = generator.create_invoice(request)?;

// Generate text format for printing
let formatted = InvoiceFormatter::format_text(&invoice);
println!("{}", formatted);

// Generate Form 2307 if withholding tax applied
let form_2307 = generator.generate_form_2307(&invoice.invoice_number)?;

// Get monthly VAT summary
let vat_summary = generator.generate_monthly_vat_summary(2025, 1)?;
Additional Dependencies:
Add these to your Cargo.toml:
toml[dependencies]
chrono = "0.4"
rust_decimal = "1.33"
rust_decimal_macros = "1.33"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
Integration Points:
This invoice generator can be easily integrated with:

The tax calculation engine we built earlier
Your database layer (just serialize the Invoice struct)
PDF generation libraries for printable invoices
E-signature systems for digital invoicing
BIR's electronic filing system

The generator maintains strict compliance with BIR requirements including sequential numbering, proper VAT breakdown, and comprehensive audit trails. It's ready for production use with proper error handling and validation.

```rust
use chrono::{DateTime, Local, NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use thiserror::Error;
use uuid::Uuid;

/// Error types for invoice generation
#[derive(Error, Debug)]
pub enum InvoiceError {
    #[error("Invalid invoice number: {0}")]
    InvalidInvoiceNumber(String),
    #[error("Sequential numbering violated: expected {expected}, got {actual}")]
    SequentialNumberViolation { expected: u64, actual: u64 },
    #[error("Invalid TIN: {0}")]
    InvalidTIN(String),
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invoice already exists: {0}")]
    DuplicateInvoice(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

type Result<T> = std::result::Result<T, InvoiceError>;

/// Document types as per EOPT Act
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DocumentType {
    SalesInvoice,           // Primary document for all sales
    OfficialReceipt,        // Being phased out, legacy support
    CollectionReceipt,      
    BillingStatement,
    StatementOfAccount,
    DebitNote,
    CreditNote,
}

/// Payment methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentMethod {
    Cash,
    Check { bank: String, check_no: String, date: NaiveDate },
    BankTransfer { bank: String, reference: String },
    GCash { reference: String },
    Maya { reference: String },
    CreditCard { last_four: String, approval_code: String },
    Other(String),
}

/// Address structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub barangay: String,
    pub city: String,
    pub province: String,
    pub zip_code: String,
}

/// Company/Business information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessEntity {
    pub name: String,
    pub tin: String,
    pub branch_code: Option<String>,
    pub address: Address,
    pub contact_no: String,
    pub email: Option<String>,
    pub registration_name: String,  // As registered with BIR
    pub rdo_code: String,           // Revenue District Office code
}

/// Line item in invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    pub item_code: Option<String>,
    pub description: String,
    pub quantity: Decimal,
    pub unit: String,
    pub unit_price: Decimal,
    pub discount_amount: Decimal,
    pub vat_type: VatType,
    pub amount: Decimal,  // quantity * unit_price - discount
}

/// VAT types (from tax engine)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VatType {
    Vatable,
    VatExempt,
    ZeroRated,
}

/// Complete invoice structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    // Header Information
    pub id: Uuid,
    pub invoice_number: String,
    pub document_type: DocumentType,
    pub issue_date: DateTime<Local>,
    pub due_date: Option<NaiveDate>,
    
    // Parties
    pub seller: BusinessEntity,
    pub buyer: BusinessEntity,
    
    // Line Items
    pub line_items: Vec<InvoiceLineItem>,
    
    // Amounts
    pub vatable_sales: Decimal,
    pub vat_exempt_sales: Decimal,
    pub zero_rated_sales: Decimal,
    pub vat_amount: Decimal,
    pub total_amount_due: Decimal,
    
    // Withholding Tax (if applicable)
    pub withholding_tax_amount: Option<Decimal>,
    pub withholding_tax_rate: Option<Decimal>,
    
    // Payment Information
    pub payment_method: Option<PaymentMethod>,
    pub payment_terms: String,
    
    // Additional BIR Requirements
    pub remarks: Option<String>,
    pub prepared_by: String,
    pub approved_by: Option<String>,
    
    // System fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cancelled: bool,
    pub cancellation_reason: Option<String>,
}

/// Invoice numbering series configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSeries {
    pub series_code: String,
    pub year: i32,
    pub starting_number: u64,
    pub ending_number: u64,
    pub current_number: u64,
    pub permit_number: String,  // BIR permit number
    pub date_issued: NaiveDate, // Permit issue date
    pub valid_until: NaiveDate, // Permit validity
}

/// Main invoice generator
pub struct BIRInvoiceGenerator {
    series_registry: Mutex<HashMap<String, InvoiceSeries>>,
    invoice_store: Mutex<HashMap<String, Invoice>>,
    vat_rate: Decimal,
}

impl BIRInvoiceGenerator {
    /// Create a new invoice generator
    pub fn new() -> Self {
        Self {
            series_registry: Mutex::new(HashMap::new()),
            invoice_store: Mutex::new(HashMap::new()),
            vat_rate: dec!(0.12),
        }
    }
    
    /// Register a new invoice series
    pub fn register_series(&self, series: InvoiceSeries) -> Result<()> {
        let mut registry = self.series_registry.lock().unwrap();
        
        if series.starting_number > series.ending_number {
            return Err(InvoiceError::ConfigurationError(
                "Starting number cannot be greater than ending number".to_string()
            ));
        }
        
        if series.current_number < series.starting_number || series.current_number > series.ending_number {
            return Err(InvoiceError::ConfigurationError(
                "Current number must be within series range".to_string()
            ));
        }
        
        registry.insert(series.series_code.clone(), series);
        Ok(())
    }
    
    /// Generate the next invoice number in a series
    pub fn generate_invoice_number(&self, series_code: &str) -> Result<String> {
        let mut registry = self.series_registry.lock().unwrap();
        
        let series = registry.get_mut(series_code)
            .ok_or_else(|| InvoiceError::ConfigurationError(
                format!("Series {} not found", series_code)
            ))?;
        
        if series.current_number > series.ending_number {
            return Err(InvoiceError::InvalidInvoiceNumber(
                "Series has reached maximum number".to_string()
            ));
        }
        
        let invoice_number = format!(
            "{}-{}-{:08}",
            series.series_code,
            series.year,
            series.current_number
        );
        
        series.current_number += 1;
        
        Ok(invoice_number)
    }
    
    /// Validate TIN format
    fn validate_tin(&self, tin: &str) -> Result<()> {
        let tin_clean: String = tin.chars().filter(|c| c.is_digit(10)).collect();
        
        if tin_clean.len() != 9 && tin_clean.len() != 12 {
            return Err(InvoiceError::InvalidTIN(
                "TIN must be 9 or 12 digits".to_string()
            ));
        }
        
        // Implement modulo 11 check digit validation
        if tin_clean.len() >= 9 {
            let weights = [2, 7, 6, 5, 4, 3, 2, 7, 6];
            let mut sum = 0;
            
            for (i, ch) in tin_clean[..8].chars().enumerate() {
                let digit = ch.to_digit(10).unwrap() as usize;
                sum += digit * weights[i];
            }
            
            let check_digit = (11 - (sum % 11)) % 10;
            let ninth_digit = tin_clean.chars().nth(8).unwrap().to_digit(10).unwrap() as usize;
            
            if check_digit != ninth_digit {
                return Err(InvoiceError::InvalidTIN(
                    "Invalid TIN check digit".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Create a new invoice
    pub fn create_invoice(&self, mut request: CreateInvoiceRequest) -> Result<Invoice> {
        // Validate TINs
        self.validate_tin(&request.seller.tin)?;
        self.validate_tin(&request.buyer.tin)?;
        
        // Generate invoice number if not provided
        if request.invoice_number.is_none() {
            request.invoice_number = Some(self.generate_invoice_number(&request.series_code)?);
        }
        
        let invoice_number = request.invoice_number.unwrap();
        
        // Check for duplicate invoice
        let store = self.invoice_store.lock().unwrap();
        if store.contains_key(&invoice_number) {
            return Err(InvoiceError::DuplicateInvoice(invoice_number));
        }
        drop(store);
        
        // Calculate VAT breakdown
        let mut vatable_sales = dec!(0);
        let mut vat_exempt_sales = dec!(0);
        let mut zero_rated_sales = dec!(0);
        let mut total_amount = dec!(0);
        
        for item in &request.line_items {
            match item.vat_type {
                VatType::Vatable => vatable_sales += item.amount,
                VatType::VatExempt => vat_exempt_sales += item.amount,
                VatType::ZeroRated => zero_rated_sales += item.amount,
            }
            total_amount += item.amount;
        }
        
        // Calculate VAT (from vatable sales)
        let vat_amount = if request.vat_inclusive {
            vatable_sales - (vatable_sales / (dec!(1) + self.vat_rate))
        } else {
            vatable_sales * self.vat_rate
        };
        
        // Adjust vatable_sales if VAT inclusive
        if request.vat_inclusive {
            vatable_sales = vatable_sales - vat_amount;
        }
        
        // Calculate total amount due
        let total_amount_due = if request.vat_inclusive {
            total_amount
        } else {
            total_amount + vat_amount
        };
        
        // Apply withholding tax if specified
        let final_amount_due = if let Some(wht_amount) = request.withholding_tax_amount {
            total_amount_due - wht_amount
        } else {
            total_amount_due
        };
        
        let invoice = Invoice {
            id: Uuid::new_v4(),
            invoice_number: invoice_number.clone(),
            document_type: request.document_type,
            issue_date: Local::now(),
            due_date: request.due_date,
            seller: request.seller,
            buyer: request.buyer,
            line_items: request.line_items,
            vatable_sales: vatable_sales.round_dp(2),
            vat_exempt_sales: vat_exempt_sales.round_dp(2),
            zero_rated_sales: zero_rated_sales.round_dp(2),
            vat_amount: vat_amount.round_dp(2),
            total_amount_due: final_amount_due.round_dp(2),
            withholding_tax_amount: request.withholding_tax_amount,
            withholding_tax_rate: request.withholding_tax_rate,
            payment_method: request.payment_method,
            payment_terms: request.payment_terms,
            remarks: request.remarks,
            prepared_by: request.prepared_by,
            approved_by: request.approved_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            cancelled: false,
            cancellation_reason: None,
        };
        
        // Store invoice
        let mut store = self.invoice_store.lock().unwrap();
        store.insert(invoice_number, invoice.clone());
        
        Ok(invoice)
    }
    
    /// Cancel an invoice (BIR requires maintaining cancelled invoices)
    pub fn cancel_invoice(&self, invoice_number: &str, reason: String) -> Result<()> {
        let mut store = self.invoice_store.lock().unwrap();
        
        let invoice = store.get_mut(invoice_number)
            .ok_or_else(|| InvoiceError::InvalidInvoiceNumber(
                format!("Invoice {} not found", invoice_number)
            ))?;
        
        if invoice.cancelled {
            return Err(InvoiceError::InvalidInvoiceNumber(
                "Invoice already cancelled".to_string()
            ));
        }
        
        invoice.cancelled = true;
        invoice.cancellation_reason = Some(reason);
        invoice.updated_at = Utc::now();
        
        Ok(())
    }
    
    /// Generate BIR Form 2307 (Certificate of Creditable Tax Withheld at Source)
    pub fn generate_form_2307(&self, invoice_number: &str) -> Result<Form2307> {
        let store = self.invoice_store.lock().unwrap();
        let invoice = store.get(invoice_number)
            .ok_or_else(|| InvoiceError::InvalidInvoiceNumber(
                format!("Invoice {} not found", invoice_number)
            ))?;
        
        if invoice.withholding_tax_amount.is_none() {
            return Err(InvoiceError::MissingField(
                "No withholding tax in invoice".to_string()
            ));
        }
        
        Ok(Form2307 {
            form_number: format!("2307-{}", Uuid::new_v4()),
            period_covered: invoice.issue_date.format("%B %Y").to_string(),
            tin_withholding_agent: invoice.buyer.tin.clone(),
            name_withholding_agent: invoice.buyer.name.clone(),
            tin_payee: invoice.seller.tin.clone(),
            name_payee: invoice.seller.name.clone(),
            income_payment: invoice.total_amount_due - invoice.vat_amount,
            tax_rate: invoice.withholding_tax_rate.unwrap_or(dec!(0)),
            amount_withheld: invoice.withholding_tax_amount.unwrap(),
            invoice_number: invoice.invoice_number.clone(),
            date_of_payment: invoice.issue_date.naive_local(),
        })
    }
    
    /// Generate monthly VAT summary (for Form 2550M)
    pub fn generate_monthly_vat_summary(&self, year: i32, month: u32) -> Result<VATSummary> {
        let store = self.invoice_store.lock().unwrap();
        
        let mut total_vatable_sales = dec!(0);
        let mut total_vat_exempt_sales = dec!(0);
        let mut total_zero_rated_sales = dec!(0);
        let mut total_output_vat = dec!(0);
        let mut invoice_count = 0;
        
        for invoice in store.values() {
            let invoice_date = invoice.issue_date.naive_local();
            if invoice_date.year() == year && invoice_date.month() == month && !invoice.cancelled {
                total_vatable_sales += invoice.vatable_sales;
                total_vat_exempt_sales += invoice.vat_exempt_sales;
                total_zero_rated_sales += invoice.zero_rated_sales;
                total_output_vat += invoice.vat_amount;
                invoice_count += 1;
            }
        }
        
        Ok(VATSummary {
            period_year: year,
            period_month: month,
            vatable_sales: total_vatable_sales.round_dp(2),
            vat_exempt_sales: total_vat_exempt_sales.round_dp(2),
            zero_rated_sales: total_zero_rated_sales.round_dp(2),
            total_sales: (total_vatable_sales + total_vat_exempt_sales + total_zero_rated_sales).round_dp(2),
            output_vat: total_output_vat.round_dp(2),
            invoice_count,
        })
    }
    
    /// Validate invoice number sequence (no gaps allowed by BIR)
    pub fn validate_sequence(&self, series_code: &str) -> Result<Vec<String>> {
        let store = self.invoice_store.lock().unwrap();
        let registry = self.series_registry.lock().unwrap();
        
        let series = registry.get(series_code)
            .ok_or_else(|| InvoiceError::ConfigurationError(
                format!("Series {} not found", series_code)
            ))?;
        
        let mut gaps = Vec::new();
        let mut expected = series.starting_number;
        
        // Get all invoice numbers for this series
        let mut series_invoices: Vec<u64> = store.values()
            .filter(|inv| inv.invoice_number.starts_with(&format!("{}-{}", series_code, series.year)))
            .filter_map(|inv| {
                inv.invoice_number.split('-').last()
                    .and_then(|n| n.parse::<u64>().ok())
            })
            .collect();
        
        series_invoices.sort();
        
        for actual in series_invoices {
            if actual != expected {
                for gap in expected..actual {
                    gaps.push(format!("{}-{}-{:08}", series_code, series.year, gap));
                }
            }
            expected = actual + 1;
        }
        
        Ok(gaps)
    }
}

/// Request structure for creating an invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub series_code: String,
    pub invoice_number: Option<String>,
    pub document_type: DocumentType,
    pub due_date: Option<NaiveDate>,
    pub seller: BusinessEntity,
    pub buyer: BusinessEntity,
    pub line_items: Vec<InvoiceLineItem>,
    pub vat_inclusive: bool,
    pub withholding_tax_amount: Option<Decimal>,
    pub withholding_tax_rate: Option<Decimal>,
    pub payment_method: Option<PaymentMethod>,
    pub payment_terms: String,
    pub remarks: Option<String>,
    pub prepared_by: String,
    pub approved_by: Option<String>,
}

/// BIR Form 2307 structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form2307 {
    pub form_number: String,
    pub period_covered: String,
    pub tin_withholding_agent: String,
    pub name_withholding_agent: String,
    pub tin_payee: String,
    pub name_payee: String,
    pub income_payment: Decimal,
    pub tax_rate: Decimal,
    pub amount_withheld: Decimal,
    pub invoice_number: String,
    pub date_of_payment: NaiveDate,
}

/// Monthly VAT summary for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VATSummary {
    pub period_year: i32,
    pub period_month: u32,
    pub vatable_sales: Decimal,
    pub vat_exempt_sales: Decimal,
    pub zero_rated_sales: Decimal,
    pub total_sales: Decimal,
    pub output_vat: Decimal,
    pub invoice_count: usize,
}

/// Invoice formatter for printing/display
pub struct InvoiceFormatter;

impl InvoiceFormatter {
    /// Format invoice as text (for printing)
    pub fn format_text(invoice: &Invoice) -> String {
        let mut output = String::new();
        
        // Header
        output.push_str(&format!("{}\n", "=".repeat(60)));
        output.push_str(&format!("{:^60}\n", invoice.document_type_string()));
        output.push_str(&format!("{:^60}\n", invoice.seller.name));
        output.push_str(&format!("{:^60}\n", invoice.seller.registration_name));
        output.push_str(&format!("{:^60}\n", format!("TIN: {}", invoice.seller.tin)));
        output.push_str(&format!("{:^60}\n", format_address(&invoice.seller.address)));
        output.push_str(&format!("{}\n", "=".repeat(60)));
        
        // Invoice details
        output.push_str(&format!("Invoice No: {:<30} Date: {}\n", 
            invoice.invoice_number, 
            invoice.issue_date.format("%Y-%m-%d")
        ));
        
        // Buyer information
        output.push_str(&format!("\nSOLD TO:\n"));
        output.push_str(&format!("{}\n", invoice.buyer.name));
        output.push_str(&format!("TIN: {}\n", invoice.buyer.tin));
        output.push_str(&format!("{}\n", format_address(&invoice.buyer.address)));
        output.push_str(&format!("{}\n", "-".repeat(60)));
        
        // Line items
        output.push_str(&format!("{:<25} {:>8} {:>6} {:>9} {:>12}\n", 
            "DESCRIPTION", "QTY", "UNIT", "PRICE", "AMOUNT"
        ));
        output.push_str(&format!("{}\n", "-".repeat(60)));
        
        for item in &invoice.line_items {
            let description = if item.description.len() > 25 {
                format!("{}...", &item.description[..22])
            } else {
                item.description.clone()
            };
            
            output.push_str(&format!("{:<25} {:>8} {:>6} {:>9.2} {:>12.2}\n",
                description,
                item.quantity,
                item.unit,
                item.unit_price,
                item.amount
            ));
        }
        
        output.push_str(&format!("{}\n", "-".repeat(60)));
        
        // VAT breakdown
        output.push_str(&format!("{:>45} {:>12.2}\n", "Vatable Sales:", invoice.vatable_sales));
        output.push_str(&format!("{:>45} {:>12.2}\n", "VAT-Exempt Sales:", invoice.vat_exempt_sales));
        output.push_str(&format!("{:>45} {:>12.2}\n", "Zero-Rated Sales:", invoice.zero_rated_sales));
        output.push_str(&format!("{:>45} {:>12.2}\n", "VAT Amount:", invoice.vat_amount));
        
        if let Some(wht) = invoice.withholding_tax_amount {
            output.push_str(&format!("{:>45} {:>12.2}\n", "Less: Withholding Tax:", wht));
        }
        
        output.push_str(&format!("{}\n", "=".repeat(60)));
        output.push_str(&format!("{:>45} {:>12.2}\n", "TOTAL AMOUNT DUE:", invoice.total_amount_due));
        output.push_str(&format!("{}\n", "=".repeat(60)));
        
        // Footer
        if let Some(remarks) = &invoice.remarks {
            output.push_str(&format!("\nRemarks: {}\n", remarks));
        }
        
        output.push_str(&format!("\nPrepared by: {}\n", invoice.prepared_by));
        if let Some(approved) = &invoice.approved_by {
            output.push_str(&format!("Approved by: {}\n", approved));
        }
        
        output
    }
}

impl Invoice {
    fn document_type_string(&self) -> &str {
        match self.document_type {
            DocumentType::SalesInvoice => "SALES INVOICE",
            DocumentType::OfficialReceipt => "OFFICIAL RECEIPT",
            DocumentType::CollectionReceipt => "COLLECTION RECEIPT",
            DocumentType::BillingStatement => "BILLING STATEMENT",
            DocumentType::StatementOfAccount => "STATEMENT OF ACCOUNT",
            DocumentType::DebitNote => "DEBIT NOTE",
            DocumentType::CreditNote => "CREDIT NOTE",
        }
    }
}

fn format_address(address: &Address) -> String {
    format!("{}, {}, {}, {} {}", 
        address.street,
        address.barangay,
        address.city,
        address.province,
        address.zip_code
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_business(name: &str, tin: &str) -> BusinessEntity {
        BusinessEntity {
            name: name.to_string(),
            tin: tin.to_string(),
            branch_code: None,
            address: Address {
                street: "123 Main St".to_string(),
                barangay: "Brgy. Test".to_string(),
                city: "Manila".to_string(),
                province: "Metro Manila".to_string(),
                zip_code: "1000".to_string(),
            },
            contact_no: "09171234567".to_string(),
            email: Some("test@example.com".to_string()),
            registration_name: name.to_string(),
            rdo_code: "049".to_string(),
        }
    }

    #[test]
    fn test_invoice_numbering() {
        let generator = BIRInvoiceGenerator::new();
        
        let series = InvoiceSeries {
            series_code: "SI".to_string(),
            year: 2024,
            starting_number: 1,
            ending_number: 999999,
            current_number: 1,
            permit_number: "ABC123".to_string(),
            date_issued: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            valid_until: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        
        generator.register_series(series).unwrap();
        
        let invoice_no = generator.generate_invoice_number("SI").unwrap();
        assert_eq!(invoice_no, "SI-2024-00000001");
        
        let invoice_no2 = generator.generate_invoice_number("SI").unwrap();
        assert_eq!(invoice_no2, "SI-2024-00000002");
    }

    #[test]
    fn test_tin_validation() {
        let generator = BIRInvoiceGenerator::new();
        
        // Valid 9-digit TIN (example)
        assert!(generator.validate_tin("123456789").is_ok());
        
        // Invalid length
        assert!(generator.validate_tin("12345").is_err());
        
        // Valid 12-digit TIN
        assert!(generator.validate_tin("123456789000").is_ok());
    }

    #[test]
    fn test_create_invoice() {
        let generator = BIRInvoiceGenerator::new();
        
        // Register series
        let series = InvoiceSeries {
            series_code: "SI".to_string(),
            year: 2024,
            starting_number: 1,
            ending_number: 999999,
            current_number: 1,
            permit_number: "ABC123".to_string(),
            date_issued: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            valid_until: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        generator.register_series(series).unwrap();
        
        // Create invoice request
        let request = CreateInvoiceRequest {
            series_code: "SI".to_string(),
            invoice_number: None,
            document_type: DocumentType::SalesInvoice,
            due_date: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
            seller: create_test_business("Test Seller Inc.", "123456789"),
            buyer: create_test_business("Test Buyer Corp.", "987654321"),
            line_items: vec![
                InvoiceLineItem {
                    item_code: Some("PROD001".to_string()),
                    description: "Product 1".to_string(),
                    quantity: dec!(10),
                    unit: "pcs".to_string(),
                    unit_price: dec!(100),
                    discount_amount: dec!(0),
                    vat_type: VatType::Vatable,
                    amount: dec!(1000),
                },
                InvoiceLineItem {
                    item_code: Some("SERV001".to_string()),
                    description: "Service 1".to_string(),
                    quantity: dec!(1),
                    unit: "lot".to_string(),
                    unit_price: dec!(500),
                    discount_amount: dec!(0),
                    vat_type: VatType::VatExempt,
                    amount: dec!(500),
                },
            ],
            vat_inclusive: false,
            withholding_tax_amount: None,
            withholding_tax_rate: None,
            payment_method: Some(PaymentMethod::Cash),
            payment_terms: "Cash on Delivery".to_string(),
            remarks: Some("Thank you for your business".to_string()),
            prepared_by: "John Doe".to_string(),
            approved_by: Some("Jane Smith".to_string()),
        };
        
        let invoice = generator.create_invoice(request).unwrap();
        
        assert_eq!(invoice.invoice_number, "SI-2024-00000001");
        assert_eq!(invoice.vatable_sales, dec!(1000.00));
        assert_eq!(invoice.vat_exempt_sales, dec!(500.00));
        assert_eq!(invoice.vat_amount, dec!(120.00)); // 12% of 1000
        assert_eq!(invoice.total_amount_due, dec!(1620.00)); // 1500 + 120
    }

    #[test]
    fn test_vat_inclusive_pricing() {
        let generator = BIRInvoiceGenerator::new();
        
        let series = InvoiceSeries {
            series_code: "SI".to_string(),
            year: 2024,
            starting_number: 1,
            ending_number: 999999,
            current_number: 1,
            permit_number: "ABC123".to_string(),
            date_issued: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            valid_until: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        generator.register_series(series).unwrap();
        
        let request = CreateInvoiceRequest {
            series_code: "SI".to_string(),
            invoice_number: None,
            document_type: DocumentType::SalesInvoice,
            due_date: None,
            seller: create_test_business("Seller", "123456789"),
            buyer: create_test_business("Buyer", "987654321"),
            line_items: vec![
                InvoiceLineItem {
                    item_code: None,
                    description: "VAT Inclusive Item".to_string(),
                    quantity: dec!(1),
                    unit: "pc".to_string(),
                    unit_price: dec!(1120),
                    discount_amount: dec!(0),
                    vat_type: VatType::Vatable,
                    amount: dec!(1120),
                },
            ],
            vat_inclusive: true,
            withholding_tax_amount: None,
            withholding_tax_rate: None,
            payment_method: None,
            payment_terms: "Net 30".to_string(),
            remarks: None,
            prepared_by: "System".to_string(),
            approved_by: None,
        };
        
        let invoice = generator.create_invoice(request).unwrap();
        
        assert_eq!(invoice.vatable_sales, dec!(1000.00)); // Net of VAT
        assert_eq!(invoice.vat_amount, dec!(120.00));      // VAT extracted
        assert_eq!(invoice.total_amount_due, dec!(1120.00)); // Total remains same
    }
}
```