use leptos::prelude::*;
use serde_json::Value;
use crate::types::*;
use crate::cell::TableCell;
use taxtalk_ui_core::ComponentType;

#[component]
pub fn DynamicTable(
    columns: Signal<Vec<TableColumn>>,
    rows: Signal<Vec<TableRow>>,
    on_change: Callback<Vec<TableRow>, ()>,
    #[prop(default = true)] can_add: bool,
    #[prop(default = true)] can_delete: bool,
    #[prop(default = 1)] min_rows: usize,
    #[prop(default = 100)] max_rows: usize,
) -> impl IntoView {
    let (local_rows, set_local_rows) = signal(rows.get());
    
    // Ensure minimum rows
    Effect::new(move |_| {
        let current_rows = local_rows.get();
        if current_rows.len() < min_rows {
            let mut new_rows = current_rows.clone();
            while new_rows.len() < min_rows {
                new_rows.push(TableRow::new());
            }
            set_local_rows.set(new_rows.clone());
            on_change.run(new_rows);
        }
    });
    
    // Calculate computed fields
    let calculate_computed = move |row: &mut TableRow, columns: &[TableColumn]| {
        for column in columns {
            if let Some(computed) = &column.computed {
                match computed.as_str() {
                    // Invoice calculation: quantity * unit_price
                    "quantity * unit_price" => {
                        let quantity = row.cells.get("quantity")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let unit_price = row.cells.get("unit_price")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        row.cells.insert("amount".into(), Value::from(quantity * unit_price));
                    },
                    // VAT calculation
                    "amount_with_vat" => {
                        let amount = row.cells.get("amount")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let vat_rate = row.cells.get("vat_rate")
                            .and_then(|v| v.as_str())
                            .unwrap_or("0");
                        
                        let subtotal = match vat_rate {
                            "12" => amount * 1.12,  // 12% VAT
                            "Exempt" | "0" => amount,
                            _ => amount,
                        };
                        row.cells.insert("subtotal".into(), Value::from(subtotal));
                    },
                    // Legacy support for old pattern
                    "quantity * price" => {
                        let quantity = row.cells.get("quantity")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let price = row.cells.get("price")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        row.cells.insert(column.key.clone(), Value::from(quantity * price));
                    },
                    _ => {}
                }
            }
        }
    };
    
    // Calculate grand total
    let grand_total = Memo::new(move |_| {
        let rows = local_rows.get();
        let columns = columns.get();
        
        // Find column with key "total" or any computed column
        columns.iter()
            .find(|col| col.key == "total" || col.computed.is_some())
            .map(|total_col| {
                rows.iter()
                    .map(|row| {
                        row.cells.get(&total_col.key)
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0)
                    })
                    .sum::<f64>()
            })
            .unwrap_or(0.0)
    });
    
    let add_row = move |_| {
        let current_rows = local_rows.get();
        if current_rows.len() < max_rows {
            let mut new_rows = current_rows.clone();
            new_rows.push(TableRow::new());
            set_local_rows.set(new_rows.clone());
            on_change.run(new_rows);
        }
    };
    
    let delete_row = move |row_id: String| {
        let current_rows = local_rows.get();
        if current_rows.len() > min_rows {
            let mut new_rows = current_rows.clone();
            new_rows.retain(|r| r.id != row_id);
            set_local_rows.set(new_rows.clone());
            on_change.run(new_rows);
        }
    };
    
    let update_cell = move |row_id: String, column_key: String, value: Value| {
        let mut current_rows = local_rows.get();
        if let Some(row) = current_rows.iter_mut().find(|r| r.id == row_id) {
            row.cells.insert(column_key, value);
            calculate_computed(row, &columns.get());
        }
        set_local_rows.set(current_rows.clone());
        on_change.run(current_rows);
    };
    
    view! {
        <div class="tui-dt-component">
            <div class="tui-dt-wrapper">
                <table class="tui-dt-table">
                    <thead class="tui-dt-header">
                        <tr>
                            {move || columns.get().iter().map(|column| {
                                let width_style = column.width.as_ref().map(|w| format!("width: {}", w)).unwrap_or_default();
                                view! {
                                    <th
                                        class="tui-dt-header-cell"
                                        style=width_style
                                    >
                                        {column.label.clone()}
                                    </th>
                                }
                            }).collect::<Vec<_>>()}
                            <Show
                                when=move || can_delete
                                fallback=|| ()
                            >
                                <th class="tui-dt-header-cell tui-dt-actions">
                                    "Actions"
                                </th>
                            </Show>
                        </tr>
                    </thead>
                    <tbody class="tui-dt-body">
                        {move || local_rows.get().iter().map(|row| {
                            let row_id = row.id.clone();
                            let row_cells = row.cells.clone();
                            
                            view! {
                                <tr class="tui-dt-row">
                                    {columns.get().iter().map(|column| {
                                        let col_key = column.key.clone();
                                        let row_id_clone = row_id.clone();
                                        let col_key_clone = col_key.clone();
                                        let is_readonly = column.readonly;
                                        let component = column.component.clone();
                                        let current_value = row_cells.get(&col_key).cloned();
                                        
                                        view! {
                                            <td class="tui-dt-cell">
                                                <TableCell
                                                    component=component
                                                    value=current_value
                                                    readonly=is_readonly
                                                    on_change=Callback::new(move |value| {
                                                        update_cell(row_id_clone.clone(), col_key_clone.clone(), value)
                                                    })
                                                />
                                            </td>
                                        }
                                    }).collect::<Vec<_>>()}
                                    <Show
                                        when=move || can_delete
                                        fallback=|| ()
                                    >
                                        <td class="tui-dt-cell tui-dt-actions">
                                            {
                                                let row_id_for_delete = row_id.clone();
                                                view! {
                                                    <Show
                                                        when=move || { local_rows.get().len() > min_rows }
                                                        fallback=|| ()
                                                    >
                                                        <button
                                                            class="tui-dt-delete-btn"
                                                            on:click={
                                                                let row_id_btn = row_id_for_delete.clone();
                                                                move |_| delete_row(row_id_btn.clone())
                                                            }
                                                        >
                                                            "✖"
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                        </td>
                                    </Show>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                    <tfoot class="tui-dt-footer">
                        <tr>
                            <td colspan="100%" class="tui-dt-footer-cell">
                                <div class="tui-dt-footer-content">
                                    <Show
                                        when=move || can_add && local_rows.get().len() < max_rows
                                        fallback=|| view! { <div></div> }
                                    >
                                        <button
                                            class="tui-dt-add-btn"
                                            on:click=add_row
                                        >
                                            "+ Add Row"
                                        </button>
                                    </Show>
                                    <div class="tui-dt-total">
                                        "Total: " {move || format!("{:.2}", grand_total.get())}
                                    </div>
                                </div>
                            </td>
                        </tr>
                    </tfoot>
                </table>
            </div>
        </div>
    }
}

// Proper invoice line items columns
pub fn create_invoice_columns() -> Vec<TableColumn> {
    vec![
        TableColumn {
            key: "description".into(),
            label: "Description".into(),
            component: ComponentType::TextInput,
            width: Some("30%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "unit_type".into(),
            label: "Unit".into(),
            component: ComponentType::Select(vec![
                "pcs".into(),
                "kg".into(),
                "g".into(),
                "L".into(),
                "mL".into(),
                "box".into(),
                "pack".into(),
                "dozen".into(),
                "ream".into(),
                "hours".into(),
                "days".into(),
            ]),
            width: Some("10%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "quantity".into(),
            label: "Quantity".into(),
            component: ComponentType::NumberInput,
            width: Some("10%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "unit_price".into(),
            label: "Unit Price".into(),
            component: ComponentType::AmountInput,
            width: Some("15%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "amount".into(),
            label: "Amount".into(),
            component: ComponentType::AmountInput,
            width: Some("15%".into()),
            computed: Some("quantity * unit_price".into()),
            readonly: true,
        },
        TableColumn {
            key: "vat_rate".into(),
            label: "VAT %".into(),
            component: ComponentType::Select(vec![
                "0".into(),
                "12".into(),
                "Exempt".into(),
            ]),
            width: Some("10%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "subtotal".into(),
            label: "Subtotal".into(),
            component: ComponentType::AmountInput,
            width: Some("10%".into()),
            computed: Some("amount_with_vat".into()),  // Will be computed based on VAT
            readonly: true,
        },
    ]
}

// Helper for expense tracking table
pub fn create_expense_columns() -> Vec<TableColumn> {
    vec![
        TableColumn {
            key: "date".into(),
            label: "Date".into(),
            component: ComponentType::DatePicker,
            width: Some("15%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "vendor".into(),
            label: "Vendor".into(),
            component: ComponentType::TextInput,
            width: Some("25%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "description".into(),
            label: "Description".into(),
            component: ComponentType::TextInput,
            width: Some("25%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "category".into(),
            label: "Category".into(),
            component: ComponentType::Select(vec![
                "Office Supplies".into(),
                "Travel".into(),
                "Utilities".into(),
                "Marketing".into(),
                "Professional Fees".into(),
                "Rent".into(),
                "Equipment".into(),
            ]),
            width: Some("15%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "amount".into(),
            label: "Amount".into(),
            component: ComponentType::AmountInput,
            width: Some("15%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "paid".into(),
            label: "Paid".into(),
            component: ComponentType::Boolean,
            width: Some("5%".into()),
            computed: None,
            readonly: false,
        },
    ]
}

// Helper for inventory tracking table
pub fn create_inventory_columns() -> Vec<TableColumn> {
    vec![
        TableColumn {
            key: "sku".into(),
            label: "SKU".into(),
            component: ComponentType::TextInput,
            width: Some("15%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "name".into(),
            label: "Product Name".into(),
            component: ComponentType::TextInput,
            width: Some("30%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "quantity".into(),
            label: "Stock Qty".into(),
            component: ComponentType::NumberInput,
            width: Some("10%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "unit".into(),
            label: "Unit".into(),
            component: ComponentType::Select(vec![
                "pcs".into(),
                "kg".into(),
                "box".into(),
            ]),
            width: Some("10%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "cost".into(),
            label: "Unit Cost".into(),
            component: ComponentType::AmountInput,
            width: Some("15%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "price".into(),
            label: "Selling Price".into(),
            component: ComponentType::AmountInput,
            width: Some("15%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "reorder_level".into(),
            label: "Reorder".into(),
            component: ComponentType::NumberInput,
            width: Some("5%".into()),
            computed: None,
            readonly: false,
        },
    ]
}