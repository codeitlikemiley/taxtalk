# Component Documentation Standards

## Overview

This document defines the documentation standards for all TaxTalk UI components. Following these standards ensures consistency, maintainability, and ease of use across the component library.

## Documentation Structure

Each component must have the following documentation:

### 1. README.md (Component Root)

```markdown
# ComponentName

Brief description of what the component does.

## Installation

```bash
cargo add taxtalk-component-name
```

## Quick Start

```rust
use taxtalk_component_name::ComponentName;

view! {
    <ComponentName
        required_prop="value"
    />
}
```

## Features

- Feature 1
- Feature 2
- Feature 3

## Examples

[Link to examples directory]

## API Reference

[Link to API docs]

## Contributing

[Link to contributing guidelines]
```

### 2. Component Source Documentation

```rust
/// Brief description of the component
/// 
/// # Overview
/// 
/// Detailed explanation of the component's purpose and use cases.
/// 
/// # Examples
/// 
/// ```rust
/// use taxtalk_component::Component;
/// 
/// view! {
///     <Component
///         prop1="value"
///         on_event=callback
///     />
/// }
/// ```
/// 
/// # Props
/// 
/// - `prop1` - Description of prop1
/// - `prop2` - Description of prop2 (optional)
/// - `on_event` - Callback when event occurs
/// 
/// # State Management
/// 
/// Explanation of how the component manages state.
/// 
/// # Styling
/// 
/// CSS classes and customization options.
#[component]
pub fn ComponentName(/* props */) -> impl IntoView {
    // Implementation
}
```

### 3. Props Documentation

```rust
/// Props for the ComponentName component
#[derive(Clone, Debug)]
pub struct ComponentProps {
    /// Required property that does X
    /// 
    /// # Example
    /// ```
    /// prop1="value"
    /// ```
    pub prop1: String,
    
    /// Optional property for Y functionality
    /// 
    /// Defaults to `DefaultValue`
    #[prop(optional)]
    pub prop2: Option<String>,
    
    /// Callback triggered when Z happens
    /// 
    /// # Arguments
    /// 
    /// * `data` - The data associated with the event
    pub on_event: Callback<EventData, ()>,
}
```

## Example Documentation Template

### examples/basic.rs

```rust
//! Basic usage example for ComponentName
//! 
//! This example demonstrates the minimal setup required
//! to use the component.

use leptos::prelude::*;
use taxtalk_component::ComponentName;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Setup state
    let (value, set_value) = signal("initial");
    
    // Handle events
    let handle_event = move |data| {
        println!("Event triggered: {:?}", data);
    };
    
    view! {
        <div>
            <h1>"Basic Example"</h1>
            <ComponentName
                prop1=value
                on_event=Callback::new(handle_event)
            />
        </div>
    }
}
```

### examples/advanced.rs

```rust
//! Advanced usage example for ComponentName
//! 
//! This example shows advanced features including:
//! - Custom styling
//! - State management
//! - Event handling
//! - Integration with other components

// Implementation...
```

## API Documentation Standards

### Function Documentation

```rust
/// Performs a specific operation on the data
/// 
/// # Arguments
/// 
/// * `input` - The input data to process
/// * `options` - Configuration options
/// 
/// # Returns
/// 
/// Returns the processed result or an error
/// 
/// # Errors
/// 
/// This function will return an error if:
/// - The input is invalid
/// - Processing fails
/// 
/// # Examples
/// 
/// ```
/// let result = process_data("input", &options)?;
/// ```
/// 
/// # Panics
/// 
/// Panics if the internal state is corrupted
pub fn process_data(input: &str, options: &Options) -> Result<Output, Error> {
    // Implementation
}
```

### Type Documentation

```rust
/// Represents the state of the component
/// 
/// # Fields
/// 
/// * `field1` - Description of field1
/// * `field2` - Description of field2
/// 
/// # Example
/// 
/// ```
/// let state = ComponentState {
///     field1: "value",
///     field2: 42,
/// };
/// ```
#[derive(Clone, Debug)]
pub struct ComponentState {
    pub field1: String,
    pub field2: i32,
}
```

## Usage Guide Template

### docs/USAGE.md

```markdown
# ComponentName Usage Guide

## Table of Contents

1. [Basic Usage](#basic-usage)
2. [Advanced Features](#advanced-features)
3. [Styling](#styling)
4. [State Management](#state-management)
5. [Event Handling](#event-handling)
6. [Performance Tips](#performance-tips)
7. [Troubleshooting](#troubleshooting)

## Basic Usage

Step-by-step guide for basic usage...

## Advanced Features

### Feature 1

Explanation and example...

### Feature 2

Explanation and example...

## Styling

### CSS Classes

| Class | Description |
|-------|-------------|
| `.component-root` | Root element |
| `.component-item` | Individual items |

### Custom Styling

```css
.my-custom-class {
    /* Custom styles */
}
```

## State Management

### Internal State

How the component manages its own state...

### External State

Integration with application state...

## Event Handling

### Available Events

| Event | Description | Payload |
|-------|-------------|---------|
| `on_select` | Item selected | `{id, value}` |
| `on_change` | Value changed | `{old, new}` |

## Performance Tips

1. Use memoization for expensive computations
2. Implement virtual scrolling for large lists
3. Debounce rapid updates

## Troubleshooting

### Common Issues

#### Issue: Component not rendering

**Solution:** Check that all required props are provided

#### Issue: Events not firing

**Solution:** Ensure callbacks are properly wrapped
```

## Interactive Demo Documentation

### demo/README.md

```markdown
# Component Demo

## Running the Demo

```bash
cd components/component-name
trunk serve
```

Open http://localhost:8080 in your browser.

## Demo Features

The demo showcases:

1. **Basic Usage** - Simple component setup
2. **Configuration** - All available options
3. **Events** - Interactive event handling
4. **Styling** - Custom styling examples
5. **State** - State management patterns

## Demo Code

The demo source is in `src/main.rs` and demonstrates:

- Component initialization
- Event handling
- State updates
- Visual feedback
```

## Testing Documentation

### tests/README.md

```markdown
# Component Tests

## Running Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin

# Run WASM tests
wasm-pack test --headless --chrome
```

## Test Structure

- `unit/` - Unit tests for individual functions
- `integration/` - Component integration tests
- `e2e/` - End-to-end browser tests

## Writing Tests

### Unit Test Example

```rust
#[test]
fn test_state_update() {
    // Test implementation
}
```

### Integration Test Example

```rust
#[wasm_bindgen_test]
fn test_component_render() {
    // Test implementation
}
```
```

## Changelog Format

### CHANGELOG.md

```markdown
# Changelog

All notable changes to this component will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New feature X

### Changed
- Updated Y behavior

### Fixed
- Bug fix for Z

## [0.2.0] - 2024-01-15

### Added
- Feature A
- Feature B

### Changed
- Breaking: Changed prop name from `oldName` to `newName`

## [0.1.0] - 2024-01-01

### Added
- Initial release
- Basic functionality
```

## Documentation Checklist

For each component, ensure:

- [ ] README.md with quick start guide
- [ ] Component-level rustdoc comments
- [ ] Props documentation with examples
- [ ] At least 2 examples (basic and advanced)
- [ ] Usage guide with common patterns
- [ ] Interactive demo with trunk serve
- [ ] Test documentation
- [ ] CHANGELOG.md for version history
- [ ] CSS documentation if applicable
- [ ] Performance considerations noted
- [ ] Accessibility features documented
- [ ] Browser compatibility noted
- [ ] Migration guide for breaking changes

## Documentation Tools

### Generate API Docs

```bash
cargo doc --no-deps --open
```

### Check Documentation Coverage

```bash
cargo doc-coverage
```

### Validate Examples

```bash
cargo test --doc
```

## Best Practices

1. **Keep It Simple** - Start with basic usage, add complexity gradually
2. **Show, Don't Tell** - Use examples liberally
3. **Be Consistent** - Follow the same structure for all components
4. **Stay Current** - Update docs with code changes
5. **Test Examples** - Ensure all examples compile and run
6. **Visual Aids** - Include screenshots or GIFs where helpful
7. **Cross-Reference** - Link to related components and concepts
8. **Version Clearly** - Document breaking changes prominently

## Review Checklist

Before releasing a component, review:

1. Does the README provide a clear quick start?
2. Are all props documented with types and examples?
3. Do examples cover common use cases?
4. Is the API documentation complete?
5. Are error cases documented?
6. Is the changelog up to date?
7. Have breaking changes been clearly marked?
8. Do all code examples compile and run?