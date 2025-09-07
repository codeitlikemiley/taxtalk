use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use leptos::prelude::*;
use serde_json::Value;
use crate::types::{Entity, SearchResponse, EntityType};

/// Cache entry for API responses
#[derive(Clone, Debug)]
struct CacheEntry {
    data: Vec<Entity>,
    timestamp: u64,
}

/// API cache for entity data
#[derive(Clone, Debug)]
pub struct ApiCache {
    entries: StoredValue<HashMap<String, CacheEntry>>,
    expiry_secs: u64,
}

impl ApiCache {
    pub fn new(expiry_secs: u64) -> Self {
        Self {
            entries: StoredValue::new(HashMap::new()),
            expiry_secs,
        }
    }
    
    fn cache_key(entity_type: &str, query: Option<&str>) -> String {
        match query {
            Some(q) => format!("{}:{}", entity_type, q),
            None => entity_type.to_string(),
        }
    }
    
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }
    
    pub fn get(&self, entity_type: &str, query: Option<&str>) -> Option<Vec<Entity>> {
        let key = Self::cache_key(entity_type, query);
        let now = Self::current_timestamp();
        
        self.entries.with_value(|entries| {
            entries.get(&key).and_then(|entry| {
                if now - entry.timestamp < self.expiry_secs {
                    Some(entry.data.clone())
                } else {
                    None
                }
            })
        })
    }
    
    pub fn set(&self, entity_type: &str, query: Option<&str>, data: Vec<Entity>) {
        let key = Self::cache_key(entity_type, query);
        let entry = CacheEntry {
            data,
            timestamp: Self::current_timestamp(),
        };
        
        self.entries.update_value(|entries| {
            entries.insert(key, entry);
        });
    }
    
    pub fn clear(&self) {
        self.entries.update_value(|entries| {
            entries.clear();
        });
    }
}

/// Mock API for demo purposes
pub mod mock {
    use super::*;
    
    pub async fn load_entities(entity_type: String) -> Result<Vec<Entity>, String> {
        // Simulate network delay
        gloo_timers::future::TimeoutFuture::new(300).await;
        
        let entities = match entity_type.as_str() {
            "client" => vec![
                Entity {
                    id: "1".to_string(),
                    name: "ABC Corporation".to_string(),
                    entity_type: "client".to_string(),
                    description: Some("Premium corporate client".to_string()),
                    metadata: None,
                },
                Entity {
                    id: "2".to_string(),
                    name: "XYZ Industries".to_string(),
                    entity_type: "client".to_string(),
                    description: Some("Manufacturing client".to_string()),
                    metadata: None,
                },
                Entity {
                    id: "3".to_string(),
                    name: "Tech Solutions Inc".to_string(),
                    entity_type: "client".to_string(),
                    description: Some("Technology services client".to_string()),
                    metadata: None,
                },
            ],
            "supplier" => vec![
                Entity {
                    id: "4".to_string(),
                    name: "Global Supplies Co".to_string(),
                    entity_type: "supplier".to_string(),
                    description: Some("Office supplies vendor".to_string()),
                    metadata: None,
                },
                Entity {
                    id: "5".to_string(),
                    name: "Tech Hardware Ltd".to_string(),
                    entity_type: "supplier".to_string(),
                    description: Some("Computer equipment supplier".to_string()),
                    metadata: None,
                },
            ],
            "product" => vec![
                Entity {
                    id: "6".to_string(),
                    name: "Rice 5kg".to_string(),
                    entity_type: "product".to_string(),
                    description: Some("Premium jasmine rice".to_string()),
                    metadata: Some(serde_json::json!({
                        "price": 250.00,
                        "unit": "bag"
                    })),
                },
                Entity {
                    id: "7".to_string(),
                    name: "Sugar 1kg".to_string(),
                    entity_type: "product".to_string(),
                    description: Some("White refined sugar".to_string()),
                    metadata: Some(serde_json::json!({
                        "price": 65.00,
                        "unit": "pack"
                    })),
                },
                Entity {
                    id: "8".to_string(),
                    name: "Cooking Oil 1L".to_string(),
                    entity_type: "product".to_string(),
                    description: Some("Vegetable cooking oil".to_string()),
                    metadata: Some(serde_json::json!({
                        "price": 120.00,
                        "unit": "bottle"
                    })),
                },
            ],
            _ => vec![],
        };
        
        Ok(entities)
    }
    
    pub async fn search_entities(query: String, entity_type: Option<String>) -> Result<Vec<Entity>, String> {
        // Simulate network delay
        gloo_timers::future::TimeoutFuture::new(200).await;
        
        // Get all entities first
        let mut all_entities = Vec::new();
        
        if let Some(etype) = entity_type {
            all_entities = load_entities(etype).await?;
        } else {
            // Load from all types
            all_entities.extend(load_entities("client".to_string()).await?);
            all_entities.extend(load_entities("supplier".to_string()).await?);
            all_entities.extend(load_entities("product".to_string()).await?);
        }
        
        // Filter by query
        let query_lower = query.to_lowercase();
        let filtered = all_entities
            .into_iter()
            .filter(|e| {
                e.name.to_lowercase().contains(&query_lower) ||
                e.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
            })
            .collect();
        
        Ok(filtered)
    }
}

/// Real API functions (can be configured with actual endpoints)
pub async fn load_entities(base_url: &str, entity_type: String) -> Result<Vec<Entity>, String> {
    // For demo, use mock API
    if base_url == "mock" || base_url.is_empty() {
        return mock::load_entities(entity_type).await;
    }
    
    let client = reqwest::Client::new();
    let url = format!("{}/api/entities/{}", base_url, entity_type);
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status().is_success() {
        let entities: Vec<Entity> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(entities)
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

pub async fn search_entities(base_url: &str, query: String, entity_type: Option<String>) -> Result<Vec<Entity>, String> {
    // For demo, use mock API
    if base_url == "mock" || base_url.is_empty() {
        return mock::search_entities(query, entity_type).await;
    }
    
    let client = reqwest::Client::new();
    
    let mut url = format!("{}/api/search?q={}", base_url, query);
    if let Some(etype) = entity_type {
        url.push_str(&format!("&type={}", etype));
    }
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status().is_success() {
        let data: SearchResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(data.results)
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}