Got it — let’s design a **domain knowledge base roadmap** for the plugins.

We’ll map out **core business domains** and **common must-have apps** that nearly every business — from small mom-and-pop shops to enterprise orgs — will need.  
This list will let us identify:

- **Plugins we must support in v1** (highest demand, cross-industry)
    
- **Plugins for specialized industries** (optional but high-value later)
    
- The **domain knowledge** each plugin will require (schemas, workflows, validation rules, tokens, commands).
    

I’ll also group by **business function**, and note interdependencies, so when we design a schema or token system, we’ll know where they overlap (e.g., **Invoices depend on Clients/Products**).

---

# **Domain Knowledge Base Roadmap for Business Apps**

> Goal: Establish a foundation of **plugin domains** representing 99% of core business operations.

These are the **initial verticals** we’ll target, with **v1 focus on universal apps**.  
Later, we expand into specialized domains for niche industries.

---

## **Tier 1 — Universal Core Modules (v1 priority)**

These are **non-negotiable**, every business needs them regardless of size or industry.

|Plugin|Purpose|Example Schema Entities|Dependencies|
|---|---|---|---|
|**Company / Organization Profile**|Define company identity, settings, branding, legal entity details.|`company`, `branch`, `department`|None|
|**Contacts / CRM Lite**|Manage customers, suppliers, partners, employees.|`contact`, `customer`, `supplier`, `employee`|Company|
|**Products / Services Catalog**|List of sellable products and services, with prices and stock keeping units.|`product`, `service`, `category`, `price`|Company|
|**Invoices & Billing**|Generate invoices, manage payment tracking.|`invoice`, `invoice_item`, `tax_rate`|Contacts, Products|
|**Payments**|Track incoming/outgoing payments, link to invoices.|`payment`, `method`, `currency`|Invoices, Bank|
|**Expenses / Purchases**|Record business expenses and purchases.|`expense`, `receipt`, `vendor`|Contacts, Bank|
|**General Ledger / Accounting Core**|Double-entry accounting and journals.|`ledger_account`, `journal_entry`, `balance`|Invoices, Payments, Expenses|
|**Authentication / Roles / Permissions**|Users, roles, and permissions for access control.|`user`, `role`, `permission`|Company|

These **seven modules** form the minimum viable foundation for any business.

---

## **Tier 2 — Growth & Operational Modules (medium priority)**

These are common for **growing small-to-medium businesses**.  
Some companies won’t need them initially, but once they scale, they become vital.

|Plugin|Purpose|Example Schema Entities|Dependencies|
|---|---|---|---|
|**Inventory Management**|Track stock levels, warehouses, transfers.|`inventory_item`, `warehouse`, `transfer`|Products|
|**Purchase Orders**|Formal request to suppliers for goods/services.|`purchase_order`, `po_item`, `supplier`|Contacts, Products, Inventory|
|**Quotations / Estimates**|Generate quotes before invoicing.|`quotation`, `quote_item`, `customer`|Products, Contacts|
|**Payroll / HR Core**|Employee management, salary processing.|`employee`, `payroll_run`, `benefit`, `deduction`|Contacts|
|**Task / Project Management**|Tasks, milestones, and projects for internal operations.|`project`, `task`, `milestone`|Company|
|**Basic Reporting / Dashboard**|High-level financial and operational KPIs.|`report`, `metric`, `chart`|All core modules|
|**Notifications & Messaging**|Cross-plugin notifications (alerts, approvals).|`notification`, `channel`, `message`|Any module|

---

## **Tier 3 — Specialized Industry Modules**

These are **optional plugins** for specific industries or advanced businesses.  
We can build these after the core is stable.

|Industry|Specialized Plugins|Example Entities|
|---|---|---|
|**E-commerce**|Shopping cart, online orders, checkout flows.|`cart`, `order`, `discount_code`|
|**Manufacturing**|Bill of materials (BOM), production runs, work orders.|`bom`, `work_order`, `machine`|
|**Hospitality**|Room booking, reservations, POS integration.|`room`, `booking`, `guest`|
|**Healthcare**|Patient management, prescriptions, medical records.|`patient`, `appointment`, `prescription`|
|**Education**|Student management, courses, grading.|`student`, `course`, `grade`|

---

## **Token & Schema Interplay**

Every plugin will define **tokens** and **schemas** so that natural language commands like this:

> `@client uriah bought 5kg of pineapple for 1000 pesos`

Resolve cleanly into structured commands.

|Token|Owned By Plugin|Example|
|---|---|---|
|`@client`|Contacts / CRM Lite|`@client uriah` → customer lookup|
|`@product`|Products Catalog|`@product pineapple` → product lookup|
|`@invoice`|Invoices|`@invoice INV-1001` → retrieve invoice|
|`@payment`|Payments|`@payment PAY-1001` → payment details|
|`@expense`|Expenses|`@expense taxi` → record taxi expense|
|`@user`|Auth / Roles|`@user admin` → assign permissions|

---

## **Interdependencies Map**

This shows how modules rely on each other so we can design plugin loading order and schema versioning.

```
[Company]
   ↓
[Contacts / CRM Lite]
   ↓
[Products / Services] → [Inventory] → [Purchase Orders]
   ↓
[Invoices] → [Payments] → [General Ledger]
   ↓
[Expenses]
   ↓
[Reporting]
```

> **Rule:**
> 
> - Each plugin must declare dependencies in its `plugin.toml`.
>     
> - Core will resolve dependencies before activation.
>     

---

## **V1 Plugin Candidates**

|Priority|Plugin|Notes|
|---|---|---|
|**P0**|Company|Required for all others.|
|**P0**|Contacts / CRM Lite|Every business needs to manage people/orgs.|
|**P0**|Products / Services|Even service businesses have catalogs.|
|**P0**|Invoices & Billing|Core to cash flow.|
|**P0**|Payments|Money tracking essential.|
|**P0**|Expenses|Spending tracking essential.|
|**P0**|Auth / Roles|Access control required.|
|**P1**|General Ledger|Needed for full accounting.|
|**P1**|Inventory|For product-based businesses.|
|**P1**|Purchase Orders|Procurement process.|
|**P1**|Quotations|Pre-sales workflow.|
|**P1**|Payroll|For growing teams.|
|**P2**|Task Management|Optional, internal use.|
|**P2**|Notifications|Cross-plugin UX improvement.|

---

## **Domain Knowledge Sources**

For each plugin, we need **domain-specific rules, workflows, and schemas**. These can come from:

|Source|Use Case|
|---|---|
|**IFRS Accounting Standards**|Ledger rules, journal validation.|
|**Tax Regulations** (by country)|Invoice, VAT, tax schemas.|
|**Odoo ERP Open Source**|Reference schemas for business entities.|
|**QuickBooks Developer Docs**|Invoice/payment data structures.|
|**Stripe API Docs**|Payment object schemas.|
|**Shopify API Docs**|Product catalog & inventory patterns.|
|**ISO 20022 Payment Standards**|Standardized payment messages.|

---

## **Next Steps**

1. **Confirm P0 and P1 modules for v1 launch.**
    
2. For each P0 module:
    
    - Define initial schema JSON.
        
    - Define token set and natural language patterns.
        
    - Identify validation rules (e.g., invoice must have at least one item).
        
3. Build **knowledge base**:
    
    - Store code snippets + schemas + workflow diagrams.
        
    - Separate **technical KB** (core runtime, Crux, Wasmtime) from **domain KB** (business rules, entities, tokens).
        

---

Would you like me to **start defining initial schema JSON and token mappings for the P0 plugins**, so we have a baseline for the core runtime to load? This would make it easier to wire them into the first running demo.