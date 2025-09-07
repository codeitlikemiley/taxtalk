use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::Path;

pub type DbPool = Pool<Sqlite>;

/// Initialize SQLite database with connection pool
pub async fn init_database(db_path: &str) -> Result<DbPool> {
    // Create database directory if it doesn't exist
    if let Some(parent) = Path::new(db_path).parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    
    // Create SQLite connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:{db_path}"))
        .await?;
    
    // Run migrations
    run_migrations(&pool).await?;
    
    Ok(pool)
}

/// Run database migrations
async fn run_migrations(pool: &DbPool) -> Result<()> {
    // Create tables for built-in plugins
    
    // Companies table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS companies (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            tin TEXT,
            address TEXT,
            phone TEXT,
            email TEXT,
            vat_registered BOOLEAN DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Clients/Customers table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS clients (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT,
            phone TEXT,
            address TEXT,
            tin TEXT,
            client_type TEXT DEFAULT 'individual',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Invoices table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS invoices (
            id TEXT PRIMARY KEY,
            invoice_number TEXT NOT NULL UNIQUE,
            client_id TEXT NOT NULL,
            amount REAL NOT NULL,
            vat_amount REAL DEFAULT 0,
            total_amount REAL NOT NULL,
            status TEXT DEFAULT 'pending',
            due_date DATE,
            invoice_date DATE DEFAULT CURRENT_DATE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (client_id) REFERENCES clients(id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Invoice items table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS invoice_items (
            id TEXT PRIMARY KEY,
            invoice_id TEXT NOT NULL,
            description TEXT NOT NULL,
            quantity REAL DEFAULT 1,
            unit_price REAL NOT NULL,
            amount REAL NOT NULL,
            vat_rate REAL DEFAULT 0.12,
            vat_amount REAL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Payments table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS payments (
            id TEXT PRIMARY KEY,
            payment_number TEXT NOT NULL UNIQUE,
            invoice_id TEXT,
            client_id TEXT NOT NULL,
            amount REAL NOT NULL,
            payment_method TEXT DEFAULT 'cash',
            payment_date DATE DEFAULT CURRENT_DATE,
            reference_number TEXT,
            notes TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (invoice_id) REFERENCES invoices(id),
            FOREIGN KEY (client_id) REFERENCES clients(id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Suppliers table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS suppliers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT,
            phone TEXT,
            address TEXT,
            tin TEXT,
            category TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Products/Services table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            unit_price REAL NOT NULL,
            unit TEXT DEFAULT 'piece',
            vat_type TEXT DEFAULT 'vatable',
            category TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Expenses table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS expenses (
            id TEXT PRIMARY KEY,
            expense_number TEXT NOT NULL UNIQUE,
            supplier_id TEXT,
            description TEXT NOT NULL,
            amount REAL NOT NULL,
            vat_amount REAL DEFAULT 0,
            total_amount REAL NOT NULL,
            expense_date DATE DEFAULT CURRENT_DATE,
            category TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (supplier_id) REFERENCES suppliers(id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Create indexes for performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoices_client_id ON invoices(client_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_payments_invoice_id ON payments(invoice_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_payments_client_id ON payments(client_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoice_items_invoice_id ON invoice_items(invoice_id)")
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Database repository trait for plugins to implement
#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    type Entity;
    type CreateInput;
    type UpdateInput;
    
    async fn create(&self, input: Self::CreateInput) -> Result<Self::Entity>;
    async fn get_by_id(&self, id: &str) -> Result<Option<Self::Entity>>;
    async fn list(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Self::Entity>>;
    async fn update(&self, id: &str, input: Self::UpdateInput) -> Result<Self::Entity>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn search(&self, query: &str) -> Result<Vec<Self::Entity>>;
}