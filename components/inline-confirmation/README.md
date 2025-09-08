# Inline Confirmation Component

Real-time validation with inline feedback and confirmation states for TaxTalk.

## Features

- ✅ Real-time validation as you type
- 🔒 Password strength indicator
- ⚡ Async validation support
- 🇵🇭 Philippine-specific validations (TIN, phone, ZIP)
- ✨ Animation feedback (shake on error)
- ⏱️ Timed confirmations

## Installation

```toml
[dependencies]
inline-confirmation = { path = "../inline-confirmation" }
```

## Usage

### Basic Input with Validation

```rust
use leptos::prelude::*;
use inline_confirmation::*;

#[component]
fn MyForm() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    
    view! {
        <InlineConfirmation
            value=email
            set_value=set_email
            input_type=InputType::Email
            label="Email Address"
            placeholder="your@email.com"
            rules=vec![
                ValidationRule::Required("Email is required".to_string()),
                ValidationRule::Email("Invalid email format".to_string()),
            ]
        />
    }
}
```

### Password with Strength Indicator

```rust
<PasswordInput
    value=password
    set_value=set_password
    show_strength=true
    on_valid=move |pwd| {
        // Handle valid password
    }
/>
```

### Philippine TIN Validation

```rust
<InlineConfirmation
    value=tin
    set_value=set_tin
    input_type=InputType::TIN
    label="Tax Identification Number"
    placeholder="123-456-789"
    rules=vec![
        ValidationRule::PhilippineTIN("Invalid TIN format".to_string()),
    ]
/>
```

## Validation Rules

- `Required` - Field must not be empty
- `MinLength` - Minimum character length
- `MaxLength` - Maximum character length
- `Pattern` - Regex pattern matching
- `Email` - Valid email format
- `Url` - Valid URL format
- `Range` - Number within min/max range
- `PhilippineTIN` - Valid Philippine TIN (with check digit)
- `PhilippinePhone` - Valid PH mobile number
- `PhilippineZip` - Valid PH ZIP code

## Input Types

- `Text` - Standard text input
- `Email` - Email input with validation
- `Password` - Password with visibility toggle
- `Number` - Numeric input
- `Tel` - Phone number
- `Url` - URL input
- `Currency` - Philippine Peso formatting
- `Percentage` - Percentage formatting
- `TIN` - Tax ID formatting
- `Date/Time/DateTime` - Date/time inputs

## Configuration

```rust
InlineConfirmConfig {
    validate_on_blur: true,        // Validate when field loses focus
    validate_on_change: false,      // Validate on every keystroke
    validation_debounce: 300,       // Debounce validation (ms)
    show_success_icon: true,        // Show ✓ on valid
    show_error_icon: true,          // Show ✗ on error
    show_loading_spinner: true,     // Show spinner during async validation
    require_confirmation: true,     // Require Enter to confirm
    confirmation_timeout: 3000,     // Auto-cancel confirmation (ms)
    auto_focus_on_error: true,      // Focus field on error
    shake_on_error: true,           // Shake animation on error
}
```

## Validation States

- `Idle` - No validation performed yet
- `Validating` - Async validation in progress
- `Valid(message)` - Validation passed
- `Invalid(error)` - Validation failed
- `Warning(message)` - Valid but needs attention

## Styling

The component uses CSS classes with `ic-` prefix:

```css
.ic-container         /* Main wrapper */
.ic-input            /* Input field */
.ic-input-valid      /* Valid state */
.ic-input-invalid    /* Invalid state */
.ic-input-warning    /* Warning state */
.ic-message-success  /* Success message */
.ic-message-error    /* Error message */
.ic-strength-strong  /* Strong password */
.ic-strength-medium  /* Medium password */
.ic-strength-weak    /* Weak password */
```

## Demo

Run the demo:

```bash
cd components/inline-confirmation
trunk serve --open
```

Visit http://localhost:8093