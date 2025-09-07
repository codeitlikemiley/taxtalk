use serde::{Deserialize, Serialize};

// Simple minimal plugin that can compile to WASM

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub invoice_id: String,
    pub amount: f64,
    pub method: String,
    pub status: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// WIT Interface Exports (simplified for WASM compilation)
#[no_mangle]
pub extern "C" fn get_manifest() -> *const u8 {
    let manifest = r#"{
        "id": "payments",
        "name": "Payments Plugin",
        "version": "0.1.0",
        "description": "Manages payments and transactions",
        "commands": [
            {
                "name": "record-payment",
                "aliases": ["payment", "pay"],
                "description": "Record a payment",
                "examples": ["record payment from Juan 1000"]
            },
            {
                "name": "list-payments",
                "aliases": ["show-payments"],
                "description": "List all payments",
                "examples": ["list payments"]
            },
            {
                "name": "payment-status",
                "aliases": ["check-payment"],
                "description": "Check payment status",
                "examples": ["check payment status INV-001"]
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
    let payment_method = extract_payment_method(&command);
    
    let response = if command.contains("record") || command.contains("payment") || command.contains("pay") || command.contains("paid") {
        let payment_num = ((amount as u32) % 999) + 1;
        
        PaymentResponse {
            success: true,
            message: "Payment recorded successfully".to_string(),
            data: Some(serde_json::json!({
                "id": format!("PAY-2024-{:03}", payment_num),
                "invoice_id": format!("INV-2024-{:03}", payment_num),
                "amount": amount,
                "client": client,
                "method": payment_method,
                "status": "completed",
                "date": "2024-01-01"
            })),
        }
    } else if command.contains("list") {
        PaymentResponse {
            success: true,
            message: "Payments listed".to_string(),
            data: Some(serde_json::json!({
                "payments": [
                    {
                        "id": "PAY-2024-001",
                        "invoice_id": "INV-2024-001",
                        "amount": 1000.0,
                        "method": "cash",
                        "status": "completed"
                    }
                ],
                "total": 1,
                "total_amount": 1000.0
            })),
        }
    } else if command.contains("status") {
        PaymentResponse {
            success: true,
            message: "Payment status retrieved".to_string(),
            data: Some(serde_json::json!({
                "invoice_id": "INV-2024-001",
                "total": 1120.0,
                "paid": 1000.0,
                "balance": 120.0,
                "status": "partial"
            })),
        }
    } else {
        PaymentResponse {
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
            "payment": {
                "type": "object",
                "properties": {
                    "id": {"type": "string"},
                    "invoice_id": {"type": "string"},
                    "amount": {"type": "number"},
                    "method": {"type": "string"},
                    "status": {"type": "string"},
                    "date": {"type": "string"}
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
    
    // Look for patterns with @client
    if let Some(idx) = lower.find("@client") {
        let after = &command[idx + 7..].trim();
        if let Some(name) = after.split_whitespace().next() {
            return Some(name.to_string());
        }
    }
    
    // Look for patterns like "from Juan", "by Maria"
    for preposition in &["from", "by", "for", "to"] {
        if let Some(idx) = lower.find(preposition) {
            let after = &command[idx + preposition.len()..].trim();
            let words: Vec<&str> = after.split_whitespace().collect();
            if !words.is_empty() {
                let first_word = words[0];
                if !first_word.chars().all(|c| c.is_numeric() || c == '.' || c == ',') {
                    // Check for surname
                    if words.len() > 1 && !words[1].chars().all(|c| c.is_numeric() || c == '.' || c == ',') 
                        && !["paid", "payment", "pesos", "php", "via", "through"].contains(&words[1].to_lowercase().as_str()) {
                        return Some(format!("{} {}", words[0], words[1]));
                    }
                    return Some(first_word.to_string());
                }
            }
        }
    }
    
    None
}

// Helper function to extract payment method
fn extract_payment_method(command: &str) -> String {
    let lower = command.to_lowercase();
    
    if lower.contains("gcash") || lower.contains("g-cash") {
        "gcash".to_string()
    } else if lower.contains("maya") || lower.contains("paymaya") {
        "maya".to_string()
    } else if lower.contains("bank") || lower.contains("transfer") {
        "bank_transfer".to_string()
    } else if lower.contains("check") || lower.contains("cheque") {
        "check".to_string()
    } else if lower.contains("card") || lower.contains("credit") || lower.contains("debit") {
        "card".to_string()
    } else {
        "cash".to_string()
    }
}