mod types;
mod component;
mod cell;

pub use types::*;
pub use component::DynamicTable;
pub use cell::TableCell;

// Re-export helper functions for common table configurations
pub use component::{
    create_invoice_columns,
    create_expense_columns,
    create_inventory_columns
};