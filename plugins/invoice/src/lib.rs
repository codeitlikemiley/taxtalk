use serde::{Deserialize, Serialize};

// Simple minimal plugin that can compile to WASM

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub client: String,
    pub amount: f64,
    pub status: String,
    pub due_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// WIT Interface Exports (simplified for WASM compilation)
#[no_mangle]
pub extern "C" fn get_manifest() -> *const u8 {
    let manifest = r#"{
        "id": "invoice",
        "name": "Invoice Plugin",
        "version": "0.1.0",
        "description": "Manages invoices and billing",
        "commands": [
            {
                "name": "create-invoice",
                "aliases": ["new-invoice"],
                "description": "Create a new invoice",
                "examples": ["create invoice for Juan 1000"]
            },
            {
                "name": "list-invoices",
                "aliases": ["show-invoices"],
                "description": "List all invoices",
                "examples": ["list invoices"]
            },
            {
                "name": "calculate-vat",
                "aliases": ["compute-vat"],
                "description": "Calculate VAT for amount",
                "examples": ["calculate vat for 1000"]
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
    
    // Extract amount and client from command
    let amount = extract_amount(&command).unwrap_or(1000.0);
    let client = extract_client(&command).unwrap_or_else(|| "Unknown Client".to_string());
    
    // Parse command for different operations
    let response = if is_create_command(&command) {
        // CREATE - create new invoice
        let vat = amount * 0.12; // Philippine VAT is 12%
        let invoice_num = ((amount as u32) % 999) + 1;
        
        InvoiceResponse {
            success: true,
            message: "Invoice created successfully".to_string(),
            data: Some(serde_json::json!({
                "id": format!("INV-2024-{:03}", invoice_num),
                "client": client,
                "amount": amount,
                "vat": vat,
                "total": amount + vat,
                "status": "pending",
                "due_date": "30 days"
            })),
        }
    } else if is_list_command(&command) {
        // LIST/READ - show all invoices
        InvoiceResponse {
            success: true,
            message: "Showing all invoices".to_string(),
            data: Some(serde_json::json!({
                "invoices": [
                    {
                        "id": "INV-2024-001",
                        "client": "Juan Cruz", 
                        "amount": 12000.0,
                        "vat": 1440.0,
                        "total": 13440.0,
                        "status": "paid",
                        "date": "2024-01-15"
                    },
                    {
                        "id": "INV-2024-002",
                        "client": "Maria Santos",
                        "amount": 25000.0,
                        "vat": 3000.0,
                        "total": 28000.0,
                        "status": "pending",
                        "date": "2024-01-20"
                    },
                    {
                        "id": "INV-2024-003",
                        "client": "ABC Corp",
                        "amount": 50000.0,
                        "vat": 6000.0,
                        "total": 56000.0,
                        "status": "overdue",
                        "date": "2024-01-10"
                    }
                ],
                "summary": {
                    "total_invoices": 3,
                    "total_amount": 97440.0,
                    "paid": 13440.0,
                    "pending": 28000.0,
                    "overdue": 56000.0
                }
            })),
        }
    } else if is_update_command(&command) {
        // UPDATE - update invoice status
        let invoice_id = extract_invoice_id(&command).unwrap_or_else(|| "INV-2024-001".to_string());
        let new_status = extract_status(&command);
        
        InvoiceResponse {
            success: true,
            message: format!("Invoice {} updated", invoice_id),
            data: Some(serde_json::json!({
                "id": invoice_id,
                "updated_fields": {
                    "status": new_status
                },
                "message": "Invoice status updated successfully"
            })),
        }
    } else if is_delete_command(&command) {
        // DELETE - cancel/void invoice
        let invoice_id = extract_invoice_id(&command).unwrap_or_else(|| "INV-2024-001".to_string());
        
        InvoiceResponse {
            success: true,
            message: format!("Invoice {} cancelled", invoice_id),
            data: Some(serde_json::json!({
                "id": invoice_id,
                "status": "cancelled",
                "message": "Invoice has been cancelled"
            })),
        }
    } else if is_search_command(&command) {
        // SEARCH - find specific invoices
        let search_term = extract_search_term(&command);
        
        InvoiceResponse {
            success: true,
            message: format!("Search results for: {}", search_term.as_deref().unwrap_or("all")),
            data: Some(serde_json::json!({
                "invoices": [
                    {
                        "id": "INV-2024-002",
                        "client": client.clone(),
                        "amount": amount,
                        "status": "pending"
                    }
                ],
                "search_term": search_term,
                "results_count": 1
            })),
        }
    } else if command.contains("vat") {
        // Philippine VAT is 12%
        let amount = 1000.0; // Parse from command in real implementation
        let vat = amount * 0.12;
        InvoiceResponse {
            success: true,
            message: "VAT calculated".to_string(),
            data: Some(serde_json::json!({
                "amount": amount,
                "vat": vat,
                "total": amount + vat,
                "rate": "12%"
            })),
        }
    } else {
        InvoiceResponse {
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
            "invoice": {
                "type": "object",
                "properties": {
                    "id": {"type": "string"},
                    "client": {"type": "string"},
                    "amount": {"type": "number"},
                    "vat": {"type": "number"},
                    "total": {"type": "number"},
                    "status": {"type": "string"},
                    "due_date": {"type": "string"}
                }
            }
        }
    }"#;
    
    schema.as_ptr()
}

// Helper function to extract amount from command
fn extract_amount(command: &str) -> Option<f64> {
    let words: Vec<&str> = command.split_whitespace().collect();
    
    for word in words {
        let cleaned = word
            .replace(",", "")
            .replace("pesos", "")
            .replace("peso", "")
            .replace("php", "")
            .replace("₱", "");
        
        // Handle "k" suffix for thousands
        if cleaned.ends_with("k") || cleaned.ends_with("K") {
            if let Ok(num) = cleaned[..cleaned.len()-1].parse::<f64>() {
                return Some(num * 1000.0);
            }
        }
        
        // Try to parse as regular number
        if let Ok(num) = cleaned.parse::<f64>() {
            if num >= 1.0 {
                return Some(num);
            }
        }
    }
    
    None
}

// Helper function to extract client name from command
fn extract_client(command: &str) -> Option<String> {
    let lower = command.to_lowercase();
    
    // Look for patterns with @client or @customer
    if let Some(idx) = lower.find("@client") {
        let after = &command[idx + 7..].trim();
        if let Some(name) = after.split_whitespace().next() {
            return Some(name.to_string());
        }
    }
    
    // Look for patterns like "for Juan", "to Maria"
    for preposition in &["for", "to", "from"] {
        if let Some(idx) = lower.find(preposition) {
            let after = &command[idx + preposition.len()..].trim();
            let words: Vec<&str> = after.split_whitespace().collect();
            if !words.is_empty() {
                let first_word = words[0];
                if !first_word.chars().all(|c| c.is_numeric() || c == '.' || c == ',') {
                    // Check for surname
                    if words.len() > 1 && !words[1].chars().all(|c| c.is_numeric() || c == '.' || c == ',') 
                        && !["invoice", "payment", "pesos", "php"].contains(&words[1].to_lowercase().as_str()) {
                        return Some(format!("{} {}", words[0], words[1]));
                    }
                    return Some(first_word.to_string());
                }
            }
        }
    }
    
    None
}

// CRUD command detection helpers
fn is_create_command(command: &str) -> bool {
    let lower = command.to_lowercase();
    lower.contains("create") || lower.contains("new") || lower.contains("add") ||
    lower.contains("generate") || 
    (lower.contains("invoice") && (lower.contains("for") || lower.contains("to")))
}

fn is_list_command(command: &str) -> bool {
    let lower = command.to_lowercase();
    lower.contains("list") || lower.contains("show") || lower.contains("display") ||
    lower.contains("get all") || lower.contains("fetch") || lower.contains("view all")
}

fn is_update_command(command: &str) -> bool {
    let lower = command.to_lowercase();
    lower.contains("update") || lower.contains("edit") || lower.contains("modify") ||
    lower.contains("change") || lower.contains("mark as") || lower.contains("set")
}

fn is_delete_command(command: &str) -> bool {
    let lower = command.to_lowercase();
    lower.contains("delete") || lower.contains("remove") || lower.contains("cancel") ||
    lower.contains("void") || lower.contains("discard")
}

fn is_search_command(command: &str) -> bool {
    let lower = command.to_lowercase();
    lower.contains("search") || lower.contains("find") || lower.contains("lookup") ||
    lower.contains("query") || lower.contains("filter")
}

// Extract invoice ID from command
fn extract_invoice_id(command: &str) -> Option<String> {
    let lower = command.to_lowercase();
    
    // Look for patterns like "INV-2024-001"
    let words: Vec<&str> = command.split_whitespace().collect();
    for word in words {
        if word.starts_with("INV-") || word.starts_with("inv-") {
            return Some(word.to_string());
        }
    }
    
    // Look for invoice number patterns
    if let Some(idx) = lower.find("invoice") {
        let after = &command[idx + 7..].trim();
        if let Some(id) = after.split_whitespace().next() {
            if id.chars().any(|c| c.is_numeric()) {
                return Some(format!("INV-2024-{}", id));
            }
        }
    }
    
    None
}

// Extract status from command
fn extract_status(command: &str) -> String {
    let lower = command.to_lowercase();
    
    if lower.contains("paid") || lower.contains("complete") {
        "paid".to_string()
    } else if lower.contains("pending") || lower.contains("unpaid") {
        "pending".to_string()
    } else if lower.contains("overdue") || lower.contains("late") {
        "overdue".to_string()
    } else if lower.contains("cancelled") || lower.contains("void") {
        "cancelled".to_string()
    } else {
        "pending".to_string()
    }
}

// Extract search term from command
fn extract_search_term(command: &str) -> Option<String> {
    let lower = command.to_lowercase();
    
    // Look for search keywords
    for keyword in &["search", "find", "lookup", "query"] {
        if let Some(idx) = lower.find(keyword) {
            let after = &command[idx + keyword.len()..].trim();
            // Skip common words
            let words: Vec<&str> = after.split_whitespace()
                .filter(|w| !["for", "in", "the", "all", "invoice", "invoices"].contains(&w.to_lowercase().as_str()))
                .collect();
            
            if !words.is_empty() {
                return Some(words.join(" "));
            }
        }
    }
    
    None
}