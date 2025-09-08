# Smart Suggestions Component

An AI-powered suggestion component with machine learning capabilities that learns from user patterns and provides intelligent recommendations.

## Features

- 🧠 **Machine Learning**: Learns from user interactions and improves suggestions over time
- ⏰ **Time-based Suggestions**: Context-aware suggestions based on time of day and date
- 🇵🇭 **Philippine Context**: Built-in understanding of BIR deadlines, VAT, and withholding tax requirements
- 📊 **Pattern Recognition**: Detects and suggests based on user behavior patterns
- 💾 **Persistent Learning**: Saves ML state to localStorage for continuous improvement
- 🎯 **Confidence Scores**: Shows prediction confidence for each suggestion
- 🏷️ **Categorized Suggestions**: Organized by type (Command, Entity, Time-based, etc.)
- ⌨️ **Keyboard Navigation**: Full keyboard support with arrow keys, Enter, and Escape

## Installation

```toml
[dependencies]
smart-suggestions = { path = "../smart-suggestions" }
```

## Usage

```rust
use smart_suggestions::{SmartSuggestions, MLConfig, Suggestion};
use leptos::prelude::*;

#[component]
fn MyApp() -> impl IntoView {
    let (context, set_context) = signal("invoice".to_string());
    let (input, set_input) = signal(String::new());
    
    // Configure ML settings
    let ml_config = MLConfig {
        max_suggestions: 8,
        min_confidence: 0.3,
        learning_rate: 0.1,
        pattern_threshold: 0.4,
        history_limit: 100,
        enable_time_patterns: true,
        enable_philippine_context: true,
    };
    
    view! {
        <SmartSuggestions
            context=context
            current_input=input
            config=ml_config
            enable_learning=true
            on_select=move |suggestion: Suggestion| {
                // Handle suggestion selection
                log!("Selected: {}", suggestion.text);
            }
        />
    }
}
```

## Configuration

### MLConfig Options

- `max_suggestions`: Maximum number of suggestions to display (default: 10)
- `min_confidence`: Minimum confidence threshold for suggestions (0.0-1.0, default: 0.3)
- `learning_rate`: How quickly the system learns from interactions (0.0-1.0, default: 0.1)
- `pattern_threshold`: Minimum confidence for pattern recognition (0.0-1.0, default: 0.5)
- `history_limit`: Maximum number of interactions to store (default: 1000)
- `enable_time_patterns`: Enable time-based suggestions (default: true)
- `enable_philippine_context`: Enable Philippine business context (default: true)

## Suggestion Categories

- **Command**: Action-based suggestions (e.g., "Generate VAT return")
- **Entity**: Entity-related suggestions (e.g., client names, products)
- **Value**: Value suggestions (e.g., common amounts, tax rates)
- **TimeBased**: Time-sensitive suggestions (e.g., month-end tasks)
- **Contextual**: Context-specific suggestions based on current activity
- **Historical**: Based on user's past behavior
- **Philippine**: Philippine-specific business suggestions (BIR, VAT, etc.)

## Machine Learning Algorithm

The component uses a multi-layered approach to generate suggestions:

1. **Frequency Analysis**: Tracks how often certain suggestions are selected in specific contexts
2. **Context Association**: Links inputs to contexts for better predictions
3. **Pattern Detection**: Identifies time-of-day, day-of-week, and periodic patterns
4. **Confidence Calculation**: Combines frequency, recency, and pattern strength
5. **Philippine Business Logic**: Special handling for BIR deadlines, tax periods, and local requirements

## Philippine Business Context

The component includes built-in understanding of:

- BIR filing deadlines (VAT returns on 20th, quarterly on 25th)
- Common tax calculations (12% VAT, EWT rates)
- Business cycles (month-end, payroll periods)
- Local payment methods (GCash, bank transfer)
- Professional service handling (10% EWT)

## Keyboard Shortcuts

- **↑/↓**: Navigate through suggestions
- **Enter**: Select current suggestion
- **Escape**: Close suggestions panel
- **Tab**: Move to next focusable element

## Development

### Running the Demo

```bash
cd smart-suggestions
trunk serve --port 8090
```

Visit http://localhost:8090 to see the interactive demo.

### Building

```bash
cargo build --target wasm32-unknown-unknown
```

## Performance

- Efficient virtual scrolling for large suggestion lists
- Debounced input processing
- Lazy loading of ML models
- Minimal bundle size (~50KB gzipped)
- Sub-100ms suggestion generation

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Requires WASM support and localStorage

## License

Part of the TaxTalk component library.