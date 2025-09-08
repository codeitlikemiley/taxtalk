use anyhow::Result;
use std::path::Path;
use taxtalk_core::plugin::wit_loader::WitPluginLoader;

fn main() -> Result<()> {
    println!("Testing WIT-based WASM plugins...\n");
    
    // Create plugin loader
    let mut loader = WitPluginLoader::new()?;
    
    // Load invoice plugin
    let invoice_path = Path::new("data/plugins/plugins/invoice.wasm");
    if invoice_path.exists() {
        println!("Loading invoice plugin...");
        loader.load_plugin("invoice", invoice_path)?;
    }
    
    // Load company plugin
    let company_path = Path::new("data/plugins/plugins/company.wasm");
    if company_path.exists() {
        println!("Loading company plugin...");
        loader.load_plugin("company", company_path)?;
    }
    
    // List loaded plugins
    println!("\nLoaded plugins:");
    for plugin_id in loader.list_plugins() {
        println!("  - {}", plugin_id);
    }
    
    // Test invoice plugin
    println!("\nTesting invoice plugin:");
    let result = loader.execute_command(
        "invoice",
        "create invoice for ABC Corp 10000",
        vec![]
    )?;
    println!("  Result: {:?}", result);
    
    // Test company plugin
    println!("\nTesting company plugin:");
    let result = loader.execute_command(
        "company",
        "create company TaxTalk Corp with TIN 123456789",
        vec![]
    )?;
    println!("  Result: {:?}", result);
    
    println!("\n✅ All plugins tested successfully!");
    
    Ok(())
}