use anyhow::{Result, Context};
use wasmtime::{Engine, Config, Store};
use wasmtime::component::{Component, Linker};
use std::path::Path;

fn main() -> Result<()> {
    println!("=== Debugging Plugin Load Issues ===\n");
    
    // Create engine with detailed configuration
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(false);
    
    // Enable WASI preview2 support
    config.wasm_component_model(true);
    
    let engine = Engine::new(&config)?;
    println!("✅ Engine created with component model support\n");
    
    // Test loading invoice plugin (built with cargo-component)
    let wasm_path = Path::new("plugins/invoice/target/wasm32-wasip1/release/invoice_plugin.wasm");
    println!("Loading plugin from: {:?}", wasm_path);
    
    // Load the component
    let component = Component::from_file(&engine, wasm_path)
        .context("Failed to load WASM component")?;
    println!("✅ Component loaded successfully\n");
    
    // Create store
    let mut store = Store::new(&engine, ());
    println!("✅ Store created\n");
    
    // Create linker
    let linker = Linker::new(&engine);
    println!("✅ Linker created\n");
    
    // Try to instantiate without any imports first
    println!("Attempting to instantiate component...");
    match linker.instantiate(&mut store, &component) {
        Ok(_instance) => {
            println!("✅ Component instantiated successfully!");
        }
        Err(e) => {
            println!("❌ Instantiation failed: {}", e);
            println!("\nDetailed error chain:");
            for cause in e.chain() {
                println!("  - {}", cause);
            }
            
            // Try to understand what imports are missing
            println!("\nAnalyzing component requirements...");
            
            // Get component exports and imports info
            let component_type = component.component_type();
            
            println!("\nComponent imports:");
            for (name, _) in component_type.imports(&engine) {
                println!("  - {}", name);
            }
            
            println!("\nComponent exports:");
            for (name, _) in component_type.exports(&engine) {
                println!("  - {}", name);
            }
        }
    }
    
    Ok(())
}