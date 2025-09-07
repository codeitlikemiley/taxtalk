use leptos::prelude::*;
use taxtalk_dynamic_table::{DynamicTable, TableColumn, TableRow, create_invoice_columns};
use taxtalk_ui_core::ComponentType;
use serde_json::json;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Example 1: Invoice line items
    let (invoice_columns, _) = signal(create_invoice_columns());
    let (invoice_rows, set_invoice_rows) = signal(vec![TableRow::new()]);
    
    // Example 2: Custom table with different column types
    let custom_columns = vec![
        TableColumn {
            key: "name".into(),
            label: "Name".into(),
            component: ComponentType::TextInput,
            width: Some("30%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "active".into(),
            label: "Active".into(),
            component: ComponentType::Boolean,
            width: Some("10%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "category".into(),
            label: "Category".into(),
            component: ComponentType::Select(vec![
                "Electronics".into(),
                "Clothing".into(),
                "Food".into(),
                "Books".into(),
            ]),
            width: Some("20%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "price".into(),
            label: "Price".into(),
            component: ComponentType::AmountInput,
            width: Some("20%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "stock".into(),
            label: "Stock".into(),
            component: ComponentType::NumberInput,
            width: Some("20%".into()),
            computed: None,
            readonly: false,
        },
    ];
    
    let (custom_columns_signal, _) = signal(custom_columns);
    let (custom_rows, set_custom_rows) = signal(vec![TableRow::new()]);
    
    // Example 3: Readonly table with pre-filled data
    let readonly_columns = vec![
        TableColumn {
            key: "date".into(),
            label: "Date".into(),
            component: ComponentType::TextInput,
            width: Some("25%".into()),
            computed: None,
            readonly: true,
        },
        TableColumn {
            key: "description".into(),
            label: "Description".into(),
            component: ComponentType::TextInput,
            width: Some("50%".into()),
            computed: None,
            readonly: true,
        },
        TableColumn {
            key: "amount".into(),
            label: "Amount".into(),
            component: ComponentType::AmountInput,
            width: Some("25%".into()),
            computed: None,
            readonly: true,
        },
    ];
    
    let mut readonly_row1 = TableRow::new();
    readonly_row1.cells.insert("date".into(), json!("2024-01-15"));
    readonly_row1.cells.insert("description".into(), json!("Office Supplies"));
    readonly_row1.cells.insert("amount".into(), json!(125.50));
    
    let mut readonly_row2 = TableRow::new();
    readonly_row2.cells.insert("date".into(), json!("2024-01-16"));
    readonly_row2.cells.insert("description".into(), json!("Software License"));
    readonly_row2.cells.insert("amount".into(), json!(299.00));
    
    let (readonly_columns_signal, _) = signal(readonly_columns);
    let (readonly_rows, _) = signal(vec![readonly_row1, readonly_row2]);
    
    view! {
        <div class="container">
            <h1>"DynamicTable Component Examples"</h1>
            
            <div class="example-section">
                <div class="example-title">"Invoice Line Items (with computed totals)"</div>
                <DynamicTable
                    columns=invoice_columns
                    rows=invoice_rows
                    on_change=Callback::new(move |rows| {
                        set_invoice_rows.set(rows.clone());
                        leptos::logging::log!("Invoice rows updated: {:?}", rows);
                    })
                    can_add=true
                    can_delete=true
                    min_rows=1
                    max_rows=10
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Product Inventory (various input types)"</div>
                <DynamicTable
                    columns=custom_columns_signal
                    rows=custom_rows
                    on_change=Callback::new(move |rows| {
                        set_custom_rows.set(rows.clone());
                        leptos::logging::log!("Custom rows updated: {:?}", rows);
                    })
                    can_add=true
                    can_delete=true
                    min_rows=2
                    max_rows=20
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Transaction History (readonly)"</div>
                <DynamicTable
                    columns=readonly_columns_signal
                    rows=readonly_rows
                    on_change=Callback::new(|_| {})
                    can_add=false
                    can_delete=false
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Current Data (Debug View)"</div>
                <div style="background: #f0f0f0; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 12px; overflow-x: auto;">
                    <strong>"Invoice Rows:"</strong>
                    <pre>{move || format!("{:#?}", invoice_rows.get())}</pre>
                </div>
            </div>
        </div>
    }
}