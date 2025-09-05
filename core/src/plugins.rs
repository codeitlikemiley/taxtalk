use crate::plugin_system::*;
use crate::tokens::*;
use async_trait::async_trait;

/// Invoice plugin implementation
pub struct InvoicePlugin {
    id: String,
    version: String,
}

impl InvoicePlugin {
    pub fn new() -> Self {
        Self {
            id: "invoice".to_string(),
            version: "0.1.0".to_string(),
        }
    }
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
                command: "create".to_string(),
                aliases: vec!["new".to_string(), "add".to_string()],
                requirements: CommandRequirements {
                    required: vec![
                        TokenRequirement {
                            token_type: "entity_ref".to_string(),
                            description: "Client reference".to_string(),
                            multiple: false,
                        },
                        TokenRequirement {
                            token_type: "amount".to_string(),
                            description: "Invoice amount".to_string(),
                            multiple: false,
                        },
                    ],
                    optional: vec![
                        TokenRequirement {
                            token_type: "quantity".to_string(),
                            description: "Item quantities".to_string(),
                            multiple: true,
                        },
                    ],
                    defaults: vec![],
                    validators: vec![],
                    allow_partial: false,
                },
                priority: 10,
                description: "Create a new invoice".to_string(),
                examples: vec![
                    "@invoice create client:uriah amount:1000".to_string(),
                    "create invoice for uriah 1000 pesos".to_string(),
                ],
            },
            CommandRegistration {
                command: "list".to_string(),
                aliases: vec!["show".to_string(), "view".to_string()],
                requirements: CommandRequirements {
                    required: vec![],
                    optional: vec![
                        TokenRequirement {
                            token_type: "entity_ref".to_string(),
                            description: "Client filter".to_string(),
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
                    "@invoice list".to_string(),
                    "show invoices for uriah".to_string(),
                ],
            },
        ]
    }

    fn can_handle(&self, command: &ActionToken, tokens: &[Box<dyn Token>]) -> CanHandleResult {
        match command.verb.as_str() {
            "create" | "new" | "add" => {
                // Check for required tokens
                let has_entity = tokens.iter().any(|t| t.token_type() == "entity_ref");
                let has_amount = tokens.iter().any(|t| t.token_type() == "amount");

                if has_entity && has_amount {
                    CanHandleResult::Yes { confidence: 0.9 }
                } else if has_entity || has_amount {
                    CanHandleResult::Partial {
                        missing: if has_entity { vec!["amount".to_string()] } else { vec!["entity_ref".to_string()] }
                    }
                } else {
                    CanHandleResult::No { reason: "Missing required tokens".to_string() }
                }
            }
            "list" | "show" | "view" => {
                CanHandleResult::Yes { confidence: 0.8 }
            }
            _ => CanHandleResult::No { reason: "Unsupported command".to_string() },
        }
    }

    async fn execute(&self, command: &ActionToken, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        match command.verb.as_str() {
            "create" | "new" | "add" => self.create_invoice(tokens).await,
            "list" | "show" | "view" => self.list_invoices(tokens).await,
            _ => Err(Error::PluginExecution("Unsupported command".to_string())),
        }
    }
}

impl InvoicePlugin {
    async fn create_invoice(&self, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        // Extract client and amount from tokens
        let client = tokens.iter()
            .find_map(|t| t.as_any().downcast_ref::<EntityRef>())
            .ok_or_else(|| Error::PluginExecution("No client specified".to_string()))?;

        let amount = tokens.iter()
            .find_map(|t| t.as_any().downcast_ref::<Amount>())
            .ok_or_else(|| Error::PluginExecution("No amount specified".to_string()))?;

        // Generate invoice ID
        let invoice_id = format!("INV-{}", uuid::Uuid::new_v4().simple());

        // Create journal entries for accounting
        let journal_entries = vec![
            JournalEntry {
                date: chrono::Utc::now().date_naive(),
                description: format!("Invoice {} for client {}", invoice_id, client.display_name()),
                lines: vec![
                    JournalLine {
                        account_code: "1100".to_string(), // Accounts Receivable
                        debit: Some(amount.value),
                        credit: None,
                        reference: Some(invoice_id.clone()),
                    },
                    JournalLine {
                        account_code: "4000".to_string(), // Sales Revenue
                        debit: None,
                        credit: Some(amount.value),
                        reference: Some(invoice_id.clone()),
                    },
                ],
            }
        ];

        // Create artifacts
        let artifacts = vec![
            Artifact {
                artifact_type: "invoice".to_string(),
                data: serde_json::json!({
                    "id": invoice_id,
                    "client_id": client.entity_id,
                    "amount": amount.value.to_string(),
                    "currency": amount.currency.as_ref().map(|c| c.to_json()),
                    "created_at": chrono::Utc::now().to_rfc3339(),
                }),
            }
        ];

        Ok(ExecutionResult {
            plugin_id: self.id.clone(),
            command: "create".to_string(),
            journal_entries,
            metadata: serde_json::json!({
                "invoice_id": invoice_id,
                "client_name": client.display_name(),
            }),
            artifacts,
        })
    }

    async fn list_invoices(&self, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        // Extract optional client filter
        let client_filter = tokens.iter()
            .find_map(|t| t.as_any().downcast_ref::<EntityRef>());

        let filter_description = if let Some(client) = client_filter {
            format!(" for client {}", client.display_name())
        } else {
            "".to_string()
        };

        // Create artifacts for invoice list
        let artifacts = vec![
            Artifact {
                artifact_type: "invoice_list".to_string(),
                data: serde_json::json!({
                    "filter": client_filter.map(|c| c.entity_id),
                    "description": format!("List of invoices{}", filter_description),
                    "count": 0, // Would be populated from actual data
                }),
            }
        ];

        Ok(ExecutionResult {
            plugin_id: self.id.clone(),
            command: "list".to_string(),
            journal_entries: vec![], // No accounting impact for listing
            metadata: serde_json::json!({
                "operation": "list",
                "filter_applied": client_filter.is_some(),
            }),
            artifacts,
        })
    }
}

/// Company plugin implementation
pub struct CompanyPlugin {
    id: String,
    version: String,
}

impl CompanyPlugin {
    pub fn new() -> Self {
        Self {
            id: "company".to_string(),
            version: "0.1.0".to_string(),
        }
    }
}

#[async_trait]
impl Plugin for CompanyPlugin {
    fn id(&self) -> &str {
        &self.id
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn commands(&self) -> Vec<CommandRegistration> {
        vec![
            CommandRegistration {
                command: "create".to_string(),
                aliases: vec!["new".to_string(), "add".to_string()],
                requirements: CommandRequirements {
                    required: vec![
                        TokenRequirement {
                            token_type: "entity_ref".to_string(),
                            description: "Company name".to_string(),
                            multiple: false,
                        },
                    ],
                    optional: vec![],
                    defaults: vec![],
                    validators: vec![],
                    allow_partial: false,
                },
                priority: 10,
                description: "Create a new company".to_string(),
                examples: vec![
                    "@company create name:ABC Corp".to_string(),
                    "create company ABC Corp".to_string(),
                ],
            },
        ]
    }

    fn can_handle(&self, command: &ActionToken, tokens: &[Box<dyn Token>]) -> CanHandleResult {
        match command.verb.as_str() {
            "create" | "new" | "add" => {
                let has_entity = tokens.iter().any(|t| t.token_type() == "entity_ref");
                if has_entity {
                    CanHandleResult::Yes { confidence: 0.8 }
                } else {
                    CanHandleResult::Partial { missing: vec!["entity_ref".to_string()] }
                }
            }
            _ => CanHandleResult::No { reason: "Unsupported command".to_string() },
        }
    }

    async fn execute(&self, command: &ActionToken, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        match command.verb.as_str() {
            "create" | "new" | "add" => self.create_company(tokens).await,
            _ => Err(Error::PluginExecution("Unsupported command".to_string())),
        }
    }
}

impl CompanyPlugin {
    async fn create_company(&self, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        let company = tokens.iter()
            .find_map(|t| t.as_any().downcast_ref::<EntityRef>())
            .ok_or_else(|| Error::PluginExecution("No company name specified".to_string()))?;

        let company_id = format!("COMP-{}", uuid::Uuid::new_v4().simple());

        let artifacts = vec![
            Artifact {
                artifact_type: "company".to_string(),
                data: serde_json::json!({
                    "id": company_id,
                    "name": company.display_name(),
                    "created_at": chrono::Utc::now().to_rfc3339(),
                }),
            }
        ];

        Ok(ExecutionResult {
            plugin_id: self.id.clone(),
            command: "create".to_string(),
            journal_entries: vec![], // Companies don't typically have journal entries
            metadata: serde_json::json!({
                "company_id": company_id,
                "company_name": company.display_name(),
            }),
            artifacts,
        })
    }
}