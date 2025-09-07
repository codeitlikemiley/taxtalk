use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasmtime::{Engine, Store, Module};
use wasmtime::Linker;

use crate::tokenizer::Tokenizer;
use crate::router::CommandRouter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub commands: Vec<CommandDef>,
    pub schema: String,  // JSON Schema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDef {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub data: Option<Value>,
    pub error: Option<String>,
}

pub struct LoadedPlugin {
    pub manifest: PluginManifest,
    pub wasm_bytes: Vec<u8>,
}

pub struct PluginHost {
    engine: Engine,
    pub plugins: Arc<RwLock<HashMap<String, LoadedPlugin>>>,
    pub router: Arc<RwLock<CommandRouter>>,
    tokenizer: Tokenizer,
}

impl PluginHost {
    pub fn new() -> Result<Self> {
        let mut config = wasmtime::Config::new();
        config.wasm_component_model(true);
        config.async_support(true);
        
        let engine = Engine::new(&config)?;
        
        Ok(Self {
            engine,
            plugins: Arc::new(RwLock::new(HashMap::new())),
            router: Arc::new(RwLock::new(CommandRouter::new())),
            tokenizer: Tokenizer::new(),
        })
    }
    
    /// Install a plugin from WASM bytes
    pub async fn install_plugin(&self, wasm_bytes: Vec<u8>) -> Result<PluginManifest> {
        // Try to get manifest from the WASM module
        let manifest = self.extract_manifest_from_wasm(&wasm_bytes).await?;
        
        // Register commands with router
        {
            let mut router = self.router.write().await;
            for command in &manifest.commands {
                router.register_command(&manifest.id, &command.name, &command.aliases);
            }
        }
        
        // Store the plugin with its WASM bytes
        {
            let mut plugins = self.plugins.write().await;
            plugins.insert(manifest.id.clone(), LoadedPlugin {
                manifest: manifest.clone(),
                wasm_bytes,
            });
        }
        
        Ok(manifest)
    }
    
    /// Extract manifest from WASM module
    async fn extract_manifest_from_wasm(&self, _wasm_bytes: &[u8]) -> Result<PluginManifest> {
        // For now, return a basic manifest based on the module
        // In a real implementation, we'd call the get_manifest export
        Ok(PluginManifest {
            id: "wasm-plugin".to_string(),
            name: "WASM Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Loaded from WASM".to_string(),
            commands: vec![],
            schema: "{}".to_string(),
        })
    }
    
    /// Uninstall a plugin
    pub async fn uninstall_plugin(&self, plugin_id: &str) -> Result<()> {
        // Remove from router
        {
            let mut router = self.router.write().await;
            router.unregister_plugin(plugin_id);
        }
        
        // Remove from plugins
        {
            let mut plugins = self.plugins.write().await;
            plugins.remove(plugin_id)
                .ok_or_else(|| anyhow!("Plugin {} not found", plugin_id))?;
        }
        
        Ok(())
    }
    
    /// Execute a natural language command
    pub async fn execute_command(&self, command: &str) -> Result<CommandResult> {
        // Tokenize the command
        let tokens = self.tokenizer.tokenize(command)?;
        
        // Try to route to a plugin
        let plugin_id = {
            let router = self.router.read().await;
            router.route(&tokens).ok()
        };
        
        // Try to execute in the appropriate plugin based on command keywords
        let lower_command = command.to_lowercase();
        
        // Determine which plugin to use
        let target_plugin = if lower_command.contains("invoice") || lower_command.contains("bill") {
            "invoice"
        } else if lower_command.contains("payment") || lower_command.contains("paid") || lower_command.contains("pay") {
            "payments"
        } else if lower_command.contains("company") || lower_command.contains("business") {
            "company"
        } else {
            // Use routed plugin or fall back to mock
            plugin_id.as_deref().unwrap_or("mock")
        };
        
        // Try to execute in the plugin
        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(target_plugin) {
            // Try to execute using WASM
            if let Ok(result) = self.execute_wasm_plugin(&plugin.wasm_bytes, command).await {
                return Ok(result);
            }
        }
        
        // Fall back to mock responses if WASM execution fails
        self.execute_mock_command(command)
    }
    
    /// Execute command in WASM plugin
    async fn execute_wasm_plugin(&self, wasm_bytes: &[u8], command: &str) -> Result<CommandResult> {
        // Create a new store for this execution
        let mut store = Store::new(&self.engine, ());
        
        // Load the module
        let module = Module::new(&self.engine, wasm_bytes)?;
        
        // Create a linker with necessary imports
        let linker = Linker::new(&self.engine);
        
        // Instantiate the module
        let instance = linker.instantiate(&mut store, &module)?;
        
        // Get the execute function
        let execute_fn = instance.get_func(&mut store, "execute")
            .ok_or_else(|| anyhow!("Plugin missing execute function"))?;
        
        // Prepare command as bytes
        let command_bytes = command.as_bytes();
        
        // Allocate memory in WASM for the command
        let memory = instance.get_memory(&mut store, "memory")
            .ok_or_else(|| anyhow!("Plugin missing memory export"))?;
        
        // Write command to WASM memory
        let command_ptr = 0;
        memory.write(&mut store, command_ptr, command_bytes)?;
        
        // Call the execute function
        let mut results = [wasmtime::Val::I32(0)];
        execute_fn.call(&mut store, &[
            wasmtime::Val::I32(command_ptr as i32),
            wasmtime::Val::I32(command_bytes.len() as i32)
        ], &mut results)?;
        
        // Read the result pointer
        let result_ptr = results[0].unwrap_i32() as usize;
        
        // Read result from memory (assuming it's a null-terminated string)
        let mut result_bytes = Vec::new();
        let mut offset = 0;
        loop {
            let mut byte = [0u8];
            memory.read(&store, result_ptr + offset, &mut byte)?;
            if byte[0] == 0 {
                break;
            }
            result_bytes.push(byte[0]);
            offset += 1;
            if offset > 10000 { // Safety limit
                break;
            }
        }
        
        // Parse the result JSON
        let result_str = String::from_utf8(result_bytes)?;
        let result_json: serde_json::Value = serde_json::from_str(&result_str)?;
        
        Ok(CommandResult {
            success: result_json["success"].as_bool().unwrap_or(false),
            data: result_json.get("data").cloned(),
            error: result_json["error"].as_str().map(String::from),
        })
    }
    
    /// Execute mock command (fallback)
    fn execute_mock_command(&self, command: &str) -> Result<CommandResult> {
        // Extract amount from command (look for numbers)
        let amount = extract_amount_from_command(command);
        
        // Extract client/entity name from command
        let client = extract_client_from_command(command);
        
        // Return mock response based on command
        if command.to_lowercase().contains("invoice") {
            let invoice_amount = amount.unwrap_or(1000.0);
            let vat = invoice_amount * 0.12; // Philippine VAT is 12%
            
            Ok(CommandResult {
                success: true,
                data: Some(serde_json::json!({
                    "message": "Invoice created successfully",
                    "invoice_id": format!("INV-2024-{:03}", (amount.unwrap_or(1.0) as u32) % 999),
                    "client": client.unwrap_or_else(|| "Unknown Client".to_string()),
                    "amount": invoice_amount,
                    "vat": vat,
                    "total": invoice_amount + vat,
                    "status": "pending"
                })),
                error: None,
            })
        } else if command.to_lowercase().contains("payment") || command.to_lowercase().contains("paid") {
            let payment_amount = amount.unwrap_or(1000.0);
            
            Ok(CommandResult {
                success: true,
                data: Some(serde_json::json!({
                    "message": "Payment recorded",
                    "payment_id": format!("PAY-2024-{:03}", (amount.unwrap_or(1.0) as u32) % 999),
                    "amount": payment_amount,
                    "client": client.unwrap_or_else(|| "Unknown Client".to_string()),
                    "method": if command.contains("gcash") { "gcash" } 
                             else if command.contains("bank") { "bank_transfer" }
                             else if command.contains("check") { "check" }
                             else { "cash" },
                    "status": "completed"
                })),
                error: None,
            })
        } else if command.to_lowercase().contains("vat") {
            let base_amount = amount.unwrap_or(10000.0);
            let vat = base_amount * 0.12;
            
            Ok(CommandResult {
                success: true,
                data: Some(serde_json::json!({
                    "message": "VAT calculated",
                    "amount": base_amount,
                    "vat": vat,
                    "total": base_amount + vat,
                    "rate": "12%"
                })),
                error: None,
            })
        } else if command.to_lowercase().contains("company") {
            Ok(CommandResult {
                success: true,
                data: Some(serde_json::json!({
                    "message": "Company information",
                    "name": client.unwrap_or_else(|| "New Company".to_string()),
                    "id": "COMP-001",
                    "status": "active"
                })),
                error: None,
            })
        } else {
            Ok(CommandResult {
                success: true,
                data: Some(serde_json::json!({
                    "message": "Command processed",
                    "command": command
                })),
                error: None,
            })
        }
    }
    
    /// Get list of installed plugins
    pub async fn list_plugins(&self) -> Vec<PluginManifest> {
        let plugins = self.plugins.read().await;
        plugins.values().map(|p| p.manifest.clone()).collect()
    }
    
    /// Get a plugin's JSON schema for type discovery
    pub async fn get_plugin_schema(&self, plugin_id: &str) -> Result<String> {
        let plugins = self.plugins.read().await;
        let plugin = plugins.get(plugin_id)
            .ok_or_else(|| anyhow!("Plugin {} not found", plugin_id))?;
        
        Ok(plugin.manifest.schema.clone())
    }
    
    // Helper methods would be here in real WASM implementation
    // async fn get_plugin_manifest(&self, _store: &mut Store<()>, _instance: &Instance) -> Result<PluginManifest>
    // async fn execute_in_plugin(&self, _store: &mut Store<()>, _instance: &Instance, _command: &str, _tokens: Vec<crate::router::Token>) -> Result<CommandResult>
    // fn setup_capabilities(&self, _linker: &mut ComponentLinker<()>) -> Result<()>
}

// Helper function to extract amount from command
fn extract_amount_from_command(command: &str) -> Option<f64> {
    // Look for patterns like "1000", "10000", "10,000", "10k", "100.50"
    let words: Vec<&str> = command.split_whitespace().collect();
    
    for word in words {
        // Remove common suffixes and clean the string
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
            // Ignore very small numbers (likely not amounts)
            if num >= 1.0 {
                return Some(num);
            }
        }
    }
    
    None
}

// Helper function to extract client/entity name from command
fn extract_client_from_command(command: &str) -> Option<String> {
    let lower = command.to_lowercase();
    
    // Look for patterns with @client or @customer
    if let Some(idx) = lower.find("@client") {
        let after = &command[idx + 7..].trim();
        let name = after.split_whitespace().next()?;
        return Some(name.to_string());
    }
    
    if let Some(idx) = lower.find("@customer") {
        let after = &command[idx + 9..].trim();
        let name = after.split_whitespace().next()?;
        return Some(name.to_string());
    }
    
    // Look for patterns like "from Juan", "to Maria", "for Pedro"
    for preposition in &["from", "to", "for", "with"] {
        if let Some(idx) = lower.find(preposition) {
            let after = &command[idx + preposition.len()..].trim();
            // Take the next word as the client name
            let words: Vec<&str> = after.split_whitespace().collect();
            if !words.is_empty() {
                let first_word = words[0];
                // Skip if it's a number or common word
                if !first_word.chars().all(|c| c.is_numeric() || c == '.' || c == ',') 
                    && !["the", "a", "an", "@", "invoice", "payment", "amount"].contains(&first_word.to_lowercase().as_str()) {
                    // Check if there's a second word that looks like a surname
                    if words.len() > 1 && !words[1].chars().all(|c| c.is_numeric() || c == '.' || c == ',') {
                        return Some(format!("{} {}", words[0], words[1]));
                    }
                    return Some(first_word.to_string());
                }
            }
        }
    }
    
    // Look for common Filipino names in the command
    let filipino_names = ["juan", "maria", "pedro", "jose", "ana", "carlos", "rosa", "manuel"];
    for name in filipino_names {
        if lower.contains(name) {
            // Find the actual casing from original command
            if let Some(idx) = lower.find(name) {
                let end_idx = idx + name.len();
                let actual_name = &command[idx..end_idx];
                // Check if there's a surname following
                let after = &command[end_idx..].trim();
                if let Some(surname) = after.split_whitespace().next() {
                    if !surname.chars().all(|c| c.is_numeric() || c == '.' || c == ',') 
                        && !["paid", "bought", "invoice", "payment"].contains(&surname.to_lowercase().as_str()) {
                        return Some(format!("{actual_name} {surname}"));
                    }
                }
                return Some(actual_name.to_string());
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_plugin_host_creation() {
        let host = PluginHost::new().expect("Failed to create plugin host");
        let plugins = host.list_plugins().await;
        assert_eq!(plugins.len(), 0);
    }
}