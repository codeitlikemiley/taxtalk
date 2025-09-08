use anyhow::{Result, Context};
use wasmtime::{Engine, Config};
use std::path::Path;
use taxtalk_core::plugin::host_bindings::{WasmPlugin, PluginManager};

fn main() -> Result<()> {
    println!("=== Testing Complete Plugin System ===\n");
    
    // Create engine with component model support
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(false);
    
    let engine = Engine::new(&config)?;
    println!("✅ Engine created with component model support\n");
    
    // Test loading invoice plugin
    let wasm_path = Path::new("plugins/invoice/target/wasm32-wasip1/release/invoice_plugin.wasm");
    println!("Loading plugin from: {:?}", wasm_path);
    
    // Load the plugin using our WasmPlugin implementation
    let mut plugin = WasmPlugin::load(&engine, "invoice", wasm_path)
        .context("Failed to load invoice plugin")?;
    
    println!("✅ Plugin loaded successfully!\n");
    
    // Get and display manifest
    println!("Getting plugin manifest...");
    let manifest = plugin.get_manifest()?;
    println!("✅ Manifest retrieved:");
    println!("  ID: {}", manifest.id);
    println!("  Name: {}", manifest.name);
    println!("  Version: {}", manifest.version);
    println!("  Description: {}", manifest.description);
    println!("  Commands: {} available", manifest.commands.len());
    for cmd in &manifest.commands {
        println!("    - {}: {}", cmd.name, cmd.description);
    }
    println!();
    
    // Test executing a command
    println!("Testing command execution...");
    let tokens = vec![];
    let result = plugin.execute("calculate vat for 1000", tokens)?;
    println!("✅ Command executed:");
    println!("  Success: {}", result.success);
    if let Some(data) = result.data {
        println!("  Data: {}", data);
    }
    if let Some(error) = result.error {
        println!("  Error: {}", error);
    }
    println!();
    
    // Get schema
    println!("Getting plugin schema...");
    let schema = plugin.get_schema()?;
    println!("✅ Schema retrieved (length: {} chars)", schema.len());
    println!();
    
    // Test plugin manager
    println!("Testing PluginManager...");
    let mut manager = PluginManager::new()?;
    
    // Load invoice plugin
    manager.load_plugin("invoice", wasm_path)?;
    
    // Also try loading company plugin if it exists
    let company_path = Path::new("plugins/company/target/wasm32-wasip1/release/company_plugin.wasm");
    if company_path.exists() {
        manager.load_plugin("company", company_path)?;
        println!("✅ Company plugin also loaded");
    }
    
    // List loaded plugins
    let plugins = manager.list_plugins();
    println!("✅ Loaded plugins: {:?}", plugins);
    
    println!("\n=== All tests passed! ===");
    Ok(())
}