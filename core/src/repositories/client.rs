use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::database::{DbPool, Repository};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub tin: Option<String>,
    pub client_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClient {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub tin: Option<String>,
    pub client_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub tin: Option<String>,
    pub client_type: Option<String>,
}

pub struct ClientRepository {
    pool: DbPool,
}

impl ClientRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    /// Generate UUID v5 for consistent client IDs
    fn generate_id(name: &str) -> String {
        let namespace = Uuid::NAMESPACE_DNS;
        let id = Uuid::new_v5(&namespace, format!("client:{name}").as_bytes());
        id.to_string()
    }
}

#[async_trait::async_trait]
impl Repository for ClientRepository {
    type Entity = Client;
    type CreateInput = CreateClient;
    type UpdateInput = UpdateClient;
    
    async fn create(&self, input: Self::CreateInput) -> Result<Self::Entity> {
        let id = Self::generate_id(&input.name);
        let client_type = input.client_type.unwrap_or_else(|| "individual".to_string());
        
        let client = sqlx::query_as::<_, Client>(
            r#"
            INSERT INTO clients (id, name, email, phone, address, tin, client_type)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(&input.name)
        .bind(&input.email)
        .bind(&input.phone)
        .bind(&input.address)
        .bind(&input.tin)
        .bind(&client_type)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(client)
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Self::Entity>> {
        let client = sqlx::query_as::<_, Client>(
            "SELECT * FROM clients WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(client)
    }
    
    async fn list(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Self::Entity>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        
        let clients = sqlx::query_as::<_, Client>(
            "SELECT * FROM clients ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(clients)
    }
    
    async fn update(&self, id: &str, input: Self::UpdateInput) -> Result<Self::Entity> {
        // Build dynamic update query
        let mut query = String::from("UPDATE clients SET updated_at = CURRENT_TIMESTAMP");
        let mut bindings = vec![];
        
        if let Some(name) = &input.name {
            query.push_str(", name = ?");
            bindings.push(name.clone());
        }
        if let Some(email) = &input.email {
            query.push_str(", email = ?");
            bindings.push(email.clone());
        }
        if let Some(phone) = &input.phone {
            query.push_str(", phone = ?");
            bindings.push(phone.clone());
        }
        if let Some(address) = &input.address {
            query.push_str(", address = ?");
            bindings.push(address.clone());
        }
        if let Some(tin) = &input.tin {
            query.push_str(", tin = ?");
            bindings.push(tin.clone());
        }
        if let Some(client_type) = &input.client_type {
            query.push_str(", client_type = ?");
            bindings.push(client_type.clone());
        }
        
        query.push_str(" WHERE id = ? RETURNING *");
        
        // Execute dynamic query
        let mut q = sqlx::query_as::<_, Client>(&query);
        for binding in bindings {
            q = q.bind(binding);
        }
        q = q.bind(id);
        
        let client = q.fetch_one(&self.pool).await?;
        Ok(client)
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM clients WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    async fn search(&self, query: &str) -> Result<Vec<Self::Entity>> {
        let search_pattern = format!("%{query}%");
        
        let clients = sqlx::query_as::<_, Client>(
            r#"
            SELECT * FROM clients 
            WHERE name LIKE ? 
               OR email LIKE ? 
               OR phone LIKE ?
               OR tin LIKE ?
            ORDER BY created_at DESC
            LIMIT 50
            "#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(clients)
    }
}