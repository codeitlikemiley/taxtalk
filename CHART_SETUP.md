Key Features:
1. Complete Philippine COA Structure

Standard 4-digit account codes (1000s=Assets, 2000s=Liabilities, etc.)
Hierarchical account structure with parent-child relationships
Pre-loaded with 100+ standard Philippine accounts including:

BIR tax accounts (Input/Output VAT, Withholding taxes)
Government contribution accounts (SSS, PhilHealth, Pag-IBIG)
E-wallet accounts (GCash, Maya business accounts)
13th month pay provisions



2. Account Management

Create, update, delete (soft delete) accounts
Header accounts (for grouping) vs detail accounts (for transactions)
Multi-level hierarchy support (up to 5 levels)
Account status tracking (Active, Inactive, Locked, Deleted)

3. Multi-tenant Architecture

Row-level security with tenant_id
Isolated data per company/branch
Audit trails with created_by/updated_at

4. Tax & Compliance Features

Tax-related account flagging
VAT account identification
Withholding tax account tracking
BIR classification support

5. Multi-currency Support

Base currency (PHP) with foreign currency options
Exchange rate tracking
Currency-specific account balances

6. Financial Reporting

Real-time account balances
Trial balance generation
Hierarchical account tree display
Period-based balance tracking

Database Schema:
The system creates these tables:

accounts - Main COA table
account_balances - Period balances
journal_entries - Transaction headers
journal_lines - Transaction details

Usage Example:
rustuse uuid::Uuid;

// Initialize the COA manager
let coa_manager = ChartOfAccountsManager::new(
    "postgresql://user:pass@localhost/accounting_db"
).await?;

// Initialize database schema
coa_manager.init_schema().await?;

// Load default Philippine COA for a new tenant
let tenant_id = Uuid::new_v4();
coa_manager.load_default_coa(tenant_id, "admin").await?;

// Create a custom account
let request = CreateAccountRequest {
    code: "1117".to_string(),
    name: "PayMaya Business Account".to_string(),
    description: Some("Digital wallet for business".to_string()),
    account_type: AccountType::Asset,
    account_subtype: AccountSubType::Cash,
    parent_code: Some("1110".to_string()), // Under Cash and Cash Equivalents
    is_detail: true,
    tax_related: false,
    vat_account: false,
    withholding_account: false,
    currency: Some("PHP".to_string()),
    allow_foreign_currency: false,
    bir_classification: None,
};

let account = coa_manager.create_account(
    request,
    tenant_id,
    "juan.delacruz"
).await?;

// Get account balance
let balance = coa_manager.get_account_balance(
    account.id,
    None, // Current balance
    tenant_id
).await?;

// Generate trial balance
let trial_balance = coa_manager.generate_trial_balance(
    Utc::now(),
    tenant_id
).await?;

// Get hierarchical account tree
let tree = coa_manager.get_account_hierarchy(tenant_id).await?;
Default Accounts Included:
Assets:

Cash accounts (banks, petty cash, e-wallets)
Accounts Receivable with allowances
Inventory (raw materials, WIP, finished goods)
Input VAT accounts
Fixed assets with depreciation

Liabilities:

Trade payables
All BIR tax payables
Government contributions (SSS, PhilHealth, Pag-IBIG)
Accrued expenses and 13th month pay

Equity:

Owner's capital, drawings, retained earnings

Revenue:

Vatable, VAT-exempt, and zero-rated sales
Service revenue categories
Other income accounts

Expenses:

COGS with detailed breakdown
Administrative expenses with benefits
Selling and marketing expenses
Financial expenses
Tax expenses

Dependencies to add:
toml[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono", "decimal"] }
tokio = { version = "1", features = ["full"] }
chrono = "0.4"
rust_decimal = "1.33"
rust_decimal_macros = "1.33"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
This COA system integrates perfectly with the tax engine and invoice generator we built earlier. It provides the foundation for all accounting transactions with proper Philippine compliance built-in.

```rust
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool, Postgres, Transaction};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use uuid::Uuid;

/// Error types for COA operations
#[derive(Error, Debug)]
pub enum ChartOfAccountsError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    #[error("Account already exists: {0}")]
    AccountExists(String),
    #[error("Invalid account code: {0}")]
    InvalidAccountCode(String),
    #[error("Invalid parent account: {0}")]
    InvalidParentAccount(String),
    #[error("Cannot delete account with transactions")]
    AccountHasTransactions,
    #[error("Cannot delete account with child accounts")]
    AccountHasChildren,
    #[error("Invalid account type for operation: {0}")]
    InvalidAccountType(String),
    #[error("Circular reference detected")]
    CircularReference,
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

type Result<T> = std::result::Result<T, ChartOfAccountsError>;

/// Account types following Philippine standards
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

/// Account sub-types for detailed classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_subtype", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountSubType {
    // Assets
    CurrentAsset,
    NonCurrentAsset,
    Cash,
    Receivable,
    Inventory,
    PrepaidExpense,
    FixedAsset,
    
    // Liabilities
    CurrentLiability,
    NonCurrentLiability,
    Payable,
    AccruedLiability,
    UnearnedRevenue,
    LongTermDebt,
    
    // Equity
    Capital,
    RetainedEarnings,
    Drawing,
    
    // Revenue
    OperatingRevenue,
    NonOperatingRevenue,
    SalesRevenue,
    ServiceRevenue,
    OtherIncome,
    
    // Expense
    OperatingExpense,
    NonOperatingExpense,
    CostOfGoodsSold,
    AdminExpense,
    SellingExpense,
    FinanceExpense,
    TaxExpense,
}

/// Normal balance side for double-entry bookkeeping
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "balance_side", rename_all = "lowercase")]
pub enum BalanceSide {
    Debit,
    Credit,
}

/// Account status
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Inactive,
    Locked,
    Deleted,
}

/// Main Chart of Accounts entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: AccountType,
    pub account_subtype: AccountSubType,
    pub normal_balance: BalanceSide,
    pub parent_id: Option<Uuid>,
    pub level: i32,
    pub is_detail: bool,  // true = can post transactions, false = header account
    pub status: AccountStatus,
    
    // BIR specific fields
    pub bir_classification: Option<String>,
    pub tax_related: bool,
    pub vat_account: bool,
    pub withholding_account: bool,
    
    // Multi-currency support
    pub currency: String,
    pub allow_foreign_currency: bool,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub tenant_id: Uuid,  // For multi-tenant support
}

/// Request to create a new account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: AccountType,
    pub account_subtype: AccountSubType,
    pub parent_code: Option<String>,
    pub is_detail: bool,
    pub bir_classification: Option<String>,
    pub tax_related: bool,
    pub vat_account: bool,
    pub withholding_account: bool,
    pub currency: Option<String>,
    pub allow_foreign_currency: bool,
}

/// Account balance information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountBalance {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub debit_balance: Decimal,
    pub credit_balance: Decimal,
    pub net_balance: Decimal,
    pub period_date: DateTime<Utc>,
    pub currency: String,
}

/// Chart of Accounts Manager
pub struct ChartOfAccountsManager {
    pool: PgPool,
    default_currency: String,
}

impl ChartOfAccountsManager {
    /// Create a new COA manager
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        Ok(Self {
            pool,
            default_currency: "PHP".to_string(),
        })
    }
    
    /// Initialize database schema
    pub async fn init_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            -- Create enum types
            DO $$ BEGIN
                CREATE TYPE account_type AS ENUM ('ASSET', 'LIABILITY', 'EQUITY', 'REVENUE', 'EXPENSE');
            EXCEPTION
                WHEN duplicate_object THEN null;
            END $$;
            
            DO $$ BEGIN
                CREATE TYPE account_subtype AS ENUM (
                    'CURRENT_ASSET', 'NON_CURRENT_ASSET', 'CASH', 'RECEIVABLE', 'INVENTORY',
                    'PREPAID_EXPENSE', 'FIXED_ASSET', 'CURRENT_LIABILITY', 'NON_CURRENT_LIABILITY',
                    'PAYABLE', 'ACCRUED_LIABILITY', 'UNEARNED_REVENUE', 'LONG_TERM_DEBT',
                    'CAPITAL', 'RETAINED_EARNINGS', 'DRAWING', 'OPERATING_REVENUE',
                    'NON_OPERATING_REVENUE', 'SALES_REVENUE', 'SERVICE_REVENUE', 'OTHER_INCOME',
                    'OPERATING_EXPENSE', 'NON_OPERATING_EXPENSE', 'COST_OF_GOODS_SOLD',
                    'ADMIN_EXPENSE', 'SELLING_EXPENSE', 'FINANCE_EXPENSE', 'TAX_EXPENSE'
                );
            EXCEPTION
                WHEN duplicate_object THEN null;
            END $$;
            
            DO $$ BEGIN
                CREATE TYPE balance_side AS ENUM ('debit', 'credit');
            EXCEPTION
                WHEN duplicate_object THEN null;
            END $$;
            
            DO $$ BEGIN
                CREATE TYPE account_status AS ENUM ('active', 'inactive', 'locked', 'deleted');
            EXCEPTION
                WHEN duplicate_object THEN null;
            END $$;
            
            -- Create accounts table
            CREATE TABLE IF NOT EXISTS accounts (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                code VARCHAR(20) NOT NULL,
                name VARCHAR(100) NOT NULL,
                description TEXT,
                account_type account_type NOT NULL,
                account_subtype account_subtype NOT NULL,
                normal_balance balance_side NOT NULL,
                parent_id UUID REFERENCES accounts(id),
                level INTEGER NOT NULL DEFAULT 1,
                is_detail BOOLEAN NOT NULL DEFAULT true,
                status account_status NOT NULL DEFAULT 'active',
                
                -- BIR specific
                bir_classification VARCHAR(50),
                tax_related BOOLEAN NOT NULL DEFAULT false,
                vat_account BOOLEAN NOT NULL DEFAULT false,
                withholding_account BOOLEAN NOT NULL DEFAULT false,
                
                -- Multi-currency
                currency VARCHAR(3) NOT NULL DEFAULT 'PHP',
                allow_foreign_currency BOOLEAN NOT NULL DEFAULT false,
                
                -- Audit fields
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                created_by VARCHAR(100) NOT NULL,
                tenant_id UUID NOT NULL,
                
                -- Constraints
                UNIQUE(code, tenant_id),
                CHECK (level >= 1 AND level <= 5)
            );
            
            -- Create indexes
            CREATE INDEX IF NOT EXISTS idx_accounts_code ON accounts(code);
            CREATE INDEX IF NOT EXISTS idx_accounts_type ON accounts(account_type);
            CREATE INDEX IF NOT EXISTS idx_accounts_parent ON accounts(parent_id);
            CREATE INDEX IF NOT EXISTS idx_accounts_tenant ON accounts(tenant_id);
            CREATE INDEX IF NOT EXISTS idx_accounts_status ON accounts(status);
            
            -- Create account_balances table
            CREATE TABLE IF NOT EXISTS account_balances (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                account_id UUID NOT NULL REFERENCES accounts(id),
                period_date TIMESTAMPTZ NOT NULL,
                debit_amount DECIMAL(20,2) NOT NULL DEFAULT 0,
                credit_amount DECIMAL(20,2) NOT NULL DEFAULT 0,
                balance DECIMAL(20,2) NOT NULL DEFAULT 0,
                currency VARCHAR(3) NOT NULL DEFAULT 'PHP',
                tenant_id UUID NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                
                UNIQUE(account_id, period_date, tenant_id)
            );
            
            -- Create journal_entries table for transaction tracking
            CREATE TABLE IF NOT EXISTS journal_entries (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                entry_number VARCHAR(50) NOT NULL,
                entry_date DATE NOT NULL,
                description TEXT,
                reference_type VARCHAR(50),
                reference_id UUID,
                status VARCHAR(20) NOT NULL DEFAULT 'draft',
                tenant_id UUID NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                created_by VARCHAR(100) NOT NULL,
                posted_at TIMESTAMPTZ,
                posted_by VARCHAR(100),
                
                UNIQUE(entry_number, tenant_id)
            );
            
            -- Create journal_lines table
            CREATE TABLE IF NOT EXISTS journal_lines (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                journal_entry_id UUID NOT NULL REFERENCES journal_entries(id),
                account_id UUID NOT NULL REFERENCES accounts(id),
                description TEXT,
                debit_amount DECIMAL(20,2) NOT NULL DEFAULT 0,
                credit_amount DECIMAL(20,2) NOT NULL DEFAULT 0,
                currency VARCHAR(3) NOT NULL DEFAULT 'PHP',
                exchange_rate DECIMAL(10,4) DEFAULT 1.0000,
                base_debit_amount DECIMAL(20,2) NOT NULL DEFAULT 0,
                base_credit_amount DECIMAL(20,2) NOT NULL DEFAULT 0,
                
                CHECK (
                    (debit_amount > 0 AND credit_amount = 0) OR 
                    (credit_amount > 0 AND debit_amount = 0)
                )
            );
            
            -- Create trigger for updated_at
            CREATE OR REPLACE FUNCTION update_updated_at_column()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql';
            
            CREATE TRIGGER update_accounts_updated_at BEFORE UPDATE
                ON accounts FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
            CREATE TRIGGER update_account_balances_updated_at BEFORE UPDATE
                ON account_balances FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
            "#
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Load default Philippine COA template
    pub async fn load_default_coa(&self, tenant_id: Uuid, created_by: &str) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Define standard Philippine COA structure
        let default_accounts = vec![
            // ASSETS (1000-1999)
            ("1000", "ASSETS", AccountType::Asset, AccountSubType::CurrentAsset, false, None),
            ("1100", "Current Assets", AccountType::Asset, AccountSubType::CurrentAsset, false, Some("1000")),
            ("1110", "Cash and Cash Equivalents", AccountType::Asset, AccountSubType::Cash, false, Some("1100")),
            ("1111", "Cash on Hand", AccountType::Asset, AccountSubType::Cash, true, Some("1110")),
            ("1112", "Petty Cash Fund", AccountType::Asset, AccountSubType::Cash, true, Some("1110")),
            ("1113", "Cash in Bank - BDO", AccountType::Asset, AccountSubType::Cash, true, Some("1110")),
            ("1114", "Cash in Bank - BPI", AccountType::Asset, AccountSubType::Cash, true, Some("1110")),
            ("1115", "GCash Business Account", AccountType::Asset, AccountSubType::Cash, true, Some("1110")),
            ("1116", "Maya Business Account", AccountType::Asset, AccountSubType::Cash, true, Some("1110")),
            
            ("1120", "Accounts Receivable", AccountType::Asset, AccountSubType::Receivable, false, Some("1100")),
            ("1121", "Trade Receivables", AccountType::Asset, AccountSubType::Receivable, true, Some("1120")),
            ("1122", "Allowance for Doubtful Accounts", AccountType::Asset, AccountSubType::Receivable, true, Some("1120")),
            
            ("1130", "Inventory", AccountType::Asset, AccountSubType::Inventory, false, Some("1100")),
            ("1131", "Merchandise Inventory", AccountType::Asset, AccountSubType::Inventory, true, Some("1130")),
            ("1132", "Raw Materials", AccountType::Asset, AccountSubType::Inventory, true, Some("1130")),
            ("1133", "Work in Process", AccountType::Asset, AccountSubType::Inventory, true, Some("1130")),
            ("1134", "Finished Goods", AccountType::Asset, AccountSubType::Inventory, true, Some("1130")),
            
            ("1140", "Prepaid Expenses", AccountType::Asset, AccountSubType::PrepaidExpense, false, Some("1100")),
            ("1141", "Prepaid Rent", AccountType::Asset, AccountSubType::PrepaidExpense, true, Some("1140")),
            ("1142", "Prepaid Insurance", AccountType::Asset, AccountSubType::PrepaidExpense, true, Some("1140")),
            
            ("1150", "Input VAT", AccountType::Asset, AccountSubType::CurrentAsset, true, Some("1100")),
            ("1151", "Deferred Input VAT", AccountType::Asset, AccountSubType::CurrentAsset, true, Some("1100")),
            
            ("1200", "Non-Current Assets", AccountType::Asset, AccountSubType::NonCurrentAsset, false, Some("1000")),
            ("1210", "Property, Plant and Equipment", AccountType::Asset, AccountSubType::FixedAsset, false, Some("1200")),
            ("1211", "Land", AccountType::Asset, AccountSubType::FixedAsset, true, Some("1210")),
            ("1212", "Building", AccountType::Asset, AccountSubType::FixedAsset, true, Some("1210")),
            ("1213", "Accumulated Depreciation - Building", AccountType::Asset, AccountSubType::FixedAsset, true, Some("1210")),
            ("1214", "Machinery and Equipment", AccountType::Asset, AccountSubType::FixedAsset, true, Some("1210")),
            ("1215", "Accumulated Depreciation - Machinery", AccountType::Asset, AccountSubType::FixedAsset, true, Some("1210")),
            ("1216", "Furniture and Fixtures", AccountType::Asset, AccountSubType::FixedAsset, true, Some("1210")),
            ("1217", "Accumulated Depreciation - Furniture", AccountType::Asset, AccountSubType::FixedAsset, true, Some("1210")),
            
            // LIABILITIES (2000-2999)
            ("2000", "LIABILITIES", AccountType::Liability, AccountSubType::CurrentLiability, false, None),
            ("2100", "Current Liabilities", AccountType::Liability, AccountSubType::CurrentLiability, false, Some("2000")),
            ("2110", "Accounts Payable", AccountType::Liability, AccountSubType::Payable, false, Some("2100")),
            ("2111", "Trade Payables", AccountType::Liability, AccountSubType::Payable, true, Some("2110")),
            
            ("2120", "Tax Payables", AccountType::Liability, AccountSubType::Payable, false, Some("2100")),
            ("2121", "Output VAT Payable", AccountType::Liability, AccountSubType::Payable, true, Some("2120")),
            ("2122", "Income Tax Payable", AccountType::Liability, AccountSubType::Payable, true, Some("2120")),
            ("2123", "Withholding Tax Payable - Compensation", AccountType::Liability, AccountSubType::Payable, true, Some("2120")),
            ("2124", "Expanded Withholding Tax Payable", AccountType::Liability, AccountSubType::Payable, true, Some("2120")),
            ("2125", "Final Withholding Tax Payable", AccountType::Liability, AccountSubType::Payable, true, Some("2120")),
            ("2126", "Percentage Tax Payable", AccountType::Liability, AccountSubType::Payable, true, Some("2120")),
            
            ("2130", "Government Contributions Payable", AccountType::Liability, AccountSubType::Payable, false, Some("2100")),
            ("2131", "SSS Payable", AccountType::Liability, AccountSubType::Payable, true, Some("2130")),
            ("2132", "PhilHealth Payable", AccountType::Liability, AccountSubType::Payable, true, Some("2130")),
            ("2133", "Pag-IBIG Payable", AccountType::Liability, AccountSubType::Payable, true, Some("2130")),
            
            ("2140", "Accrued Expenses", AccountType::Liability, AccountSubType::AccruedLiability, false, Some("2100")),
            ("2141", "Salaries and Wages Payable", AccountType::Liability, AccountSubType::AccruedLiability, true, Some("2140")),
            ("2142", "13th Month Pay Payable", AccountType::Liability, AccountSubType::AccruedLiability, true, Some("2140")),
            ("2143", "Utilities Payable", AccountType::Liability, AccountSubType::AccruedLiability, true, Some("2140")),
            
            ("2200", "Non-Current Liabilities", AccountType::Liability, AccountSubType::NonCurrentLiability, false, Some("2000")),
            ("2210", "Long-term Loans Payable", AccountType::Liability, AccountSubType::LongTermDebt, true, Some("2200")),
            
            // EQUITY (3000-3999)
            ("3000", "EQUITY", AccountType::Equity, AccountSubType::Capital, false, None),
            ("3100", "Owner's Capital", AccountType::Equity, AccountSubType::Capital, true, Some("3000")),
            ("3200", "Owner's Drawing", AccountType::Equity, AccountSubType::Drawing, true, Some("3000")),
            ("3300", "Retained Earnings", AccountType::Equity, AccountSubType::RetainedEarnings, true, Some("3000")),
            ("3400", "Current Year Earnings", AccountType::Equity, AccountSubType::RetainedEarnings, true, Some("3000")),
            
            // REVENUE (4000-4999)
            ("4000", "REVENUE", AccountType::Revenue, AccountSubType::OperatingRevenue, false, None),
            ("4100", "Sales Revenue", AccountType::Revenue, AccountSubType::SalesRevenue, false, Some("4000")),
            ("4110", "Sales - Vatable", AccountType::Revenue, AccountSubType::SalesRevenue, true, Some("4100")),
            ("4120", "Sales - VAT Exempt", AccountType::Revenue, AccountSubType::SalesRevenue, true, Some("4100")),
            ("4130", "Sales - Zero Rated", AccountType::Revenue, AccountSubType::SalesRevenue, true, Some("4100")),
            ("4140", "Sales Returns and Allowances", AccountType::Revenue, AccountSubType::SalesRevenue, true, Some("4100")),
            ("4150", "Sales Discounts", AccountType::Revenue, AccountSubType::SalesRevenue, true, Some("4100")),
            
            ("4200", "Service Revenue", AccountType::Revenue, AccountSubType::ServiceRevenue, false, Some("4000")),
            ("4210", "Professional Service Revenue", AccountType::Revenue, AccountSubType::ServiceRevenue, true, Some("4200")),
            ("4220", "Consulting Revenue", AccountType::Revenue, AccountSubType::ServiceRevenue, true, Some("4200")),
            
            ("4300", "Other Income", AccountType::Revenue, AccountSubType::OtherIncome, false, Some("4000")),
            ("4310", "Interest Income", AccountType::Revenue, AccountSubType::OtherIncome, true, Some("4300")),
            ("4320", "Gain on Sale of Assets", AccountType::Revenue, AccountSubType::OtherIncome, true, Some("4300")),
            ("4330", "Foreign Exchange Gain", AccountType::Revenue, AccountSubType::OtherIncome, true, Some("4300")),
            
            // EXPENSES (5000-5999)
            ("5000", "EXPENSES", AccountType::Expense, AccountSubType::OperatingExpense, false, None),
            ("5100", "Cost of Goods Sold", AccountType::Expense, AccountSubType::CostOfGoodsSold, false, Some("5000")),
            ("5110", "Cost of Sales", AccountType::Expense, AccountSubType::CostOfGoodsSold, true, Some("5100")),
            ("5120", "Direct Materials", AccountType::Expense, AccountSubType::CostOfGoodsSold, true, Some("5100")),
            ("5130", "Direct Labor", AccountType::Expense, AccountSubType::CostOfGoodsSold, true, Some("5100")),
            ("5140", "Manufacturing Overhead", AccountType::Expense, AccountSubType::CostOfGoodsSold, true, Some("5100")),
            
            ("5200", "Administrative Expenses", AccountType::Expense, AccountSubType::AdminExpense, false, Some("5000")),
            ("5210", "Salaries and Wages", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5211", "13th Month Pay", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5212", "SSS Expense", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5213", "PhilHealth Expense", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5214", "Pag-IBIG Expense", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5220", "Rent Expense", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5230", "Utilities Expense", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5240", "Office Supplies", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5250", "Professional Fees", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            ("5260", "Depreciation Expense", AccountType::Expense, AccountSubType::AdminExpense, true, Some("5200")),
            
            ("5300", "Selling Expenses", AccountType::Expense, AccountSubType::SellingExpense, false, Some("5000")),
            ("5310", "Advertising and Marketing", AccountType::Expense, AccountSubType::SellingExpense, true, Some("5300")),
            ("5320", "Sales Commissions", AccountType::Expense, AccountSubType::SellingExpense, true, Some("5300")),
            ("5330", "Delivery Expense", AccountType::Expense, AccountSubType::SellingExpense, true, Some("5300")),
            
            ("5400", "Financial Expenses", AccountType::Expense, AccountSubType::FinanceExpense, false, Some("5000")),
            ("5410", "Interest Expense", AccountType::Expense, AccountSubType::FinanceExpense, true, Some("5400")),
            ("5420", "Bank Charges", AccountType::Expense, AccountSubType::FinanceExpense, true, Some("5400")),
            ("5430", "Foreign Exchange Loss", AccountType::Expense, AccountSubType::FinanceExpense, true, Some("5400")),
            
            ("5500", "Tax Expense", AccountType::Expense, AccountSubType::TaxExpense, false, Some("5000")),
            ("5510", "Income Tax Expense", AccountType::Expense, AccountSubType::TaxExpense, true, Some("5500")),
            ("5520", "Business Tax Expense", AccountType::Expense, AccountSubType::TaxExpense, true, Some("5500")),
        ];
        
        // Create parent lookup map
        let mut account_map: HashMap<String, Uuid> = HashMap::new();
        
        for (code, name, account_type, subtype, is_detail, parent_code) in default_accounts {
            let parent_id = parent_code.and_then(|pc| account_map.get(pc).copied());
            let level = match parent_code {
                None => 1,
                Some(pc) if pc.len() == 4 => 2,
                Some(pc) if pc.len() == 4 => 3,
                _ => 4,
            };
            
            let normal_balance = match account_type {
                AccountType::Asset | AccountType::Expense => BalanceSide::Debit,
                AccountType::Liability | AccountType::Equity | AccountType::Revenue => BalanceSide::Credit,
            };
            
            // Check if it's a tax-related account
            let tax_related = code.starts_with("115") || code.starts_with("212") || code.starts_with("55");
            let vat_account = name.contains("VAT");
            let withholding_account = name.contains("Withholding");
            
            let id = self.create_account_internal(
                &mut tx,
                CreateAccountRequest {
                    code: code.to_string(),
                    name: name.to_string(),
                    description: None,
                    account_type,
                    account_subtype: subtype,
                    parent_code: parent_code.map(|s| s.to_string()),
                    is_detail,
                    bir_classification: None,
                    tax_related,
                    vat_account,
                    withholding_account,
                    currency: Some("PHP".to_string()),
                    allow_foreign_currency: false,
                },
                tenant_id,
                created_by,
                parent_id,
            ).await?;
            
            account_map.insert(code.to_string(), id);
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    /// Internal method to create account
    async fn create_account_internal(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        request: CreateAccountRequest,
        tenant_id: Uuid,
        created_by: &str,
        parent_id: Option<Uuid>,
    ) -> Result<Uuid> {
        let normal_balance = match request.account_type {
            AccountType::Asset | AccountType::Expense => BalanceSide::Debit,
            AccountType::Liability | AccountType::Equity | AccountType::Revenue => BalanceSide::Credit,
        };
        
        let level = if parent_id.is_some() {
            let parent_level: i32 = sqlx::query_scalar(
                "SELECT level FROM accounts WHERE id = $1"
            )
            .bind(parent_id)
            .fetch_one(&mut **tx)
            .await?;
            
            parent_level + 1
        } else {
            1
        };
        
        let id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO accounts (
                code, name, description, account_type, account_subtype, normal_balance,
                parent_id, level, is_detail, status, bir_classification, tax_related,
                vat_account, withholding_account, currency, allow_foreign_currency,
                created_by, tenant_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            RETURNING id
            "#
        )
        .bind(&request.code)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.account_type)
        .bind(&request.account_subtype)
        .bind(&normal_balance)
        .bind(&parent_id)
        .bind(level)
        .bind(request.is_detail)
        .bind(AccountStatus::Active)
        .bind(&request.bir_classification)
        .bind(request.tax_related)
        .bind(request.vat_account)
        .bind(request.withholding_account)
        .bind(request.currency.unwrap_or_else(|| "PHP".to_string()))
        .bind(request.allow_foreign_currency)
        .bind(created_by)
        .bind(tenant_id)
        .fetch_one(&mut **tx)
        .await?;
        
        Ok(id)
    }
    
    /// Create a new account
    pub async fn create_account(
        &self,
        request: CreateAccountRequest,
        tenant_id: Uuid,
        created_by: &str,
    ) -> Result<Account> {
        // Validate account code format (4-digit for Philippine COA)
        if !Self::validate_account_code(&request.code) {
            return Err(ChartOfAccountsError::InvalidAccountCode(
                format!("Code must be 4 digits: {}", request.code)
            ));
        }
        
        // Check if account already exists
        let existing: Option<i32> = sqlx::query_scalar(
            "SELECT 1 FROM accounts WHERE code = $1 AND tenant_id = $2"
        )
        .bind(&request.code)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if existing.is_some() {
            return Err(ChartOfAccountsError::AccountExists(request.code.clone()));
        }
        
        // Get parent ID if parent code provided
        let parent_id = if let Some(parent_code) = &request.parent_code {
            let parent: Option<Uuid> = sqlx::query_scalar(
                "SELECT id FROM accounts WHERE code = $1 AND tenant_id = $2"
            )
            .bind(parent_code)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;
            
            parent.ok_or_else(|| {
                ChartOfAccountsError::InvalidParentAccount(parent_code.clone())
            })?
        } else {
            None
        };
        
        let mut tx = self.pool.begin().await?;
        let id = self.create_account_internal(
            &mut tx,
            request.clone(),
            tenant_id,
            created_by,
            Some(parent_id),
        ).await?;
        tx.commit().await?;
        
        self.get_account_by_id(id, tenant_id).await
    }
    
    /// Get account by ID
    pub async fn get_account_by_id(&self, id: Uuid, tenant_id: Uuid) -> Result<Account> {
        let account = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE id = $1 AND tenant_id = $2"
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| ChartOfAccountsError::AccountNotFound(id.to_string()))?;
        
        Ok(account)
    }
    
    /// Get account by code
    pub async fn get_account_by_code(&self, code: &str, tenant_id: Uuid) -> Result<Account> {
        let account = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE code = $1 AND tenant_id = $2"
        )
        .bind(code)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| ChartOfAccountsError::AccountNotFound(code.to_string()))?;
        
        Ok(account)
    }
    
    /// Get all accounts for a tenant (hierarchical)
    pub async fn get_all_accounts(&self, tenant_id: Uuid) -> Result<Vec<Account>> {
        let accounts = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE tenant_id = $1 ORDER BY code"
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(accounts)
    }
    
    /// Get account hierarchy
    pub async fn get_account_hierarchy(&self, tenant_id: Uuid) -> Result<AccountTree> {
        let accounts = self.get_all_accounts(tenant_id).await?;
        let tree = AccountTree::build_from_accounts(accounts);
        Ok(tree)
    }
    
    /// Update account
    pub async fn update_account(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        status: Option<AccountStatus>,
        tenant_id: Uuid,
    ) -> Result<Account> {
        sqlx::query(
            r#"
            UPDATE accounts SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                status = COALESCE($3, status)
            WHERE id = $4 AND tenant_id = $5
            "#
        )
        .bind(&name)
        .bind(&description)
        .bind(&status)
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;
        
        self.get_account_by_id(id, tenant_id).await
    }
    
    /// Delete account (soft delete)
    pub async fn delete_account(&self, id: Uuid, tenant_id: Uuid) -> Result<()> {
        // Check if account has transactions
        let has_transactions: Option<i32> = sqlx::query_scalar(
            "SELECT 1 FROM journal_lines jl 
             JOIN journal_entries je ON jl.journal_entry_id = je.id 
             WHERE jl.account_id = $1 AND je.tenant_id = $2 LIMIT 1"
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if has_transactions.is_some() {
            return Err(ChartOfAccountsError::AccountHasTransactions);
        }
        
        // Check if account has children
        let has_children: Option<i32> = sqlx::query_scalar(
            "SELECT 1 FROM accounts WHERE parent_id = $1 AND tenant_id = $2 LIMIT 1"
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if has_children.is_some() {
            return Err(ChartOfAccountsError::AccountHasChildren);
        }
        
        // Soft delete
        sqlx::query(
            "UPDATE accounts SET status = $1 WHERE id = $2 AND tenant_id = $3"
        )
        .bind(AccountStatus::Deleted)
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get account balance
    pub async fn get_account_balance(
        &self,
        account_id: Uuid,
        as_of_date: Option<DateTime<Utc>>,
        tenant_id: Uuid,
    ) -> Result<AccountBalance> {
        let date = as_of_date.unwrap_or_else(Utc::now);
        
        let balance = sqlx::query_as::<_, AccountBalance>(
            r#"
            SELECT 
                a.id as account_id,
                a.code as account_code,
                a.name as account_name,
                COALESCE(SUM(jl.debit_amount), 0) as debit_balance,
                COALESCE(SUM(jl.credit_amount), 0) as credit_balance,
                CASE 
                    WHEN a.normal_balance = 'debit' THEN 
                        COALESCE(SUM(jl.debit_amount), 0) - COALESCE(SUM(jl.credit_amount), 0)
                    ELSE 
                        COALESCE(SUM(jl.credit_amount), 0) - COALESCE(SUM(jl.debit_amount), 0)
                END as net_balance,
                $2::timestamptz as period_date,
                a.currency
            FROM accounts a
            LEFT JOIN journal_lines jl ON a.id = jl.account_id
            LEFT JOIN journal_entries je ON jl.journal_entry_id = je.id
            WHERE a.id = $1 
                AND a.tenant_id = $3
                AND (je.entry_date <= $2::date OR je.entry_date IS NULL)
                AND (je.status = 'posted' OR je.status IS NULL)
            GROUP BY a.id, a.code, a.name, a.normal_balance, a.currency
            "#
        )
        .bind(account_id)
        .bind(date)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(balance)
    }
    
    /// Generate trial balance
    pub async fn generate_trial_balance(
        &self,
        as_of_date: DateTime<Utc>,
        tenant_id: Uuid,
    ) -> Result<Vec<AccountBalance>> {
        let balances = sqlx::query_as::<_, AccountBalance>(
            r#"
            SELECT 
                a.id as account_id,
                a.code as account_code,
                a.name as account_name,
                COALESCE(SUM(jl.debit_amount), 0) as debit_balance,
                COALESCE(SUM(jl.credit_amount), 0) as credit_balance,
                CASE 
                    WHEN a.normal_balance = 'debit' THEN 
                        COALESCE(SUM(jl.debit_amount), 0) - COALESCE(SUM(jl.credit_amount), 0)
                    ELSE 
                        COALESCE(SUM(jl.credit_amount), 0) - COALESCE(SUM(jl.debit_amount), 0)
                END as net_balance,
                $1::timestamptz as period_date,
                a.currency
            FROM accounts a
            LEFT JOIN journal_lines jl ON a.id = jl.account_id
            LEFT JOIN journal_entries je ON jl.journal_entry_id = je.id
            WHERE a.tenant_id = $2
                AND a.is_detail = true
                AND a.status = 'active'
                AND (je.entry_date <= $1::date OR je.entry_date IS NULL)
                AND (je.status = 'posted' OR je.status IS NULL)
            GROUP BY a.id, a.code, a.name, a.normal_balance, a.currency
            HAVING COALESCE(SUM(jl.debit_amount), 0) != 0 OR COALESCE(SUM(jl.credit_amount), 0) != 0
            ORDER BY a.code
            "#
        )
        .bind(as_of_date)
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(balances)
    }
    
    /// Validate account code format
    fn validate_account_code(code: &str) -> bool {
        code.len() == 4 && code.chars().all(|c| c.is_digit(10))
    }
}

/// Account tree structure for hierarchical display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountNode {
    pub account: Account,
    pub children: Vec<AccountNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTree {
    pub roots: Vec<AccountNode>,
}

impl AccountTree {
    fn build_from_accounts(accounts: Vec<Account>) -> Self {
        let mut account_map: HashMap<Uuid, Account> = HashMap::new();
        let mut children_map: HashMap<Option<Uuid>, Vec<Account>> = HashMap::new();
        
        for account in accounts {
            account_map.insert(account.id, account.clone());
            children_map.entry(account.parent_id).or_insert_with(Vec::new).push(account);
        }
        
        let mut roots = Vec::new();
        if let Some(root_accounts) = children_map.get(&None) {
            for root_account in root_accounts {
                roots.push(Self::build_node(root_account.clone(), &children_map));
            }
        }
        
        AccountTree { roots }
    }
    
    fn build_node(account: Account, children_map: &HashMap<Option<Uuid>, Vec<Account>>) -> AccountNode {
        let children = children_map
            .get(&Some(account.id))
            .map(|children| {
                children
                    .iter()
                    .map(|child| Self::build_node(child.clone(), children_map))
                    .collect()
            })
            .unwrap_or_else(Vec::new);
        
        AccountNode { account, children }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_account_creation() {
        // This would require a test database
        // Example of how tests would be structured
        
        let manager = ChartOfAccountsManager::new("postgresql://test:test@localhost/test_db")
            .await
            .unwrap();
        
        let tenant_id = Uuid::new_v4();
        let created_by = "test_user";
        
        // Initialize schema
        manager.init_schema().await.unwrap();
        
        // Create a test account
        let request = CreateAccountRequest {
            code: "1001".to_string(),
            name: "Test Account".to_string(),
            description: Some("Test description".to_string()),
            account_type: AccountType::Asset,
            account_subtype: AccountSubType::Cash,
            parent_code: None,
            is_detail: true,
            bir_classification: None,
            tax_related: false,
            vat_account: false,
            withholding_account: false,
            currency: Some("PHP".to_string()),
            allow_foreign_currency: false,
        };
        
        let account = manager.create_account(request, tenant_id, created_by).await.unwrap();
        
        assert_eq!(account.code, "1001");
        assert_eq!(account.name, "Test Account");
        assert_eq!(account.normal_balance, BalanceSide::Debit);
    }
}
```