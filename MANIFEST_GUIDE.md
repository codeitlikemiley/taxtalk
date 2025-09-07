# TaxTalk Plugin Manifest Guide

This guide explains how to define plugin manifests for the TaxTalk tokenization system. The manifest controls how your plugin integrates with the natural language processing system and what UI components are used for different token types.

## Manifest Structure

```json
{
  "id": "plugin_id",
  "name": "Plugin Name",
  "version": "0.1.0",
  "entry": "plugin.wasm",
  "capabilities": ["http", "kv"],
  "commands": [...],
  "token_types": [...],
  "dependencies": [],
  "config_schema": {...},
  "required_capabilities": [],
  "metadata": {...}
}
```

## Commands Section

Commands define the actions your plugin can handle. Each command specifies its required and optional tokens.

### Command Structure

```json
{
  "command": "payment",
  "aliases": ["pay", "receive", "collect"],
  "description": "Record a payment received from a customer",
  "requirements": {
    "required": [...],
    "optional": [...],
    "defaults": [...]
  }
}
```

### Token Requirements

Each token requirement defines:
- `token_type`: The type of token (e.g., "entity_ref", "amount", "date")
- `description`: Human-readable description
- `multiple`: Whether multiple values are allowed
- `component`: The UI component to use (optional, defaults based on token_type)
- `cardinality`: How many values are expected

```json
{
  "token_type": "entity_ref",
  "description": "Customer who made the payment",
  "multiple": false,
  "component": "entity-selector",
  "cardinality": "single"
}
```

## Token Types and Components

### Built-in Token Types

The system automatically maps token types to appropriate UI components:

| Token Type | Default Component | Description |
|------------|------------------|-------------|
| `entity_ref` | `entity-selector` | Select existing entities with "Create new" option |
| `amount` | `amount-input` | Numeric input with currency selector |
| `money` | `amount-input` | Same as amount |
| `date` | `date-picker` | Date selection with today as default |
| `date_range` | `date-range` | Start and end date selection |
| `text` | `text-input` | Free-form text input |
| `description` | `text-input` | Multi-line text input |
| `number` | `number-input` | Numeric input |
| `quantity` | `number-input` | Integer input for quantities |
| `boolean` | `boolean` | Yes/No toggle |
| `yes_no` | `boolean` | Same as boolean |
| `payment_method` | `select` | Dropdown with predefined options |
| `identifier` | `text-input` | Reference numbers, IDs |

### Component Types

#### 1. Entity Selector (`entity-selector`)
```json
{
  "token_type": "client",
  "component": "entity-selector",
  "cardinality": "single"
}
```
- Shows searchable dropdown of existing entities
- Includes "Create new" option at the top
- Supports single or multiple selection based on cardinality

#### 2. Amount Input (`amount-input`)
```json
{
  "token_type": "amount",
  "component": "amount-input",
  "default_currency": "PHP"
}
```
- Numeric input with decimal support
- Currency selector (PHP, USD, EUR)
- Smart parsing: "1000", "1000.00", "1000 PHP"

#### 3. Date Picker (`date-picker`)
```json
{
  "token_type": "date",
  "component": "date-picker",
  "default": "today"
}
```
- Calendar widget with today as default
- Supports natural language: "today", "tomorrow", "yesterday"
- Multiple formats: "08/17/1988", "08-17-1988", "2024-09-07"

#### 4. Text Input (`text-input`)
```json
{
  "token_type": "description",
  "component": "text-input",
  "multiline": true
}
```
- Single or multi-line text input
- Used for descriptions, notes, references

#### 5. Select Dropdown (`select`)
```json
{
  "token_type": "payment_method",
  "component": "select",
  "options": ["cash", "check", "bank_transfer", "gcash", "maya"]
}
```
- Dropdown with predefined options
- Can be searchable for long lists

#### 6. Multi-Select (`multi-select`)
```json
{
  "token_type": "tags",
  "component": "multi-select",
  "options": ["urgent", "pending", "completed"],
  "cardinality": "multiple"
}
```
- Checkbox list or tag selector
- Allows multiple selections

#### 7. Boolean Toggle (`boolean`)
```json
{
  "token_type": "taxable",
  "component": "boolean",
  "default": true
}
```
- Simple yes/no toggle
- Checkbox or switch UI

### Cardinality Options

Define how many values can be selected:

```json
// Single value only
"cardinality": "single"

// Multiple values allowed
"cardinality": "multiple"

// Exactly N values required
"cardinality": { "exact": 3 }

// Range of values
"cardinality": { "min": 1, "max": 5 }
```

## Complete Example: Payment Plugin

```json
{
  "id": "payments",
  "name": "Payments Management Plugin",
  "version": "0.1.0",
  "entry": "payments.wasm",
  "capabilities": ["http", "kv"],
  "commands": [
    {
      "command": "payment",
      "aliases": ["pay", "receive", "collect"],
      "description": "Record a payment received from a customer",
      "requirements": {
        "required": [
          {
            "token_type": "entity_ref",
            "description": "Customer who made the payment",
            "multiple": false,
            "component": "entity-selector",
            "cardinality": "single",
            "entity_filter": "client"
          },
          {
            "token_type": "amount",
            "description": "Payment amount",
            "multiple": false,
            "component": "amount-input",
            "default_currency": "PHP"
          }
        ],
        "optional": [
          {
            "token_type": "date",
            "description": "Payment date",
            "multiple": false,
            "component": "date-picker",
            "default": "today"
          },
          {
            "token_type": "identifier",
            "description": "Invoice or reference number",
            "multiple": false,
            "component": "text-input",
            "pattern": "^(INV|PAY)-\\d{6}$"
          },
          {
            "token_type": "payment_method",
            "description": "Payment method",
            "multiple": false,
            "component": "select",
            "options": ["cash", "check", "bank_transfer", "gcash", "maya", "credit_card"],
            "default": "cash"
          },
          {
            "token_type": "notes",
            "description": "Additional notes",
            "multiple": false,
            "component": "text-input",
            "multiline": true
          }
        ],
        "defaults": [
          {
            "token_type": "date",
            "value": "today"
          },
          {
            "token_type": "payment_method",
            "value": "cash"
          }
        ]
      }
    }
  ],
  "token_types": [
    {
      "type_name": "payment:reference",
      "patterns": ["PAY-\\d+", "REC-\\d+"],
      "creator_class": "PaymentReferenceCreator"
    }
  ],
  "config_schema": {
    "type": "object",
    "properties": {
      "default_currency": {
        "type": "string",
        "default": "PHP",
        "enum": ["PHP", "USD", "EUR"]
      },
      "require_reference": {
        "type": "boolean",
        "default": false,
        "description": "Require reference number for all payments"
      }
    }
  }
}
```

## UI Component Behavior

### Focus Management

The system automatically manages focus flow:
1. Main input field → Token type selector → Entity/Value selector → Back to main input
2. `Tab` navigates forward, `Shift+Tab` navigates backward
3. `Escape` cancels current selection and returns focus to main input

### Keyboard Shortcuts

- `@` - Triggers token selection
- `Enter` - Confirms selection (single mode)
- `Cmd/Alt + Enter` - Confirms selection (multi mode) or submits command
- `Escape` - Cancels current operation
- `Arrow Up/Down` - Navigate options
- `Ctrl+N/P` - Alternative navigation

### Smart Parsing

The backend can intelligently parse values without explicit @ tokens:
- "1000 PHP" → amount token with PHP currency
- "tomorrow" → date token for next day
- "Juan dela Cruz" → attempts entity resolution

### Visual Feedback

Selected tokens appear as colored tags:
- Blue: Commands (create, update, delete)
- Green: Entities (invoice, payment)
- Purple: Selected values (clients, amounts, dates)

### Progress Indicator

A circular progress indicator shows completion status:
- Displays fraction of required tokens provided (e.g., "2/4")
- Green when complete and ready to submit
- Blue when in progress
- Shows "Ready to submit (Cmd+Enter)" when complete

## Best Practices

1. **Use appropriate components** - Choose components that match the data type
2. **Provide defaults** - Set sensible defaults for optional tokens
3. **Add descriptions** - Clear descriptions help users understand requirements
4. **Define cardinality** - Be explicit about single vs multiple selection
5. **Use patterns** - Add regex patterns for validation where appropriate
6. **Consider localization** - Support local formats (dates, currency, etc.)
7. **Test focus flow** - Ensure smooth keyboard navigation
8. **Handle edge cases** - Support "Create new" for entities that don't exist

## Philippine-Specific Considerations

For Philippine tax compliance:
- Default currency should be PHP
- Support peso symbol (₱) in amount displays
- Date formats: MM/DD/YYYY (US style) or DD/MM/YYYY (international)
- Payment methods should include GCash and Maya
- Tax tokens: VAT (12%), EWT (various rates), Senior/PWD discount (20%)

## Testing Your Manifest

1. Validate JSON syntax
2. Test each command with sample inputs
3. Verify component rendering for each token type
4. Check keyboard navigation flow
5. Test "Create new" functionality for entities
6. Verify default values are applied
7. Test multi-select behavior with cardinality limits
8. Ensure validation patterns work correctly

## Token Resolution Flow

1. User types command: "create invoice @"
2. System shows token type selector (client, product, etc.)
3. User selects "client"
4. System shows entity selector with search
5. User can select existing or create new
6. Token is resolved and shown as tag
7. Input field preserves context: "create invoice"
8. Process repeats for additional tokens
9. When all required tokens are provided, submit is enabled