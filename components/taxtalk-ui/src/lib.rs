// Re-export core types and utilities
pub use taxtalk_ui_core::*;

// Re-export components
pub mod file_upload {
    pub use taxtalk_file_upload::*;
}

pub mod table {
    pub use taxtalk_dynamic_table::*;
}

// Future component re-exports will be added here:
// pub mod form {
//     pub use taxtalk_conditional_form::*;
// }
// 
// pub mod scanner {
//     pub use taxtalk_receipt_scanner::*;
// }

// Convenience re-exports at root level
pub use file_upload::{FileUpload, FileUploadMode, UploadedFile};
pub use table::{DynamicTable, TableColumn, TableRow, create_invoice_columns};