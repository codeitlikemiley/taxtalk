use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use core::tokens::*;
use core::plugin_system::*;

/// Invoice Plugin - Complete example of token-driven plugin architecture
#[derive(Default)]
pub struct InvoicePlugin {
    id: String,
    version: String,
}

#[async_trait]
impl Plugin for InvoicePlugin {
    fn id(&self) -> &str {
        &self.id
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn commands(&self) -> Vec<CommandRegistration> {
        vec![
            CommandRegistration {
                command: "create_invoice".to_string(),
                aliases: vec!["@invoice".to_string(), "@inv".to_string()],
                requirements: CommandRequirements {
                    required: vec![
                        TokenRequirement {
                            token_type: "entity_ref".to_string(),
                            description: "Client reference".to_string(),
                            validator: Some(Arc::new(|token| {
                                if let Ok(entity_ref) = token.as_any().downcast_ref::<EntityRef>() {
                                    entity_ref.entity_type == "client"
                                } else {
                                    false
                                }
                            })),
                            multiple: false,
                        },
                        TokenRequirement {
                            token_type: "amount".to_string(),
                            description: "Invoice amount".to_string(),
                            validator: None,
                            multiple: false,
                        },
                    ],
                    optional: vec![
                        TokenRequirement {
                            token_type: "quantity".to_string(),
                            description: "Item quantities".to_string(),
                            validator: None,
                            multiple: true,
                        },
                    ],
                    defaults: vec![
                        TokenDefault {
                            token_type: "tax".to_string(),
                            default_value: Box::new(PhilippineVatToken {
                                base: BaseTokenFields::new("12% VAT".to_string(), TokenPosition { start: 0, end: 0, line: 0, column: 0 }),
                                rate: Decimal::from_str("0.12").unwrap(),
                                inclusive: false,
                                applies_to: None,
                            }),
                            condition: None,
                        },
                    ],
                    validators: vec![],
                    allow_partial: false,
                },
                priority: 10,
                description: "Create a new invoice".to_string(),
                examples: vec![
                    "@invoice create client:uriah amount:1000".to_string(),
                    "create invoice for uriah with 5kg pineapple at 1000 pesos".to_string(),
                ],
            },
            CommandRegistration {
                command: "list_invoices".to_string(),
                aliases: vec!["@invoices".to_string()],
                requirements: CommandRequirements {
                    required: vec![],
                    optional: vec![
                        TokenRequirement {
                            token_type: "entity_ref".to_string(),
                            description: "Filter by client".to_string(),
                            validator: Some(Arc::new(|token| {
                                if let Ok(entity_ref) = token.as_any().downcast_ref::<EntityRef>() {
                                    entity_ref.entity_type == "client"
                                } else {
                                    false
                                }
                            })),
                            multiple: false,
                        },
                    ],
                    defaults: vec![],
                    validators: vec![],
                    allow_partial: true,
                },
                priority: 5,
                description: "List invoices".to_string(),
                examples: vec![
                    "@invoices".to_string(),
                    "@invoices client:uriah".to_string(),
                ],
            },
        ]
    }

    fn can_handle(&self, command: &CommandToken, tokens: &[Box<dyn Token>]) -> CanHandleResult {
        match command.verb.as_str() {
            "create" | "add" | "new" => {
                // Check if we have required tokens for invoice creation
                let has_client = tokens.iter().any(|t| {
                    if let Ok(entity_ref) = t.as_any().downcast_ref::<EntityRef>() {
                        entity_ref.entity_type == "client"
                    } else {
                        false
                    }
                });

                let has_amount = tokens.iter().any(|t| t.token_type() == "amount");

                if has_client && has_amount {
                    CanHandleResult::Yes { confidence: 0.9 }
                } else if has_client || has_amount {
                    CanHandleResult::Partial {
                        missing: if has_client { vec!["amount".to_string()] } else { vec!["client".to_string()] }
                    }
                } else {
                    CanHandleResult::No { reason: "Missing required tokens for invoice creation".to_string() }
                }
            }
            "list" | "show" | "get" => {
                CanHandleResult::Yes { confidence: 0.8 }
            }
            _ => CanHandleResult::No { reason: format!("Unknown command: {}", command.verb) },
        }
    }

    async fn execute(&self, command: &CommandToken, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        match command.verb.as_str() {
            "create" | "add" | "new" => {
                self.create_invoice(tokens).await
            }
            "list" | "show" | "get" => {
                self.list_invoices(tokens).await
            }
            _ => Err(Error::CommandNotSupported(command.verb.clone())),
        }
    }

    async fn export_state(&self) -> Result<Value, Error> {
        // Export plugin state (invoices, etc.)
        Ok(json!({
            "plugin_id": self.id,
            "version": self.version,
            "exported_at": Utc::now().to_rfc3339(),
            "invoices": [] // Would contain actual invoice data
        }))
    }

    async fn import_state(&mut self, state: Value) -> Result<(), Error> {
        // Import plugin state
        if let Some(version) = state.get("version").and_then(|v| v.as_str()) {
            if version != self.version {
                tracing::warn!("Importing state from different version: {} -> {}", version, self.version);
            }
        }
        Ok(())
    }
}

impl InvoicePlugin {
    pub fn new() -> Self {
        Self {
            id: "invoice-plugin".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    async fn create_invoice(&self, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        // Extract client reference
        let client_ref = tokens.iter()
            .find_map(|t| t.as_any().downcast_ref::<EntityRef>())
            .filter(|er| er.entity_type == "client")
            .ok_or_else(|| Error::CommandNotSupported("No client reference found".to_string()))?;

        // Extract amount
        let amount = tokens.iter()
            .find_map(|t| t.as_any().downcast_ref::<Amount>())
            .ok_or_else(|| Error::CommandNotSupported("No amount found".to_string()))?;

        // Extract quantities (optional)
        let quantities: Vec<&Quantity> = tokens.iter()
            .filter_map(|t| t.as_any().downcast_ref::<Quantity>())
            .collect();

        // Create invoice
        let invoice_id = Uuid::new_v4();
        let invoice = Invoice {
            id: invoice_id,
            client_id: client_ref.entity_id.clone(),
            amount: amount.value,
            currency: amount.currency.as_ref().map(|c| c.to_string()).unwrap_or_else(|| "PHP".to_string()),
            items: quantities.iter().map(|q| InvoiceItem {
                description: format!("{} {}", q.value, q.unit.as_deref().unwrap_or("units")),
                quantity: q.value,
                unit_price: amount.value / Decimal::from(quantities.len()),
            }).collect(),
            created_at: Utc::now(),
            status: InvoiceStatus::Draft,
        };

        // Generate journal entries for accounting
        let journal_entries = self.generate_journal_entries(&invoice);

        // Create artifacts
        let artifacts = vec![
            Artifact {
                artifact_type: "invoice".to_string(),
                data: serde_json::to_value(&invoice).unwrap(),
            },
            Artifact {
                artifact_type: "pdf".to_string(),
                data: json!({
                    "filename": format!("invoice-{}.pdf", invoice_id),
                    "content": "PDF content would be generated here"
                }),
            },
        ];

        Ok(ExecutionResult {
            plugin_id: self.id.clone(),
            command: "create_invoice".to_string(),
            journal_entries,
            metadata: json!({
                "invoice_id": invoice_id,
                "client_id": client_ref.entity_id,
                "amount": amount.value.to_string(),
                "currency": invoice.currency,
                "item_count": invoice.items.len(),
            }),
            artifacts,
        })
    }

    async fn list_invoices(&self, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        // Extract optional client filter
        let client_filter = tokens.iter()
            .find_map(|t| t.as_any().downcast_ref::<EntityRef>())
            .filter(|er| er.entity_type == "client")
            .map(|er| er.entity_id.clone());

        // In a real implementation, this would query a database
        let invoices = vec![
            InvoiceSummary {
                id: Uuid::new_v4(),
                client_name: "Sample Client".to_string(),
                amount: Decimal::from_str("1000.00").unwrap(),
                currency: "PHP".to_string(),
                status: "draft".to_string(),
                created_at: Utc::now(),
            }
        ];

        let filtered_invoices: Vec<&InvoiceSummary> = if let Some(client_id) = client_filter {
            invoices.iter().filter(|inv| inv.client_name.contains(&client_id)).collect()
        } else {
            invoices.iter().collect()
        };

        Ok(ExecutionResult {
            plugin_id: self.id.clone(),
            command: "list_invoices".to_string(),
            journal_entries: vec![],
            metadata: json!({
                "count": filtered_invoices.len(),
                "client_filter": client_filter,
            }),
            artifacts: vec![
                Artifact {
                    artifact_type: "invoice_list".to_string(),
                    data: serde_json::to_value(&filtered_invoices).unwrap(),
                }
            ],
        })
    }

    fn generate_journal_entries(&self, invoice: &Invoice) -> Vec<JournalEntry> {
        vec![
            JournalEntry {
                date: invoice.created_at,
                description: format!("Invoice {} for client {}", invoice.id, invoice.client_id),
                lines: vec![
                    JournalLine {
                        account_code: "1100".to_string(), // Accounts Receivable
                        debit: Some(invoice.amount),
                        credit: None,
                        reference: Some(format!("INV-{}", invoice.id)),
                    },
                    JournalLine {
                        account_code: "4000".to_string(), // Sales Revenue
                        debit: None,
                        credit: Some(invoice.amount),
                        reference: Some(format!("INV-{}", invoice.id)),
                    },
                ],
            }
        ]
    }
}

/// Invoice data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub client_id: String,
    pub amount: Decimal,
    pub currency: String,
    pub items: Vec<InvoiceItem>,
    pub created_at: DateTime<Utc>,
    pub status: InvoiceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
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
pub struct InvoiceSummary {
    pub id: Uuid,
    pub client_name: String,
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Philippine VAT Token - Specialized tax token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilippineVatToken {
    pub base: BaseTokenFields,
    pub rate: Decimal,
    pub inclusive: bool,
    pub applies_to: Option<Uuid>,
}

impl Token for PhilippineVatToken {
    fn token_type(&self) -> &str {
        "tax:vat:ph"
    }

    fn uuid(&self) -> Uuid {
        self.base.uuid
    }

    fn raw_text(&self) -> &str {
        &self.base.raw_text
    }

    fn position(&self) -> &TokenPosition {
        &self.base.position
    }

    fn clone_box(&self) -> Box<dyn Token> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<PhilippineVatToken>()
    }

    fn to_json(&self) -> Value {
        json!({
            "type": "tax:vat:ph",
            "uuid": self.uuid().to_string(),
            "rate": self.rate.to_string(),
            "inclusive": self.inclusive,
            "applies_to": self.applies_to.map(|u| u.to_string()),
            "raw_text": self.raw_text(),
        })
    }

    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }

    fn validate(&self) -> Result<(), ValidationError> {
        if self.rate != Decimal::from_str("0.12").unwrap() {
            return Err(ValidationError::InvalidToken {
                reason: "Philippine VAT rate must be 12%".to_string(),
            });
        }
        Ok(())
    }

    fn can_compose_with(&self, other: &dyn Token) -> bool {
        matches!(other.token_type(), "amount")
    }

    fn display_text(&self) -> String {
        format!("VAT {}%", self.rate * Decimal::from(100))
    }

    fn metadata(&self) -> TokenMetadata {
        self.base.metadata.clone()
    }
}

impl TaxToken for PhilippineVatToken {
    fn tax_type(&self) -> &str {
        "VAT"
    }

    fn rate(&self) -> Option<Decimal> {
        Some(self.rate)
    }

    fn is_inclusive(&self) -> bool {
        self.inclusive
    }

    fn compute(&self, amount: Decimal) -> TaxComputation {
        if self.inclusive {
            let base = amount / (Decimal::ONE + self.rate);
            let tax = amount - base;
            TaxComputation { base, tax, total: amount }
        } else {
            let tax = amount * self.rate;
            TaxComputation { base: amount, tax, total: amount + tax }
        }
    }

    fn applies_to(&self) -> Vec<String> {
        vec!["amount".to_string()]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxComputation {
    pub base: Decimal,
    pub tax: Decimal,
    pub total: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::tokens::*;

    #[test]
    fn test_invoice_plugin_creation() {
        let plugin = InvoicePlugin::new();
        assert_eq!(plugin.id(), "invoice-plugin");
        assert!(!plugin.version().is_empty());
    }

    #[test]
    fn test_philippine_vat_token() {
        let token = PhilippineVatToken {
            base: BaseTokenFields::new("12% VAT".to_string(), TokenPosition { start: 0, end: 0, line: 0, column: 0 }),
            rate: Decimal::from_str("0.12").unwrap(),
            inclusive: false,
            applies_to: None,
        };

        assert_eq!(token.token_type(), "tax:vat:ph");
        assert_eq!(token.display_text(), "VAT 12%");
        assert!(token.validate().is_ok());
    }

    #[test]
    fn test_vat_computation() {
        let token = PhilippineVatToken {
            base: BaseTokenFields::new("12% VAT".to_string(), TokenPosition { start: 0, end: 0, line: 0, column: 0 }),
            rate: Decimal::from_str("0.12").unwrap(),
            inclusive: false,
            applies_to: None,
        };

        let computation = token.compute(Decimal::from(1000));
        assert_eq!(computation.base, Decimal::from(1000));
        assert_eq!(computation.tax, Decimal::from(120));
        assert_eq!(computation.total, Decimal::from(1120));
    }
}