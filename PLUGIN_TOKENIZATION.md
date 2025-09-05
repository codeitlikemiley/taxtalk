# Complete Plugin System Integration Guide

Let me show you how to build a complete working plugin system using our token architecture. I'll create a real example with invoice and inventory plugins that can be hot-swapped.

## 1. Core Plugin System Setup

```rust
// core/src/lib.rs
use std::sync::Arc;
use tokio::sync::RwLock;
use wasmtime::{Engine, Module, Store, Instance, Linker};

pub struct PluginSystem {
    /// Command router for plugin dispatch
    router: Arc<RwLock<CommandRouter>>,
    
    /// Token factory for creating tokens
    token_factory: Arc<TokenFactory>,
    
    /// Token validator
    validator: Arc<TokenValidator>,
    
    /// Wasmtime engine for WASM plugins
    engine: Engine,
    
    /// Loaded plugins
    plugins: Arc<RwLock<HashMap<String, LoadedPlugin>>>,
}

pub struct LoadedPlugin {
    pub id: String,
    pub instance: PluginInstance,
    pub manifest: PluginManifest,
    pub state: PluginState,
}

pub enum PluginInstance {
    Native(Box<dyn Plugin>),
    Wasm(Instance),
}

impl PluginSystem {
    pub async fn new() -> Result<Self, Error> {
        let engine = Engine::default();
        let router = Arc::new(RwLock::new(CommandRouter::new()));
        let token_factory = Arc::new(TokenFactory::new());
        let validator = Arc::new(TokenValidator::new());
        
        Ok(Self {
            router,
            token_factory,
            validator,
            engine,
            plugins: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Load a plugin from file or registry
    pub async fn load_plugin(&self, path: &Path) -> Result<(), Error> {
        let manifest = self.load_manifest(path)?;
        
        match manifest.plugin_type {
            PluginType::Native => {
                self.load_native_plugin(path, manifest).await
            }
            PluginType::Wasm => {
                self.load_wasm_plugin(path, manifest).await
            }
        }
    }
    
    /// Process a natural language command
    pub async fn process_command(&self, input: &str) -> Result<ExecutionResult, Error> {
        // Step 1: Tokenize the input
        let tokens = self.tokenize(input).await?;
        
        // Step 2: Validate tokens
        self.validator.validate_composition(&tokens)?;
        
        // Step 3: Route to appropriate plugin
        let router = self.router.read().await;
        router.route_command(tokens).await
    }
    
    /// Hot-swap a plugin
    pub async fn hot_swap(&self, old_id: &str, new_path: &Path) -> Result<(), Error> {
        // Load new plugin
        let new_manifest = self.load_manifest(new_path)?;
        let new_plugin = self.create_plugin_instance(new_path, &new_manifest).await?;
        
        // Export state from old plugin
        let mut plugins = self.plugins.write().await;
        let old_state = if let Some(old) = plugins.get(old_id) {
            match &old.instance {
                PluginInstance::Native(plugin) => plugin.export_state().await?,
                PluginInstance::Wasm(instance) => self.export_wasm_state(instance).await?,
            }
        } else {
            json!({})
        };
        
        // Import state to new plugin
        match &new_plugin {
            PluginInstance::Native(plugin) => {
                plugin.import_state(old_state).await?;
            }
            PluginInstance::Wasm(instance) => {
                self.import_wasm_state(instance, old_state).await?;
            }
        }
        
        // Update router
        let mut router = self.router.write().await;
        router.hot_swap_plugin(old_id, new_plugin).await?;
        
        Ok(())
    }
}
```

## 2. Plugin Manifest Definition

```rust
// core/src/manifest.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub plugin_type: PluginType,
    pub entry_point: String,
    
    /// Commands this plugin provides
    pub commands: Vec<CommandManifest>,
    
    /// Token types this plugin creates
    pub token_types: Vec<TokenTypeManifest>,
    
    /// Dependencies on other plugins
    pub dependencies: Vec<PluginDependency>,
    
    /// Configuration schema
    pub config_schema: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandManifest {
    pub command: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub requirements: RequirementsManifest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementsManifest {
    pub required: Vec<TokenRequirementManifest>,
    pub optional: Vec<TokenRequirementManifest>,
    pub defaults: Vec<TokenDefaultManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequirementManifest {
    pub token_type: String,
    pub description: String,
    pub multiple: bool,
    pub validation: Option<String>, // Script or rule name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTypeManifest {
    pub type_name: String,
    pub patterns: Vec<String>,
    pub creator_class: String,
}
```

## 3. Example: Invoice Plugin Implementation

```rust
// plugins/invoice/src/lib.rs
use core::{Plugin, Token, CommandToken, ExecutionResult};
use async_trait::async_trait;

pub struct InvoicePlugin {
    id: String,
    config: InvoiceConfig,
    invoice_counter: Arc<Mutex<u32>>,
}

#[async_trait]
impl Plugin for InvoicePlugin {
    fn id(&self) -> &str {
        "invoice"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn commands(&self) -> Vec<CommandRegistration> {
        vec![
            CommandRegistration {
                command: "invoice".to_string(),
                aliases: vec!["bill", "charge", "invoice_to".to_string()],
                requirements: CommandRequirements {
                    command: "invoice".to_string(),
                    required: vec![
                        TokenRequirement {
                            token_type: "entity:customer".to_string(),
                            description: "Customer to invoice".to_string(),
                            validator: Some(Arc::new(|token| {
                                // Validate customer exists
                                if let Some(entity) = token.as_any().downcast_ref::<EntityRef>() {
                                    entity.entity_type == "customer"
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
                                    amount.value > Decimal::zero()
                                } else {
                                    false
                                }
                            })),
                            multiple: false,
                        },
                    ],
                    optional: vec![
                        TokenRequirement {
                            token_type: "entity:product".to_string(),
                            description: "Products/services".to_string(),
                            validator: None,
                            multiple: true,
                        },
                        TokenRequirement {
                            token_type: "terms".to_string(),
                            description: "Payment terms".to_string(),
                            validator: None,
                            multiple: false,
                        },
                        TokenRequirement {
                            token_type: "date".to_string(),
                            description: "Invoice date".to_string(),
                            validator: None,
                            multiple: false,
                        },
                    ],
                    defaults: vec![
                        TokenDefault {
                            token_type: "tax:vat:ph".to_string(),
                            default_value: Box::new(PhilippineVatToken {
                                base: BaseTokenFields::new("VAT 12%".to_string(), TokenPosition::default()),
                                rate: Decimal::from_str("0.12").unwrap(),
                                inclusive: true,
                                applies_to: None,
                            }),
                            condition: Some(Arc::new(|tokens| {
                                // Apply VAT unless exempt
                                !tokens.iter().any(|t| t.token_type() == "tax:exempt")
                            })),
                        },
                        TokenDefault {
                            token_type: "terms".to_string(),
                            default_value: Box::new(TermsToken {
                                base: BaseTokenFields::new("30 days".to_string(), TokenPosition::default()),
                                days: 30,
                                term_type: TermType::Net,
                            }),
                            condition: None,
                        },
                    ],
                    validators: vec![
                        Box::new(InvoiceValidator),
                    ],
                    allow_partial: false,
                },
                priority: 100,
                description: "Create an invoice".to_string(),
                examples: vec![
                    "@customer ABC invoice 50000 for consulting services".to_string(),
                    "bill @client Juan 1000 plus VAT".to_string(),
                ],
            },
        ]
    }
    
    fn can_handle(&self, command: &CommandToken, tokens: &[Box<dyn Token>]) -> CanHandleResult {
        // Check if this is our command
        if !self.commands().iter().any(|c| 
            c.command == command.verb || c.aliases.contains(&command.verb)
        ) {
            return CanHandleResult::No { 
                reason: "Not an invoice command".to_string() 
            };
        }
        
        // Get requirements for this command
        let requirements = &self.commands()[0].requirements;
        
        // Check required tokens
        let mut missing = Vec::new();
        for req in &requirements.required {
            if !tokens.iter().any(|t| t.token_type() == req.token_type) {
                missing.push(req.description.clone());
            }
        }
        
        if missing.is_empty() {
            CanHandleResult::Yes { confidence: 1.0 }
        } else {
            CanHandleResult::Partial { missing }
        }
    }
    
    async fn execute(&self, command: &CommandToken, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        // Apply defaults
        let mut tokens = self.apply_defaults(tokens)?;
        
        // Extract required tokens
        let customer = self.extract_token::<EntityRef>(&tokens, "entity:customer")?;
        let amount = self.extract_token::<Amount>(&tokens, "amount")?;
        
        // Extract optional tokens
        let products = self.extract_multiple_tokens::<EntityRef>(&tokens, "entity:product");
        let terms = self.extract_token_or_default::<TermsToken>(&tokens, "terms");
        let date = self.extract_token_or_default::<DateToken>(&tokens, "date");
        
        // Generate invoice number
        let invoice_number = self.generate_invoice_number().await;
        
        // Resolve customer data
        let customer_data = self.resolve_customer(&customer).await?;
        
        // Calculate with tax
        let tax_computation = self.calculate_tax(&amount, &tokens)?;
        
        // Create invoice
        let invoice = Invoice {
            number: invoice_number,
            customer_id: customer.entity_id.clone(),
            customer_name: customer_data["name"].as_str().unwrap().to_string(),
            date: date.map(|d| d.date).unwrap_or_else(|| Utc::today().naive_utc()),
            amount: amount.value,
            tax: tax_computation.tax,
            total: tax_computation.total,
            terms: terms.days,
            line_items: self.create_line_items(products, amount)?,
        };
        
        // Generate journal entries
        let entries = self.generate_journal_entries(&invoice)?;
        
        // Save invoice
        self.save_invoice(&invoice).await?;
        
        Ok(ExecutionResult {
            plugin_id: self.id().to_string(),
            command: command.verb.clone(),
            journal_entries: entries,
            metadata: json!({
                "invoice_number": invoice.number,
                "customer": customer.entity_id,
                "total": invoice.total.to_string(),
                "entries_created": entries.len(),
            }),
            artifacts: vec![
                Artifact {
                    artifact_type: "invoice".to_string(),
                    data: serde_json::to_value(&invoice)?,
                }
            ],
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
        Ok(())
    }
}

// Helper methods
impl InvoicePlugin {
    fn generate_journal_entries(&self, invoice: &Invoice) -> Result<Vec<JournalEntry>, Error> {
        let mut entries = Vec::new();
        
        // Debit: Accounts Receivable
        // Credit: Sales Revenue
        // Credit: Output VAT
        
        entries.push(JournalEntry {
            date: invoice.date,
            description: format!("Invoice {} to {}", invoice.number, invoice.customer_name),
            lines: vec![
                JournalLine {
                    account_code: "10200".to_string(), // Accounts Receivable
                    debit: Some(invoice.total),
                    credit: None,
                    reference: Some(invoice.number.clone()),
                },
                JournalLine {
                    account_code: "40100".to_string(), // Sales Revenue
                    debit: None,
                    credit: Some(invoice.amount),
                    reference: Some(invoice.number.clone()),
                },
                JournalLine {
                    account_code: "20310".to_string(), // Output VAT
                    debit: None,
                    credit: Some(invoice.tax),
                    reference: Some(invoice.number.clone()),
                },
            ],
        });
        
        Ok(entries)
    }
}
```

## 4. Example: Using the Plugin System

```rust
// server/src/main.rs
use core::PluginSystem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize plugin system
    let plugin_system = PluginSystem::new().await?;
    
    // Load built-in plugins
    plugin_system.load_plugin(Path::new("plugins/invoice")).await?;
    plugin_system.load_plugin(Path::new("plugins/inventory")).await?;
    plugin_system.load_plugin(Path::new("plugins/crm")).await?;
    
    // Start server
    let app = Router::new()
        .route("/command", post(handle_command))
        .with_state(plugin_system);
    
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

async fn handle_command(
    State(plugin_system): State<Arc<PluginSystem>>,
    Json(request): Json<CommandRequest>,
) -> Result<Json<CommandResponse>, StatusCode> {
    
    // Process the natural language command
    match plugin_system.process_command(&request.command).await {
        Ok(result) => {
            Ok(Json(CommandResponse {
                success: true,
                result: Some(result),
                error: None,
                suggestions: vec![],
            }))
        }
        Err(Error::MissingTokens(missing)) => {
            // Return what's missing
            Ok(Json(CommandResponse {
                success: false,
                result: None,
                error: Some(format!("Missing: {}", missing.join(", "))),
                suggestions: plugin_system.suggest_completions(&request.command).await?,
            }))
        }
        Err(e) => {
            Ok(Json(CommandResponse {
                success: false,
                result: None,
                error: Some(e.to_string()),
                suggestions: vec![],
            }))
        }
    }
}
```

## 5. Example Commands in Action

```rust
// Example 1: Simple invoice
let input = "@customer ABC invoice 50000";

// Tokenization produces:
// 1. EntityRef { entity_type: "customer", entity_id: "ABC" }
// 2. CommandToken { verb: "invoice" }
// 3. Amount { value: 50000 }

// Invoice plugin:
// - Recognizes "invoice" command
// - Has required tokens (customer, amount)
// - Applies default VAT (12%)
// - Applies default terms (30 days)
// - Generates invoice and journal entries

// Example 2: Complex invoice
let input = "bill @client Juan 5000 for @product consulting @product training due in 15 days";

// Tokenization produces:
// 1. CommandToken { verb: "bill" } // Alias for invoice
// 2. EntityRef { entity_type: "client", entity_id: "Juan" }
// 3. Amount { value: 5000 }
// 4. EntityRef { entity_type: "product", entity_id: "consulting" }
// 5. EntityRef { entity_type: "product", entity_id: "training" }
// 6. TermsToken { days: 15 }

// Example 3: Hot swap scenario
let input = "@customer XYZ bought 10 @product laptops for 500000";

// Current inventory plugin handles this
let result1 = plugin_system.process_command(input).await?;

// Hot swap to external inventory system
plugin_system.hot_swap("inventory", Path::new("plugins/external_inventory")).await?;

// Same command now routes to new plugin
let result2 = plugin_system.process_command(input).await?;
// New plugin has same command registration but different implementation
```

## 6. Plugin Development Guide

```rust
// Template for new plugin developers
// plugins/my_plugin/src/lib.rs

use core::*;
use async_trait::async_trait;

pub struct MyPlugin {
    // Plugin state
}

#[async_trait]
impl Plugin for MyPlugin {
    fn commands(&self) -> Vec<CommandRegistration> {
        vec![
            CommandRegistration {
                command: "my_command".to_string(),
                aliases: vec!["mc", "myc"],
                requirements: CommandRequirements {
                    required: vec![
                        // What tokens you MUST have
                        TokenRequirement {
                            token_type: "entity:something".to_string(),
                            description: "Something required".to_string(),
                            validator: None,
                            multiple: false,
                        },
                    ],
                    optional: vec![
                        // What tokens are nice to have
                    ],
                    defaults: vec![
                        // What tokens to add if missing
                    ],
                    validators: vec![
                        // Custom validation rules
                    ],
                    allow_partial: false,
                },
                priority: 100,
                description: "Does something".to_string(),
                examples: vec![],
            },
        ]
    }
    
    async fn execute(&self, command: &CommandToken, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        // Your business logic here
        
        // 1. Extract tokens you need
        let something = self.extract_token::<EntityRef>(&tokens, "entity:something")?;
        
        // 2. Do your processing
        
        // 3. Generate journal entries if needed
        
        // 4. Return result
        Ok(ExecutionResult {
            plugin_id: self.id().to_string(),
            command: command.verb.clone(),
            journal_entries: vec![],
            metadata: json!({}),
            artifacts: vec![],
        })
    }
}

// Export for WASM if needed
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyPlugin::new()))
}
```

## 7. Plugin manifest.json

```json
{
  "id": "invoice",
  "name": "Invoice Management Plugin",
  "version": "1.0.0",
  "plugin_type": "native",
  "entry_point": "lib.rs",
  "commands": [
    {
      "command": "invoice",
      "aliases": ["bill", "charge"],
      "description": "Create an invoice",
      "requirements": {
        "required": [
          {
            "token_type": "entity:customer",
            "description": "Customer to invoice",
            "multiple": false
          },
          {
            "token_type": "amount",
            "description": "Invoice amount",
            "multiple": false
          }
        ],
        "optional": [
          {
            "token_type": "entity:product",
            "description": "Products or services",
            "multiple": true
          }
        ],
        "defaults": [
          {
            "token_type": "tax:vat:ph",
            "value": {
              "rate": 0.12,
              "inclusive": true
            }
          }
        ]
      }
    }
  ],
  "token_types": [
    {
      "type_name": "invoice:number",
      "patterns": ["INV-\\d+", "SI-\\d+"],
      "creator_class": "InvoiceNumberCreator"
    }
  ],
  "dependencies": [
    {
      "plugin_id": "crm",
      "version": ">=1.0.0",
      "required": true
    }
  ],
  "config_schema": {
    "type": "object",
    "properties": {
      "invoice_prefix": {
        "type": "string",
        "default": "INV"
      },
      "auto_generate_number": {
        "type": "boolean",
        "default": true
      }
    }
  }
}
```

## Key Benefits in Action

1. **Command-driven**: "invoice" command automatically routes to invoice plugin
2. **Token validation**: Plugin validates it has customer and amount
3. **Smart defaults**: VAT automatically applied unless specified
4. **Hot-swappable**: Can replace invoice plugin without changing commands
5. **Order-independent**: "@customer ABC invoice 50000" = "invoice 50000 to @customer ABC"
6. **Extensible**: New plugins just register their commands
7. **Type-safe**: Token downcasting ensures type safety

This complete system allows developers to create plugins that work together seamlessly while maintaining independence!