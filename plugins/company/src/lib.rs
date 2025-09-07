use serde::{Deserialize, Serialize};

// Simple minimal plugin that can compile to WASM

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub tax_id: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// WIT Interface Exports (simplified for WASM compilation)
#[no_mangle]
pub extern "C" fn get_manifest() -> *const u8 {
    let manifest = r#"{
        "id": "company",
        "name": "Company Plugin",
        "version": "0.1.0",
        "description": "Manages company profiles and information",
        "commands": [
            {
                "name": "create-company",
                "aliases": ["new-company"],
                "description": "Create a new company",
                "examples": ["create company ABC Corp"]
            },
            {
                "name": "list-companies",
                "aliases": ["show-companies"],
                "description": "List all companies",
                "examples": ["list companies"]
            }
        ],
        "schema": "{}"
    }"#;
    
    manifest.as_ptr()
}

#[no_mangle]
pub extern "C" fn execute(command_ptr: *const u8, command_len: usize) -> *const u8 {
    let command_bytes = unsafe { std::slice::from_raw_parts(command_ptr, command_len) };
    let command = String::from_utf8_lossy(command_bytes);
    
    let response = if command.contains("create") {
        CompanyResponse {
            success: true,
            message: "Company created successfully".to_string(),
            data: Some(serde_json::json!({
                "id": "comp-001",
                "name": "Test Company"
            })),
        }
    } else if command.contains("list") {
        CompanyResponse {
            success: true,
            message: "Companies listed".to_string(),
            data: Some(serde_json::json!({
                "companies": [],
                "total": 0
            })),
        }
    } else {
        CompanyResponse {
            success: false,
            message: "Unknown command".to_string(),
            data: None,
        }
    };
    
    let response_json = serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string());
    response_json.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_schema() -> *const u8 {
    let schema = r#"{
        "type": "object",
        "properties": {
            "company": {
                "type": "object",
                "properties": {
                    "id": {"type": "string"},
                    "name": {"type": "string"},
                    "tax_id": {"type": "string"},
                    "address": {"type": "string"},
                    "phone": {"type": "string"},
                    "email": {"type": "string"}
                }
            }
        }
    }"#;
    
    schema.as_ptr()
}