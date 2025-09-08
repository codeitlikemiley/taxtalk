use anyhow::{Result, Context};
use wasmtime::{Engine, Config};
use std::path::Path;
use taxtalk_core::plugin::host_bindings::{WasmPlugin, PluginManager};

fn main() -> Result<()> {
    println!("=== TaxTalk Plugin System Demo ===\n");
    
    // Create engine
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(false);
    
    let engine = Engine::new(&config)?;
    
    // Load plugins
    let mut invoice_plugin = WasmPlugin::load(
        &engine, 
        "invoice", 
        Path::new("plugins/invoice/target/wasm32-wasip1/release/invoice_plugin.wasm")
    )?;
    
    let mut company_plugin = WasmPlugin::load(
        &engine,
        "company",
        Path::new("plugins/company/target/wasm32-wasip1/release/company_plugin.wasm")
    )?;
    
    println!("✅ Plugins loaded successfully!\n");
    
    // Demo 1: Create a company
    println!("📦 Creating a company...");
    let result = company_plugin.execute(
        "create company ABC Corporation with TIN 123456789",
        vec![]
    )?;
    
    if result.success {
        println!("✅ Company created successfully!");
        if let Some(data) = result.data {
            println!("   {}", data);
        }
    }
    println!();
    
    // Demo 2: Calculate VAT for an invoice
    println!("💰 Calculating VAT for an invoice...");
    let result = invoice_plugin.execute(
        "calculate vat for 50000",
        vec![]
    )?;
    
    if result.success {
        println!("✅ VAT calculated:");
        if let Some(data) = result.data {
            // Parse and display nicely
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                println!("   Net Amount: ₱{}", json["net_amount"]);
                println!("   VAT (12%): ₱{}", json["vat_amount"]);  
                println!("   Total Amount: ₱{}", json["total_amount"]);
            }
        }
    }
    println!();
    
    // Demo 3: Create an invoice
    println!("📄 Creating an invoice...");
    let result = invoice_plugin.execute(
        "create invoice for ABC Corporation 50000",
        vec![]
    )?;
    
    if result.success {
        println!("✅ Invoice created:");
        if let Some(data) = result.data {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                println!("   {}", json["message"]);
            }
        }
    }
    println!();
    
    // Demo 4: List companies
    println!("🏢 Listing companies...");
    let result = company_plugin.execute(
        "list companies",
        vec![]
    )?;
    
    if result.success {
        if let Some(data) = result.data {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                println!("✅ Companies found: {}", json["count"]);
                if let Some(companies) = json["companies"].as_array() {
                    for company in companies {
                        println!("   - {} (TIN: {})", 
                            company["name"], 
                            company["tax_id"].as_str().unwrap_or("N/A")
                        );
                    }
                }
            }
        }
    }
    println!();
    
    // Demo 5: List invoices
    println!("📋 Listing invoices...");
    let result = invoice_plugin.execute(
        "list invoices",
        vec![]
    )?;
    
    if result.success {
        if let Some(data) = result.data {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                println!("✅ Invoices found: {}", json["count"]);
                if let Some(invoices) = json["invoices"].as_array() {
                    for invoice in invoices {
                        println!("   - {} for {} - Total: ₱{}", 
                            invoice["invoice_number"],
                            invoice["client"],
                            invoice["total"]
                        );
                    }
                }
            }
        }
    }
    
    println!("\n=== Demo Complete! ===");
    println!("The plugin system is working with:");
    println!("✅ WIT-based interfaces");
    println!("✅ WASM Component Model");
    println!("✅ Sandboxed execution");
    println!("✅ Store capability for persistence");
    println!("✅ Philippine tax compliance (12% VAT)");
    
    Ok(())
}