# EntityTag Component

## Overview
The EntityTag component provides [brief description].

## Installation
```toml
[dependencies]
taxtalk-entity-tag = { path = "../entity-tag" }
```

## Basic Usage
```rust
use taxtalk_entity_tag::EntityTag;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <EntityTag
            // props here
        />
    }
}
```

## Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| value | Signal<String> | Required | The component value |
| on_change | Callback<String> | Required | Called when value changes |

## Events
- `on_change`: Fired when the value changes
- `on_submit`: Fired on form submission

## Styling
The component uses CSS classes with `tui-entity-tag-` prefix.

## Examples
See `examples/` directory for more usage patterns.

## Testing
```bash
cargo test
cargo test --target wasm32-unknown-unknown
```
