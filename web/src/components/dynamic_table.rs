use leptos::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

// Reuse ComponentType from smart_command_input
use crate::components::smart_command_input::ComponentType;

#[derive(Debug, Clone)]
pub struct TableColumn {
    pub key: String,
    pub label: String,
    pub component: ComponentType,
    pub width: Option<String>,
    pub computed: Option<String>, // e.g., "quantity * price"
    pub readonly: bool,
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub id: String,
    pub cells: HashMap<String, Value>,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            cells: HashMap::new(),
        }
    }
}

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
                // Simple implementation for "quantity * price" pattern
                if computed == "quantity * price" {
                    let quantity = row.cells.get("quantity")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let price = row.cells.get("price")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    row.cells.insert(column.key.clone(), Value::from(quantity * price));
                }
                // Add more computation patterns as needed
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
        <div class="dynamic-table-component">
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            {move || columns.get().iter().map(|column| {
                                let width_style = column.width.as_ref().map(|w| format!("width: {}", w)).unwrap_or_default();
                                view! {
                                    <th
                                        class="px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                        style=width_style
                                    >
                                        {column.label.clone()}
                                    </th>
                                }
                            }).collect::<Vec<_>>()}
                            {move || if can_delete {
                                view! {
                                    <th class="px-3 py-2 text-center text-xs font-medium text-gray-500 uppercase tracking-wider w-16">
                                        "Actions"
                                    </th>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        {move || local_rows.get().iter().map(|row| {
                            let row_id = row.id.clone();
                            let row_cells = row.cells.clone();
                            
                            view! {
                                <tr class="hover:bg-gray-50">
                                    {columns.get().iter().map(|column| {
                                        let col_key = column.key.clone();
                                        let row_id_clone = row_id.clone();
                                        let col_key_clone = col_key.clone();
                                        let is_readonly = column.readonly;
                                        let component = column.component.clone();
                                        let current_value = row_cells.get(&col_key).cloned();
                                        
                                        view! {
                                            <td class="px-3 py-2 whitespace-nowrap">
                                                <TableCell
                                                    component=component
                                                    value=current_value
                                                    readonly=is_readonly
                                                    on_change=Box::new(move |value| {
                                                        update_cell(row_id_clone.clone(), col_key_clone.clone(), value)
                                                    })
                                                />
                                            </td>
                                        }
                                    }).collect::<Vec<_>>()}
                                    {move || if can_delete && local_rows.get().len() > min_rows {
                                        let row_id = row_id.clone();
                                        view! {
                                            <td class="px-3 py-2 text-center">
                                                <button
                                                    class="text-red-500 hover:text-red-700"
                                                    on:click=move |_| delete_row(row_id.clone())
                                                >
                                                    "✖"
                                                </button>
                                            </td>
                                        }.into_any()
                                    } else {
                                        view! { <td></td> }.into_any()
                                    }}
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                    <tfoot class="bg-gray-50">
                        <tr>
                            <td colspan="100%" class="px-3 py-2">
                                <div class="flex justify-between items-center">
                                    {move || if can_add && local_rows.get().len() < max_rows {
                                        view! {
                                            <button
                                                class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                                                on:click=add_row
                                            >
                                                "+ Add Row"
                                            </button>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}
                                    <div class="text-right font-semibold">
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

#[component]
fn TableCell(
    component: ComponentType,
    value: Option<Value>,
    readonly: bool,
    on_change: Box<dyn Fn(Value)>,
) -> impl IntoView {
    let (local_value, set_local_value) = signal(value.clone());
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let component_clone = component.clone();
    let component_for_display = component.clone();
    let component_for_match = component.clone();
    
    let handle_change = move |e: web_sys::Event| {
        if let Some(input) = e.target() {
            let input: HtmlInputElement = input.dyn_into().unwrap();
            let raw_value = input.value();
            
            let new_value = match component_clone.clone() {
                ComponentType::NumberInput | ComponentType::AmountInput => {
                    raw_value.parse::<f64>()
                        .map(Value::from)
                        .unwrap_or(Value::Null)
                },
                ComponentType::Boolean => {
                    Value::from(input.checked())
                },
                _ => Value::from(raw_value)
            };
            
            set_local_value.set(Some(new_value.clone()));
            on_change(new_value);
        }
    };
    
    let display_value = move || {
        let comp = component_for_display.clone();
        local_value.get()
            .and_then(|v| match comp {
                ComponentType::AmountInput => v.as_f64().map(|f| format!("{:.2}", f)),
                ComponentType::NumberInput => v.as_f64().map(|f| f.to_string()),
                ComponentType::Boolean => v.as_bool().map(|b| b.to_string()),
                _ => v.as_str().map(|s| s.to_string())
            })
            .unwrap_or_default()
    };
    
    if readonly {
        view! {
            <span class="text-gray-600">
                {display_value}
            </span>
        }.into_any()
    } else {
        match component_for_match {
            ComponentType::Boolean => {
                let checked = local_value.get()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                view! {
                    <input
                        type="checkbox"
                        class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                        checked=checked
                        on:change=handle_change
                    />
                }.into_any()
            },
            ComponentType::Select(options) => {
                view! {
                    <select
                        class="block w-full px-2 py-1 text-sm border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                        on:change=handle_change
                    >
                        <option value="">"-"</option>
                        {options.iter().map(|opt| {
                            view! {
                                <option value=opt.clone()>{opt.clone()}</option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                }.into_any()
            },
            ComponentType::NumberInput => {
                view! {
                    <input
                        type="number"
                        class="block w-full px-2 py-1 text-sm border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                        value=display_value
                        on:change=handle_change
                        node_ref=input_ref
                        step="1"
                    />
                }.into_any()
            },
            ComponentType::AmountInput => {
                view! {
                    <input
                        type="number"
                        class="block w-full px-2 py-1 text-sm border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                        value=display_value
                        on:change=handle_change
                        node_ref=input_ref
                        step="0.01"
                        placeholder="0.00"
                    />
                }.into_any()
            },
            _ => {
                view! {
                    <input
                        type="text"
                        class="block w-full px-2 py-1 text-sm border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                        value=display_value
                        on:change=handle_change
                        node_ref=input_ref
                    />
                }.into_any()
            }
        }
    }
}

// Example usage for invoice line items
pub fn create_invoice_columns() -> Vec<TableColumn> {
    vec![
        TableColumn {
            key: "product".into(),
            label: "Product".into(),
            component: ComponentType::TextInput,
            width: Some("40%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "quantity".into(),
            label: "Qty".into(),
            component: ComponentType::NumberInput,
            width: Some("15%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "price".into(),
            label: "Unit Price".into(),
            component: ComponentType::AmountInput,
            width: Some("20%".into()),
            computed: None,
            readonly: false,
        },
        TableColumn {
            key: "total".into(),
            label: "Total".into(),
            component: ComponentType::AmountInput,
            width: Some("25%".into()),
            computed: Some("quantity * price".into()),
            readonly: true,
        },
    ]
}