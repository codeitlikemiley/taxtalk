use serde::{Deserialize, Serialize};
use serde_json::json;

// Generate bindings from WIT
wit_bindgen::generate!({
    world: "invoice-plugin",
    path: "./wit",
});

// Use the generated types
use crate::taxtalk::plugin::types::{
    CommandDef, CommandResult, PluginManifest, Token,
};
use crate::exports::taxtalk::plugin::plugin::Guest;

// Invoice-specific types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub client_id: String,
    pub client_name: String,
    pub amount: f64,
    pub vat_amount: f64,
    pub total_amount: f64,
    pub status: String,
    pub due_date: Option<String>,
    pub items: Vec<InvoiceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub amount: f64,
}

// Plugin implementation
struct InvoicePlugin;

impl Guest for InvoicePlugin {
    fn get_manifest() -> PluginManifest {
        PluginManifest {
            id: "invoice".to_string(),
            name: "Invoice Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Manages invoices and billing with Philippine VAT compliance".to_string(),
            commands: vec![
                CommandDef {
                    name: "create-invoice".to_string(),
                    aliases: vec!["new-invoice".to_string(), "invoice".to_string()],
                    description: "Create a new invoice".to_string(),
                    examples: vec![
                        "create invoice for Juan dela Cruz 10000".to_string(),
                        "invoice ABC Corp 5000 plus VAT".to_string(),
                    ],
                },
                CommandDef {
                    name: "list-invoices".to_string(),
                    aliases: vec!["show-invoices".to_string(), "invoices".to_string()],
                    description: "List all invoices".to_string(),
                    examples: vec!["list invoices".to_string()],
                },
                CommandDef {
                    name: "calculate-vat".to_string(),
                    aliases: vec!["compute-vat".to_string(), "vat".to_string()],
                    description: "Calculate VAT for an amount".to_string(),
                    examples: vec![
                        "calculate vat for 1000".to_string(),
                        "vat 5000 inclusive".to_string(),
                    ],
                },
                CommandDef {
                    name: "invoice-status".to_string(),
                    aliases: vec!["check-invoice".to_string()],
                    description: "Check invoice status".to_string(),
                    examples: vec!["invoice status INV-001".to_string()],
                },
            ],
            schema: json!({
                "type": "object",
                "properties": {
                    "invoice": {
                        "type": "object",
                        "properties": {
                            "client": {"type": "string"},
                            "amount": {"type": "number"},
                            "vat_type": {"enum": ["inclusive", "exclusive", "exempt"]},
                            "items": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "description": {"type": "string"},
                                        "quantity": {"type": "number"},
                                        "unit_price": {"type": "number"}
                                    }
                                }
                            }
                        }
                    }
                }
            }).to_string(),
        }
    }

    fn execute(command: String, tokens: Vec<Token>) -> CommandResult {
        let lower_command = command.to_lowercase();
        
        // Parse command type
        if lower_command.contains("create") || lower_command.contains("new") {
            create_invoice(&command, &tokens)
        } else if lower_command.contains("list") || lower_command.contains("show") {
            list_invoices()
        } else if lower_command.contains("vat") || lower_command.contains("tax") {
            calculate_vat(&command, &tokens)
        } else if lower_command.contains("status") || lower_command.contains("check") {
            check_invoice_status(&command, &tokens)
        } else {
            CommandResult {
                success: false,
                data: None,
                error: Some("Unknown invoice command".to_string()),
            }
        }
    }

    fn get_schema() -> String {
        json!({
            "commands": {
                "create_invoice": {
                    "required": ["client", "amount"],
                    "optional": ["vat_type", "due_date", "items"]
                },
                "list_invoices": {
                    "optional": ["status", "client", "date_range"]
                },
                "calculate_vat": {
                    "required": ["amount"],
                    "optional": ["type"]
                }
            }
        }).to_string()
    }

    fn get_state() -> Option<String> {
        // Could return stored invoices from the store capability
        None
    }

    fn set_state(_state: String) -> Result<(), String> {
        // Could restore invoice state
        Ok(())
    }
}

// Command implementations
fn create_invoice(command: &str, tokens: &[Token]) -> CommandResult {
    // Extract amount from tokens
    let mut amount = 0.0;
    let mut client = String::new();
    let mut vat_type = "exclusive"; // Default to VAT exclusive
    
    for token in tokens {
        match token.token_type.as_str() {
            "amount" | "money" => {
                if let Ok(val) = token.value.parse::<f64>() {
                    amount = val;
                }
            }
            "entity" | "client" => {
                client = token.value.clone();
            }
            "vat_type" => {
                vat_type = &token.value;
            }
            _ => {}
        }
    }
    
    // Parse amount from command if not in tokens
    if amount == 0.0 {
        // Look for numbers in the command
        for word in command.split_whitespace() {
            if let Ok(val) = word.replace(",", "").parse::<f64>() {
                amount = val;
                break;
            }
        }
    }
    
    // Extract client name if not in tokens
    if client.is_empty() {
        // Simple extraction: look for "for <client>"
        if let Some(pos) = command.find(" for ") {
            let after_for = &command[pos + 5..];
            client = after_for.split_whitespace()
                .take_while(|w| !w.parse::<f64>().is_ok())
                .collect::<Vec<_>>()
                .join(" ");
        }
    }
    
    if client.is_empty() || amount == 0.0 {
        return CommandResult {
            success: false,
            data: None,
            error: Some("Missing required fields: client and amount".to_string()),
        };
    }
    
    // Calculate VAT (12% Philippine rate)
    let (vat_amount, total_amount) = match vat_type {
        "inclusive" => {
            let vat = amount / 1.12 * 0.12;
            (vat, amount)
        }
        "exclusive" => {
            let vat = amount * 0.12;
            (vat, amount + vat)
        }
        "exempt" => (0.0, amount),
        _ => {
            let vat = amount * 0.12;
            (vat, amount + vat)
        }
    };
    
    // Generate invoice number
    let invoice_number = format!("INV-{:04}", rand_number());
    
    let invoice = Invoice {
        id: format!("inv_{}", rand_number()),
        invoice_number: invoice_number.clone(),
        client_id: format!("client_{}", client.to_lowercase().replace(" ", "_")),
        client_name: client.clone(),
        amount,
        vat_amount,
        total_amount,
        status: "draft".to_string(),
        due_date: None,
        items: vec![],
    };
    
    // Store invoice using the store capability
    if let Err(e) = crate::taxtalk::plugin::store::set(
        &format!("invoice:{}", invoice.id),
        &serde_json::to_string(&invoice).unwrap_or_default(),
    ) {
        return CommandResult {
            success: false,
            data: None,
            error: Some(format!("Failed to store invoice: {}", e)),
        };
    }
    
    CommandResult {
        success: true,
        data: Some(json!({
            "invoice_number": invoice_number,
            "client": client,
            "amount": amount,
            "vat_amount": vat_amount,
            "total_amount": total_amount,
            "message": format!(
                "Invoice {} created for {} - Amount: ₱{:.2}, VAT: ₱{:.2}, Total: ₱{:.2}",
                invoice_number, client, amount, vat_amount, total_amount
            )
        }).to_string()),
        error: None,
    }
}

fn list_invoices() -> CommandResult {
    // List all invoices from store
    let invoice_keys = crate::taxtalk::plugin::store::list_keys("invoice:");
    
    let mut invoices = Vec::new();
    for key in invoice_keys {
        if let Some(data) = crate::taxtalk::plugin::store::get(&key) {
            if let Ok(invoice) = serde_json::from_str::<Invoice>(&data) {
                invoices.push(json!({
                    "invoice_number": invoice.invoice_number,
                    "client": invoice.client_name,
                    "total": invoice.total_amount,
                    "status": invoice.status,
                }));
            }
        }
    }
    
    CommandResult {
        success: true,
        data: Some(json!({
            "count": invoices.len(),
            "invoices": invoices,
            "message": format!("Found {} invoices", invoices.len())
        }).to_string()),
        error: None,
    }
}

fn calculate_vat(command: &str, _tokens: &[Token]) -> CommandResult {
    // Extract amount from command
    let mut amount = 0.0;
    let is_inclusive = command.contains("inclusive");
    
    for word in command.split_whitespace() {
        if let Ok(val) = word.replace(",", "").parse::<f64>() {
            amount = val;
            break;
        }
    }
    
    if amount == 0.0 {
        return CommandResult {
            success: false,
            data: None,
            error: Some("Please provide an amount to calculate VAT".to_string()),
        };
    }
    
    let (vat, net, total) = if is_inclusive {
        // Amount includes VAT
        let net = amount / 1.12;
        let vat = amount - net;
        (vat, net, amount)
    } else {
        // Amount excludes VAT
        let vat = amount * 0.12;
        let total = amount + vat;
        (vat, amount, total)
    };
    
    CommandResult {
        success: true,
        data: Some(json!({
            "amount": amount,
            "net_amount": net,
            "vat_amount": vat,
            "total_amount": total,
            "vat_rate": "12%",
            "type": if is_inclusive { "inclusive" } else { "exclusive" },
            "message": format!(
                "VAT Calculation: Net: ₱{:.2}, VAT (12%): ₱{:.2}, Total: ₱{:.2}",
                net, vat, total
            )
        }).to_string()),
        error: None,
    }
}

fn check_invoice_status(command: &str, _tokens: &[Token]) -> CommandResult {
    // Extract invoice number from command
    let invoice_number = command.split_whitespace()
        .find(|w| w.starts_with("INV-"))
        .unwrap_or("");
    
    if invoice_number.is_empty() {
        return CommandResult {
            success: false,
            data: None,
            error: Some("Please provide an invoice number (e.g., INV-0001)".to_string()),
        };
    }
    
    // Look for invoice in store
    let invoice_keys = crate::taxtalk::plugin::store::list_keys("invoice:");
    for key in invoice_keys {
        if let Some(data) = crate::taxtalk::plugin::store::get(&key) {
            if let Ok(invoice) = serde_json::from_str::<Invoice>(&data) {
                if invoice.invoice_number == invoice_number {
                    return CommandResult {
                        success: true,
                        data: Some(json!({
                            "invoice_number": invoice.invoice_number,
                            "client": invoice.client_name,
                            "status": invoice.status,
                            "total": invoice.total_amount,
                            "message": format!(
                                "Invoice {} for {} - Status: {}, Total: ₱{:.2}",
                                invoice.invoice_number, invoice.client_name, 
                                invoice.status, invoice.total_amount
                            )
                        }).to_string()),
                        error: None,
                    };
                }
            }
        }
    }
    
    CommandResult {
        success: false,
        data: None,
        error: Some(format!("Invoice {} not found", invoice_number)),
    }
}

// Simple random number generator for demo
fn rand_number() -> u32 {
    // In a real implementation, use proper random generation
    // For WASM without WASI, use a simple counter-based approach
    static mut COUNTER: u32 = 1000;
    unsafe {
        COUNTER += 1;
        COUNTER % 10000
    }
}

// Export the plugin implementation
export!(InvoicePlugin);