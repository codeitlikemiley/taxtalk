use anyhow::{anyhow, Error};
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use std::sync::{Arc, Mutex};

use taxtalk_core::plugin::{
    Artifact, CanHandleResult, CommandRegistration, CommandRequirements,
    ExecutionResult, JournalEntry, JournalLine, Plugin, TokenRequirement,
};
use taxtalk_core::token::{
    ActionToken, Amount, BaseTokenFields, DateToken, EntityRef, PhilippineVatToken, TaxToken,
    Token, TokenPosition,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub number: String,
    pub customer_id: String,
    pub customer_name: String,
    pub date: NaiveDate,
    pub amount: Decimal,
    pub tax: Decimal,
    pub total: Decimal,
    pub terms: u32,
    pub line_items: Vec<LineItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub total: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceConfig {
    pub invoice_prefix: String,
    pub auto_generate_number: bool,
    pub default_terms: u32,
    pub apply_vat_by_default: bool,
}

impl Default for InvoiceConfig {
    fn default() -> Self {
        Self {
            invoice_prefix: "INV".to_string(),
            auto_generate_number: true,
            default_terms: 30,
            apply_vat_by_default: true,
        }
    }
}

pub struct InvoicePlugin {
    id: String,
    config: InvoiceConfig,
    invoice_counter: Arc<Mutex<u32>>,
}

impl InvoicePlugin {
    pub fn new() -> Self {
        Self {
            id: "invoice".to_string(),
            config: InvoiceConfig::default(),
            invoice_counter: Arc::new(Mutex::new(1000)),
        }
    }

    async fn generate_invoice_number(&self) -> String {
        let mut counter = self.invoice_counter.lock().unwrap();
        *counter += 1;
        format!("{}-{:06}", self.config.invoice_prefix, *counter)
    }

    fn extract_token<T: 'static>(
        &self,
        tokens: &[Box<dyn Token>],
        token_type: &str,
    ) -> Result<T, Error>
    where
        T: Clone,
    {
        tokens
            .iter()
            .find(|t| t.token_type() == token_type)
            .and_then(|t| t.as_any().downcast_ref::<T>())
            .cloned()
            .ok_or_else(|| anyhow!("Required token not found: {}", token_type))
    }

    fn extract_token_optional<T: 'static>(
        &self,
        tokens: &[Box<dyn Token>],
        token_type: &str,
    ) -> Option<T>
    where
        T: Clone,
    {
        tokens
            .iter()
            .find(|t| t.token_type() == token_type)
            .and_then(|t| t.as_any().downcast_ref::<T>())
            .cloned()
    }

    fn calculate_tax(
        &self,
        amount: &Amount,
        tokens: &[Box<dyn Token>],
    ) -> Result<taxtalk_core::token::TaxComputation, Error> {
        // Look for tax token
        if let Some(vat) = tokens
            .iter()
            .find_map(|t| t.as_any().downcast_ref::<PhilippineVatToken>())
        {
            Ok(vat.compute(amount.value))
        } else if self.config.apply_vat_by_default {
            // Apply default VAT
            let vat = PhilippineVatToken {
                base: BaseTokenFields::new(
                    "VAT 12%".to_string(),
                    TokenPosition {
                        start: 0,
                        end: 7,
                        line: 1,
                        column: 1,
                    },
                ),
                rate: dec!(0.12),
                inclusive: true,
                applies_to: None,
            };
            Ok(vat.compute(amount.value))
        } else {
            Ok(taxtalk_core::token::TaxComputation {
                base: amount.value,
                tax: dec!(0),
                total: amount.value,
            })
        }
    }

    fn generate_journal_entries(&self, invoice: &Invoice) -> Result<Vec<JournalEntry>, Error> {
        let mut entries = Vec::new();

        entries.push(JournalEntry {
            date: invoice.date,
            description: format!("Invoice {} to {}", invoice.number, invoice.customer_name),
            reference: Some(invoice.number.clone()),
            tags: vec!["invoice".to_string(), "sales".to_string()],
            lines: vec![
                JournalLine {
                    account_code: "10200".to_string(), // Accounts Receivable
                    account_name: Some("Accounts Receivable".to_string()),
                    debit: Some(invoice.total),
                    credit: None,
                    reference: Some(invoice.number.clone()),
                    memo: Some(format!("Invoice to {}", invoice.customer_name)),
                },
                JournalLine {
                    account_code: "40100".to_string(), // Sales Revenue
                    account_name: Some("Sales Revenue".to_string()),
                    debit: None,
                    credit: Some(invoice.amount),
                    reference: Some(invoice.number.clone()),
                    memo: None,
                },
                JournalLine {
                    account_code: "20310".to_string(), // Output VAT
                    account_name: Some("Output VAT Payable".to_string()),
                    debit: None,
                    credit: Some(invoice.tax),
                    reference: Some(invoice.number.clone()),
                    memo: None,
                },
            ],
        });

        Ok(entries)
    }
}

#[async_trait]
impl Plugin for InvoicePlugin {
    fn id(&self) -> &str {
        &self.id
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn commands(&self) -> Vec<CommandRegistration> {
        vec![CommandRegistration {
            command: "invoice".to_string(),
            aliases: vec!["bill".to_string(), "charge".to_string()],
            requirements: CommandRequirements {
                command: "invoice".to_string(),
                required: vec![
                    TokenRequirement {
                        token_type: "entity_ref".to_string(),
                        description: "Customer to invoice".to_string(),
                        validator: Some(Arc::new(|token| {
                            if let Some(entity) = token.as_any().downcast_ref::<EntityRef>() {
                                entity.entity_type == "customer" || entity.entity_type == "client"
                            } else {
                                false
                            }
                        })),
                        multiple: false,
                    },
                    TokenRequirement {
                        token_type: "amount".to_string(),
                        description: "Invoice amount".to_string(),
                        validator: Some(Arc::new(|token| {
                            if let Some(amount) = token.as_any().downcast_ref::<Amount>() {
                                amount.value > Decimal::ZERO
                            } else {
                                false
                            }
                        })),
                        multiple: false,
                    },
                ],
                optional: vec![
                    TokenRequirement {
                        token_type: "date".to_string(),
                        description: "Invoice date".to_string(),
                        validator: None,
                        multiple: false,
                    },
                    TokenRequirement {
                        token_type: "terms".to_string(),
                        description: "Payment terms".to_string(),
                        validator: None,
                        multiple: false,
                    },
                ],
                defaults: vec![],
                validators: vec![],
                allow_partial: false,
            },
            priority: 100,
            description: "Create an invoice for a customer".to_string(),
            examples: vec![
                "@customer ABC invoice 50000".to_string(),
                "bill @client Juan 1000 plus VAT".to_string(),
            ],
        }]
    }

    fn can_handle(&self, command: &ActionToken, tokens: &[Box<dyn Token>]) -> CanHandleResult {
        // Check if this is our command
        let our_commands = ["invoice", "bill", "charge"];
        if !our_commands.contains(&command.verb.as_str()) {
            return CanHandleResult::No {
                reason: format!("Not an invoice command: {}", command.verb),
            };
        }

        // Check for required tokens
        let has_customer = tokens.iter().any(|t| {
            if let Some(entity) = t.as_any().downcast_ref::<EntityRef>() {
                entity.entity_type == "customer" || entity.entity_type == "client"
            } else {
                false
            }
        });

        let has_amount = tokens.iter().any(|t| t.token_type() == "amount");

        if has_customer && has_amount {
            CanHandleResult::Yes { confidence: 1.0 }
        } else {
            let mut missing = Vec::new();
            if !has_customer {
                missing.push("Customer".to_string());
            }
            if !has_amount {
                missing.push("Amount".to_string());
            }
            CanHandleResult::Partial { missing }
        }
    }

    async fn execute(
        &self,
        command: &ActionToken,
        tokens: Vec<Box<dyn Token>>,
    ) -> Result<ExecutionResult, Error> {
        // Extract required tokens
        let customer = self.extract_token::<EntityRef>(&tokens, "entity_ref")?;
        let amount = self.extract_token::<Amount>(&tokens, "amount")?;

        // Extract optional tokens
        let date = self.extract_token_optional::<DateToken>(&tokens, "date");

        // Generate invoice number
        let invoice_number = self.generate_invoice_number().await;

        // Calculate tax
        let tax_computation = self.calculate_tax(&amount, &tokens)?;

        // Create invoice
        let invoice = Invoice {
            number: invoice_number.clone(),
            customer_id: customer.entity_id.clone(),
            customer_name: customer.display_name.unwrap_or(customer.entity_id.clone()),
            date: date
                .map(|d| d.date)
                .unwrap_or_else(|| Utc::now().naive_utc().date()),
            amount: amount.value,
            tax: tax_computation.tax,
            total: tax_computation.total,
            terms: self.config.default_terms,
            line_items: vec![LineItem {
                description: "Services".to_string(),
                quantity: dec!(1),
                unit_price: amount.value,
                total: amount.value,
            }],
        };

        // Generate journal entries
        let entries = self.generate_journal_entries(&invoice)?;

        Ok(ExecutionResult {
            plugin_id: self.id().to_string(),
            command: command.verb.clone(),
            journal_entries: entries,
            metadata: json!({
                "invoice_number": invoice.number,
                "customer": customer.entity_id,
                "total": invoice.total.to_string(),
            }),
            artifacts: vec![Artifact {
                artifact_type: "invoice".to_string(),
                data: serde_json::to_value(&invoice)?,
                metadata: None,
            }],
        })
    }

    async fn export_state(&self) -> Result<Value, Error> {
        Ok(json!({
            "last_invoice_number": *self.invoice_counter.lock().unwrap(),
            "config": self.config,
        }))
    }

    async fn import_state(&mut self, state: Value) -> Result<(), Error> {
        if let Some(last_num) = state["last_invoice_number"].as_u64() {
            *self.invoice_counter.lock().unwrap() = last_num as u32;
        }

        if let Ok(config) = serde_json::from_value::<InvoiceConfig>(state["config"].clone()) {
            self.config = config;
        }

        Ok(())
    }
}
