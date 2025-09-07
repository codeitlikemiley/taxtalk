use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{Utc, NaiveDate};

use crate::database::{DbPool, Repository};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub id: String,
    pub payment_number: String,
    pub invoice_id: Option<String>,
    pub client_id: String,
    pub amount: f64,
    pub payment_method: String,
    pub payment_date: NaiveDate,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayment {
    pub invoice_id: Option<String>,
    pub client_id: String,
    pub amount: f64,
    pub payment_method: Option<String>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePayment {
    pub reference_number: Option<String>,
    pub notes: Option<String>,
}

pub struct PaymentRepository {
    pool: DbPool,
}

impl PaymentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    /// Generate next payment number
    async fn generate_payment_number(&self) -> Result<String> {
        let year = Utc::now().format("%Y");
        
        // Get the last payment number for this year
        let last_payment: Option<(String,)> = sqlx::query_as(
            "SELECT payment_number FROM payments WHERE payment_number LIKE ? ORDER BY payment_number DESC LIMIT 1"
        )
        .bind(format!("PAY-{year}-"))
        .fetch_optional(&self.pool)
        .await?;
        
        let next_num = if let Some((last_num,)) = last_payment {
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
        
        Ok(format!("PAY-{year}-{next_num:04}"))
    }
    
    /// Update invoice status after payment
    async fn update_invoice_status(&self, invoice_id: &str) -> Result<()> {
        // Get total invoice amount
        let invoice_total: Option<(f64,)> = sqlx::query_as(
            "SELECT total_amount FROM invoices WHERE id = ?"
        )
        .bind(invoice_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some((total,)) = invoice_total {
            // Get total payments for this invoice
            let total_paid: Option<(f64,)> = sqlx::query_as(
                "SELECT SUM(amount) FROM payments WHERE invoice_id = ?"
            )
            .bind(invoice_id)
            .fetch_optional(&self.pool)
            .await?;
            
            let paid = total_paid.map(|(p,)| p).unwrap_or(0.0);
            
            // Update invoice status based on payment
            let status = if paid >= total {
                "paid"
            } else if paid > 0.0 {
                "partial"
            } else {
                "pending"
            };
            
            sqlx::query(
                "UPDATE invoices SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
            )
            .bind(status)
            .bind(invoice_id)
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl Repository for PaymentRepository {
    type Entity = Payment;
    type CreateInput = CreatePayment;
    type UpdateInput = UpdatePayment;
    
    async fn create(&self, input: Self::CreateInput) -> Result<Self::Entity> {
        let id = uuid::Uuid::new_v4().to_string();
        let payment_number = self.generate_payment_number().await?;
        let payment_date = Utc::now().date_naive();
        let payment_method = input.payment_method.unwrap_or_else(|| "cash".to_string());
        
        let payment = sqlx::query_as::<_, Payment>(
            r#"
            INSERT INTO payments (
                id, payment_number, invoice_id, client_id, amount, 
                payment_method, payment_date, reference_number, notes
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(&payment_number)
        .bind(&input.invoice_id)
        .bind(&input.client_id)
        .bind(input.amount)
        .bind(&payment_method)
        .bind(payment_date)
        .bind(&input.reference_number)
        .bind(&input.notes)
        .fetch_one(&self.pool)
        .await?;
        
        // Update invoice status if payment is for an invoice
        if let Some(invoice_id) = &input.invoice_id {
            self.update_invoice_status(invoice_id).await?;
        }
        
        Ok(payment)
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Self::Entity>> {
        let payment = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE id = ? OR payment_number = ?"
        )
        .bind(id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(payment)
    }
    
    async fn list(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Self::Entity>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        
        let payments = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments ORDER BY payment_date DESC, created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(payments)
    }
    
    async fn update(&self, id: &str, input: Self::UpdateInput) -> Result<Self::Entity> {
        let mut query = String::from("UPDATE payments SET updated_at = CURRENT_TIMESTAMP");
        let mut bindings: Vec<String> = vec![];
        
        if let Some(ref_num) = &input.reference_number {
            query.push_str(", reference_number = ?");
            bindings.push(ref_num.clone());
        }
        
        if let Some(notes) = &input.notes {
            query.push_str(", notes = ?");
            bindings.push(notes.clone());
        }
        
        query.push_str(" WHERE id = ? OR payment_number = ? RETURNING *");
        
        // Build and execute query
        let mut q = sqlx::query_as::<_, Payment>(&query);
        for binding in bindings {
            q = q.bind(binding);
        }
        q = q.bind(id).bind(id);
        
        let payment = q.fetch_one(&self.pool).await?;
        Ok(payment)
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        // Get invoice_id before deleting
        let invoice_id: Option<(Option<String>,)> = sqlx::query_as(
            "SELECT invoice_id FROM payments WHERE id = ? OR payment_number = ?"
        )
        .bind(id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        // Delete the payment
        sqlx::query("DELETE FROM payments WHERE id = ? OR payment_number = ?")
            .bind(id)
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        // Update invoice status if payment was for an invoice
        if let Some((Some(inv_id),)) = invoice_id {
            self.update_invoice_status(&inv_id).await?;
        }
        
        Ok(())
    }
    
    async fn search(&self, query: &str) -> Result<Vec<Self::Entity>> {
        let search_pattern = format!("%{query}%");
        
        let payments = sqlx::query_as::<_, Payment>(
            r#"
            SELECT p.* FROM payments p
            LEFT JOIN clients c ON p.client_id = c.id
            WHERE p.payment_number LIKE ?
               OR p.reference_number LIKE ?
               OR c.name LIKE ?
               OR p.payment_method LIKE ?
            ORDER BY p.payment_date DESC
            LIMIT 50
            "#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(payments)
    }
}

impl PaymentRepository {
    /// Get payments for an invoice
    pub async fn get_by_invoice(&self, invoice_id: &str) -> Result<Vec<Payment>> {
        let payments = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE invoice_id = ? ORDER BY payment_date DESC"
        )
        .bind(invoice_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(payments)
    }
    
    /// Get payments for a client
    pub async fn get_by_client(&self, client_id: &str) -> Result<Vec<Payment>> {
        let payments = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE client_id = ? ORDER BY payment_date DESC"
        )
        .bind(client_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(payments)
    }
    
    /// Get payments by method
    pub async fn get_by_method(&self, method: &str) -> Result<Vec<Payment>> {
        let payments = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE payment_method = ? ORDER BY payment_date DESC"
        )
        .bind(method)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(payments)
    }
}