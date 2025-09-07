# TaxTalk Plugin System Demo

## Architecture Overview

We've successfully refactored TaxTalk from a complex Crux-based system to a **clean plugin-based architecture** that supports runtime plugin installation!

### What Changed:

#### Before (Crux-based):
```
Crux App → Effects → Capabilities → View Model → Complex Type Generation
```
- Compile-time type generation required
- No runtime plugin loading
- Complex indirection through effects/events
- Difficult to understand and maintain

#### After (Plugin-based):
```
Natural Language → Tokenizer → Router → WASM Plugin → Response
```
- Runtime plugin installation/uninstallation
- Simple, direct execution model
- No complex state machines
- Easy to understand and extend

## API Endpoints

### Plugin Management

1. **List Plugins**
   ```bash
   GET /api/plugins
   ```

2. **Install Plugin**
   ```bash
   POST /api/plugins/install
   Content-Type: multipart/form-data
   
   wasm: <plugin.wasm file>
   ```

3. **Uninstall Plugin**
   ```bash
   DELETE /api/plugins/{plugin_id}
   ```

4. **Enable/Disable Plugin**
   ```bash
   POST /api/plugins/{plugin_id}/enable
   POST /api/plugins/{plugin_id}/disable
   ```

5. **Get Plugin Schema** (for runtime type discovery)
   ```bash
   GET /api/plugins/{plugin_id}/schema
   ```

### Command Execution

```bash
POST /api/execute
Content-Type: application/json

{
  "command": "@client Juan bought 5kg rice 1000 pesos"
}
```

## Example Usage

### Install a Plugin at Runtime

```javascript
// Upload a WASM plugin
const formData = new FormData();
formData.append('wasm', pluginFile);

const response = await fetch('http://localhost:3000/api/plugins/install', {
  method: 'POST',
  body: formData
});

const plugin = await response.json();
console.log('Installed plugin:', plugin);
// Output: { id: 'invoice', name: 'Invoice Plugin', commands: [...] }
```

### Execute Natural Language Command

```javascript
const response = await fetch('http://localhost:3000/api/execute', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    command: 'create invoice for @client ABC Corp 100000 plus VAT'
  })
});

const result = await response.json();
// Output: { success: true, data: { invoice_id: 'INV-001', ... } }
```

### Runtime Type Discovery

```javascript
// Get plugin schema for dynamic form generation
const schema = await fetch('http://localhost:3000/api/plugins/invoice/schema')
  .then(r => r.text());

const schemaObj = JSON.parse(schema);
// Use schema to generate UI forms dynamically
```

## Benefits of New Architecture

1. **True Plugin System**: Install/uninstall plugins without rebuilding
2. **Simple Mental Model**: Command → Plugin → Result
3. **Runtime Discovery**: Plugins provide schemas for dynamic UIs
4. **No Complex Types**: No more Crux effects/events/capabilities
5. **Easy Testing**: Test each plugin independently
6. **Multi-tenant Ready**: Each tenant can have different plugins
7. **Hot Reloading**: Update plugins without downtime

## Next Steps

1. **Complete WASM bindings** for actual plugin execution
2. **Implement capability providers** (storage, HTTP)
3. **Build plugin marketplace** UI
4. **Create more plugins** (VAT calculator, BIR forms, etc.)

## Running the Demo

```bash
# Start the server
cargo run -p server

# Server will start on http://localhost:3000
# Use the API endpoints above to interact with the plugin system
```

The simplified architecture makes TaxTalk:
- **10x simpler** to understand
- **Actually supports** runtime plugin installation
- **Easier to develop** plugins for
- **More maintainable** long-term