# Guided Chat Component

Step-by-step guided workflows for TaxTalk with built-in validation and progress tracking.

## Features

- 🚀 **Step-by-step workflows** - Guide users through complex processes
- ✅ **Smart validation** - Real-time input validation with helpful feedback  
- 📊 **Progress tracking** - Visual progress bar and step indicators
- ⚡ **Quick replies** - Predefined response buttons for common inputs
- ↩️ **Back navigation** - Allow users to go back and modify previous answers
- 🇵🇭 **Philippine-specific** - Built-in VAT, EWT, and BIR compliance workflows
- 💬 **Typing indicators** - Show when assistant is processing
- 📱 **Mobile responsive** - Works great on all screen sizes

## Installation

```toml
[dependencies]
guided-chat = { path = "../guided-chat" }
```

## Usage

### Basic Guided Chat

```rust
use leptos::prelude::*;
use guided_chat::*;

#[component]
fn MyApp() -> impl IntoView {
    // Use built-in workflow
    let workflow = Workflow::invoice_workflow();
    
    // Handle completion
    let handle_complete = move |data: HashMap<String, serde_json::Value>| {
        println!("Workflow completed with data: {:?}", data);
    };
    
    view! {
        <GuidedChat
            workflow=workflow
            on_complete=handle_complete
        />
    }
}
```

### Custom Workflow

```rust
let custom_workflow = Workflow {
    id: "custom_flow".to_string(),
    name: "Custom Workflow".to_string(),
    description: "My custom workflow".to_string(),
    steps: vec![
        WorkflowStep {
            id: "name".to_string(),
            name: "Name".to_string(),
            prompt: "What's your name?".to_string(),
            input_type: InputType::Text,
            validation: Some(ValidationConfig {
                min_length: Some(2),
                max_length: Some(50),
                ..Default::default()
            }),
            required: true,
            depends_on: vec![],
            quick_replies: vec![],
            help_text: Some("Enter your full name".to_string()),
        },
        WorkflowStep {
            id: "amount".to_string(),
            name: "Amount".to_string(),
            prompt: "How much?".to_string(),
            input_type: InputType::Currency,
            validation: Some(ValidationConfig {
                min_value: Some(0.01),
                ..Default::default()
            }),
            required: true,
            depends_on: vec!["name".to_string()],
            quick_replies: vec![
                QuickReply {
                    label: "₱1,000".to_string(),
                    value: "1000".to_string(),
                    style: None,
                },
            ],
            help_text: None,
        },
    ],
    on_complete: None,
};
```

## Built-in Workflows

### Invoice Workflow
- Client selection
- Product/service selection
- Amount entry
- VAT type selection (inclusive/exclusive/exempt)
- Due date setting

### Payment Workflow
- Invoice or client payment selection
- Payment amount
- Payment method (cash, check, bank transfer, GCash, Maya)
- Withholding tax (EWT) calculation

## Input Types

- `Text` - Standard text input
- `Number` - Numeric input
- `Currency` - Philippine Peso formatting
- `Date` - Date picker
- `Select` - Single selection from options
- `MultiSelect` - Multiple selection
- `EntityRef` - Reference to entity (client, product, etc.)
- `Boolean` - Yes/no input
- `File` - File upload

## Configuration

```rust
GuidedChatConfig {
    placeholder: "Type your answer...".to_string(),
    welcome_message: Some("Welcome!".to_string()),
    completion_message: Some("Thank you!".to_string()),
    show_timestamps: true,
    show_step_progress: true,
    allow_back_navigation: true,
    auto_scroll: true,
    enable_markdown: false,
    max_messages: Some(100),
    typing_indicator_delay: 500,
}
```

## Validation

Each step can have validation rules:

```rust
ValidationConfig {
    min_length: Some(2),        // Minimum text length
    max_length: Some(100),      // Maximum text length
    pattern: Some(r"^\d+$"),    // Regex pattern
    min_value: Some(0.0),       // Minimum numeric value
    max_value: Some(1000000.0), // Maximum numeric value
    custom_validator: None,      // Custom validation function
}
```

## Quick Replies

Provide quick response options:

```rust
quick_replies: vec![
    QuickReply {
        label: "Yes".to_string(),
        value: "yes".to_string(),
        style: Some(ActionStyle::Success),
    },
    QuickReply {
        label: "No".to_string(),
        value: "no".to_string(),
        style: Some(ActionStyle::Danger),
    },
]
```

## Step Dependencies

Steps can depend on other steps being completed:

```rust
WorkflowStep {
    id: "vat_amount".to_string(),
    // ...
    depends_on: vec!["amount".to_string(), "vat_type".to_string()],
    // ...
}
```

## Styling

The component uses CSS classes with `gc-` prefix:

```css
.gc-container        /* Main container */
.gc-progress         /* Progress section */
.gc-messages         /* Messages area */
.gc-message-user     /* User messages */
.gc-message-assistant /* Assistant messages */
.gc-input-area       /* Input section */
.gc-quick-replies    /* Quick reply buttons */
```

## Demo

Run the demo:

```bash
cd components/guided-chat
trunk serve --open
```

Visit http://localhost:8080

## Philippine Tax Features

- **VAT Handling**: Inclusive/exclusive/exempt options
- **EWT Calculation**: 2% for goods, 5% for services
- **Payment Methods**: GCash, Maya, bank transfer support
- **Currency Formatting**: Philippine Peso (₱) formatting
- **Filipino Language**: Supports "oo" (yes) and "hindi" (no)