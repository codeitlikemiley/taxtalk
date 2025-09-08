# Styling Cheatsheet

## Inline Styles

### Basic Inline Styles
```rust
view! { cx,
    <div style="color: red; font-size: 14px;">
        "Styled text"
    </div>
}
```

### Dynamic Inline Styles
```rust
let color = "blue";
let font_size = 16;

view! { cx,
    <div style=format!("color: {}; font-size: {}px;", color, font_size)>
        "Dynamic styles"
    </div>
}
```

### Style Objects
```rust
let styles = "color: green; border: 1px solid black;";

view! { cx,
    <div style=styles>
        "Object styles"
    </div>
}
```

## CSS Classes

### Static Classes
```rust
view! { cx,
    <div class="container primary">
        "Static classes"
    </div>
}
```

### Dynamic Classes
```rust
let is_active = true;
let theme = "dark";

view! { cx,
    <div class=format!("button {} {}", 
        if is_active { "active" } else { "inactive" },
        theme
    )>
        "Dynamic classes"
    </div>
}
```

### Conditional Classes
```rust
let is_primary = true;
let is_large = false;

view! { cx,
    <button class={
        format!("btn {} {}", 
            if is_primary { "btn-primary" } else { "btn-secondary" },
            if is_large { "btn-large" } else { "btn-small" }
        )
    }>
        "Conditional classes"
    </button>
}
```

### Class Attribute Variants
```rust
// Using class: for conditional classes
view! { cx,
    <div 
        class="base-class"
        class:active=is_active
        class:disabled=!enabled
    >
        "Class variants"
    </div>
}
```

## CSS Modules

### Basic CSS Module Usage
```rust
// styles.module.css
.container {
    padding: 20px;
    background: #f0f0f0;
}

.title {
    color: blue;
    font-size: 24px;
}

// In component
use crate::styles; // Generated from CSS module

view! { cx,
    <div class=styles.container>
        <h1 class=styles.title>"CSS Modules"</h1>
    </div>
}
```

### Dynamic CSS Modules
```rust
let variant = "primary";

view! { cx,
    <button class={
        match variant {
            "primary" => styles.primary_button,
            "secondary" => styles.secondary_button,
            _ => styles.default_button,
        }
    }>
        "CSS Module variants"
    </button>
}
```

## Tailwind CSS Integration

### Basic Tailwind Classes
```rust
view! { cx,
    <div class="bg-blue-500 text-white p-4 rounded-lg shadow-md">
        <h1 class="text-2xl font-bold mb-2">"Tailwind Example"</h1>
        <p class="text-gray-200">"Using Tailwind utility classes"</p>
    </div>
}
```

### Dynamic Tailwind Classes
```rust
let color = "blue";
let size = "lg";

view! { cx,
    <button class=format!("bg-{}-500 text-white px-4 py-2 rounded hover:bg-{}-600", 
        color, color)>
        "Dynamic Tailwind"
    </button>
}
```

### Responsive Design
```rust
view! { cx,
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div class="bg-white p-4 shadow rounded">"Card 1"</div>
        <div class="bg-white p-4 shadow rounded">"Card 2"</div>
        <div class="bg-white p-4 shadow rounded">"Card 3"</div>
    </div>
}
```

## Styled Components

### Basic Styled Component
```rust
#[component]
pub fn StyledButton(cx: Scope) -> impl IntoView {
    view! { cx,
        <style>
            ".styled-btn {
                background: linear-gradient(45deg, #667eea 0%, #764ba2 100%);
                border: none;
                color: white;
                padding: 12px 24px;
                border-radius: 8px;
                cursor: pointer;
                font-size: 16px;
                transition: transform 0.2s;
            }
            .styled-btn:hover {
                transform: translateY(-2px);
            }"
        </style>
        <button class="styled-btn">
            "Styled Component"
        </button>
    }
}
```

### Theme-based Styling
```rust
#[derive(Clone)]
pub struct Theme {
    pub primary: String,
    pub secondary: String,
    pub background: String,
}

let theme = Theme {
    primary: "#3b82f6".to_string(),
    secondary: "#64748b".to_string(),
    background: "#ffffff".to_string(),
};

view! { cx,
    <style>
        {format!("
            .themed-card {{
                background: {};
                border: 2px solid {};
                color: {};
            }}
        ", theme.background, theme.primary, theme.secondary)}
    </style>
    <div class="themed-card p-4 rounded">
        "Themed content"
    </div>
}
```

## CSS-in-Rust Solutions

### Styled Components with Macros
```rust
use leptos::style;

let button_style = style! {"
    background: #4f46e5;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
"};

view! { cx,
    <button style=button_style>
        "Styled with macro"
    </button>
}
```

### Dynamic Style Objects
```rust
let button_styles = move || format!("
    background: {};
    color: {};
    padding: {}px;
", 
    if is_primary { "#3b82f6" } else { "#6b7280" },
    "white",
    if is_large { 12 } else { 8 }
);

view! { cx,
    <button style=button_styles>
        "Dynamic style object"
    </button>
}
```

## Global Styles

### Global CSS Injection
```rust
// In main.rs or lib.rs
use leptos::mount_to_body;

fn main() {
    // Inject global styles
    leptos::document().head().unwrap().append_child(
        &leptos::html::style().inner_html("
            * {
                box-sizing: border-box;
            }
            body {
                margin: 0;
                font-family: -apple-system, BlinkMacSystemFont, sans-serif;
            }
        ")
    );
    
    mount_to_body(|cx| view! { cx, <App /> });
}
```

### CSS Custom Properties (CSS Variables)
```rust
view! { cx,
    <style>
        ":root {
            --primary-color: #3b82f6;
            --secondary-color: #64748b;
            --spacing-unit: 8px;
        }
        .variable-btn {
            background: var(--primary-color);
            padding: calc(var(--spacing-unit) * 2);
        }"
    </style>
    <button class="variable-btn">
        "CSS Variables"
    </button>
}
```

## Animation and Transitions

### CSS Transitions
```rust
view! { cx,
    <style>
        ".animated-btn {
            background: #3b82f6;
            transition: all 0.3s ease;
        }
        .animated-btn:hover {
            background: #2563eb;
            transform: scale(1.05);
        }"
    </style>
    <button class="animated-btn">
        "Hover me"
    </button>
}
```

### Keyframe Animations
```rust
view! { cx,
    <style>
        "@keyframes spin {
            from { transform: rotate(0deg); }
            to { transform: rotate(360deg); }
        }
        .spinner {
            animation: spin 1s linear infinite;
        }"
    </style>
    <div class="spinner">
        "⟳"
    </div>
}
```

## Responsive Design

### Media Queries
```rust
view! { cx,
    <style>
        ".responsive {
            width: 100%;
            padding: 16px;
        }
        @media (min-width: 768px) {
            .responsive {
                width: 50%;
                padding: 24px;
            }
        }
        @media (min-width: 1024px) {
            .responsive {
                width: 33.333%;
                padding: 32px;
            }
        }"
    </style>
    <div class="responsive">
        "Responsive content"
    </div>
}
```

### Container Queries (Modern CSS)
```rust
view! { cx,
    <style>
        ".container-query {
            container-type: inline-size;
        }
        .child {
            padding: 16px;
        }
        @container (min-width: 400px) {
            .child {
                padding: 24px;
            }
        }"
    </style>
    <div class="container-query">
        <div class="child">
            "Container query content"
        </div>
    </div>
}
```

## Dark Mode Implementation

### CSS Custom Properties for Themes
```rust
view! { cx,
    <style>
        ":root {
            --bg-color: #ffffff;
            --text-color: #000000;
            --border-color: #e5e7eb;
        }
        [data-theme='dark'] {
            --bg-color: #1f2937;
            --text-color: #ffffff;
            --border-color: #374151;
        }
        .themed {
            background: var(--bg-color);
            color: var(--text-color);
            border: 1px solid var(--border-color);
        }"
    </style>
    <div class="themed">
        "Theme-aware content"
    </div>
}
```

### Dynamic Theme Switching
```rust
let is_dark = create_signal(cx, false);

create_effect(cx, move |_| {
    let document = leptos::document();
    if is_dark.get() {
        document.document_element().unwrap().set_attribute("data-theme", "dark");
    } else {
        document.document_element().unwrap().remove_attribute("data-theme");
    }
});

view! { cx,
    <button on:click=move |_| is_dark.update(|d| *d = !*d)>
        {move || if is_dark.get() { "Light Mode" } else { "Dark Mode" }}
    </button>
}
```

## Performance Considerations

### Minimize Style Recalculations
```rust
// ✅ Good: Style changes are batched
let styles = create_memo(cx, move |_| {
    format!("color: {}; font-size: {}px;", color.get(), size.get())
});

// ❌ Bad: Style recalculated on every render
view! { cx,
    <div style=format!("color: {}; font-size: {}px;", color.get(), size.get())>
        "Content"
    </div>
}
```

### Use CSS Classes Instead of Inline Styles
```rust
// ✅ Good: Use classes for static styles
view! { cx,
    <div class="card shadow rounded">
        "Content"
    </div>
}

// ❌ Bad: Inline styles for static properties
view! { cx,
    <div style="box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1); border-radius: 0.5rem;">
        "Content"
    </div>
}
```

### Avoid Style Thrashing
```rust
// ✅ Good: Batch style updates
let update_styles = move || {
    // Update multiple style-related signals at once
    set_color.set("red");
    set_size.set(20);
    set_visible.set(true);
};

// ❌ Bad: Individual style updates
let update_color = move |_| set_color.set("red");
let update_size = move |_| set_size.set(20);
let update_visible = move |_| set_visible.set(true);
```

## CSS Frameworks Integration

### Bootstrap Classes
```rust
view! { cx,
    <div class="container-fluid">
        <div class="row">
            <div class="col-md-6 col-lg-4">
                <div class="card">
                    <div class="card-body">
                        <h5 class="card-title">"Bootstrap Card"</h5>
                        <p class="card-text">"Using Bootstrap classes"</p>
                    </div>
                </div>
            </div>
        </div>
    </div>
}
```

### Material Design Components
```rust
view! { cx,
    <button class="mdc-button mdc-button--raised">
        <span class="mdc-button__label">"Material Button"</span>
    </button>
}
```

## Custom CSS Properties with Leptos

### Reactive CSS Variables
```rust
let primary_color = create_signal(cx, "#3b82f6");
let border_radius = create_signal(cx, 4);

let css_vars = create_memo(cx, move |_| {
    format!("
        --primary: {};
        --radius: {}px;
    ", primary_color.get(), border_radius.get())
});

view! { cx,
    <style>{css_vars}</style>
    <button 
        style="--local-color: red;"
        class="custom-btn"
    >
        "Custom properties"
    </button>
}
```

## Best Practices

### ✅ Do
```rust
// Use semantic class names
view! { cx, <div class="user-profile-card"> }

// Prefer CSS modules for component-scoped styles
// Use Tailwind for rapid prototyping
// Implement dark mode with CSS custom properties
// Use CSS Grid and Flexbox for layouts
// Optimize images and fonts
```

### ❌ Don't
```rust
// Don't use inline styles for static properties
// Don't create styles in render functions
// Don't use !important unless absolutely necessary
// Don't override framework styles directly
// Don't forget to handle focus states for accessibility
```

## Quick Reference

### Style Binding Patterns
- `style="color: red;"` - Static inline styles
- `style=variable` - Dynamic style string
- `style=move || format!(...)` - Reactive styles
- `class="static classes"` - Static classes
- `class=variable` - Dynamic classes
- `class:active=condition` - Conditional classes

### CSS Integration Methods
- **Inline Styles**: Quick, dynamic, low specificity
- **CSS Classes**: Reusable, performant, high specificity
- **CSS Modules**: Scoped, maintainable, build-time
- **Tailwind**: Utility-first, consistent, rapid development
- **Styled Components**: Component-scoped, dynamic, framework-specific

### Performance Tips
- Use classes for static styles
- Use memos for dynamic styles
- Batch style updates
- Minimize style recalculations
- Use CSS containment when possible
- Optimize CSS delivery (critical CSS, code splitting)

### Accessibility
- Always provide focus indicators
- Use semantic HTML elements
- Maintain sufficient color contrast
- Support reduced motion preferences
- Test with screen readers