use axum::{
    Router,
    routing::{get, post, delete},
    extract::{Path, State, Multipart},
    response::Json,
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::RwLock;
use taxtalk_core::{
    PluginHost, PluginRegistry, DbPool, init_database,
    TokenValidator, GuidedSession,
    nlp::{NLPParser, ParsedCommand}
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod command_registry;
mod command_registry_v2;
use command_registry::CommandRegistry;
use command_registry_v2::{CommandRegistryV2, Command, Entity as EntityV2, ParseResult};

#[derive(Clone)]
struct AppState {
    plugin_host: Arc<PluginHost>,
    plugin_registry: Arc<RwLock<PluginRegistry>>,
    db_pool: DbPool,
    validator: Arc<TokenValidator>,
    session_manager: Arc<RwLock<taxtalk_core::validation::session::SessionManager>>,
    command_registry: Arc<RwLock<CommandRegistry>>,
    command_registry_v2: Arc<RwLock<CommandRegistryV2>>,
}

#[derive(Deserialize)]
struct ExecuteCommand {
    command: String,
}

#[derive(Serialize)]
struct ExecuteResponse {
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<String>,
}

#[derive(Serialize)]
struct PluginInfo {
    id: String,
    name: String,
    version: String,
    description: String,
    commands: Vec<CommandInfo>,
    enabled: bool,
}

// CommandInfo moved to later in file with full definition

#[derive(Serialize)]
struct Entity {
    id: String,
    name: String,
    entity_type: String,
    description: Option<String>,
    metadata: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct SearchResponse {
    results: Vec<Entity>,
}

#[derive(Deserialize)]
struct SearchQuery {
    #[serde(rename = "q")]
    query: String,
    #[serde(rename = "type")]
    entity_type: Option<String>,
}

#[derive(Deserialize)]
struct EntityTypeSuggestionsQuery {
    command: String,
    cursor_position: usize,
    #[serde(default)]
    filled_tokens: Vec<FilledToken>,
}

#[derive(Debug, Deserialize, Clone)]
struct FilledToken {
    token_type: String,
    entity_id: String,
    entity_type: String,
}

#[derive(Serialize)]
struct EntityTypeSuggestion {
    entity_type: String,
    label: String,
    description: String,
    cardinality: String,  // "single" or "multiple"
    hint: Option<String>,
}

#[derive(Deserialize)]
struct ValidateCommand {
    command: String,
    session_id: Option<String>,
}

#[derive(Serialize)]
struct ValidateResponse {
    valid: bool,
    action: Option<String>,
    entity: Option<String>,
    plugin: Option<String>,
    missing_required: Vec<serde_json::Value>,
    missing_optional: Vec<serde_json::Value>,
    provided_tokens: serde_json::Value,
    defaults_applied: serde_json::Value,
    session: Option<SessionInfo>,
}

#[derive(Serialize)]
struct SessionInfo {
    id: String,
    status: String,
    progress: u32,
    total_required: usize,
    next_prompt: Option<String>,
}

#[derive(Deserialize)]
struct SubmitToken {
    token: String,
    value: serde_json::Value,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let db_pool = init_database("data/taxtalk.db")
        .await
        .expect("Failed to initialize database");
    
    // Initialize plugin system
    let plugin_host = Arc::new(PluginHost::new().expect("Failed to create plugin host"));
    let plugin_registry = Arc::new(RwLock::new(
        PluginRegistry::new("./data/plugins")
    ));
    
    // Load registry
    plugin_registry.write().await.load().await.ok();
    
    // Load built-in plugins first
    load_builtin_plugins(&plugin_host, &plugin_registry).await;
    
    // Load all enabled plugins from registry
    let registry = plugin_registry.read().await;
    for plugin_meta in registry.list() {
        if plugin_meta.enabled && plugin_meta.wasm_path.exists() {
            let wasm_bytes = tokio::fs::read(&plugin_meta.wasm_path).await.ok();
            if let Some(bytes) = wasm_bytes {
                plugin_host.install_plugin(bytes).await.ok();
            }
        }
    }
    drop(registry);
    
    // Initialize command registry and load plugin manifests
    let mut command_registry = CommandRegistry::new();
    // Load manifests from plugins directory
    if let Err(e) = command_registry.load_all_from_directory("./plugins").await {
        eprintln!("Warning: Failed to load some plugin manifests: {e}");
    }
    println!("Loaded commands: {:?}", command_registry.get_all_commands());
    let command_registry = Arc::new(RwLock::new(command_registry));
    
    // Initialize v2 command registry
    let mut command_registry_v2 = CommandRegistryV2::new();
    // TODO: Load manifests from v2 format plugins
    // For now, manually add some test data
    command_registry_v2.add_entity(EntityV2 {
        name: "invoice".to_string(),
        plural: "invoices".to_string(),
        description: "Sales invoice or bill".to_string(),
        aliases: vec!["bill".to_string(), "charge".to_string()],
    });
    command_registry_v2.add_entity(EntityV2 {
        name: "client".to_string(),
        plural: "clients".to_string(),
        description: "Customer or client".to_string(),
        aliases: vec!["customer".to_string()],
    });
    command_registry_v2.add_entity(EntityV2 {
        name: "payment".to_string(),
        plural: "payments".to_string(),
        description: "Payment record".to_string(),
        aliases: vec![],
    });
    command_registry_v2.add_entity(EntityV2 {
        name: "product".to_string(),
        plural: "products".to_string(),
        description: "Product or service".to_string(),
        aliases: vec!["service".to_string(), "item".to_string()],
    });
    let command_registry_v2 = Arc::new(RwLock::new(command_registry_v2));
    
    // Initialize validator and session manager
    let validator = Arc::new(TokenValidator::new());
    let session_manager = Arc::new(RwLock::new(
        taxtalk_core::validation::session::SessionManager::new()
    ));
    
    let state = AppState {
        plugin_host,
        plugin_registry,
        db_pool,
        validator,
        session_manager,
        command_registry,
        command_registry_v2,
    };
    
    // Build router
    let app = Router::new()
        // Plugin management
        .route("/api/plugins", get(list_plugins))
        .route("/api/plugins/install", post(install_plugin))
        .route("/api/plugins/{id}", delete(uninstall_plugin))
        .route("/api/plugins/{id}/enable", post(enable_plugin))
        .route("/api/plugins/{id}/disable", post(disable_plugin))
        .route("/api/plugins/{id}/schema", get(get_plugin_schema))
        
        // Command execution
        .route("/api/execute", post(execute_command))
        .route("/api/commands", get(list_commands))
        .route("/api/v2/commands", get(list_commands_v2))
        .route("/api/v2/parse", post(parse_input_v2))
        
        // Entity management
        .route("/api/entities/{type}", get(list_entities))
        .route("/api/entities/search", post(search_entities))
        .route("/api/search", get(search_entities_get))
        .route("/api/entity-suggestions", post(get_entity_type_suggestions))
        
        // Validation and guided sessions
        .route("/api/validate", post(validate_command))
        .route("/api/session/{id}", get(get_session))
        .route("/api/session/{id}/token", post(submit_session_token))
        .route("/api/session/{id}/execute", post(execute_session))
        .route("/api/session/{id}", delete(cancel_session))
        
        // NLP parsing
        .route("/api/nlp/parse", post(parse_natural_language))
        
        // Health check
        .route("/health", get(health))
        
        // Add CORS support
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        
        .with_state(state);
    
    // Get port from environment variable or default to 3000
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("127.0.0.1:{port}");
    println!("🚀 TaxTalk server starting on http://{addr}");
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn health() -> &'static str {
    "OK"
}

async fn list_plugins(State(state): State<AppState>) -> Json<Vec<PluginInfo>> {
    let plugins = state.plugin_host.list_plugins().await;
    let registry = state.plugin_registry.read().await;
    
    let plugin_infos: Vec<PluginInfo> = plugins
        .into_iter()
        .map(|p| {
            let metadata = registry.get(&p.id);
            let plugin_id = p.id.clone();
            PluginInfo {
                id: p.id,
                name: p.name,
                version: p.version,
                description: p.description,
                commands: p.commands.into_iter().map(|c| CommandInfo {
                    name: c.name,
                    aliases: c.aliases,
                    description: c.description,
                    plugin_id: plugin_id.clone(),
                    required_tokens: vec![],  // TODO: Add token info when available
                    optional_tokens: vec![],
                }).collect(),
                enabled: metadata.map(|m| m.enabled).unwrap_or(true),
            }
        })
        .collect();
    
    Json(plugin_infos)
}

async fn install_plugin(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<PluginInfo>, StatusCode> {
    // Get WASM file from multipart upload
    let mut wasm_bytes = Vec::new();
    
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        if field.name() == Some("wasm") {
            wasm_bytes = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
            break;
        }
    }
    
    if wasm_bytes.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Install the plugin
    let manifest = state.plugin_host
        .install_plugin(wasm_bytes.clone())
        .await
        .map_err(|e| {
            eprintln!("Failed to install plugin: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Register in registry
    let mut registry = state.plugin_registry.write().await;
    let wasm_path = registry.store_plugin_wasm(&manifest.id, &wasm_bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let metadata = taxtalk_core::plugin_registry::PluginMetadata {
        id: manifest.id.clone(),
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        description: manifest.description.clone(),
        wasm_path,
        installed_at: chrono::Utc::now(),
        enabled: true,
    };
    
    registry.register(metadata).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let manifest_id = manifest.id.clone();
    Ok(Json(PluginInfo {
        id: manifest.id,
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        commands: manifest.commands.into_iter().map(|c| CommandInfo {
            name: c.name,
            aliases: c.aliases,
            description: c.description,
            plugin_id: manifest_id.clone(),
            required_tokens: vec![],  // TODO: Add token info when available
            optional_tokens: vec![],
        }).collect(),
        enabled: true,
    }))
}

async fn uninstall_plugin(
    State(state): State<AppState>,
    Path(plugin_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Uninstall from host
    state.plugin_host
        .uninstall_plugin(&plugin_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    // Remove from registry
    let mut registry = state.plugin_registry.write().await;
    registry.unregister(&plugin_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}

async fn enable_plugin(
    State(state): State<AppState>,
    Path(plugin_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut registry = state.plugin_registry.write().await;
    
    // Get the plugin metadata
    let metadata = registry.get(&plugin_id)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();
    
    // If plugin exists and has WASM, load it
    if metadata.wasm_path.exists() {
        let wasm_bytes = tokio::fs::read(&metadata.wasm_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        state.plugin_host
            .install_plugin(wasm_bytes)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    registry.set_enabled(&plugin_id, true)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::OK)
}

async fn disable_plugin(
    State(state): State<AppState>,
    Path(plugin_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Unload from host
    state.plugin_host
        .uninstall_plugin(&plugin_id)
        .await
        .ok(); // Ignore if not loaded
    
    // Mark as disabled in registry
    let mut registry = state.plugin_registry.write().await;
    registry.set_enabled(&plugin_id, false)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(StatusCode::OK)
}

async fn get_plugin_schema(
    State(state): State<AppState>,
    Path(plugin_id): Path<String>,
) -> Result<String, StatusCode> {
    state.plugin_host
        .get_plugin_schema(&plugin_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn execute_command(
    State(state): State<AppState>,
    Json(payload): Json<ExecuteCommand>,
) -> Result<Json<ExecuteResponse>, StatusCode> {
    let result = state.plugin_host
        .execute_command(&payload.command)
        .await
        .map_err(|e| {
            eprintln!("Command execution failed: {e}");
            StatusCode::BAD_REQUEST
        })?;
    
    Ok(Json(ExecuteResponse {
        success: result.success,
        data: result.data,
        error: result.error,
    }))
}

async fn load_builtin_plugins(
    plugin_host: &Arc<PluginHost>,
    plugin_registry: &Arc<RwLock<PluginRegistry>>,
) {
    println!("Loading built-in plugins...");
    
    let plugins_dir = PathBuf::from("./plugins");
    if !plugins_dir.exists() {
        eprintln!("Plugins directory not found");
        return;
    }
    
    // List of built-in plugins to load
    let builtin_plugins = vec!["company", "invoice", "payments"];
    
    for plugin_name in builtin_plugins {
        let plugin_dir = plugins_dir.join(plugin_name);
        if !plugin_dir.exists() {
            eprintln!("Plugin directory not found: {plugin_dir:?}");
            continue;
        }
        
        // Look for compiled WASM file (try different naming patterns)
        let possible_names = vec![
            format!("{}_plugin.wasm", plugin_name.replace('-', "_")),
            format!("{}.wasm", plugin_name.replace('-', "_")),
            format!("lib{}.wasm", plugin_name.replace('-', "_")),
        ];
        
        let mut found = false;
        for wasm_name in &possible_names {
            let wasm_path = plugin_dir
                .join("target")
                .join("wasm32-wasip2")
                .join("release")
                .join(wasm_name);
            
            if wasm_path.exists() {
                load_plugin_from_path(plugin_host, plugin_registry, &wasm_path, plugin_name).await;
                found = true;
                break;
            }
        }
        
        if !found {
            eprintln!("WASM file not found for {plugin_name}");
            eprintln!("Tried: {possible_names:?}");
            eprintln!("Please run ./build_plugins.sh to compile plugins");
        }
    }
}

async fn load_plugin_from_path(
    plugin_host: &Arc<PluginHost>,
    plugin_registry: &Arc<RwLock<PluginRegistry>>,
    wasm_path: &PathBuf,
    plugin_name: &str,
) {
    match tokio::fs::read(&wasm_path).await {
        Ok(wasm_bytes) => {
            // For now, create a mock manifest since we can't call WASM functions yet
            // In a real implementation, we'd get this from the plugin
            let mock_manifest = taxtalk_core::plugin_host::PluginManifest {
                id: plugin_name.to_string(),
                name: format!("{} Plugin", capitalize_first(plugin_name)),
                version: "0.1.0".to_string(),
                description: format!("Built-in {plugin_name} management plugin"),
                commands: vec![
                    taxtalk_core::plugin_host::CommandDef {
                        name: format!("create-{plugin_name}"),
                        aliases: vec![format!("new-{}", plugin_name)],
                        description: format!("Create a new {plugin_name}"),
                        examples: vec![format!("create {} ABC Corp", plugin_name)],
                    },
                    taxtalk_core::plugin_host::CommandDef {
                        name: format!("list-{plugin_name}s"),
                        aliases: vec![format!("show-{}s", plugin_name)],
                        description: format!("List all {plugin_name}s"),
                        examples: vec![format!("list {}s", plugin_name)],
                    },
                ],
                schema: "{}".to_string(), // Empty JSON schema for now
            };
            
            // Register the plugin with its WASM bytes
            {
                let mut plugins = plugin_host.plugins.write().await;
                plugins.insert(
                    plugin_name.to_string(),
                    taxtalk_core::plugin_host::LoadedPlugin {
                        manifest: mock_manifest.clone(),
                        wasm_bytes: wasm_bytes.clone(),
                    },
                );
            }
            
            // Register commands with router
            {
                let mut router = plugin_host.router.write().await;
                for command in &mock_manifest.commands {
                    router.register_command(&mock_manifest.id, &command.name, &command.aliases);
                }
            }
            
            // Store in registry
            let mut registry = plugin_registry.write().await;
            let stored_path = registry
                .store_plugin_wasm(plugin_name, &wasm_bytes)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Failed to store plugin WASM: {e}");
                    wasm_path.clone()
                });
            
            let metadata = taxtalk_core::plugin_registry::PluginMetadata {
                id: plugin_name.to_string(),
                name: mock_manifest.name,
                version: mock_manifest.version,
                description: mock_manifest.description,
                wasm_path: stored_path,
                installed_at: chrono::Utc::now(),
                enabled: true,
            };
            
            if let Err(e) = registry.register(metadata).await {
                eprintln!("Failed to register plugin {plugin_name}: {e}");
            } else {
                println!("✓ Loaded built-in plugin: {plugin_name}");
            }
        }
        Err(e) => {
            eprintln!("Failed to read WASM file for {plugin_name}: {e}");
        }
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

async fn list_entities(
    State(state): State<AppState>,
    Path(entity_type): Path<String>,
) -> Result<Json<Vec<Entity>>, StatusCode> {
    use taxtalk_core::repositories::{ClientRepository, InvoiceRepository, PaymentRepository, ProductRepository};
    use taxtalk_core::database::Repository;
    
    let mut entities = Vec::new();
    
    match entity_type.as_str() {
        "client" | "customer" => {
            let repo = ClientRepository::new(state.db_pool.clone());
            let clients = repo.list(Some(50), None).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            for client in clients {
                entities.push(Entity {
                    id: client.id.clone(),
                    name: client.name.clone(),
                    entity_type: "client".to_string(),
                    description: client.email.clone(),
                    metadata: Some(serde_json::json!({
                        "email": client.email,
                        "phone": client.phone,
                        "address": client.address,
                        "tin": client.tin,
                        "type": client.client_type
                    })),
                });
            }
        },
        "invoice" => {
            let repo = InvoiceRepository::new(state.db_pool.clone());
            let invoices = repo.list(Some(50), None).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            for invoice in invoices {
                entities.push(Entity {
                    id: invoice.id.clone(),
                    name: format!("{} - ${:.2}", invoice.invoice_number, invoice.total_amount),
                    entity_type: "invoice".to_string(),
                    description: Some(format!("Invoice #{}", invoice.invoice_number)),
                    metadata: Some(serde_json::json!({
                        "invoice_number": invoice.invoice_number,
                        "amount": invoice.amount,
                        "vat": invoice.vat_amount,
                        "total": invoice.total_amount,
                        "status": invoice.status,
                        "date": invoice.invoice_date.to_string()
                    })),
                });
            }
        },
        "payment" => {
            let repo = PaymentRepository::new(state.db_pool.clone());
            let payments = repo.list(Some(50), None).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            for payment in payments {
                entities.push(Entity {
                    id: payment.id.clone(),
                    name: format!("{} - ${:.2}", payment.payment_number, payment.amount),
                    entity_type: "payment".to_string(),
                    description: Some(format!("Payment #{}", payment.payment_number)),
                    metadata: Some(serde_json::json!({
                        "payment_number": payment.payment_number,
                        "amount": payment.amount,
                        "method": payment.payment_method,
                        "date": payment.payment_date.to_string(),
                        "reference": payment.reference_number
                    })),
                });
            }
        },
        "product" => {
            let repo = ProductRepository::new(state.db_pool.clone());
            let products = repo.list(Some(50), None).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            for product in products {
                entities.push(Entity {
                    id: product.id.clone(),
                    name: product.name.clone(),
                    entity_type: "product".to_string(),
                    description: product.description.clone(),
                    metadata: Some(serde_json::json!({
                        "unit_price": product.unit_price,
                        "unit": product.unit,
                        "vat_type": product.vat_type,
                        "category": product.category
                    })),
                });
            }
        },
        _ => {
            // Return empty array for unknown entity types
        }
    }
    
    Ok(Json(entities))
}

async fn search_entities_get(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<SearchQuery>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let entities = search_entities_impl(state, params).await?;
    Ok(Json(SearchResponse { results: entities.0 }))
}

async fn search_entities(
    State(state): State<AppState>,
    Json(query): Json<SearchQuery>,
) -> Result<Json<Vec<Entity>>, StatusCode> {
    search_entities_impl(state, query).await
}

async fn search_entities_impl(
    state: AppState,
    query: SearchQuery,
) -> Result<Json<Vec<Entity>>, StatusCode> {
    use taxtalk_core::repositories::{ClientRepository, InvoiceRepository, PaymentRepository, ProductRepository};
    use taxtalk_core::database::Repository;
    
    let mut results = Vec::new();
    
    // Search clients if no type specified or type is "client"
    if query.entity_type.is_none() || query.entity_type.as_deref() == Some("client") {
        let repo = ClientRepository::new(state.db_pool.clone());
        let clients = repo.search(&query.query).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for client in clients {
            results.push(Entity {
                id: client.id.clone(),
                name: client.name.clone(),
                entity_type: "client".to_string(),
                description: client.email.clone(),
                metadata: Some(serde_json::json!({
                    "email": client.email,
                    "phone": client.phone,
                    "tin": client.tin
                })),
            });
        }
    }
    
    // Search invoices if no type specified or type is "invoice"
    if query.entity_type.is_none() || query.entity_type.as_deref() == Some("invoice") {
        let repo = InvoiceRepository::new(state.db_pool.clone());
        let invoices = repo.search(&query.query).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for invoice in invoices {
            results.push(Entity {
                id: invoice.id.clone(),
                name: format!("{} - ${:.2}", invoice.invoice_number, invoice.total_amount),
                entity_type: "invoice".to_string(),
                description: Some(format!("Invoice #{}", invoice.invoice_number)),
                metadata: Some(serde_json::json!({
                    "amount": invoice.total_amount,
                    "status": invoice.status,
                    "date": invoice.invoice_date.to_string()
                })),
            });
        }
    }
    
    // Search payments if no type specified or type is "payment"
    if query.entity_type.is_none() || query.entity_type.as_deref() == Some("payment") {
        let repo = PaymentRepository::new(state.db_pool.clone());
        let payments = repo.search(&query.query).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for payment in payments {
            results.push(Entity {
                id: payment.id.clone(),
                name: format!("{} - ${:.2}", payment.payment_number, payment.amount),
                entity_type: "payment".to_string(),
                description: Some(format!("Payment #{}", payment.payment_number)),
                metadata: Some(serde_json::json!({
                    "amount": payment.amount,
                    "method": payment.payment_method,
                    "date": payment.payment_date.to_string()
                })),
            });
        }
    }
    
    // Search products if no type specified or type is "product"
    if query.entity_type.is_none() || query.entity_type.as_deref() == Some("product") {
        let repo = ProductRepository::new(state.db_pool.clone());
        let products = repo.search(&query.query).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for product in products {
            results.push(Entity {
                id: product.id.clone(),
                name: product.name.clone(),
                entity_type: "product".to_string(),
                description: product.description.clone(),
                metadata: Some(serde_json::json!({
                    "unit_price": product.unit_price,
                    "unit": product.unit,
                    "vat_type": product.vat_type,
                    "category": product.category
                })),
            });
        }
    }
    
    Ok(Json(results))
}

// Generate UUID v5 for consistent entity IDs
#[allow(dead_code)]
fn generate_uuid_v5(namespace: &str, name: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    namespace.hash(&mut hasher);
    name.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Format as UUID-like string
    format!("{:08x}-{:04x}-5{:03x}-{:04x}-{:012x}",
        (hash >> 32) as u32,
        ((hash >> 16) & 0xFFFF) as u16,
        (hash & 0xFFF) as u16,
        ((hash >> 48) & 0xFFFF) as u16,
        hash & 0xFFFFFFFFFFFF
    )
}

// Validation and Session Management Endpoints

async fn validate_command(
    State(state): State<AppState>,
    Json(payload): Json<ValidateCommand>,
) -> Result<Json<ValidateResponse>, StatusCode> {
    // Validate the command
    let validation_result = state.validator
        .validate(&payload.command)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Create or update session if needed
    let session_info = if !validation_result.valid {
        let mut session_mgr = state.session_manager.write().await;
        
        // Check if we have an existing session
        let session = if let Some(session_id) = payload.session_id {
            session_mgr.get_session(&session_id).cloned()
        } else {
            // Create new session
            if let (Some(plugin), Some(action)) = (validation_result.plugin.clone(), validation_result.action.clone()) {
                // Get schema (hardcoded for now, should come from plugin)
                let schema = match (plugin.as_str(), action.as_str()) {
                    ("invoice", "create") => taxtalk_core::validation::token_schema::TokenSchema::invoice_create(),
                    ("payment", "create") => taxtalk_core::validation::token_schema::TokenSchema::payment_create(),
                    _ => return Err(StatusCode::BAD_REQUEST),
                };
                
                session_mgr.create_session(
                    payload.command.clone(),
                    plugin,
                    action,
                    &schema,
                ).ok()
            } else {
                None
            }
        };
        
        // Get session progress if we have a session
        session.map(|s| {
            let progress = session_mgr.get_progress(&s.id).unwrap_or_else(|_| {
                taxtalk_core::validation::session::SessionProgress {
                    session_id: s.id.clone(),
                    state: s.state.clone(),
                    collected_count: 0,
                    required_count: 0,
                    optional_count: 0,
                    percentage: 0,
                }
            });
            
            println!("[SERVER] Validate endpoint - Session {} progress: {}/{}", s.id, progress.collected_count, progress.required_count);
            
            SessionInfo {
                id: s.id,
                status: format!("{:?}", s.state),
                progress: progress.collected_count as u32,
                total_required: progress.required_count,
                next_prompt: validation_result.next_token.clone(),
            }
        })
    } else {
        None
    };
    
    // Convert validation result to response
    let response = ValidateResponse {
        valid: validation_result.valid,
        action: validation_result.action,
        entity: validation_result.entity,
        plugin: validation_result.plugin,
        missing_required: validation_result.missing_required
            .into_iter()
            .map(|t| serde_json::json!({
                "name": t.name,
                "prompt": t.definition.prompt,
                "type": format!("{:?}", t.definition.token_type),
                "input_type": format!("{:?}", t.definition.input_type),
            }))
            .collect(),
        missing_optional: validation_result.missing_optional
            .into_iter()
            .map(|t| serde_json::json!({
                "name": t.name,
                "prompt": t.definition.prompt,
                "type": format!("{:?}", t.definition.token_type),
                "input_type": format!("{:?}", t.definition.input_type),
            }))
            .collect(),
        provided_tokens: serde_json::to_value(validation_result.provided_tokens)
            .unwrap_or_else(|_| serde_json::json!({})),
        defaults_applied: serde_json::to_value(validation_result.defaults_applied)
            .unwrap_or_else(|_| serde_json::json!({})),
        session: session_info,
    };
    
    Ok(Json(response))
}

async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Json<GuidedSession>, StatusCode> {
    let session_mgr = state.session_manager.read().await;
    
    session_mgr.get_session(&session_id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn submit_session_token(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(payload): Json<SubmitToken>,
) -> Result<StatusCode, StatusCode> {
    use taxtalk_core::validation::validator::ProvidedToken;
    
    let mut session_mgr = state.session_manager.write().await;
    
    // Create ProvidedToken from the submitted value
    let provided_token = ProvidedToken {
        raw_value: payload.value.to_string(),
        parsed_value: payload.value,
        resolved_entity: None, // Would be resolved from database if needed
    };
    
    println!("[SERVER] Submitting token '{}' to session '{}'", payload.token, session_id);
    session_mgr.submit_token(&session_id, payload.token.clone(), provided_token)
        .map_err(|e| {
            eprintln!("[SERVER] Error submitting token: {e:?}");
            StatusCode::BAD_REQUEST
        })?;
    
    // Log the updated progress
    if let Ok(progress) = session_mgr.get_progress(&session_id) {
        println!("[SERVER] Session progress after token submission: {}/{}", progress.collected_count, progress.required_count);
    }
    
    Ok(StatusCode::OK)
}

async fn execute_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Json<ExecuteResponse>, StatusCode> {
    let mut session_mgr = state.session_manager.write().await;
    
    // Mark session as ready
    session_mgr.mark_ready(&session_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Execute the session
    let tokens = session_mgr.execute_session(&session_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // TODO: Actually execute the command with collected tokens
    // For now, return success with the collected data
    Ok(Json(ExecuteResponse {
        success: true,
        data: Some(serde_json::json!({
            "message": "Session executed successfully",
            "tokens": tokens,
        })),
        error: None,
    }))
}

async fn cancel_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut session_mgr = state.session_manager.write().await;
    
    session_mgr.cancel_session(&session_id)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(StatusCode::NO_CONTENT)
}

// NLP Parsing Endpoints

#[derive(Deserialize)]
struct NLPRequest {
    text: String,
}

#[derive(Serialize)]
struct NLPResponse {
    commands: Vec<ParsedCommand>,
    confidence: f32,
    suggestions: Vec<String>,
}

async fn parse_natural_language(
    Json(payload): Json<NLPRequest>,
) -> Result<Json<NLPResponse>, StatusCode> {
    let parser = NLPParser::new();
    
    // Parse the natural language text
    let commands = if payload.text.contains(',') || payload.text.contains(';') {
        parser.extract_mixed_command(&payload.text)
    } else {
        vec![parser.parse_command(&payload.text)]
    };
    
    // Calculate overall confidence
    let confidence = if !commands.is_empty() {
        commands.iter().map(|c| c.confidence).sum::<f32>() / commands.len() as f32
    } else {
        0.0
    };
    
    // Generate suggestions based on what's missing
    let mut suggestions = Vec::new();
    for command in &commands {
        if command.action.is_none() {
            suggestions.push("Try starting with: 'create invoice', 'record payment', or 'record expense'".to_string());
        }
        
        let has_client = command.tokens.iter().any(|t| t.name == "client");
        let has_amount = command.tokens.iter().any(|t| t.name == "amount");
        
        if !has_client && command.action.as_deref() == Some("create_invoice") {
            suggestions.push("Add client name: 'for ABC Corporation'".to_string());
        }
        if !has_amount {
            suggestions.push("Add amount: '5000 pesos' or '₱5,000'".to_string());
        }
    }
    
    Ok(Json(NLPResponse {
        commands,
        confidence,
        suggestions,
    }))
}

// Get context-aware entity type suggestions based on the command
async fn get_entity_type_suggestions(
    state: State<AppState>,
    Json(payload): Json<EntityTypeSuggestionsQuery>,
) -> Result<Json<Vec<EntityTypeSuggestion>>, StatusCode> {
    // Get the text up to cursor position
    let command_text = &payload.command[..payload.cursor_position.min(payload.command.len())];
    let command_lower = command_text.to_lowercase();
    
    // Log for debugging
    eprintln!("Entity suggestions request - command: '{}', cursor: {}, filled_tokens: {:?}", 
             command_text, payload.cursor_position, payload.filled_tokens);
    
    // Check which tokens are already filled
    let filled_types: Vec<String> = payload.filled_tokens.iter()
        .map(|t| t.token_type.clone())
        .collect();
    
    let mut suggestions = Vec::new();
    
    // Check if the command ends with @ or @ followed by partial text
    // This indicates user is looking for entity suggestions
    let requesting_entity = command_text.trim_end().ends_with('@') || 
                           command_text.contains("@ ");
    
    // Use CommandRegistry to check for valid commands
    let command_registry = state.command_registry.read().await;
    let has_valid_command = command_registry.has_valid_command(command_text);
    
    // If only @ is typed without any valid command context, return empty
    if !has_valid_command || command_text.trim() == "@" || command_text.trim() == "" {
        eprintln!("No valid command found before @, returning no suggestions");
        return Ok(Json(vec![]));
    }
    
    // Get the command definition to know what entities are required
    if let Some(cmd_def) = command_registry.get_command_for_text(command_text) {
        // Add entity suggestions based on command requirements
        for req in &cmd_def.requirements.required {
            if req.token_type == "entity_ref" {
                let entity_type = req.entity_type.clone().unwrap_or_else(|| {
                    // Infer entity type from description
                    if req.description.to_lowercase().contains("customer") || 
                       req.description.to_lowercase().contains("client") {
                        "client".to_string()
                    } else if req.description.to_lowercase().contains("supplier") ||
                              req.description.to_lowercase().contains("vendor") {
                        "supplier".to_string()
                    } else if req.description.to_lowercase().contains("product") {
                        "product".to_string()
                    } else if req.description.to_lowercase().contains("employee") {
                        "employee".to_string()
                    } else {
                        "entity".to_string()
                    }
                });
                
                // Don't suggest if already filled
                if !filled_types.contains(&entity_type) {
                    suggestions.push(EntityTypeSuggestion {
                        entity_type: entity_type.clone(),
                        label: format!("@{entity_type}"),
                        description: req.description.clone(),
                        cardinality: if req.multiple { "multiple".to_string() } else { "single".to_string() },
                        hint: Some(req.description.clone()),
                    });
                }
            }
        }
        
        // Also check optional tokens for entity refs
        for req in &cmd_def.requirements.optional {
            if req.token_type == "entity_ref" {
                let entity_type = req.entity_type.clone().unwrap_or_else(|| {
                    // Same inference logic
                    if req.description.to_lowercase().contains("customer") || 
                       req.description.to_lowercase().contains("client") {
                        "client".to_string()
                    } else if req.description.to_lowercase().contains("product") {
                        "product".to_string()
                    } else {
                        "entity".to_string()
                    }
                });
                
                if !filled_types.contains(&entity_type) {
                    suggestions.push(EntityTypeSuggestion {
                        entity_type: entity_type.clone(),
                        label: format!("@{entity_type}"),
                        description: req.description.clone(),
                        cardinality: if req.multiple { "multiple".to_string() } else { "single".to_string() },
                        hint: Some(format!("Optional: {}", req.description)),
                    });
                }
            }
        }
    }
    
    // Fallback to hardcoded suggestions if no command definition found
    // (keeping existing logic as fallback)
    if suggestions.is_empty() && (command_lower.contains("create invoice") || command_lower.contains("invoice")) {
        // For invoices, we need client (single cardinality)
        if !filled_types.contains(&"client".to_string()) {
            suggestions.push(EntityTypeSuggestion {
                entity_type: "client".to_string(),
                label: "@client".to_string(),
                description: "Select a client for the invoice".to_string(),
                cardinality: "single".to_string(),
                hint: Some("The customer receiving this invoice".to_string()),
            });
        }
        
        // Products can be multiple
        suggestions.push(EntityTypeSuggestion {
            entity_type: "product".to_string(),
            label: "@product".to_string(),
            description: "Add product/service to invoice".to_string(),
            cardinality: "multiple".to_string(),
            hint: Some("Products or services being invoiced".to_string()),
        });
    } else if command_lower.contains("payment") || command_lower.contains("paid") || command_lower.contains("record payment") {
        // For payments, we can have client or invoice (single cardinality each)
        if !filled_types.contains(&"client".to_string()) {
            suggestions.push(EntityTypeSuggestion {
                entity_type: "client".to_string(),
                label: "@client".to_string(),
                description: "Client who made the payment".to_string(),
                cardinality: "single".to_string(),
                hint: Some("The customer making the payment".to_string()),
            });
        }
        
        if !filled_types.contains(&"invoice".to_string()) {
            suggestions.push(EntityTypeSuggestion {
                entity_type: "invoice".to_string(),
                label: "@invoice".to_string(),
                description: "Invoice being paid".to_string(),
                cardinality: "single".to_string(),
                hint: Some("The invoice this payment is for".to_string()),
            });
        }
    } else if command_lower.contains("expense") || command_lower.contains("bought") || command_lower.contains("purchase") {
        // For expenses, we need supplier (single) and products (multiple)
        if !filled_types.contains(&"supplier".to_string()) {
            suggestions.push(EntityTypeSuggestion {
                entity_type: "supplier".to_string(),
                label: "@supplier".to_string(),
                description: "Supplier/vendor for the expense".to_string(),
                cardinality: "single".to_string(),
                hint: Some("The vendor or supplier".to_string()),
            });
        }
        
        suggestions.push(EntityTypeSuggestion {
            entity_type: "product".to_string(),
            label: "@product".to_string(),
            description: "Product or service purchased".to_string(),
            cardinality: "multiple".to_string(),
            hint: Some("Items or services purchased".to_string()),
        });
    } else if command_lower.contains("salary") || command_lower.contains("payroll") {
        // For payroll, we need employee (can be multiple for batch processing)
        suggestions.push(EntityTypeSuggestion {
            entity_type: "employee".to_string(),
            label: "@employee".to_string(),
            description: "Employee receiving payment".to_string(),
            cardinality: "multiple".to_string(),
            hint: Some("Employee(s) to process payroll for".to_string()),
        });
    } else if !requesting_entity && command_text.trim() == "" {
        // No command typed yet and not requesting entity, return empty array
        return Ok(Json(vec![]));
    } else if requesting_entity || command_text.trim() != "" {
        // Default suggestions if no clear context
        // Show the most common entity types
        if !filled_types.contains(&"client".to_string()) {
            suggestions.push(EntityTypeSuggestion {
                entity_type: "client".to_string(),
                label: "@client".to_string(),
                description: "Client/Customer".to_string(),
                cardinality: "single".to_string(),
                hint: None,
            });
        }
        
        if !filled_types.contains(&"supplier".to_string()) {
            suggestions.push(EntityTypeSuggestion {
                entity_type: "supplier".to_string(),
                label: "@supplier".to_string(),
                description: "Supplier/Vendor".to_string(),
                cardinality: "single".to_string(),
                hint: None,
            });
        }
        
        suggestions.push(EntityTypeSuggestion {
            entity_type: "product".to_string(),
            label: "@product".to_string(),
            description: "Product/Service".to_string(),
            cardinality: "multiple".to_string(),
            hint: None,
        });
    }
    
    Ok(Json(suggestions))
}

async fn list_commands(
    State(state): State<AppState>,
) -> Result<Json<Vec<CommandInfo>>, StatusCode> {
    let command_registry = state.command_registry.read().await;
    
    let mut commands = Vec::new();
    let mut seen = std::collections::HashSet::new();
    
    // Get all unique commands (avoiding duplicates from aliases)
    for command_str in command_registry.get_all_commands() {
        if let Some(cmd_def) = command_registry.get_command_for_text(&command_str) {
            // Only add each unique command once
            if !seen.contains(&cmd_def.command) {
                seen.insert(cmd_def.command.clone());
                
                commands.push(CommandInfo {
                    name: cmd_def.command.clone(),  // Changed from 'command' to 'name'
                    aliases: cmd_def.aliases.clone(),
                    description: cmd_def.description.clone(),
                    plugin_id: cmd_def.plugin_id.clone(),
                    required_tokens: cmd_def.requirements.required.iter().map(|req| TokenInfo {
                        token_type: req.token_type.clone(),
                        description: req.description.clone(),
                        multiple: req.multiple,
                        entity_type: req.entity_type.clone(),
                    }).collect(),
                    optional_tokens: cmd_def.requirements.optional.iter().map(|req| TokenInfo {
                        token_type: req.token_type.clone(),
                        description: req.description.clone(),
                        multiple: req.multiple,
                        entity_type: req.entity_type.clone(),
                    }).collect(),
                });
            }
        }
    }
    
    Ok(Json(commands))
}

// CommandInfo and TokenInfo structs moved here from duplicate location
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommandInfo {
    name: String,  // Changed from 'command' to match earlier usage
    aliases: Vec<String>,
    description: String,
    plugin_id: String,
    required_tokens: Vec<TokenInfo>,
    optional_tokens: Vec<TokenInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenInfo {
    token_type: String,
    description: String,
    multiple: bool,
    entity_type: Option<String>,
}

// V2 API structures
#[derive(Serialize)]
struct CommandsV2Response {
    commands: Vec<Command>,
    entities: Vec<EntityV2>,
}

#[derive(Deserialize)]
struct ParseInputRequest {
    text: String,
}

async fn list_commands_v2(
    State(state): State<AppState>,
) -> Result<Json<CommandsV2Response>, StatusCode> {
    let registry = state.command_registry_v2.read().await;
    
    Ok(Json(CommandsV2Response {
        commands: registry.get_all_commands(),
        entities: registry.get_all_entities(),
    }))
}

async fn parse_input_v2(
    State(state): State<AppState>,
    Json(payload): Json<ParseInputRequest>,
) -> Result<Json<ParseResult>, StatusCode> {
    let registry = state.command_registry_v2.read().await;
    let result = registry.parse_input(&payload.text);
    Ok(Json(result))
}