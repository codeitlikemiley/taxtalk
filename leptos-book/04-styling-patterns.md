# 04: Styling Patterns

## Overview

Leptos provides flexible styling options that work seamlessly with modern web development. This chapter covers CSS integration, styling patterns, and best practices for creating maintainable, performant styled components.

## CSS Integration Methods

### Inline Styles

```rust
#[component]
pub fn InlineStyledButton() -> impl IntoView {
    let (is_active, set_is_active) = create_signal(false);

    view! {
        <button
            style=move || if is_active() {
                "background-color: blue; color: white; padding: 10px 20px; border: none; border-radius: 4px;"
            } else {
                "background-color: gray; color: black; padding: 10px 20px; border: none; border-radius: 4px;"
            }
            on:click=move |_| set_is_active.update(|active| *active = !*active)
        >
            {move || if is_active() { "Active" } else { "Inactive" }}
        </button>
    }
}
```

### Class-Based Styling

```rust
#[component]
pub fn ClassStyledCard() -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-header">
                <h3>"Card Title"</h3>
            </div>
            <div class="card-body">
                <p>"Card content goes here"</p>
            </div>
            <div class="card-footer">
                <button class="btn btn-primary">"Action"</button>
            </div>
        </div>
    }
}
```

### CSS Modules (with Trunk)

```rust
// styles.module.css
/*
.card {
    border: 1px solid #ddd;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.cardHeader {
    background-color: #f8f9fa;
    padding: 1rem;
    border-bottom: 1px solid #dee2e6;
}

.cardBody {
    padding: 1rem;
}
*/

#[component]
pub fn CssModuleCard() -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-header">
                <h3>"CSS Module Card"</h3>
            </div>
            <div class="card-body">
                <p>"This card uses CSS modules"</p>
            </div>
        </div>
    }
}
```

## Tailwind CSS Integration

### Basic Tailwind Classes

```rust
#[component]
pub fn TailwindButton() -> impl IntoView {
    view! {
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            "Tailwind Button"
        </button>
    }
}
```

### Dynamic Tailwind Classes

```rust
#[component]
pub fn DynamicTailwindButton(variant: String) -> impl IntoView {
    let base_classes = "font-bold py-2 px-4 rounded";
    let variant_classes = move || match variant.as_str() {
        "primary" => "bg-blue-500 hover:bg-blue-700 text-white",
        "secondary" => "bg-gray-500 hover:bg-gray-700 text-white",
        "danger" => "bg-red-500 hover:bg-red-700 text-white",
        _ => "bg-gray-500 hover:bg-gray-700 text-white",
    };

    view! {
        <button class=move || format!("{} {}", base_classes, variant_classes())>
            "Dynamic Button"
        </button>
    }
}
```

### Responsive Design with Tailwind

```rust
#[component]
pub fn ResponsiveGrid() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div class="bg-blue-100 p-4 rounded">"Item 1"</div>
            <div class="bg-green-100 p-4 rounded">"Item 2"</div>
            <div class="bg-yellow-100 p-4 rounded">"Item 3"</div>
            <div class="bg-red-100 p-4 rounded">"Item 4"</div>
            <div class="bg-purple-100 p-4 rounded">"Item 5"</div>
            <div class="bg-pink-100 p-4 rounded">"Item 6"</div>
        </div>
    }
}
```

## Styled Components Pattern

### Creating Styled Components

```rust
use leptos::*;

// Styled component function
pub fn styled_button<F>(children: F) -> impl IntoView
where
    F: Fn() -> View + 'static,
{
    view! {
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition duration-300">
            {children()}
        </button>
    }
}

#[component]
pub fn StyledButtonApp() -> impl IntoView {
    view! {
        <div>
            {styled_button(|| view! { "Click me!" })}
        </div>
    }
}
```

### Theme-Aware Styled Components

```rust
#[derive(Clone, Debug)]
pub struct Theme {
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub text_color: String,
}

#[component]
pub fn ThemedButton(theme: Theme) -> impl IntoView {
    view! {
        <button
            style=move || format!(
                "background-color: {}; color: {}; border: none; padding: 10px 20px; border-radius: 4px;",
                theme.primary_color, theme.text_color
            )
        >
            "Themed Button"
        </button>
    }
}
```

## CSS-in-Rust Patterns

### Style Objects

```rust
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct StyleProps {
    pub background_color: Option<String>,
    pub color: Option<String>,
    pub padding: Option<String>,
    pub margin: Option<String>,
    pub border_radius: Option<String>,
}

impl StyleProps {
    pub fn to_css_string(&self) -> String {
        let mut styles = Vec::new();

        if let Some(bg) = &self.background_color {
            styles.push(format!("background-color: {}", bg));
        }
        if let Some(color) = &self.color {
            styles.push(format!("color: {}", color));
        }
        if let Some(padding) = &self.padding {
            styles.push(format!("padding: {}", padding));
        }
        if let Some(margin) = &self.margin {
            styles.push(format!("margin: {}", margin));
        }
        if let Some(radius) = &self.border_radius {
            styles.push(format!("border-radius: {}", radius));
        }

        styles.join("; ")
    }
}

#[component]
pub fn StyledDiv(style_props: StyleProps, children: Children) -> impl IntoView {
    view! {
        <div style=style_props.to_css_string()>
            {children()}
        </div>
    }
}
```

### Conditional Styling

```rust
#[component]
pub fn ConditionalStyledButton() -> impl IntoView {
    let (is_loading, set_is_loading) = create_signal(false);
    let (is_success, set_is_success) = create_signal(false);

    let button_style = move || {
        if is_loading() {
            "background-color: #gray; color: white; cursor: not-allowed;"
        } else if is_success() {
            "background-color: #green; color: white;"
        } else {
            "background-color: #blue; color: white;"
        }
    };

    let button_text = move || {
        if is_loading() {
            "Loading..."
        } else if is_success() {
            "Success!"
        } else {
            "Click me"
        }
    };

    view! {
        <button
            style=button_style
            disabled=is_loading
            on:click=move |_| {
                set_is_loading(true);
                set_is_success(false);
                // Simulate async operation
                set_timeout(move || {
                    set_is_loading(false);
                    set_is_success(true);
                }, 2000);
            }
        >
            {button_text}
        </button>
    }
}
```

## CSS Variables and Theming

### CSS Custom Properties

```rust
#[component]
pub fn ThemedApp() -> impl IntoView {
    let (theme, set_theme) = create_signal("light");

    let root_style = move || {
        if theme() == "dark" {
            "--primary-color: #1f2937; --secondary-color: #374151; --text-color: #f9fafb; --background-color: #111827;"
        } else {
            "--primary-color: #3b82f6; --secondary-color: #6b7280; --text-color: #1f2937; --background-color: #ffffff;"
        }
    };

    view! {
        <div style=root_style class="min-h-screen" style=move || format!("background-color: var(--background-color); color: var(--text-color)")>
            <button
                on:click=move |_| set_theme.update(|t| *t = if *t == "light" { "dark" } else { "light" })
                style="background-color: var(--primary-color); color: var(--text-color); padding: 10px 20px; border: none; border-radius: 4px;"
            >
                {move || format!("Switch to {} mode", if theme() == "light" { "dark" } else { "light" })}
            </button>
            <div style="margin-top: 20px; padding: 20px; background-color: var(--secondary-color); border-radius: 8px;">
                "This content uses CSS custom properties for theming"
            </div>
        </div>
    }
}
```

## Animation and Transitions

### CSS Transitions

```rust
#[component]
pub fn AnimatedButton() -> impl IntoView {
    let (is_hovered, set_is_hovered) = create_signal(false);

    view! {
        <button
            class=move || if is_hovered() {
                "bg-blue-600 text-white px-4 py-2 rounded transition-all duration-300 transform scale-105"
            } else {
                "bg-blue-500 text-white px-4 py-2 rounded transition-all duration-300"
            }
            on:mouseenter=move |_| set_is_hovered(true)
            on:mouseleave=move |_| set_is_hovered(false)
        >
            "Hover me!"
        </button>
    }
}
```

### Keyframe Animations

```rust
#[component]
pub fn SpinningLoader() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>
    }
}

// CSS for the spinner (in your stylesheet)
/*
@keyframes spin {
    to {
        transform: rotate(360deg);
    }
}

.animate-spin {
    animation: spin 1s linear infinite;
}
*/
```

## Responsive Design Patterns

### Mobile-First Approach

```rust
#[component]
pub fn ResponsiveCard() -> impl IntoView {
    view! {
        <div class="w-full md:w-1/2 lg:w-1/3 p-4">
            <div class="bg-white rounded-lg shadow-md overflow-hidden">
                <div class="p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">
                        "Responsive Card"
                    </h3>
                    <p class="text-gray-600 text-sm md:text-base">
                        "This card adapts to different screen sizes"
                    </p>
                    <button class="mt-4 bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition duration-300 text-sm md:text-base">
                        "Learn More"
                    </button>
                </div>
            </div>
        </div>
    }
}
```

### Container Queries

```rust
#[component]
pub fn ContainerQueryCard() -> impl IntoView {
    view! {
        <div class="container-card">
            <div class="card-content">
                <h3>"Container Query Card"</h3>
                <p>"This card uses container queries for responsive behavior"</p>
                <div class="card-actions">
                    <button class="btn-primary">"Action 1"</button>
                    <button class="btn-secondary">"Action 2"</button>
                </div>
            </div>
        </div>
    }
}

/* CSS with container queries */
/*
.container-card {
    container-type: inline-size;
}

.card-content {
    padding: 1rem;
}

.card-actions {
    display: flex;
    gap: 0.5rem;
    flex-direction: column;
}

@container (min-width: 300px) {
    .card-actions {
        flex-direction: row;
    }
}

@container (min-width: 500px) {
    .card-content {
        padding: 2rem;
    }
}
*/
```

## CSS Grid and Flexbox

### CSS Grid Layout

```rust
#[component]
pub fn GridLayout() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
            {(0..6).map(|i| view! {
                <div class="bg-blue-100 p-4 rounded-lg shadow">
                    <h3 class="font-bold">"Item " {i + 1}</h3>
                    <p class="text-sm text-gray-600">"Grid item content"</p>
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

### Flexbox Layout

```rust
#[component]
pub fn FlexLayout() -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row gap-4 p-4">
            <div class="flex-1 bg-green-100 p-4 rounded">
                <h3>"Main Content"</h3>
                <p>"This is the main content area"</p>
            </div>
            <div class="w-full md:w-64 bg-blue-100 p-4 rounded">
                <h3>"Sidebar"</h3>
                <p>"This is the sidebar"</p>
            </div>
        </div>
    }
}
```

## Dark Mode Implementation

### System Preference Detection

```rust
use leptos::wasm_bindgen::JsCast;

#[component]
pub fn DarkModeToggle() -> impl IntoView {
    let (is_dark, set_is_dark) = create_signal(false);

    // Check system preference on mount
    create_effect(move |_| {
        if let Ok(Some(window)) = web_sys::window() {
            if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
                set_is_dark(media_query.matches());
            }
        }
    });

    let toggle_dark_mode = move |_| {
        set_is_dark.update(|dark| *dark = !*dark);
    };

    view! {
        <div class=move || if is_dark() { "dark bg-gray-900 text-white" } else { "light bg-white text-black" }>
            <button
                on:click=toggle_dark_mode
                class="p-2 rounded bg-gray-200 dark:bg-gray-700"
            >
                {move || if is_dark() { "☀️" } else { "🌙" }}
            </button>
            <div class="p-4">
                <h1 class="text-2xl font-bold">"Dark Mode Demo"</h1>
                <p>"This content changes based on the theme"</p>
            </div>
        </div>
    }
}
```

## Performance Considerations

### CSS-in-JS Performance

```rust
// Avoid recreating style strings on every render
#[component]
pub fn OptimizedStyledComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    // Memoize the style string
    let button_style = create_memo(move |_| {
        format!(
            "background-color: {}; padding: 10px 20px; border-radius: 4px; border: none; color: white;",
            if count() > 5 { "#ff4444" } else { "#4444ff" }
        )
    });

    view! {
        <div>
            <button style=button_style on:click=move |_| set_count.update(|n| *n + 1)>
                "Count: " {count}
            </button>
        </div>
    }
}
```

### Class Name Optimization

```rust
// Pre-compute class names
const BASE_CLASSES: &str = "px-4 py-2 rounded font-medium transition-colors duration-200";
const PRIMARY_CLASSES: &str = "bg-blue-500 hover:bg-blue-600 text-white";
const SECONDARY_CLASSES: &str = "bg-gray-500 hover:bg-gray-600 text-white";

#[component]
pub fn OptimizedButton(variant: String) -> impl IntoView {
    let class_name = move || {
        let variant_classes = match variant.as_str() {
            "primary" => PRIMARY_CLASSES,
            "secondary" => SECONDARY_CLASSES,
            _ => PRIMARY_CLASSES,
        };
        format!("{} {}", BASE_CLASSES, variant_classes)
    };

    view! {
        <button class=class_name>
            "Optimized Button"
        </button>
    }
}
```

## CSS Organization Patterns

### Component-Scoped Styles

```rust
// styles.rs
pub const CARD_STYLES: &str = r#"
.card {
    border: 1px solid #e2e8f0;
    border-radius: 0.5rem;
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
    background-color: white;
}

.card-header {
    padding: 1rem;
    border-bottom: 1px solid #e2e8f0;
    background-color: #f8fafc;
}

.card-body {
    padding: 1rem;
}

.card-footer {
    padding: 1rem;
    border-top: 1px solid #e2e8f0;
    background-color: #f8fafc;
}
"#;

#[component]
pub fn StyledCardWithCss() -> impl IntoView {
    // In a real app, you'd inject these styles into the document head
    view! {
        <div class="card">
            <div class="card-header">
                <h3>"Styled Card"</h3>
            </div>
            <div class="card-body">
                <p>"This card uses component-scoped CSS"</p>
            </div>
            <div class="card-footer">
                <button>"Action"</button>
            </div>
        </div>
    }
}
```

## Best Practices

1. **Prefer utility classes**: Use Tailwind CSS or similar for rapid development
2. **Keep styles close to components**: Co-locate styles with their components
3. **Use CSS custom properties**: For theming and dynamic values
4. **Memoize style calculations**: Avoid recreating style strings unnecessarily
5. **Consider performance**: CSS-in-JS can be expensive if not optimized
6. **Use semantic class names**: Make styles maintainable and readable
7. **Support both light and dark modes**: Provide accessible color schemes
8. **Test responsive designs**: Ensure layouts work across all screen sizes
9. **Use CSS Grid and Flexbox**: For modern, flexible layouts
10. **Document style APIs**: Make component styling interfaces clear

## Summary

Leptos offers multiple styling approaches:

- **Inline styles**: Simple but can cause performance issues
- **CSS classes**: Traditional approach with external stylesheets
- **CSS modules**: Scoped styles to avoid naming conflicts
- **Tailwind CSS**: Utility-first approach for rapid development
- **Styled components**: Component-based styling patterns
- **CSS-in-Rust**: Type-safe styling with Rust code
- **CSS custom properties**: Dynamic theming and configuration

Choose the approach that best fits your project's needs, considering factors like team preferences, performance requirements, and maintainability goals.