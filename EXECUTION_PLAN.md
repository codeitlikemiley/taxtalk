Locked in: P0 and P1 modules are the foundation for v1. The next move is to immediately operationalize them by defining schemas and token sets so we can get the runtime running and load these modules dynamically.

Here's the streamlined execution plan:

---

## **Execution Plan for v1 (P0 + P1 Modules)**

### **1. Schema Definition (JSON)**

We need clean, versioned JSON schemas for every P0 and P1 plugin.
Each schema must include:

* **Entities:** Core tables/objects for that domain.
* **Relationships:** Foreign keys and dependencies across modules.
* **Validation rules:** Required fields, numeric limits, enum values.
* **Audit fields:** Created/updated timestamps, soft delete flags.

Example snippet for **Invoices**:

```json
{
  "invoice": {
    "id": "string",
    "customer_id": "string",
    "date": "date",
    "status": "enum[draft, issued, paid, cancelled]",
    "total_amount": "decimal",
    "currency": "string",
    "line_items": [
      {
        "product_id": "string",
        "description": "string",
        "quantity": "decimal",
        "unit_price": "decimal",
        "tax_rate_id": "string"
      }
    ]
  }
}
```

---

### **2. Token Set for Natural Language Commands**

Define token mappings for each module so text commands translate cleanly into structured actions.

Example for P0/P1:

| Token      | Module          | Purpose                               |
| ---------- | --------------- | ------------------------------------- |
| `@client`  | Contacts        | Lookup or create customers, suppliers |
| `@product` | Products        | Reference product or service          |
| `@invoice` | Invoices        | Retrieve or update invoices           |
| `@payment` | Payments        | Manage payment records                |
| `@expense` | Expenses        | Log expenses                          |
| `@user`    | Auth            | User and permissions management       |
| `@ledger`  | General Ledger  | Direct journal entry access           |
| `@stock`   | Inventory       | Check stock levels                    |
| `@po`      | Purchase Orders | Procurement workflow                  |
| `@quote`   | Quotations      | Sales quoting                         |
| `@payroll` | Payroll         | Employee payroll operations           |

---

### **3. Plugin Activation Flow**

Define activation sequence to handle dependencies:

```
Company → Auth → Contacts → Products → Inventory → Purchase Orders
         ↓
       Invoices → Payments → General Ledger
         ↓
       Expenses
```

Rule:
Plugins declare dependencies in `plugin.toml`.
Core runtime resolves and loads in correct order.

---

### **4. Deliverables by Module Group**

| Group                         | Modules                                             | Deliverable                              |
| ----------------------------- | --------------------------------------------------- | ---------------------------------------- |
| **P0 Core Identity & Access** | Company, Auth                                       | Base schemas, token sets                 |
| **P0 Core Finance**           | Invoices, Payments, Expenses                        | Schemas, tokens, validation rules        |
| **P1 Growth Modules**         | Inventory, POs, Quotations, Payroll, General Ledger | Schemas + cross-module integration rules |

---

### **5. Build Knowledge Base Structure**

Two layers:

1. **Technical KB** – Runtime logic, Crux code, Wasmtime plugin loader.
2. **Domain KB** – Schemas, tokens, validation, workflows.

Proposed folder structure:

```
/knowledge-base
  /technical
    /runtime
    /plugin-loader
  /domain
    /company
    /auth
    /invoices
    /payments
    /expenses
    /ledger
    /inventory
    /purchase-orders
    /quotations
    /payroll
```

This is now on [ULTRA_BANK](./ULTRA_BANK.md)

