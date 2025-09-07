# Component Implementation Specification
## Executable Plan for Business-Critical UI Components

### Document Version: 1.0
### Date: 2025-09-07
### Priority: IMMEDIATE EXECUTION

---

## 🎯 Executive Summary

This specification defines the implementation of critical business components for the TaxTalk platform, focusing on file upload, form composition, and table management. Each component is designed for immediate productivity gains in bookkeeping workflows.

---

## 📋 Implementation Priority Queue

### Phase 1: File Upload Component (2 hours)
**Business Impact**: Enable receipt/document management
**User Story**: "As a bookkeeper, I need to upload receipts and documents quickly from mobile or desktop"

### Phase 2: Dynamic Table Component (3 hours)
**Business Impact**: Enable invoice line items, expense lists
**User Story**: "As a user, I need to add multiple items to an invoice with automatic calculations"

### Phase 3: Conditional Form Component (2 hours)
**Business Impact**: Smart forms that adapt based on selections
**User Story**: "As a user, when I select GCash payment, I need to see reference number field"

### Phase 4: Receipt Scanner with OCR (3 hours)
**Business Impact**: 10x faster expense recording
**User Story**: "As a bookkeeper, I want to scan receipts and auto-extract vendor, amount, and OR number"

---

## 🔧 Component 1: FileUploadComponent

### Specification

**File**: `/Users/uriah/Code/taxtalk/web/src/components/file_upload.rs`

```rust
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{File, DragEvent, HtmlInputElement};

#[derive(Debug, Clone, PartialEq)]
pub enum FileUploadMode {
    Single,
    Multiple,
    Image,
    Document,
    Receipt,
}

#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub id: String,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub preview_url: Option<String>,
    pub upload_progress: f32,
}

#[component]
pub fn FileUpload(
    mode: Signal<FileUploadMode>,
    on_upload: Callback<Vec<UploadedFile>>,
    #[prop(default = 10_485_760)] max_size: usize, // 10MB default
    #[prop(default = 5)] max_files: usize,
) -> impl IntoView {
    // Implementation details below
}
```

### Features Required:
1. **Drag & Drop Zone**
   - Visual feedback on drag over
   - File type validation
   - Size validation
   - Multiple file support

2. **Click to Upload**
   - Native file picker
   - Camera option on mobile
   - Type filtering based on mode

3. **Preview Gallery**
   - Thumbnail for images
   - File icon for documents
   - Progress bar during upload
   - Remove button per file

4. **Validation**
   - File size limits
   - File type restrictions
   - Maximum file count
   - Error messages

### UI Layout:
```
┌─────────────────────────────────────┐
│  ╭─────────────────────────────╮    │
│  │                             │    │
│  │     📁 Drop files here      │    │
│  │         or click to          │    │
│  │          browse              │    │
│  │                             │    │
│  ╰─────────────────────────────╯    │
│                                     │
│  Uploaded Files:                   │
│  ┌────────┐ ┌────────┐ ┌────────┐ │
│  │ 📄 doc │ │ 🖼 img │ │ 📊 xls │ │
│  │  ✖️    │ │  ✖️    │ │  ✖️    │ │
│  └────────┘ └────────┘ └────────┘ │
└─────────────────────────────────────┘
```

### Test Cases:
- [ ] Upload single image file < 10MB
- [ ] Upload multiple files via drag & drop
- [ ] Reject file > max_size
- [ ] Show preview for images
- [ ] Remove uploaded file
- [ ] Handle network failure gracefully

---

## 🔧 Component 2: DynamicTableComponent

### Specification

**File**: `/Users/uriah/Code/taxtalk/web/src/components/dynamic_table.rs`

```rust
use leptos::prelude::*;
use serde_json::Value;

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

#[component]
pub fn DynamicTable(
    columns: Signal<Vec<TableColumn>>,
    rows: Signal<Vec<TableRow>>,
    on_change: Callback<Vec<TableRow>>,
    #[prop(default = true)] can_add: bool,
    #[prop(default = true)] can_delete: bool,
    #[prop(default = 1)] min_rows: usize,
    #[prop(default = 100)] max_rows: usize,
) -> impl IntoView {
    // Implementation below
}
```

### Features Required:
1. **Dynamic Columns**
   - Configurable column types
   - Custom components per cell
   - Computed columns
   - Column widths

2. **Row Management**
   - Add new row button
   - Delete row button per row
   - Minimum row enforcement
   - Maximum row limit

3. **Cell Editing**
   - Inline editing
   - Component-based input
   - Validation per cell
   - Tab navigation

4. **Calculations**
   - Automatic computed fields
   - Row totals
   - Grand total
   - Real-time updates

### UI Layout:
```
┌─────────────────────────────────────────────┐
│ Product         │ Qty │ Price  │ Total      │
├─────────────────┼─────┼────────┼────────────┤
│ [Select ▼]      │ [1] │ [0.00] │ 0.00       │
│                 │     │        │ [✖]        │
├─────────────────┼─────┼────────┼────────────┤
│ [Select ▼]      │ [1] │ [0.00] │ 0.00       │
│                 │     │        │ [✖]        │
├─────────────────┼─────┼────────┼────────────┤
│ [+ Add Row]                    │ Total: 0.00│
└─────────────────────────────────────────────┘
```

### Invoice Line Items Example:
```rust
let columns = vec![
    TableColumn {
        key: "product".into(),
        label: "Product".into(),
        component: ComponentType::EntitySelector,
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
];
```

### Test Cases:
- [ ] Add 3 invoice line items
- [ ] Calculate row totals automatically
- [ ] Delete middle row, verify reindexing
- [ ] Tab through cells seamlessly
- [ ] Validate min_rows = 1 enforced
- [ ] Computed field updates on input change

---

## 🔧 Component 3: ConditionalFormComponent

### Specification

**File**: `/Users/uriah/Code/taxtalk/web/src/components/conditional_form.rs`

```rust
use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub component: ComponentType,
    pub required: bool,
    pub condition: Option<FieldCondition>,
    pub default_value: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct FieldCondition {
    pub when: String,        // Field name to watch
    pub operator: ConditionOperator,
    pub value: Value,
    pub action: ConditionAction,
}

#[derive(Debug, Clone)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    GreaterThan,
    LessThan,
    In,
}

#[derive(Debug, Clone)]
pub enum ConditionAction {
    Show,
    Hide,
    Require,
    Disable,
}

#[component]
pub fn ConditionalForm(
    fields: Signal<Vec<FormField>>,
    on_submit: Callback<HashMap<String, Value>>,
    #[prop(default = false)] inline: bool,
) -> impl IntoView {
    // Implementation below
}
```

### Payment Method Example:
```rust
let fields = vec![
    FormField {
        name: "payment_method".into(),
        label: "Payment Method".into(),
        component: ComponentType::Select(vec![
            "cash".into(),
            "gcash".into(),
            "maya".into(),
            "bank_transfer".into(),
        ]),
        required: true,
        condition: None,
        default_value: Some(json!("cash")),
    },
    FormField {
        name: "reference_number".into(),
        label: "Reference Number".into(),
        component: ComponentType::TextInput,
        required: false,
        condition: Some(FieldCondition {
            when: "payment_method".into(),
            operator: ConditionOperator::In,
            value: json!(["gcash", "maya", "bank_transfer"]),
            action: ConditionAction::Show,
        }),
        default_value: None,
    },
    FormField {
        name: "proof".into(),
        label: "Upload Proof".into(),
        component: ComponentType::FileUpload,
        required: false,
        condition: Some(FieldCondition {
            when: "payment_method".into(),
            operator: ConditionOperator::NotEquals,
            value: json!("cash"),
            action: ConditionAction::Require,
        }),
        default_value: None,
    },
];
```

### Test Cases:
- [ ] Select cash, verify no extra fields shown
- [ ] Select GCash, verify reference field appears
- [ ] Select Maya, verify proof upload required
- [ ] Submit with missing required conditional field
- [ ] Change selection, verify fields update
- [ ] Submit valid form, verify data structure

---

## 🔧 Component 4: ReceiptScannerComponent

### Specification

**File**: `/Users/uriah/Code/taxtalk/web/src/components/receipt_scanner.rs`

```rust
use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct ScannedReceipt {
    pub vendor: Option<String>,
    pub amount: Option<f64>,
    pub date: Option<String>,
    pub or_number: Option<String>,
    pub tin: Option<String>,
    pub items: Vec<LineItem>,
    pub confidence: f32,
}

#[component]
pub fn ReceiptScanner(
    on_scan: Callback<ScannedReceipt>,
    #[prop(default = true)] auto_enhance: bool,
    #[prop(default = true)] extract_line_items: bool,
) -> impl IntoView {
    // Implementation below
}
```

### OCR Integration:
```rust
async fn process_image(image_data: Vec<u8>) -> Result<ScannedReceipt, String> {
    // 1. Send to OCR API (Tesseract WASM or cloud service)
    // 2. Parse text for Philippine receipt patterns
    // 3. Extract key fields with regex
    // 4. Return structured data
}
```

### Philippine Receipt Patterns:
```rust
const OR_PATTERN: &str = r"OR\s*NO\s*[:.]?\s*(\d{10})";
const TIN_PATTERN: &str = r"TIN\s*[:.]?\s*(\d{3}-\d{3}-\d{3}-\d{3})";
const AMOUNT_PATTERN: &str = r"TOTAL\s*[:.]?\s*₱?\s*([\d,]+\.?\d*)";
const VAT_PATTERN: &str = r"VAT\s*\(12%\)\s*[:.]?\s*₱?\s*([\d,]+\.?\d*)";
```

### Test Cases:
- [ ] Scan SM receipt, extract vendor and total
- [ ] Scan 7-Eleven receipt, extract OR number
- [ ] Scan restaurant receipt, extract VAT amount
- [ ] Handle blurry image with enhancement
- [ ] Extract line items from detailed receipt
- [ ] Validate extracted TIN format

---

## 📊 Implementation Schedule

### Day 1 (Today)
- [ ] 09:00-11:00: Implement FileUploadComponent
- [ ] 11:00-11:30: Test file upload with cargo check
- [ ] 11:30-14:30: Implement DynamicTableComponent
- [ ] 14:30-15:00: Integration testing

### Day 2
- [ ] 09:00-11:00: Implement ConditionalFormComponent
- [ ] 11:00-14:00: Implement ReceiptScannerComponent
- [ ] 14:00-15:00: OCR integration
- [ ] 15:00-16:00: End-to-end testing

---

## ✅ Success Criteria

### Technical Requirements
- [ ] All components compile without errors (`cargo check`)
- [ ] All components render without runtime panics
- [ ] Keyboard navigation works (Tab, Shift+Tab, Enter, Escape)
- [ ] Mobile responsive (touch-friendly, appropriate sizing)
- [ ] Accessibility (ARIA labels, screen reader support)

### Business Requirements
- [ ] Invoice creation time reduced from 5 minutes to 1 minute
- [ ] Receipt data entry time reduced by 80%
- [ ] Zero data loss on network failure (local storage backup)
- [ ] Support offline mode with sync when online
- [ ] BIR compliance (OR numbers, TIN validation)

### Performance Metrics
- [ ] Component render < 100ms
- [ ] File upload feedback < 50ms
- [ ] Table with 100 rows scrolls at 60fps
- [ ] OCR processing < 3 seconds
- [ ] Form validation feedback < 100ms

---

## 🚀 Execution Commands

```bash
# Start development
cd /Users/uriah/Code/taxtalk/web

# Create component files
touch src/components/file_upload.rs
touch src/components/dynamic_table.rs
touch src/components/conditional_form.rs
touch src/components/receipt_scanner.rs

# Update mod.rs
echo "pub mod file_upload;" >> src/components/mod.rs
echo "pub mod dynamic_table;" >> src/components/mod.rs
echo "pub mod conditional_form;" >> src/components/mod.rs
echo "pub mod receipt_scanner;" >> src/components/mod.rs

# Check compilation after each component
cargo check

# Run dev server
trunk serve --port 4001

# Test in browser
open http://localhost:4001
```

---

## 📝 Notes for AI Implementation

1. **Start with FileUploadComponent** - It's the foundation for receipt scanning
2. **Use existing ComponentType enum** from smart_command_input.rs
3. **Reuse styling patterns** from existing components for consistency
4. **Test each component in isolation** before integration
5. **Use `cargo check` after every major function** to catch errors early
6. **Focus on happy path first**, then add error handling
7. **Keep components pure** - side effects only in callbacks

---

## 🎯 Definition of Done

- [ ] All 4 components implemented and compile
- [ ] Integration with smart_command_input.rs complete
- [ ] Manual testing checklist complete
- [ ] Documentation updated in MANIFEST_GUIDE.md
- [ ] Example usage added to each component file
- [ ] Accessibility audit passed
- [ ] Mobile testing complete
- [ ] Performance benchmarks met