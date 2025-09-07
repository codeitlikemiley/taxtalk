# DynamicTable Component

A flexible, reusable table component for Leptos that supports different column types, computed fields, and dynamic row management.

## Features

- **Multiple Input Types**: Text, Number, Amount, Date, Select, Boolean
- **Computed Fields**: Automatic calculation of fields based on formulas
- **Dynamic Rows**: Add/delete rows with min/max constraints
- **Customizable Columns**: Define width, readonly status, and computation rules
- **Pre-built Templates**: Invoice, Expense, and Inventory table configurations

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
taxtalk-dynamic-table = { path = "../dynamic-table" }
```

## Basic Usage

```rust
use leptos::prelude::*;
use taxtalk_dynamic_table::{DynamicTable, TableColumn, TableRow, create_invoice_columns};
use taxtalk_ui_core::ComponentType;

#[component]
fn InvoiceForm() -> impl IntoView {
    // Use pre-built invoice columns
    let columns = Signal::derive(|| create_invoice_columns());
    
    // Initialize with empty rows
    let (rows, set_rows) = signal(vec![TableRow::new()]);
    let rows_signal = Signal::derive(move || rows.get());
    
    view! {
        <DynamicTable
            columns=columns
            rows=rows_signal
            on_change=Callback::new(move |new_rows| {
                set_rows.set(new_rows);
                // Process the data...
            })
            can_add=true
            can_delete=true
            min_rows=1
            max_rows=20
        />
    }
}
```

## Pre-built Table Configurations

### Invoice Line Items
```rust
use taxtalk_dynamic_table::create_invoice_columns;

// Creates columns for:
// - Description (text)
// - Unit Type (select: pcs, kg, g, L, etc.)
// - Quantity (number)
// - Unit Price (amount)
// - Amount (computed: quantity × unit_price)
// - VAT Rate (select: 0%, 12%, Exempt)
// - Subtotal (computed with VAT)
```

### Expense Tracking
```rust
use taxtalk_dynamic_table::create_expense_columns;

// Creates columns for:
// - Date (date picker)
// - Vendor (text)
// - Description (text)
// - Category (select: Office Supplies, Travel, etc.)
// - Amount (amount)
// - Paid (checkbox)
```

### Inventory Management
```rust
use taxtalk_dynamic_table::create_inventory_columns;

// Creates columns for:
// - SKU (text)
// - Product Name (text)
// - Stock Quantity (number)
// - Unit (select: pcs, kg, box)
// - Unit Cost (amount)
// - Selling Price (amount)
// - Reorder Level (number)
```

## Custom Table Definition

```rust
let custom_columns = vec![
    TableColumn {
        key: "item_code".into(),
        label: "Item Code".into(),
        component: ComponentType::TextInput,
        width: Some("20%".into()),
        computed: None,
        readonly: false,
    },
    TableColumn {
        key: "quantity".into(),
        label: "Qty".into(),
        component: ComponentType::NumberInput,
        width: Some("10%".into()),
        computed: None,
        readonly: false,
    },
    TableColumn {
        key: "price".into(),
        label: "Price".into(),
        component: ComponentType::AmountInput,
        width: Some("15%".into()),
        computed: None,
        readonly: false,
    },
    TableColumn {
        key: "total".into(),
        label: "Total".into(),
        component: ComponentType::AmountInput,
        width: Some("15%".into()),
        computed: Some("quantity * price".into()),
        readonly: true,  // Computed fields should be readonly
    },
];
```

## Working with Table Data

### Setting Initial Data
```rust
let initial_row = {
    let mut row = TableRow::new();
    row.cells.insert("description".to_string(), serde_json::json!("Product A"));
    row.cells.insert("quantity".to_string(), serde_json::json!(2.0));
    row.cells.insert("unit_price".to_string(), serde_json::json!(100.0));
    row
};

let (rows, set_rows) = signal(vec![initial_row]);
```

### Processing Table Data
```rust
on_change=Callback::new(move |rows: Vec<TableRow>| {
    // Calculate total
    let total: f64 = rows.iter()
        .filter_map(|row| {
            row.cells.get("amount")
                .and_then(|v| v.as_f64())
        })
        .sum();
    
    // Convert to your domain model
    let invoice_items: Vec<InvoiceItem> = rows.iter()
        .map(|row| InvoiceItem {
            description: row.cells.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            quantity: row.cells.get("quantity")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            // ... etc
        })
        .collect();
})
```

## Component Props

| Prop | Type | Description | Default |
|------|------|-------------|---------|
| `columns` | `Signal<Vec<TableColumn>>` | Column definitions | Required |
| `rows` | `Signal<Vec<TableRow>>` | Current table data | Required |
| `on_change` | `Callback<Vec<TableRow>, ()>` | Called when data changes | Required |
| `can_add` | `bool` | Allow adding new rows | `true` |
| `can_delete` | `bool` | Allow deleting rows | `true` |
| `min_rows` | `usize` | Minimum number of rows | `1` |
| `max_rows` | `usize` | Maximum number of rows | `100` |

## Column Types

### ComponentType Options
- `TextInput` - Standard text input
- `NumberInput` - Numeric input (integers)
- `AmountInput` - Currency input (2 decimal places)
- `DatePicker` - Date selection
- `Boolean` - Checkbox
- `Select(Vec<String>)` - Dropdown with options

### Computed Fields

Supported computation patterns:
- `"quantity * unit_price"` - Multiplies quantity by unit_price
- `"amount_with_vat"` - Adds VAT to amount based on vat_rate field

## Styling

The component uses CSS classes with `tui-dt-` prefix to avoid conflicts:

```css
.tui-dt-table { /* Main table */ }
.tui-dt-header-cell { /* Header cells */ }
.tui-dt-cell { /* Data cells */ }
.tui-dt-input { /* Input fields */ }
.tui-dt-select { /* Select dropdowns */ }
.tui-dt-checkbox { /* Checkboxes */ }
.tui-dt-add-btn { /* Add row button */ }
.tui-dt-delete-btn { /* Delete row button */ }
.tui-dt-total { /* Total display */ }
```

## Example: Complete Invoice Form

```rust
use leptos::prelude::*;
use taxtalk_dynamic_table::{DynamicTable, create_invoice_columns, TableRow};
use serde_json::json;

#[component]
pub fn InvoiceEntry() -> impl IntoView {
    let columns = Signal::derive(|| create_invoice_columns());
    let (rows, set_rows) = signal(vec![TableRow::new()]);
    let rows_signal = Signal::derive(move || rows.get());
    
    let (invoice_total, set_invoice_total) = signal(0.0);
    
    view! {
        <div class="invoice-form">
            <h2>"Invoice Entry"</h2>
            
            <DynamicTable
                columns=columns
                rows=rows_signal
                on_change=Callback::new(move |new_rows| {
                    // Calculate total
                    let total: f64 = new_rows.iter()
                        .filter_map(|row| {
                            row.cells.get("subtotal")
                                .and_then(|v| v.as_f64())
                        })
                        .sum();
                    
                    set_invoice_total.set(total);
                    set_rows.set(new_rows);
                })
                can_add=true
                can_delete=true
                min_rows=1
                max_rows=50
            />
            
            <div class="invoice-summary">
                <h3>"Total: ₱" {move || format!("{:.2}", invoice_total.get())}</h3>
            </div>
        </div>
    }
}
```

## Testing

Run the demo:
```bash
cd components/dynamic-table
trunk serve --port 8082
```

Then visit http://localhost:8082 to see the component in action with different table types.