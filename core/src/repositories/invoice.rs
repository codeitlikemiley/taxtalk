use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{Utc, NaiveDate};

use crate::database::{DbPool, Repository};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub client_id: String,
    pub amount: f64,
    pub vat_amount: f64,
    pub total_amount: f64,
    pub status: String,
    pub due_date: Option<NaiveDate>,
    pub invoice_date: NaiveDate,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoice {
    pub client_id: String,
    pub amount: f64,
    pub vat_rate: Option<f64>,
    pub due_days: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInvoice {
    pub status: Option<String>,
    pub due_date: Option<NaiveDate>,
}

pub struct InvoiceRepository {
    pool: DbPool,
}

impl InvoiceRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    /// Generate next invoice number
    async fn generate_invoice_number(&self) -> Result<String> {
        let year = Utc::now().format("%Y");
        
        // Get the last invoice number for this year
        let last_invoice: Option<(String,)> = sqlx::query_as(
            "SELECT invoice_number FROM invoices WHERE invoice_number LIKE ? ORDER BY invoice_number DESC LIMIT 1"
        )
        .bind(format!("INV-{year}-"))
        .fetch_optional(&self.pool)
        .await?;
        
        let next_num = if let Some((last_num,)) = last_invoice {
            // Extract the number part and increment
            let parts: Vec<&str> = last_num.split('-').collect();
            if parts.len() >= 3 {
                let num: u32 = parts[2].parse().unwrap_or(0);
                num + 1
            } else {
                1
            }
        } else {
            1
        };
        
        Ok(format!("INV-{year}-{next_num:04}"))
    }
    
    /// Calculate VAT and total
    fn calculate_vat(&self, amount: f64, vat_rate: f64) -> (f64, f64) {
        let vat_amount = amount * vat_rate;
        let total = amount + vat_amount;
        (vat_amount, total)
    }
}

#[async_trait::async_trait]
impl Repository for InvoiceRepository {
    type Entity = Invoice;
    type CreateInput = CreateInvoice;
    type UpdateInput = UpdateInvoice;
    
    async fn create(&self, input: Self::CreateInput) -> Result<Self::Entity> {
        let id = uuid::Uuid::new_v4().to_string();
        let invoice_number = self.generate_invoice_number().await?;
        
        // Calculate VAT (default 12% for Philippines)
        let vat_rate = input.vat_rate.unwrap_or(0.12);
        let (vat_amount, total_amount) = self.calculate_vat(input.amount, vat_rate);
        
        // Calculate due date
        let invoice_date = Utc::now().date_naive();
        let due_date = input.due_days.map(|days| invoice_date + chrono::Duration::days(days as i64));
        
        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            INSERT INTO invoices (
                id, invoice_number, client_id, amount, vat_amount, 
                total_amount, status, due_date, invoice_date
            )
            VALUES (?, ?, ?, ?, ?, ?, 'pending', ?, ?)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(&invoice_number)
        .bind(&input.client_id)
        .bind(input.amount)
        .bind(vat_amount)
        .bind(total_amount)
        .bind(due_date)
        .bind(invoice_date)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(invoice)
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Self::Entity>> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE id = ? OR invoice_number = ?"
        )
        .bind(id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(invoice)
    }
    
    async fn list(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Self::Entity>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices ORDER BY invoice_date DESC, created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(invoices)
    }
    
    async fn update(&self, id: &str, input: Self::UpdateInput) -> Result<Self::Entity> {
        let mut query = String::from("UPDATE invoices SET updated_at = CURRENT_TIMESTAMP");
        let mut bindings: Vec<String> = vec![];
        
        if let Some(status) = &input.status {
            query.push_str(", status = ?");
            bindings.push(status.clone());
        }
        
        query.push_str(" WHERE id = ? OR invoice_number = ? RETURNING *");
        
        // Build and execute query
        let mut q = sqlx::query_as::<_, Invoice>(&query);
        for binding in bindings {
            q = q.bind(binding);
        }
        q = q.bind(id).bind(id);
        
        let invoice = q.fetch_one(&self.pool).await?;
        Ok(invoice)
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        // Soft delete by setting status to 'cancelled'
        sqlx::query(
            "UPDATE invoices SET status = 'cancelled', updated_at = CURRENT_TIMESTAMP WHERE id = ? OR invoice_number = ?"
        )
        .bind(id)
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn search(&self, query: &str) -> Result<Vec<Self::Entity>> {
        let search_pattern = format!("%{query}%");
        
        let invoices = sqlx::query_as::<_, Invoice>(
            r#"
            SELECT i.* FROM invoices i
            LEFT JOIN clients c ON i.client_id = c.id
            WHERE i.invoice_number LIKE ?
               OR c.name LIKE ?
               OR i.status LIKE ?
            ORDER BY i.invoice_date DESC
            LIMIT 50
            "#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(invoices)
    }
}

impl InvoiceRepository {
    /// Get invoices by status
    pub async fn get_by_status(&self, status: &str) -> Result<Vec<Invoice>> {
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE status = ? ORDER BY invoice_date DESC"
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(invoices)
    }
    
    /// Get invoices for a client
    pub async fn get_by_client(&self, client_id: &str) -> Result<Vec<Invoice>> {
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE client_id = ? ORDER BY invoice_date DESC"
        )
        .bind(client_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(invoices)
    }
    
    /// Get overdue invoices
    pub async fn get_overdue(&self) -> Result<Vec<Invoice>> {
        let today = Utc::now().date_naive();
        
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE status = 'pending' AND due_date < ? ORDER BY due_date ASC"
        )
        .bind(today)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(invoices)
    }
}