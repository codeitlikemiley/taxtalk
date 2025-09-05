// Invoice Plugin for Crux + WASM Architecture
// Handles invoice creation, management, and processing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub client_id: String,
    pub amount: f64,
    pub currency: String,
    pub items: Vec<InvoiceItem>,
    pub status: InvoiceStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceEvent {
    CreateInvoice {
        client_id: String,
        items: Vec<InvoiceItem>,
        currency: String,
    },
    UpdateInvoice {
        invoice_id: Uuid,
        updates: HashMap<String, serde_json::Value>,
    },
    SendInvoice {
        invoice_id: Uuid,
    },
    MarkPaid {
        invoice_id: Uuid,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub struct InvoicePlugin {
    invoices: HashMap<Uuid, Invoice>,
}

impl InvoicePlugin {
    pub fn new() -> Self {
        Self {
            invoices: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, event: InvoiceEvent) -> InvoiceResponse {
        match event {
            InvoiceEvent::CreateInvoice { client_id, items, currency } => {
                self.create_invoice(client_id, items, currency)
            }
            InvoiceEvent::UpdateInvoice { invoice_id, updates } => {
                self.update_invoice(invoice_id, updates)
            }
            InvoiceEvent::SendInvoice { invoice_id } => {
                self.send_invoice(invoice_id)
            }
            InvoiceEvent::MarkPaid { invoice_id } => {
                self.mark_paid(invoice_id)
            }
        }
    }

    fn create_invoice(&mut self, client_id: String, items: Vec<InvoiceItem>, currency: String) -> InvoiceResponse {
        let total_amount = items.iter().map(|item| item.total).sum();
        let invoice = Invoice {
            id: Uuid::new_v4(),
            client_id,
            amount: total_amount,
            currency,
            items,
            status: InvoiceStatus::Draft,
            created_at: chrono::Utc::now(),
            due_date: Some(chrono::Utc::now() + chrono::Duration::days(30)),
        };

        self.invoices.insert(invoice.id, invoice.clone());

        InvoiceResponse {
            success: true,
            message: format!("Invoice {} created successfully", invoice.id),
            data: Some(serde_json::to_value(&invoice).unwrap()),
        }
    }

    fn update_invoice(&mut self, invoice_id: Uuid, updates: HashMap<String, serde_json::Value>) -> InvoiceResponse {
        if let Some(invoice) = self.invoices.get_mut(&invoice_id) {
            // Apply updates (simplified - in real implementation, validate and apply each update)
            for (field, value) in updates {
                match field.as_str() {
                    "status" => {
                        if let Some(status_str) = value.as_str() {
                            match status_str {
                                "sent" => invoice.status = InvoiceStatus::Sent,
                                "paid" => invoice.status = InvoiceStatus::Paid,
                                "cancelled" => invoice.status = InvoiceStatus::Cancelled,
                                _ => {}
                            }
                        }
                    }
                    _ => {} // Handle other fields as needed
                }
            }

            InvoiceResponse {
                success: true,
                message: format!("Invoice {} updated", invoice_id),
                data: Some(serde_json::to_value(&invoice).unwrap()),
            }
        } else {
            InvoiceResponse {
                success: false,
                message: format!("Invoice {} not found", invoice_id),
                data: None,
            }
        }
    }

    fn send_invoice(&mut self, invoice_id: Uuid) -> InvoiceResponse {
        if let Some(invoice) = self.invoices.get_mut(&invoice_id) {
            invoice.status = InvoiceStatus::Sent;
            InvoiceResponse {
                success: true,
                message: format!("Invoice {} sent to client", invoice_id),
                data: Some(serde_json::to_value(&invoice).unwrap()),
            }
        } else {
            InvoiceResponse {
                success: false,
                message: format!("Invoice {} not found", invoice_id),
                data: None,
            }
        }
    }

    fn mark_paid(&mut self, invoice_id: Uuid) -> InvoiceResponse {
        if let Some(invoice) = self.invoices.get_mut(&invoice_id) {
            invoice.status = InvoiceStatus::Paid;
            InvoiceResponse {
                success: true,
                message: format!("Invoice {} marked as paid", invoice_id),
                data: Some(serde_json::to_value(&invoice).unwrap()),
            }
        } else {
            InvoiceResponse {
                success: false,
                message: format!("Invoice {} not found", invoice_id),
                data: None,
            }
        }
    }

    pub fn get_invoice(&self, invoice_id: Uuid) -> Option<&Invoice> {
        self.invoices.get(&invoice_id)
    }

    pub fn list_invoices(&self) -> Vec<&Invoice> {
        self.invoices.values().collect()
    }
}

impl Default for InvoicePlugin {
    fn default() -> Self {
        Self::new()
    }
}

// WIT Interface Exports
#[unsafe(no_mangle)]
pub extern "C" fn constructor() -> *mut InvoicePlugin {
    Box::into_raw(Box::new(InvoicePlugin::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn update(plugin: *mut InvoicePlugin, data: *const u8, len: usize) -> *mut u8 {
    let plugin = unsafe { &mut *plugin };
    let data_slice = unsafe { std::slice::from_raw_parts(data, len) };

    if let Ok(event_str) = std::str::from_utf8(data_slice) {
        if let Ok(event) = serde_json::from_str::<InvoiceEvent>(event_str) {
            let response = plugin.handle_event(event);
            let response_json = serde_json::to_string(&response).unwrap();
            let response_bytes = response_json.into_bytes();
            let response_ptr = response_bytes.as_ptr() as *mut u8;
            std::mem::forget(response_bytes); // Prevent deallocation
            response_ptr
        } else {
            let error_response = InvoiceResponse {
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
        let error_response = InvoiceResponse {
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
pub extern "C" fn view(plugin: *mut InvoicePlugin) -> *mut u8 {
    let plugin = unsafe { &*plugin };
    let invoices: Vec<&Invoice> = plugin.list_invoices();
    let view_data = serde_json::json!({
        "total_invoices": invoices.len(),
        "invoices": invoices
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
            "client_id": {"type": "string"},
            "amount": {"type": "number"},
            "currency": {"type": "string"},
            "status": {"type": "string", "enum": ["draft", "sent", "paid", "overdue", "cancelled"]},
            "created_at": {"type": "string", "format": "date-time"},
            "due_date": {"type": "string", "format": "date-time"}
        },
        "required": ["id", "client_id", "amount", "currency", "status", "created_at"]
    }"#;

    let c_str = std::ffi::CString::new(schema).unwrap();
    c_str.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_invoice() {
        let mut plugin = InvoicePlugin::new();

        let items = vec![InvoiceItem {
            description: "Pineapple".to_string(),
            quantity: 5.0,
            unit_price: 200.0,
            total: 1000.0,
        }];

        let event = InvoiceEvent::CreateInvoice {
            client_id: "client123".to_string(),
            items,
            currency: "PHP".to_string(),
        };

        let response = plugin.handle_event(event);
        assert!(response.success);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_mark_paid() {
        let mut plugin = InvoicePlugin::new();

        // Create invoice first
        let items = vec![InvoiceItem {
            description: "Test Item".to_string(),
            quantity: 1.0,
            unit_price: 100.0,
            total: 100.0,
        }];

        let create_event = InvoiceEvent::CreateInvoice {
            client_id: "client123".to_string(),
            items,
            currency: "USD".to_string(),
        };

        let create_response = plugin.handle_event(create_event);
        assert!(create_response.success);

        // Extract invoice ID from response
        if let Some(data) = create_response.data {
            if let Ok(invoice) = serde_json::from_value::<Invoice>(data) {
                let mark_paid_event = InvoiceEvent::MarkPaid {
                    invoice_id: invoice.id,
                };

                let mark_paid_response = plugin.handle_event(mark_paid_event);
                assert!(mark_paid_response.success);

                // Verify status changed
                if let Some(updated_invoice) = plugin.get_invoice(invoice.id) {
                    match updated_invoice.status {
                        InvoiceStatus::Paid => assert!(true),
                        _ => panic!("Invoice should be marked as paid"),
                    }
                }
            }
        }
    }
}