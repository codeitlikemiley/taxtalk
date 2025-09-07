# Form Fields and Component Manifest System
## Comprehensive Component Architecture for Business Applications

## Executive Summary

This document outlines a comprehensive component system for the TaxTalk plugin platform, focusing on business application needs in the Philippine market. We propose a modular, composable component architecture that can handle simple inputs to complex multi-step forms while maintaining compatibility with both guided form filling and AI-powered natural language processing.

## Core Design Principles

1. **Composability**: Components can be combined to create complex forms
2. **Progressive Disclosure**: Start simple, add complexity as needed
3. **Mode Variants**: Components support different modes via colon notation (e.g., `file:multi`, `date:range`)
4. **Business-First**: Designed for real business workflows (invoicing, receipts, compliance)
5. **Philippine-Ready**: Native support for local requirements (BIR forms, GCash/Maya, IDs)
6. **AI-Compatible**: Structure supports both guided forms and AI interpretation

## Component Taxonomy

### 1. Primitive Components (Single Value)

These are the atomic building blocks referenced in `/web/src/components/smart_command_input.rs`:

```typescript
// Current implementation (lines 71-91)
enum ComponentType {
    EntitySelector,    // Select existing entities
    TextInput,        // Single-line text
    NumberInput,      // Numeric values
    AmountInput,      // Money with currency
    DatePicker,       // Single date
    DateRange,        // Start and end dates
    Select,           // Dropdown single choice
    MultiSelect,      // Dropdown multiple choices
    Boolean,          // Yes/No toggle
    FileUpload,       // File attachment (not yet implemented)
}
```

### 2. Enhanced Primitive Components (Proposed)

#### Media Components
```json
{
  "component": "file",
  "modes": [
    "file:single",        // Single file upload
    "file:multi",         // Multiple files
    "file:image",         // Images only
    "file:document",      // Documents only (PDF, DOC)
    "file:spreadsheet",   // Excel/CSV files
    "file:receipt"        // Receipt images with OCR
  ],
  "constraints": {
    "max_size": "10MB",
    "max_files": 5,
    "accepted_types": [".pdf", ".jpg", ".png"]
  }
}
```

#### Camera/Image Components
```json
{
  "component": "camera",
  "modes": [
    "camera:photo",       // Take photo
    "camera:scan",        // Document scanner mode
    "camera:barcode",     // Barcode/QR scanner
    "camera:id"           // ID card scanner with OCR
  ],
  "features": {
    "auto_crop": true,
    "enhance": true,
    "ocr": true
  }
}
```

#### Location Components
```json
{
  "component": "location",
  "modes": [
    "location:address",   // Full address input
    "location:coords",    // GPS coordinates
    "location:map",       // Map picker
    "location:barangay"   // Philippine barangay selector
  ]
}
```

#### Time Components
```json
{
  "component": "time",
  "modes": [
    "time:single",        // Single time
    "time:range",         // Time range
    "time:duration",      // Duration picker
    "time:schedule"       // Recurring schedule
  ]
}
```

#### Signature Component
```json
{
  "component": "signature",
  "modes": [
    "signature:draw",     // Draw signature
    "signature:type",     // Type name as signature
    "signature:upload"    // Upload signature image
  ]
}
```

### 3. Composite Components (Multiple Values)

#### Form Component (Multi-field Object)
```json
{
  "component": "form",
  "fields": [
    {
      "name": "payment_method",
      "component": "select",
      "options": ["cash", "gcash", "maya", "bank_transfer"]
    },
    {
      "name": "reference_number",
      "component": "text",
      "conditional": {
        "when": "payment_method",
        "equals": ["gcash", "maya", "bank_transfer"],
        "required": true
      }
    },
    {
      "name": "proof",
      "component": "file:image",
      "conditional": {
        "when": "payment_method",
        "not_equals": "cash"
      }
    }
  ]
}
```

#### Table Component (Dynamic Rows)
```json
{
  "component": "table",
  "columns": [
    {
      "name": "item",
      "component": "entity:product",
      "label": "Product"
    },
    {
      "name": "quantity",
      "component": "number",
      "label": "Qty"
    },
    {
      "name": "price",
      "component": "amount",
      "label": "Unit Price"
    },
    {
      "name": "total",
      "component": "amount",
      "computed": "quantity * price",
      "readonly": true
    }
  ],
  "features": {
    "add_row": true,
    "delete_row": true,
    "reorder": true,
    "min_rows": 1,
    "max_rows": 100
  }
}
```

#### Wizard Component (Multi-step Form)
```json
{
  "component": "wizard",
  "steps": [
    {
      "name": "customer_info",
      "title": "Customer Information",
      "fields": [
        {
          "name": "customer",
          "component": "entity:client",
          "required": true
        },
        {
          "name": "tin",
          "component": "text:tin",
          "pattern": "^[0-9]{9}$|^[0-9]{12}$"
        }
      ]
    },
    {
      "name": "items",
      "title": "Invoice Items",
      "component": "table",
      "required": true
    },
    {
      "name": "payment",
      "title": "Payment Details",
      "fields": [
        {
          "name": "terms",
          "component": "select",
          "options": ["immediate", "30days", "60days"]
        },
        {
          "name": "due_date",
          "component": "date",
          "computed": "invoice_date + terms"
        }
      ]
    }
  ],
  "navigation": {
    "allow_skip": false,
    "allow_back": true,
    "save_progress": true
  }
}
```

### 4. Conditional Logic System

#### Basic Conditions
```json
{
  "conditional": {
    "when": "field_name",
    "operator": "equals|not_equals|contains|greater_than|less_than",
    "value": "comparison_value",
    "action": "show|hide|require|disable"
  }
}
```

#### Complex Conditions
```json
{
  "conditional": {
    "all": [  // AND condition
      {
        "when": "payment_method",
        "equals": "gcash"
      },
      {
        "when": "amount",
        "greater_than": 500
      }
    ],
    "action": "require",
    "target": "reference_number"
  }
}
```

## Business Use Cases

### Use Case 1: Invoice Creation (from `/plugins/invoice/manifest.json`)

Current simple approach:
```json
{
  "token_type": "entity_ref",
  "token_type": "amount",
  "token_type": "date"
}
```

Enhanced approach with line items:
```json
{
  "token_type": "invoice_data",
  "component": "wizard",
  "steps": [
    {
      "name": "header",
      "fields": [
        {"name": "customer", "component": "entity:client"},
        {"name": "invoice_date", "component": "date"},
        {"name": "due_date", "component": "date"}
      ]
    },
    {
      "name": "line_items",
      "component": "table",
      "columns": [
        {"name": "product", "component": "entity:product"},
        {"name": "quantity", "component": "number"},
        {"name": "price", "component": "amount"},
        {"name": "vat", "component": "select", "options": ["12%", "0%", "exempt"]}
      ]
    },
    {
      "name": "totals",
      "readonly": true,
      "computed": {
        "subtotal": "sum(line_items.total)",
        "vat": "subtotal * 0.12",
        "total": "subtotal + vat"
      }
    }
  ]
}
```

### Use Case 2: Expense Recording with Receipt

```json
{
  "token_type": "expense",
  "component": "form",
  "fields": [
    {
      "name": "vendor",
      "component": "entity:supplier",
      "can_create": true
    },
    {
      "name": "amount",
      "component": "amount"
    },
    {
      "name": "category",
      "component": "select",
      "options": ["office_supplies", "utilities", "travel", "meals"]
    },
    {
      "name": "receipt",
      "component": "camera:receipt",
      "features": {
        "ocr": true,
        "auto_extract": ["vendor", "amount", "date", "or_number"]
      }
    },
    {
      "name": "or_number",
      "component": "text",
      "pattern": "^\\d{10}$",
      "auto_filled_from": "receipt.ocr.or_number"
    }
  ]
}
```

### Use Case 3: Payment Collection (GCash/Maya)

```json
{
  "token_type": "payment",
  "component": "form",
  "fields": [
    {
      "name": "method",
      "component": "select",
      "options": ["cash", "check", "gcash", "maya", "bank_transfer"]
    },
    {
      "name": "gcash_details",
      "component": "form",
      "conditional": {
        "when": "method",
        "equals": "gcash"
      },
      "fields": [
        {
          "name": "reference_number",
          "component": "text",
          "pattern": "^[0-9]{13}$",
          "required": true
        },
        {
          "name": "sender_name",
          "component": "text"
        },
        {
          "name": "screenshot",
          "component": "file:image",
          "required": true,
          "features": {
            "ocr": true,
            "verify_amount": true
          }
        }
      ]
    }
  ]
}
```

### Use Case 4: BIR Form 2307 (Withholding Tax Certificate)

```json
{
  "token_type": "bir_2307",
  "component": "wizard",
  "steps": [
    {
      "name": "payor_info",
      "fields": [
        {"name": "payor_tin", "component": "text:tin"},
        {"name": "payor_name", "component": "text"},
        {"name": "payor_address", "component": "location:address"}
      ]
    },
    {
      "name": "payee_info",
      "fields": [
        {"name": "payee_tin", "component": "text:tin"},
        {"name": "payee_name", "component": "text"},
        {"name": "payee_address", "component": "location:address"}
      ]
    },
    {
      "name": "income_payments",
      "component": "table",
      "columns": [
        {"name": "atc_code", "component": "select:atc_codes"},
        {"name": "nature", "component": "text"},
        {"name": "amount", "component": "amount"},
        {"name": "tax_rate", "component": "percent"},
        {"name": "tax_withheld", "component": "amount", "computed": "amount * tax_rate"}
      ]
    }
  ]
}
```

## Component Mode System

### Notation Convention
```
component:mode:submode
```

Examples:
- `file` → Single file upload
- `file:multi` → Multiple files
- `file:image:multi` → Multiple images
- `date` → Single date
- `date:range` → Date range
- `date:recurring` → Recurring date pattern
- `entity:client` → Client selector
- `entity:client:multi` → Multiple client selector
- `text` → Plain text
- `text:email` → Email validation
- `text:tin` → TIN validation
- `text:phone:ph` → Philippine phone number

### Validation Modes

Built into component modes:
```json
{
  "component": "text:email",     // Auto-validates email
  "component": "text:url",       // Auto-validates URL
  "component": "text:tin",       // Philippine TIN format
  "component": "text:phone:ph",  // +63 format
  "component": "number:positive", // Only positive numbers
  "component": "number:integer",  // No decimals
  "component": "amount:php",     // PHP currency only
}
```

## Implementation Strategy

### Phase 1: Core Components (Current)
✅ EntitySelector (`/web/src/components/smart_command_input.rs:72-73`)
✅ TextInput, NumberInput, AmountInput
✅ DatePicker, DateRange
✅ Select, MultiSelect
✅ Boolean

### Phase 2: Media Components (Priority)
- [ ] FileUpload (single/multi)
- [ ] Camera (photo/scan)
- [ ] ImageUpload with preview
- [ ] Receipt scanner with OCR

### Phase 3: Composite Components
- [ ] Form (nested fields)
- [ ] Table (dynamic rows)
- [ ] Wizard (multi-step)
- [ ] Conditional logic engine

### Phase 4: Business Components
- [ ] Signature pad
- [ ] Barcode/QR scanner
- [ ] Location picker
- [ ] Time/schedule picker
- [ ] Rich text editor

### Phase 5: Integration Components
- [ ] GCash/Maya verifier
- [ ] BIR form templates
- [ ] Bank statement parser
- [ ] Email invoice sender

## Manifest Schema Evolution

Current schema (from `/plugins/invoice/manifest.json`):
```json
{
  "token_type": "amount",
  "description": "Invoice amount",
  "multiple": false
}
```

Proposed enhanced schema:
```json
{
  "token_type": "amount",
  "description": "Invoice amount",
  "component": "amount:php",
  "validation": {
    "min": 0,
    "max": 999999999.99,
    "required": true
  },
  "display": {
    "label": "Invoice Amount",
    "placeholder": "Enter amount in PHP",
    "help_text": "VAT will be calculated automatically",
    "icon": "₱"
  },
  "behavior": {
    "auto_calculate_vat": true,
    "show_in_words": true,
    "allow_installments": false
  }
}
```

## AI Integration Considerations

### Natural Language to Component Mapping

When user types: "Juan paid 5000 via gcash ref 1234567890123"

AI should map to:
```json
{
  "entity": "Juan",
  "amount": 5000,
  "payment_method": "gcash",
  "reference_number": "1234567890123"
}
```

Which triggers:
1. Entity selector (finds/creates Juan)
2. Amount input (5000 PHP)
3. Payment method selector (GCash selected)
4. Reference number field (auto-filled)

### Progressive Enhancement

Start with simple text → AI interprets → Suggests structured form → User confirms

Example flow:
1. User: "create invoice for Juan 10k plus vat due next month"
2. AI: Parses and suggests structured form
3. System: Shows wizard with pre-filled values
4. User: Reviews and adjusts in guided form
5. System: Validates and submits

## Component Reusability Matrix

| Component | Single | Multi | Range | Conditional | Validated | Computed |
|-----------|--------|-------|-------|-------------|-----------|----------|
| Text | ✓ | - | - | ✓ | ✓ | - |
| Number | ✓ | - | ✓ | ✓ | ✓ | ✓ |
| Amount | ✓ | - | ✓ | ✓ | ✓ | ✓ |
| Date | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Time | ✓ | ✓ | ✓ | ✓ | ✓ | - |
| File | ✓ | ✓ | - | ✓ | ✓ | - |
| Entity | ✓ | ✓ | - | ✓ | ✓ | - |
| Location | ✓ | ✓ | - | ✓ | ✓ | - |
| Boolean | ✓ | - | - | ✓ | - | ✓ |
| Select | ✓ | ✓ | - | ✓ | ✓ | - |

## Recommendations

### Immediate Priorities

1. **Implement FileUpload component** - Critical for receipts and documents
   - Single/multi modes
   - Image preview
   - Drag & drop support
   - Mobile camera integration

2. **Add Form component** - Enables complex data structures
   - Nested field support
   - Conditional logic
   - Validation groups
   - Error handling

3. **Create Table component** - Essential for line items
   - Dynamic rows
   - Computed columns
   - Sorting/filtering
   - Export capability

### Architecture Decisions

1. **Use component:mode notation** consistently
2. **Implement validation at component level** not just backend
3. **Support both guided and AI modes** from the start
4. **Make all components keyboard accessible**
5. **Ensure mobile-first design** (critical for Philippine market)

### Philippine Market Specifics

1. **Payment verification** - GCash/Maya screenshot validation
2. **BIR compliance** - Built-in form templates
3. **ID validation** - TIN, SSS, PhilHealth checkers
4. **Receipt scanning** - OCR for OR/SI numbers
5. **Barangay support** - Location picker with barangay database

## Conclusion

The proposed component system provides a comprehensive foundation for business applications while maintaining flexibility for both guided form filling and AI-powered natural language processing. By implementing these components progressively, we can build a robust plugin platform that serves the unique needs of Philippine SMEs while remaining extensible for international markets.

Key success factors:
1. **Start with high-impact components** (file upload, forms, tables)
2. **Maintain consistency** in component API and manifest schema
3. **Test with real business workflows** (invoicing, payments, compliance)
4. **Iterate based on user feedback** from actual bookkeepers/accountants
5. **Keep AI compatibility** in mind for future enhancements

This architecture balances complexity with usability, providing powerful tools for developers while keeping the end-user experience simple and intuitive.