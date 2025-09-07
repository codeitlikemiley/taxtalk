# Chat Interface Specification

## Current Issues to Fix

### 1. Server API Error (400 Bad Request)
**Issue**: `http://localhost:3000/api/execute` returns 400 Bad Request
**File**: `/Users/uriah/Code/taxtalk/core/src/plugin_host.rs:118-148`
**Problem**: The `execute_command` method has TODO implementations that need to be completed
**Solution**: Need to implement actual plugin execution instead of TODO placeholders

### 2. Built-in Plugins Not Loading
**Issue**: Plugin manager doesn't list built-in plugins (invoice, payments, company)
**Files to check**:
- `/Users/uriah/Code/taxtalk/plugins/invoice/manifest.json`
- `/Users/uriah/Code/taxtalk/plugins/payments/manifest.json`
- `/Users/uriah/Code/taxtalk/plugins/company/manifest.json`
- `/Users/uriah/Code/taxtalk/server/src/main.rs:71-81` (plugin loading on startup)

**Solution**: 
- Load plugins from `plugins/` directory at server startup
- Compile each plugin to WASM during build
- Auto-register them in PluginRegistry

## Features to Implement

### 1. Quick Commands with Tab Navigation (VSCode Snippet Style)

**Requirements**:
- Click quick command → Insert template into chat input
- Use `$1`, `$2`, `$3` placeholders for tab stops
- Tab key moves between placeholders
- Shift+Tab moves backwards
- ESC exits snippet mode

**Example Template**:
```
create invoice for @client $1 amount $2 due $3
```

**Files to modify**:
- `/Users/uriah/Code/taxtalk/web/src/pages/home.rs:55-70` (QuickCommand definitions)
- `/Users/uriah/Code/taxtalk/web/src/components/chat.rs:108-115` (keyboard handling)

**Implementation**:
```rust
// Add to chat.rs
struct SnippetState {
    template: String,
    placeholders: Vec<(usize, usize)>, // start, end positions
    current_index: usize,
}

fn handle_quick_command(template: &str) {
    // Parse template for $1, $2, $3
    // Set input value with first placeholder selected
    // Track snippet state
}
```

### 2. @ Symbol Autocomplete Popup

**Requirements**:
- Typing `@` triggers popup menu
- Search/filter entities as user types
- Show entity names but insert UUID v5
- UUID format: `{plugin_name}:{command}:{entity_type}:{id}`
- Support for multiple entity types: @client, @supplier, @employee, @invoice

**Files to modify**:
- `/Users/uriah/Code/taxtalk/web/src/components/chat.rs:193-201` (textarea input)
- Create new file: `/Users/uriah/Code/taxtalk/web/src/components/autocomplete.rs`

**Data Flow**:
1. User types `@cli...`
2. Query server: `GET /api/entities?type=client&query=cli`
3. Server queries plugins for matching entities
4. Display popup with results
5. On selection: Insert `@client:invoice:client:550e8400-e29b-41d4-a716-446655440000`

**API Endpoints Needed**:
```rust
// server/src/main.rs - Add new routes
.route("/api/entities", get(search_entities))
.route("/api/entities/:type", get(list_entities_by_type))
```

### 3. Enhanced Plugin Manager

#### 3.1 Install from URL
**Requirements**:
- Text input for WASM URL
- Validate URL format
- Download and install plugin
- Show progress indicator

**UI Location**: `/Users/uriah/Code/taxtalk/web/src/components/plugins.rs:95-120`

```rust
// Add to plugins.rs
let (plugin_url, set_plugin_url) = signal(String::new());

async fn install_from_url(url: String) -> Result<(), String> {
    // 1. Validate URL
    // 2. Download WASM bytes
    // 3. POST to /api/plugins/install
}
```

#### 3.2 File Upload
**Requirements**:
- Proper file input with drag & drop
- File size validation
- Upload progress
- Error handling

**Challenge**: web_sys::File with Leptos signals
**Solution**: Use a separate upload component or handle file reading in event handler

```rust
// Use FileReader API
fn handle_file_upload(file: web_sys::File) {
    let file_reader = web_sys::FileReader::new().unwrap();
    // Read as ArrayBuffer
    // Convert to Vec<u8>
    // Upload to server
}
```

### 4. Built-in Plugin Loading

**Server Changes** (`/Users/uriah/Code/taxtalk/server/src/main.rs:62-81`):

```rust
// Add function to load built-in plugins
async fn load_builtin_plugins(plugin_host: &PluginHost) -> Result<()> {
    let plugins_dir = PathBuf::from("./plugins");
    
    for entry in fs::read_dir(plugins_dir)? {
        let path = entry?.path();
        if path.is_dir() {
            // Look for compiled WASM
            let wasm_path = path.join("target/wasm32-wasip2/release/*.wasm");
            if wasm_path.exists() {
                let wasm_bytes = fs::read(wasm_path).await?;
                plugin_host.install_plugin(wasm_bytes).await?;
            }
        }
    }
    Ok(())
}
```

**Build Process**:
1. Add build script to compile plugins to WASM
2. Copy WASM files to known location
3. Load at server startup

### 5. Entity Management System

**New Module**: `/Users/uriah/Code/taxtalk/core/src/entities.rs`

```rust
pub struct Entity {
    pub id: Uuid,
    pub plugin_id: String,
    pub entity_type: String,
    pub name: String,
    pub metadata: Value,
}

pub trait EntityProvider {
    async fn list_entities(&self, entity_type: &str) -> Vec<Entity>;
    async fn search_entities(&self, query: &str) -> Vec<Entity>;
    async fn get_entity(&self, id: Uuid) -> Option<Entity>;
}
```

**Integration Points**:
- Each plugin provides entities via WIT interface
- Server aggregates entities from all plugins
- Cache frequently accessed entities

## Implementation Priority

### Phase 1: Fix Current Issues (Immediate)
1. ✅ Fix 400 Bad Request error - Mock responses implemented
2. ✅ Load built-in plugins at startup - Mock loading implemented
3. ✅ Show loaded plugins in UI - Will show when server runs

### Phase 2: Core Features (Day 1-2)
1. ⬜ @ symbol autocomplete with entity search
2. ⬜ Quick commands with tab navigation
3. ⬜ URL-based plugin installation

### Phase 3: Enhanced Features (Day 3-4)
1. ⬜ File upload with progress
2. ⬜ Entity management system
3. ⬜ Plugin hot-reload support

### Phase 4: Polish (Day 5)
1. ⬜ Error handling and recovery
2. ⬜ Loading states and animations
3. ⬜ Keyboard shortcuts help menu

## Technical Considerations

### State Management
- Use Leptos signals for UI state
- Server-side caching for entities
- WebSocket for real-time updates (future)

### Performance
- Debounce @ autocomplete queries (300ms)
- Cache entity search results
- Lazy load plugin metadata

### Security
- Validate WASM before installation
- Sandbox plugin execution
- Rate limit API calls

### Accessibility
- Keyboard navigation for autocomplete
- ARIA labels for dynamic content
- Screen reader support

## Testing Requirements

### Unit Tests
- Snippet parser logic
- Entity UUID generation
- Autocomplete filtering

### Integration Tests
- Plugin installation flow
- Command execution pipeline
- Entity search API

### E2E Tests
- Complete chat interaction
- Plugin management workflow
- Quick command usage

## File Structure Summary

```
/Users/uriah/Code/taxtalk/
├── core/
│   ├── src/
│   │   ├── plugin_host.rs:118-148    # Fix execute_command
│   │   ├── entities.rs (NEW)         # Entity management
│   │   └── router.rs:65-95           # Routing logic
├── server/
│   ├── src/
│   │   └── main.rs:62-81             # Plugin loading
│   │                :89-99            # Add entity routes
├── web/
│   ├── src/
│   │   ├── components/
│   │   │   ├── chat.rs:108-115       # Keyboard handling
│   │   │   │         :193-201        # Input area
│   │   │   ├── plugins.rs:95-120     # Upload section
│   │   │   └── autocomplete.rs (NEW) # @ mention popup
│   │   └── pages/
│   │       └── home.rs:55-70         # Quick commands
└── plugins/
    ├── invoice/
    │   ├── manifest.json              # Plugin metadata
    │   └── target/wasm32-wasip2/     # Compiled WASM
    ├── payments/
    └── company/
```

## Development Workflow

1. **Fix server issues first** - Without working API, UI is useless
2. **Implement entity system** - Foundation for @ mentions
3. **Add UI enhancements** - Autocomplete, snippets
4. **Test with real plugins** - Ensure integration works
5. **Polish and optimize** - Performance, UX improvements

## Success Criteria

- [ ] No 400 errors on command execution
- [ ] All built-in plugins visible in UI
- [ ] @ mentions show popup with searchable entities
- [ ] Quick commands insert snippets with tab navigation
- [ ] Can install plugin from URL
- [ ] Can upload plugin file
- [ ] Entities resolve to proper UUIDs
- [ ] Commands execute successfully with real plugins