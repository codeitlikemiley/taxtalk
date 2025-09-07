# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TaxTalk is a natural language bookkeeping system for Philippine businesses that uses tokenized commands to automate accounting workflows. It's built on a Crux-based plugin architecture with WASM plugins for modularity.

## Build Commands

```bash
# Build entire workspace
cargo build

# Build core runtime only
cargo build -p core

# Build specific plugin for WASM
cd plugins/{plugin-name}
cargo build --target wasm32-wasip2 --release

# Run tests
cargo test

# Run with hot-reload development
cargo run --bin server -- --watch-plugins

# Check code quality
cargo clippy --all-targets --all-features
cargo fmt --check
```

## Architecture Overview

### Three-Layer Architecture
1. **Core Layer** (`core/`) - Crux state management, plugin runtime, tokenizer, validation
2. **Plugin Layer** (`plugins/*/`) - WASM modules for business logic (invoice, VAT, accounting)  
3. **Server Layer** (`server/`) - HTTP/WebSocket API, plugin host, chat interface

### Critical Design Patterns

#### Crux State Flow
All state changes MUST flow through Crux's update/effect/view pattern:
```
User Input → Tokenizer → Crux Update → Event Bus → Plugin → Effect → View
```

#### Plugin Communication
- Plugins NEVER communicate directly
- All inter-plugin communication via Crux event bus
- Plugins consume capabilities (http, kv, render) but never provide them
- Each plugin runs in isolated WASM sandbox

#### Token Processing Pipeline
Natural language commands like `@client Juan bought 5kg rice 1000 pesos` are processed:
1. Tokenizer (`core/src/tokenizer.rs`) parses using nom
2. Validates against plugin schemas
3. Routes through Crux to appropriate plugin
4. Plugin executes business logic
5. Results flow back through Crux for UI updates

## Philippine Tax & Bookkeeping Context

The system automates manual bookkeeping processes specific to Philippine BIR requirements:

### Key Tax Rules Implemented
- **VAT**: 12% computation with senior/PWD exemptions
- **Withholding Tax (EWT)**: 
  - Professional services: 10-15%
  - Goods: 1%
  - Rental: 5%
- **BIR Forms**: Automated generation of 2550M (monthly VAT return)
- **Receipt Requirements**: Sequential OR/SI numbering

### Token Patterns for Filipino Context
```
"binili ni @client" → bought by client
"nagbayad si @client" → client paid  
"may discount" → has discount
"senior/PWD" → special 20% discount
"plus VAT" → VAT exclusive amount
"with VAT" → VAT inclusive amount
```

## Module Dependencies & Loading Order

### P0 (Core Modules - Required)
1. Company → Base entity for all others
2. Auth → User management and permissions
3. Contacts → Customers, suppliers, employees
4. Products → Product/service catalog
5. Invoices → Sales invoicing with VAT
6. Payments → Payment tracking and reconciliation
7. Expenses → Expense recording

### P1 (Growth Modules)
- General Ledger → Double-entry bookkeeping
- Inventory → Stock management
- Purchase Orders → Procurement workflow
- Quotations → Pre-sales quotes
- Payroll → Employee salary processing

## Critical Implementation Rules

### Dependency Management (from .kilocode/rules/dependencies.md)
- Core MUST use: wasmtime 36.0.1, serde 1.0.219, nom 8.0.0, tokenizers 0.21.2
- Plugins MUST use: wit-bindgen 0.44.0, minimal dependencies
- NEVER add tokio or filesystem access to plugins
- Maintain version consistency across all layers

### Plugin Development (from .kilocode/rules/plugin.md)
Every plugin MUST:
- Export required WIT interface functions (constructor, update, resolve, view, schema)
- Include complete manifest.json with capabilities and dependencies
- Use consistent key patterns: `{plugin_id}:{entity_type}:{id}`
- Handle errors without panicking
- Support hot-reload with state preservation

### File Structure Requirements
```
core/src/
├── lib.rs             # Crux App state (REQUIRED)
├── plugin_loader.rs   # Wasmtime loader (REQUIRED)
├── manifest.rs        # Plugin manifest handler
├── event_bus.rs       # Event routing system
├── tokenizer.rs       # NLP parsing with nom
└── schema.rs          # JSON schema validation
```

## Development Workflow

### Adding New Features
1. Define schema in plugin's manifest.json
2. Add token patterns to tokenizer
3. Implement plugin logic with WIT exports
4. Test with natural language commands
5. Ensure VAT/tax calculations are correct

### Testing Tax Calculations
Always verify against Philippine tax rules:
- VAT: Amount ÷ 1.12 for VAT-exclusive computation
- Senior discount: 20% on VAT-exempt amount
- EWT: Deducted from gross, not from net

### Common Commands for Testing
```
# Basic sale with VAT
@client Juan bought groceries 1000

# Professional service with EWT  
@supplier lawyer invoice 50000 legal fees

# Senior citizen transaction
@customer senior ordered food 600 drinks 400

# Generate monthly VAT return
generate VAT return for October 2024
```

## MVP Implementation Phases (from THE_PLAN.md)

### Phase 1: Core Runtime (Week 1-2)
- Tokenizer for Filipino/English commands
- Transaction validator (debit=credit)
- Journal engine for double-entry
- VAT engine for 12% computation

### Phase 2: Invoice Plugin (Week 3-4)
- Accept tokenized sales input
- Compute VAT with exemptions
- Generate journal entries
- Track payment status

### Phase 3: BIR Form Generator (Week 5-6)
- Aggregate monthly transactions
- Compute Output VAT - Input VAT
- Generate 2550M in BIR format
- Validate against rules

### Phase 4: Natural Language Interface (Week 7-8)
- Implement priority tokens (@client, @invoice, @payment)
- Support Taglish code-switching
- Handle common bookkeeping phrases

## Error Handling & Validation

### Business Rule Enforcement
- Every transaction MUST balance (debit = credit)
- VAT rate MUST be 12% unless exempt/zero-rated
- Senior/PWD discount MUST be 20% on VAT-exempt
- Receipt numbers MUST be sequential

### Common Validation Errors
- Missing OR/SI number → Flag for audit
- Invalid VAT computation → Auto-correct and warn
- Missing EWT on professional services → Apply automatically
- Unbalanced journal entry → Reject transaction

## Performance Targets

### MVP Success Metrics
- Invoice creation: 5 minutes → 30 seconds
- VAT computation: Zero errors
- BIR form generation: 100% accuracy
- Natural language understanding: No training required

### Optimization Focus
- Cache compiled WASM modules
- Minimize JSON serialization overhead
- Use efficient event routing
- Implement lazy plugin loading
- make sure to enforce @TAX_GPT.md as that is the core of our product
- all markdown on .kilocode/rules are to be followed
- make sure to reference @ACCOUNTING.md as a knowledge base on bir compliance
- when adding new dependencies make sure to run this to check for the latest stable version of the crate -> ```shell
curl -s https://crates.io/api/v1/crates/sha2/versions | grep -o '"num":"[^"]*"' | grep -E '^"num":"[0-9]+\.[0-9]+\.[0-9]+"' | head -1 | cut -d'"' -f4
```