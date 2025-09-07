use leptos::prelude::*;
use taxtalk_dynamic_table::{
    DynamicTable, TableRow, 
    create_invoice_columns, create_expense_columns, create_inventory_columns
};
use serde_json::Value;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Table type selector
    let (table_type, set_table_type) = signal("invoice".to_string());
    
    // Dynamic columns based on selected type
    let columns = Signal::derive(move || {
        match table_type.get().as_str() {
            "invoice" => create_invoice_columns(),
            "expense" => create_expense_columns(),
            "inventory" => create_inventory_columns(),
            _ => create_invoice_columns(),
        }
    });
    
    // Initialize with sample data for invoice
    let initial_invoice_row = {
        let mut row = TableRow::new();
        row.cells.insert("description".to_string(), Value::from("Widget A"));
        row.cells.insert("unit_type".to_string(), Value::from("pcs"));
        row.cells.insert("quantity".to_string(), Value::from(2.0));
        row.cells.insert("unit_price".to_string(), Value::from(100.0));
        row.cells.insert("vat_rate".to_string(), Value::from("12"));
        row
    };
    
    let (rows_state, set_rows) = signal(vec![initial_invoice_row]);
    let rows = Signal::derive(move || rows_state.get());
    
    view! {
        <div class="container">
            <h1>"DynamicTable Component Demo"</h1>
            
            <div class="example-section">
                <div class="example-title">"Select Table Type"</div>
                <div style="margin-bottom: 20px;">
                    <label>"Choose table type: "</label>
                    <select 
                        on:change=move |e| {
                            let value = event_target_value(&e);
                            set_table_type.set(value.clone());
                            // Reset rows when changing type
                            let new_row = match value.as_str() {
                                "expense" => {
                                    let mut row = TableRow::new();
                                    row.cells.insert("date".to_string(), Value::from("2024-01-15"));
                                    row.cells.insert("vendor".to_string(), Value::from("Office Depot"));
                                    row.cells.insert("category".to_string(), Value::from("Office Supplies"));
                                    row
                                },
                                "inventory" => {
                                    let mut row = TableRow::new();
                                    row.cells.insert("sku".to_string(), Value::from("SKU001"));
                                    row.cells.insert("name".to_string(), Value::from("Sample Product"));
                                    row.cells.insert("unit".to_string(), Value::from("pcs"));
                                    row
                                },
                                _ => {
                                    let mut row = TableRow::new();
                                    row.cells.insert("description".to_string(), Value::from(""));
                                    row.cells.insert("unit_type".to_string(), Value::from("pcs"));
                                    row
                                }
                            };
                            set_rows.set(vec![new_row]);
                        }
                        style="padding: 8px; font-size: 14px;"
                    >
                        <option value="invoice">"Invoice Line Items"</option>
                        <option value="expense">"Expense Tracking"</option>
                        <option value="inventory">"Inventory Management"</option>
                    </select>
                </div>
                
                <DynamicTable
                    columns=columns
                    rows=rows
                    on_change=Callback::new(move |new_rows: Vec<TableRow>| {
                        set_rows.set(new_rows.clone());
                        leptos::logging::log!("Table updated: {:?}", new_rows);
                    })
                    can_add=true
                    can_delete=true
                    min_rows=1
                    max_rows=20
                />
                
                <div style="margin-top: 20px; padding: 15px; background: #f8f9fa; border-radius: 8px;">
                    <strong>"Table Data (JSON):"</strong>
                    <pre style="overflow-x: auto; font-size: 12px; margin-top: 10px;">
                        {move || {
                            serde_json::to_string_pretty(&rows_state.get())
                                .unwrap_or_else(|_| "Error serializing data".to_string())
                        }}
                    </pre>
                </div>
            </div>
        </div>
    }
}