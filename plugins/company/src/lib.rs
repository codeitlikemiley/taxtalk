use serde::{Deserialize, Serialize};
use serde_json::json;

// Generate bindings from WIT
wit_bindgen::generate!({
    world: "company-plugin",
    path: "./wit",
});

// Use the generated types
use crate::taxtalk::plugin::types::{
    CommandDef, CommandResult, PluginManifest, Token,
};
use crate::exports::taxtalk::plugin::plugin::Guest;

// Company-specific types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub tax_id: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub registration_date: Option<String>,
    pub industry: Option<String>,
}

// Plugin implementation
struct CompanyPlugin;

impl Guest for CompanyPlugin {
    fn get_manifest() -> PluginManifest {
        PluginManifest {
            id: "company".to_string(),
            name: "Company Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Manages company profiles and business information".to_string(),
            commands: vec![
                CommandDef {
                    name: "create-company".to_string(),
                    aliases: vec!["new-company".to_string(), "add-company".to_string()],
                    description: "Create a new company profile".to_string(),
                    examples: vec![
                        "create company ABC Corporation".to_string(),
                        "new company XYZ Trading with TIN 123456789".to_string(),
                    ],
                },
                CommandDef {
                    name: "list-companies".to_string(),
                    aliases: vec!["show-companies".to_string(), "companies".to_string()],
                    description: "List all company profiles".to_string(),
                    examples: vec!["list companies".to_string()],
                },
                CommandDef {
                    name: "update-company".to_string(),
                    aliases: vec!["edit-company".to_string()],
                    description: "Update company information".to_string(),
                    examples: vec![
                        "update company ABC Corp address 123 Main St".to_string(),
                        "edit company XYZ phone 555-1234".to_string(),
                    ],
                },
                CommandDef {
                    name: "company-info".to_string(),
                    aliases: vec!["show-company".to_string(), "get-company".to_string()],
                    description: "Get detailed company information".to_string(),
                    examples: vec!["company info ABC Corp".to_string()],
                },
            ],
            schema: json!({
                "type": "object",
                "properties": {
                    "company": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "tax_id": {"type": "string", "pattern": "^[0-9]{9}$|^[0-9]{12}$"},
                            "address": {"type": "string"},
                            "phone": {"type": "string"},
                            "email": {"type": "string", "format": "email"},
                            "industry": {"type": "string"}
                        },
                        "required": ["name"]
                    }
                }
            }).to_string(),
        }
    }

    fn execute(command: String, tokens: Vec<Token>) -> CommandResult {
        let lower_command = command.to_lowercase();
        
        if lower_command.contains("create") || lower_command.contains("new") || lower_command.contains("add") {
            create_company(&command, &tokens)
        } else if lower_command.contains("list") || lower_command.contains("show") && lower_command.contains("companies") {
            list_companies()
        } else if lower_command.contains("update") || lower_command.contains("edit") {
            update_company(&command, &tokens)
        } else if lower_command.contains("info") || (lower_command.contains("show") || lower_command.contains("get")) && !lower_command.contains("companies") {
            get_company_info(&command, &tokens)
        } else {
            CommandResult {
                success: false,
                data: None,
                error: Some("Unknown company command".to_string()),
            }
        }
    }

    fn get_schema() -> String {
        json!({
            "commands": {
                "create_company": {
                    "required": ["name"],
                    "optional": ["tax_id", "address", "phone", "email", "industry"]
                },
                "list_companies": {
                    "optional": ["industry", "search"]
                },
                "update_company": {
                    "required": ["name", "field", "value"],
                    "fields": ["tax_id", "address", "phone", "email", "industry"]
                },
                "company_info": {
                    "required": ["name"]
                }
            },
            "validations": {
                "tax_id": {
                    "pattern": "^[0-9]{9}$|^[0-9]{12}$",
                    "description": "Philippine TIN format (9 or 12 digits)"
                },
                "email": {
                    "format": "email"
                },
                "phone": {
                    "pattern": "^[0-9+\\-\\s()]+$"
                }
            }
        }).to_string()
    }

    fn get_state() -> Option<String> {
        // Could return stored companies from the store capability
        None
    }

    fn set_state(_state: String) -> Result<(), String> {
        // Could restore company state
        Ok(())
    }
}

// Command implementations
fn create_company(command: &str, tokens: &[Token]) -> CommandResult {
    let mut company_name = String::new();
    let mut tax_id = None;
    let mut address = None;
    let mut phone = None;
    let mut email = None;
    let mut industry = None;
    
    // Extract from tokens
    for token in tokens {
        match token.token_type.as_str() {
            "entity" | "company" => {
                if company_name.is_empty() {
                    company_name = token.value.clone();
                }
            }
            "tax_id" | "tin" => {
                tax_id = Some(token.value.clone());
            }
            "address" | "location" => {
                address = Some(token.value.clone());
            }
            "phone" | "contact" => {
                phone = Some(token.value.clone());
            }
            "email" => {
                email = Some(token.value.clone());
            }
            "industry" | "business_type" => {
                industry = Some(token.value.clone());
            }
            _ => {}
        }
    }
    
    // Parse from command if not in tokens
    if company_name.is_empty() {
        // Look for company name after "company" keyword
        if let Some(pos) = command.find(" company ") {
            let after_company = &command[pos + 9..];
            company_name = after_company
                .split_whitespace()
                .take_while(|w| !w.to_lowercase().contains("with") && !w.to_lowercase().contains("tin"))
                .collect::<Vec<_>>()
                .join(" ");
        }
    }
    
    // Extract TIN if mentioned
    if tax_id.is_none() {
        if let Some(pos) = command.to_lowercase().find("tin ") {
            let after_tin = &command[pos + 4..];
            if let Some(tin) = after_tin.split_whitespace().next() {
                tax_id = Some(tin.to_string());
            }
        }
    }
    
    if company_name.is_empty() {
        return CommandResult {
            success: false,
            data: None,
            error: Some("Company name is required".to_string()),
        };
    }
    
    // Validate TIN format if provided
    if let Some(ref tin) = tax_id {
        if !tin.chars().all(char::is_numeric) || (tin.len() != 9 && tin.len() != 12) {
            return CommandResult {
                success: false,
                data: None,
                error: Some("Invalid TIN format. Must be 9 or 12 digits.".to_string()),
            };
        }
    }
    
    let company = Company {
        id: format!("company_{}", company_name.to_lowercase().replace(" ", "_")),
        name: company_name.clone(),
        tax_id: tax_id.clone(),
        address,
        phone,
        email,
        registration_date: Some("2024-01-01".to_string()), // Fixed date for WASM without WASI
        industry,
    };
    
    // Store company using the store capability
    if let Err(e) = crate::taxtalk::plugin::store::set(
        &format!("company:{}", company.id),
        &serde_json::to_string(&company).unwrap_or_default(),
    ) {
        return CommandResult {
            success: false,
            data: None,
            error: Some(format!("Failed to store company: {}", e)),
        };
    }
    
    CommandResult {
        success: true,
        data: Some(json!({
            "id": company.id,
            "name": company_name,
            "tax_id": tax_id,
            "message": format!("Company '{}' created successfully", company_name)
        }).to_string()),
        error: None,
    }
}

fn list_companies() -> CommandResult {
    let company_keys = crate::taxtalk::plugin::store::list_keys("company:");
    
    let mut companies = Vec::new();
    for key in company_keys {
        if let Some(data) = crate::taxtalk::plugin::store::get(&key) {
            if let Ok(company) = serde_json::from_str::<Company>(&data) {
                companies.push(json!({
                    "id": company.id,
                    "name": company.name,
                    "tax_id": company.tax_id,
                    "industry": company.industry,
                }));
            }
        }
    }
    
    CommandResult {
        success: true,
        data: Some(json!({
            "count": companies.len(),
            "companies": companies,
            "message": format!("Found {} companies", companies.len())
        }).to_string()),
        error: None,
    }
}

fn update_company(command: &str, _tokens: &[Token]) -> CommandResult {
    // Extract company name and field to update
    let mut company_name = String::new();
    let mut field = String::new();
    let mut value = String::new();
    
    // Simple parsing from command
    let words: Vec<&str> = command.split_whitespace().collect();
    
    // Find company name
    if let Some(pos) = words.iter().position(|&w| w.to_lowercase() == "company") {
        if pos + 1 < words.len() {
            // Get company name (might be multiple words)
            let mut name_parts = vec![];
            for i in (pos + 1)..words.len() {
                if ["address", "phone", "email", "tin", "tax_id", "industry"].contains(&words[i].to_lowercase().as_str()) {
                    field = words[i].to_lowercase();
                    // Rest is the value
                    value = words[(i + 1)..].join(" ");
                    break;
                }
                name_parts.push(words[i]);
            }
            company_name = name_parts.join(" ");
        }
    }
    
    if company_name.is_empty() || field.is_empty() || value.is_empty() {
        return CommandResult {
            success: false,
            data: None,
            error: Some("Usage: update company <name> <field> <value>".to_string()),
        };
    }
    
    // Load company
    let company_id = format!("company_{}", company_name.to_lowercase().replace(" ", "_"));
    let key = format!("company:{}", company_id);
    
    let company_data = match crate::taxtalk::plugin::store::get(&key) {
        Some(data) => data,
        None => {
            return CommandResult {
                success: false,
                data: None,
                error: Some(format!("Company '{}' not found", company_name)),
            };
        }
    };
    
    let mut company: Company = match serde_json::from_str(&company_data) {
        Ok(c) => c,
        Err(_) => {
            return CommandResult {
                success: false,
                data: None,
                error: Some("Failed to parse company data".to_string()),
            };
        }
    };
    
    // Update the field
    match field.as_str() {
        "address" => company.address = Some(value.clone()),
        "phone" => company.phone = Some(value.clone()),
        "email" => company.email = Some(value.clone()),
        "tin" | "tax_id" => {
            if !value.chars().all(char::is_numeric) || (value.len() != 9 && value.len() != 12) {
                return CommandResult {
                    success: false,
                    data: None,
                    error: Some("Invalid TIN format. Must be 9 or 12 digits.".to_string()),
                };
            }
            company.tax_id = Some(value.clone());
        }
        "industry" => company.industry = Some(value.clone()),
        _ => {
            return CommandResult {
                success: false,
                data: None,
                error: Some(format!("Unknown field: {}", field)),
            };
        }
    }
    
    // Save updated company
    if let Err(e) = crate::taxtalk::plugin::store::set(
        &key,
        &serde_json::to_string(&company).unwrap_or_default(),
    ) {
        return CommandResult {
            success: false,
            data: None,
            error: Some(format!("Failed to update company: {}", e)),
        };
    }
    
    CommandResult {
        success: true,
        data: Some(json!({
            "company": company_name,
            "field": field,
            "value": value,
            "message": format!("Updated {} for company '{}'", field, company_name)
        }).to_string()),
        error: None,
    }
}

fn get_company_info(command: &str, _tokens: &[Token]) -> CommandResult {
    // Extract company name from command
    let company_name = command
        .split_whitespace()
        .skip_while(|&w| !["info", "show", "get"].contains(&w.to_lowercase().as_str()))
        .skip(1)  // Skip the action word
        .collect::<Vec<_>>()
        .join(" ");
    
    if company_name.is_empty() {
        return CommandResult {
            success: false,
            data: None,
            error: Some("Please provide a company name".to_string()),
        };
    }
    
    let company_id = format!("company_{}", company_name.to_lowercase().replace(" ", "_"));
    let key = format!("company:{}", company_id);
    
    match crate::taxtalk::plugin::store::get(&key) {
        Some(data) => {
            if let Ok(company) = serde_json::from_str::<Company>(&data) {
                CommandResult {
                    success: true,
                    data: Some(json!({
                        "company": company,
                        "message": format!("Company information for '{}'", company.name)
                    }).to_string()),
                    error: None,
                }
            } else {
                CommandResult {
                    success: false,
                    data: None,
                    error: Some("Failed to parse company data".to_string()),
                }
            }
        }
        None => CommandResult {
            success: false,
            data: None,
            error: Some(format!("Company '{}' not found", company_name)),
        },
    }
}

// Date handling removed for WASM compatibility

// Export the plugin implementation
export!(CompanyPlugin);