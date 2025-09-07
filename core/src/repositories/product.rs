use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::database::{DbPool, Repository};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: f64,
    pub unit: String,
    pub vat_type: String,
    pub category: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub unit_price: f64,
    pub unit: String,
    pub vat_type: String,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub unit_price: Option<f64>,
    pub unit: Option<String>,
    pub vat_type: Option<String>,
    pub category: Option<String>,
}

pub struct ProductRepository {
    pool: DbPool,
}

impl ProductRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    /// Generate UUID v5 for consistent product IDs
    fn generate_id(name: &str) -> String {
        let namespace = Uuid::NAMESPACE_DNS;
        let id = Uuid::new_v5(&namespace, format!("product:{name}").as_bytes());
        id.to_string()
    }
    
}

#[async_trait::async_trait]
impl Repository for ProductRepository {
    type Entity = Product;
    type CreateInput = CreateProduct;
    type UpdateInput = UpdateProduct;
    
    async fn create(&self, input: Self::CreateInput) -> Result<Self::Entity> {
        let id = Self::generate_id(&input.name);
        
        let product = sqlx::query_as::<_, Product>(
            r#"
            INSERT INTO products (id, name, description, unit_price, unit, vat_type, category)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(input.unit_price)
        .bind(&input.unit)
        .bind(&input.vat_type)
        .bind(&input.category)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(product)
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Self::Entity>> {
        let product = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(product)
    }
    
    async fn list(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Self::Entity>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        
        let products = sqlx::query_as::<_, Product>(
            "SELECT * FROM products ORDER BY name LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(products)
    }
    
    async fn update(&self, id: &str, input: Self::UpdateInput) -> Result<Self::Entity> {
        // Build dynamic update query based on provided fields
        let mut query = String::from("UPDATE products SET ");
        let mut updates = Vec::new();
        
        if input.name.is_some() {
            updates.push("name = ?");
        }
        if input.description.is_some() {
            updates.push("description = ?");
        }
        if input.unit_price.is_some() {
            updates.push("unit_price = ?");
        }
        if input.unit.is_some() {
            updates.push("unit = ?");
        }
        if input.vat_type.is_some() {
            updates.push("vat_type = ?");
        }
        if input.category.is_some() {
            updates.push("category = ?");
        }
        
        if updates.is_empty() {
            // No updates, just return existing
            return self.get_by_id(id).await?.ok_or_else(|| anyhow::anyhow!("Product not found"));
        }
        
        query.push_str(&updates.join(", "));
        query.push_str(" WHERE id = ? RETURNING *");
        
        let mut q = sqlx::query_as::<_, Product>(&query);
        
        // Bind values in the same order
        if let Some(name) = &input.name {
            q = q.bind(name);
        }
        if let Some(description) = &input.description {
            q = q.bind(description);
        }
        if let Some(unit_price) = input.unit_price {
            q = q.bind(unit_price);
        }
        if let Some(unit) = &input.unit {
            q = q.bind(unit);
        }
        if let Some(vat_type) = &input.vat_type {
            q = q.bind(vat_type);
        }
        if let Some(category) = &input.category {
            q = q.bind(category);
        }
        
        // Bind the ID last
        q = q.bind(id);
        
        let product = q.fetch_one(&self.pool).await?;
        
        Ok(product)
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    async fn search(&self, query: &str) -> Result<Vec<Self::Entity>> {
        let search_pattern = format!("%{query}%");
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT * FROM products 
            WHERE LOWER(name) LIKE LOWER(?) 
                OR LOWER(description) LIKE LOWER(?) 
                OR LOWER(category) LIKE LOWER(?)
            ORDER BY name
            LIMIT 50
            "#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(products)
    }
}