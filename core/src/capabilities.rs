// Capability wrappers for plugin access
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvRequest {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvResponse {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderRequest {
    pub view: String,
}

pub struct Capabilities {
    pub http: Box<dyn Fn(HttpRequest) -> Result<HttpResponse, String> + Send + Sync>,
    pub kv: Box<dyn Fn(KvRequest) -> Result<KvResponse, String> + Send + Sync>,
    pub render: Box<dyn Fn(RenderRequest) -> Result<(), String> + Send + Sync>,
}

impl Capabilities {
    pub fn new() -> Self {
        Self {
            http: Box::new(|_| Err("HTTP capability not implemented".to_string())),
            kv: Box::new(|_| Err("KV capability not implemented".to_string())),
            render: Box::new(|_| Err("Render capability not implemented".to_string())),
        }
    }
}