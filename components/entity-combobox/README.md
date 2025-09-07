# EntityCombobox Component

A searchable dropdown component for entity selection with API caching support.

## Features

- 🔍 **Smart Search** - Real-time search with debouncing
- 💾 **API Caching** - Intelligent caching to reduce API calls
- ⌨️ **Keyboard Navigation** - Full keyboard support (Arrow keys, Enter, Escape)
- 🎨 **Entity Types** - Support for multiple entity types (clients, suppliers, products, etc.)
- ⚡ **Performance** - Optimized rendering with configurable result limits
- 📱 **Responsive** - Works on desktop and mobile devices

## Installation

```bash
cargo add entity-combobox
```

## Quick Start

```rust
use leptos::prelude::*;
use entity_combobox::{EntityCombobox, Entity, EntityType};

#[component]
fn App() -> impl IntoView {
    let handle_select = move |entity: Entity| {
        log!("Selected: {}", entity.name);
    };
    
    view! {
        <EntityCombobox
            entity_type=EntityType::Client
            on_select=Callback::new(handle_select)
            api_url="https://api.example.com"
        />
    }
}
```

## Configuration

```rust
use entity_combobox::EntityComboboxConfig;

let config = EntityComboboxConfig {
    max_suggestions: 10,        // Maximum results to show
    debounce_ms: 300,           // Search debounce delay
    enable_keyboard_nav: true,  // Enable keyboard navigation
    show_loading: true,         // Show loading indicator
    cache_results: true,        // Enable API caching
    cache_expiry_secs: 300,     // Cache expiry (5 minutes)
};
```

## API Integration

The component can work with both real APIs and mock data:

### Mock Data (for development)
```rust
<EntityCombobox
    entity_type=EntityType::Client
    api_url="mock"  // Use mock data
    on_select=callback
/>
```

### Real API
```rust
<EntityCombobox
    entity_type=EntityType::Client
    api_url="https://api.example.com"
    on_select=callback
/>
```

### Expected API Endpoints

- `GET /api/entities/{entity_type}` - Load all entities of a type
- `GET /api/search?q={query}&type={entity_type}` - Search entities

### Response Format

```json
{
    "results": [
        {
            "id": "1",
            "name": "ABC Corporation",
            "entity_type": "client",
            "description": "Premium client",
            "metadata": {...}
        }
    ]
}
```

## Entity Types

Built-in entity types:
- `EntityType::Client`
- `EntityType::Supplier`
- `EntityType::Product`
- `EntityType::Employee`
- `EntityType::Invoice`
- `EntityType::Payment`
- `EntityType::Custom(String)`

## Styling

The component uses CSS classes with the `ec-` prefix. You can override these styles:

```css
.ec-modal {
    /* Modal container */
}

.ec-search-input {
    /* Search input field */
}

.ec-item {
    /* List items */
}

.ec-item-selected {
    /* Selected item */
}
```

## Examples

### With Initial Query
```rust
<EntityCombobox
    entity_type=EntityType::Product
    initial_query=Some("Rice".to_string())
    on_select=callback
/>
```

### With Cancel Handler
```rust
<EntityCombobox
    entity_type=EntityType::Client
    on_select=handle_select
    on_cancel=Some(Callback::new(handle_cancel))
/>
```

### Custom Configuration
```rust
<EntityCombobox
    entity_type=EntityType::Supplier
    config=Some(custom_config)
    prompt=Some("Select a supplier for this order".to_string())
    on_select=callback
/>
```

## Development

Run the demo:
```bash
cd components/entity-combobox
trunk serve
```

Run tests:
```bash
cargo test
```

## License

MIT