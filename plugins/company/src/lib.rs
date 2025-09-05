// Company Plugin for Crux + WASM Architecture
// Handles company information management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub tax_id: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompanyEvent {
    CreateCompany {
        name: String,
        tax_id: Option<String>,
        address: Option<String>,
        phone: Option<String>,
        email: Option<String>,
    },
    UpdateCompany {
        company_id: Uuid,
        updates: HashMap<String, serde_json::Value>,
    },
    GetCompany {
        company_id: Uuid,
    },
    ListCompanies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub struct CompanyPlugin {
    companies: HashMap<Uuid, Company>,
}

impl CompanyPlugin {
    pub fn new() -> Self {
        Self {
            companies: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, event: CompanyEvent) -> CompanyResponse {
        match event {
            CompanyEvent::CreateCompany { name, tax_id, address, phone, email } => {
                self.create_company(name, tax_id, address, phone, email)
            }
            CompanyEvent::UpdateCompany { company_id, updates } => {
                self.update_company(company_id, updates)
            }
            CompanyEvent::GetCompany { company_id } => {
                self.get_company(company_id)
            }
            CompanyEvent::ListCompanies => {
                self.list_companies()
            }
        }
    }

    fn create_company(&mut self, name: String, tax_id: Option<String>, address: Option<String>, phone: Option<String>, email: Option<String>) -> CompanyResponse {
        let company = Company {
            id: Uuid::new_v4(),
            name,
            tax_id,
            address,
            phone,
            email,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.companies.insert(company.id, company.clone());

        CompanyResponse {
            success: true,
            message: format!("Company '{}' created successfully", company.name),
            data: Some(serde_json::to_value(&company).unwrap()),
        }
    }

    fn update_company(&mut self, company_id: Uuid, updates: HashMap<String, serde_json::Value>) -> CompanyResponse {
        if let Some(company) = self.companies.get_mut(&company_id) {
            // Apply updates (simplified - in real implementation, validate and apply each update)
            for (field, value) in updates {
                match field.as_str() {
                    "name" => {
                        if let Some(name) = value.as_str() {
                            company.name = name.to_string();
                        }
                    }
                    "tax_id" => {
                        company.tax_id = value.as_str().map(|s| s.to_string());
                    }
                    "address" => {
                        company.address = value.as_str().map(|s| s.to_string());
                    }
                    "phone" => {
                        company.phone = value.as_str().map(|s| s.to_string());
                    }
                    "email" => {
                        company.email = value.as_str().map(|s| s.to_string());
                    }
                    _ => {} // Handle other fields as needed
                }
            }
            company.updated_at = chrono::Utc::now();

            CompanyResponse {
                success: true,
                message: format!("Company {} updated", company_id),
                data: Some(serde_json::to_value(&company).unwrap()),
            }
        } else {
            CompanyResponse {
                success: false,
                message: format!("Company {} not found", company_id),
                data: None,
            }
        }
    }

    fn get_company(&self, company_id: Uuid) -> CompanyResponse {
        if let Some(company) = self.companies.get(&company_id) {
            CompanyResponse {
                success: true,
                message: format!("Company {} retrieved", company_id),
                data: Some(serde_json::to_value(&company).unwrap()),
            }
        } else {
            CompanyResponse {
                success: false,
                message: format!("Company {} not found", company_id),
                data: None,
            }
        }
    }

    fn list_companies(&self) -> CompanyResponse {
        let companies: Vec<&Company> = self.companies.values().collect();
        CompanyResponse {
            success: true,
            message: format!("Found {} companies", companies.len()),
            data: Some(serde_json::json!({
                "companies": companies,
                "total": companies.len()
            })),
        }
    }

    pub fn get_company_by_id(&self, company_id: Uuid) -> Option<&Company> {
        self.companies.get(&company_id)
    }

    pub fn get_all_companies(&self) -> Vec<&Company> {
        self.companies.values().collect()
    }
}

impl Default for CompanyPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// WIT Interface Exports
#[unsafe(no_mangle)]
pub extern "C" fn constructor() -> *mut CompanyPlugin {
    Box::into_raw(Box::new(CompanyPlugin::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn update(plugin: *mut CompanyPlugin, data: *const u8, len: usize) -> *mut u8 {
    let plugin = unsafe { &mut *plugin };
    let data_slice = unsafe { std::slice::from_raw_parts(data, len) };

    if let Ok(event_str) = std::str::from_utf8(data_slice) {
        if let Ok(event) = serde_json::from_str::<CompanyEvent>(event_str) {
            let response = plugin.handle_event(event);
            let response_json = serde_json::to_string(&response).unwrap();
            let response_bytes = response_json.into_bytes();
            let response_ptr = response_bytes.as_ptr() as *mut u8;
            std::mem::forget(response_bytes); // Prevent deallocation
            response_ptr
        } else {
            let error_response = CompanyResponse {
                success: false,
                message: "Invalid event data".to_string(),
                data: None,
            };
            let response_json = serde_json::to_string(&error_response).unwrap();
            let response_bytes = response_json.into_bytes();
            let response_ptr = response_bytes.as_ptr() as *mut u8;
            std::mem::forget(response_bytes);
            response_ptr
        }
    } else {
        let error_response = CompanyResponse {
            success: false,
            message: "Invalid UTF-8 data".to_string(),
            data: None,
        };
        let response_json = serde_json::to_string(&error_response).unwrap();
        let response_bytes = response_json.into_bytes();
        let response_ptr = response_bytes.as_ptr() as *mut u8;
        std::mem::forget(response_bytes);
        response_ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn view(plugin: *mut CompanyPlugin) -> *mut u8 {
    let plugin = unsafe { &*plugin };
    let companies: Vec<&Company> = plugin.get_all_companies();
    let view_data = serde_json::json!({
        "total_companies": companies.len(),
        "companies": companies
    });
    let view_json = serde_json::to_string(&view_data).unwrap();
    let view_bytes = view_json.into_bytes();
    let view_ptr = view_bytes.as_ptr() as *mut u8;
    std::mem::forget(view_bytes);
    view_ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn schema() -> *const std::ffi::c_char {
    let schema = r#"{
        "type": "object",
        "properties": {
            "id": {"type": "string", "format": "uuid"},
            "name": {"type": "string"},
            "tax_id": {"type": "string"},
            "address": {"type": "string"},
            "phone": {"type": "string"},
            "email": {"type": "string", "format": "email"},
            "created_at": {"type": "string", "format": "date-time"},
            "updated_at": {"type": "string", "format": "date-time"}
        },
        "required": ["id", "name", "created_at", "updated_at"]
    }"#;

    let c_str = std::ffi::CString::new(schema).unwrap();
    c_str.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_company() {
        let mut plugin = CompanyPlugin::new();

        let event = CompanyEvent::CreateCompany {
            name: "Test Company".to_string(),
            tax_id: Some("123456789".to_string()),
            address: Some("123 Test St".to_string()),
            phone: Some("555-1234".to_string()),
            email: Some("test@company.com".to_string()),
        };

        let response = plugin.handle_event(event);
        assert!(response.success);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_get_company() {
        let mut plugin = CompanyPlugin::new();

        // Create company first
        let create_event = CompanyEvent::CreateCompany {
            name: "Test Company".to_string(),
            tax_id: None,
            address: None,
            phone: None,
            email: None,
        };

        let create_response = plugin.handle_event(create_event);
        assert!(create_response.success);

        // Extract company ID from response
        if let Some(data) = create_response.data {
            if let Ok(company) = serde_json::from_value::<Company>(data) {
                let get_event = CompanyEvent::GetCompany {
                    company_id: company.id,
                };

                let get_response = plugin.handle_event(get_event);
                assert!(get_response.success);
                assert!(get_response.data.is_some());
            }
        }
    }

    #[test]
    fn test_list_companies() {
        let mut plugin = CompanyPlugin::new();

        // Create a couple companies
        let event1 = CompanyEvent::CreateCompany {
            name: "Company 1".to_string(),
            tax_id: None,
            address: None,
            phone: None,
            email: None,
        };

        let event2 = CompanyEvent::CreateCompany {
            name: "Company 2".to_string(),
            tax_id: None,
            address: None,
            phone: None,
            email: None,
        };

        plugin.handle_event(event1);
        plugin.handle_event(event2);

        let list_response = plugin.handle_event(CompanyEvent::ListCompanies);
        assert!(list_response.success);

        if let Some(data) = list_response.data {
            if let Some(total) = data.get("total") {
                if let Some(total_num) = total.as_u64() {
                    assert_eq!(total_num, 2);
                }
            }
        }
    }
}