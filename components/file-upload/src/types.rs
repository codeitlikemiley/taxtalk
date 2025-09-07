use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileUploadMode {
    Single,
    Multiple,
    Image,
    Document,
    Receipt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedFile {
    pub id: String,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub preview_url: Option<String>,
    pub upload_progress: f32,
}