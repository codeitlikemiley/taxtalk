# Fix Specification: Entity Combobox and Session Management Issues

## Issue Analysis

### 1. Entity Combobox Search Not Working
**Problem**: When typing in the EntityCombobox search field, the API returns an empty array wrapped in `Vec<Entity>` instead of `SearchResponse`
**Location**: `web/src/components/entity_combobox.rs:278-282`
**Root Cause**: Server returns `Json<Vec<Entity>>` but client expects `SearchResponse { results: Vec<Entity> }`

### 2. Cancel Button Loop Issue
**Problem**: Cancel button dismisses modal but it immediately reappears
**Location**: `web/src/components/guided_chat.rs:470-479`
**Root Cause**: Effect watching `show_entity_combobox` triggers when it becomes false, but doesn't properly handle the cancellation flow

### 3. Session Not Progressing After Entity Selection
**Problem**: After selecting a client, session stays at step 1/3 instead of moving to 2/3
**Location**: `web/src/components/guided_chat.rs:437-466`
**Root Cause**: The session progress isn't being properly updated after token submission

### 4. Missing Escape Key Handler for Session
**Problem**: Escape key should cancel the entire session, not just the modal
**Locations**: 
- `web/src/components/entity_combobox.rs:106-109` (handles modal escape only)
- `web/src/components/guided_chat.rs` (missing global escape handler)

## Fixes to Implement

### Fix 1: Server Search Response Format
**File**: `server/src/main.rs:626-630`
```rust
// Change return type to wrap in SearchResponse
async fn search_entities_get(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<SearchQuery>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let entities = search_entities_impl(state, params).await?;
    Ok(Json(SearchResponse { results: entities.0 }))
}
```

**File**: `server/src/main.rs:60` (add struct)
```rust
#[derive(Serialize)]
struct SearchResponse {
    results: Vec<Entity>,
}
```

### Fix 2: EntityCombobox Search Effect Debouncing
**File**: `web/src/components/entity_combobox.rs:46-80`
- Add debouncing to prevent immediate "No results" message
- Remove the double effect (currently loads on mount AND on query change)
- Combine into single effect with proper debouncing

### Fix 3: Cancel Button Flow Fix
**File**: `web/src/components/guided_chat.rs:470-479`
```rust
// Remove the effect that's causing the loop
// Instead, handle cancellation directly in the entity selection effect
```

**File**: `web/src/components/guided_chat.rs:437-466`
```rust
// Add proper cancellation handling when entity_selected is None and show_entity_combobox is false
```

### Fix 4: Session Progression Fix
**File**: `web/src/components/guided_chat.rs:455-460`
```rust
// Ensure continue_session is properly called with all parameters
// Check that the session is actually progressing on the server
```

### Fix 5: Global Escape Key Handler
**File**: `web/src/components/guided_chat.rs:481-499`
```rust
// Add escape key handler to cancel active session
if ev.key() == "Escape" && active_session.get().is_some() {
    ev.prevent_default();
    // Cancel the session
    cancel_session();
}
```

### Fix 6: Entity Combobox Props Enhancement
**File**: `web/src/components/entity_combobox.rs:106-109`
```rust
// Pass through a session cancel handler in addition to modal cancel
"Escape" => {
    ev.prevent_default();
    on_cancel.set(true);
    // Also trigger session cancellation if needed
}
```

## Implementation Order

1. Fix server search response format (Fix 1)
2. Fix EntityCombobox search effect and debouncing (Fix 2)
3. Fix cancel button loop (Fix 3)
4. Fix session progression (Fix 4)
5. Add global escape key handler (Fix 5)
6. Enhance escape handling in combobox (Fix 6)

## Test Cases

1. **Search Test**: Type "a" in entity combobox → Should show filtered results
2. **Cancel Test**: Click cancel button → Modal should close and not reappear
3. **Progression Test**: Select a client → Session should move to step 2/3
4. **Escape Test**: Press Escape during entity selection → Should cancel entire session
5. **Empty Search Test**: Clear search field → Should show all entities again

## Validation Commands

```bash
# Check compilation
cargo check --workspace --all-targets --all-features

# Build web
cd web && cargo build --target wasm32-unknown-unknown

# Run server
PORT=4001 cargo run --bin server

# Run web
CLIENT_PORT=4001 SERVER_PORT=4001 ./web/serve.sh
```