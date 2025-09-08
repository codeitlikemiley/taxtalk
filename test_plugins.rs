use anyhow::Result;
use wasmtime::*;
use wasmtime::component::*;
use serde_json::json;

fn main() -> Result<()> {
    println!("Testing WASM plugins with WIT bindings...\n");
    
    // Create engine with component model enabled
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    
    // Test invoice plugin
    test_plugin(&engine, "data/plugins/plugins/invoice.wasm", "Invoice Plugin")?;
    
    // Test company plugin  
    test_plugin(&engine, "data/plugins/plugins/company.wasm", "Company Plugin")?;
    
    Ok(())
}

fn test_plugin(engine: &Engine, path: &str, name: &str) -> Result<()> {
    println!("Testing {}...", name);
    println!("Loading from: {}", path);
    
    // Load the component
    let component = Component::from_file(&engine, path)?;
    
    // Create a store with host state
    let mut store = Store::new(&engine, HostState::new());
    
    // Create linker with host implementations
    let mut linker = Linker::new(&engine);
    
    // Add store interface implementations
    linker.func_wrap(
        "taxtalk:plugin/store@0.1.0",
        "get",
        |mut caller: Caller<'_, HostState>, key: String| -> Option<String> {
            println!("  Store::get('{}')", key);
            caller.data_mut().store.get(&key).cloned()
        }
    )?;
    
    linker.func_wrap(
        "taxtalk:plugin/store@0.1.0", 
        "set",
        |mut caller: Caller<'_, HostState>, key: String, value: String| -> Result<()> {
            println!("  Store::set('{}', '{}')", key, value);
            caller.data_mut().store.insert(key, value);
            Ok(())
        }
    )?;
    
    linker.func_wrap(
        "taxtalk:plugin/store@0.1.0",
        "delete", 
        |mut caller: Caller<'_, HostState>, key: String| -> Result<()> {
            println!("  Store::delete('{}')", key);
            caller.data_mut().store.remove(&key);
            Ok(())
        }
    )?;
    
    linker.func_wrap(
        "taxtalk:plugin/store@0.1.0",
        "list-keys",
        |caller: Caller<'_, HostState>, prefix: String| -> Vec<String> {
            println!("  Store::list-keys('{}')", prefix);
            caller.data().store
                .keys()
                .filter(|k| k.starts_with(&prefix))
                .cloned()
                .collect()
        }
    )?;
    
    // Add minimal HTTP implementation
    linker.func_wrap(
        "taxtalk:plugin/http@0.1.0",
        "fetch",
        |_caller: Caller<'_, HostState>, _req: String| -> Result<String> {
            println!("  HTTP::fetch() - not implemented");
            Ok(json!({
                "status": 200,
                "headers": [],
                "body": "{}"
            }).to_string())
        }
    )?;
    
    // Instantiate the component
    let instance = linker.instantiate(&mut store, &component)?;
    
    // Get the plugin export
    let plugin = instance.get_export(&mut store, None, "taxtalk:plugin/plugin@0.1.0")?;
    
    // Call get-manifest
    let get_manifest = plugin.get_func(&mut store, "get-manifest")?;
    let manifest = get_manifest.call(&mut store, &[])?;
    println!("  Manifest: {:?}", manifest);
    
    // Test execute function with a sample command
    let execute = plugin.get_func(&mut store, "execute")?;
    let result = execute.call(&mut store, &[
        Val::String("create invoice for ABC Corp 10000".into()),
        Val::List(vec![]) // Empty tokens for now
    ])?;
    println!("  Execute result: {:?}", result);
    
    println!("  ✅ {} loaded and tested successfully!\n", name);
    Ok(())
}

// Host state for the store
struct HostState {
    store: std::collections::HashMap<String, String>,
}

impl HostState {
    fn new() -> Self {
        Self {
            store: std::collections::HashMap::new(),
        }
    }
}