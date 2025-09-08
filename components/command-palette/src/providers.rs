use crate::types::*;

/// Default commands for Philippine business context
pub fn default_commands() -> Vec<Command> {
    vec![
        // Sales commands
        Command {
            id: "create_invoice".to_string(),
            label: "Create Invoice".to_string(),
            description: "Generate a new invoice for a client".to_string(),
            icon: Some("📄".to_string()),
            shortcut: Some("Ctrl+I".to_string()),
            category: CommandCategory::Sales,
            keywords: vec!["invoice", "bill", "sales", "client"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "invoice".to_string(),
                action: "create".to_string(),
            },
        },
        Command {
            id: "create_quotation".to_string(),
            label: "Create Quotation".to_string(),
            description: "Create a price quotation for a potential client".to_string(),
            icon: Some("📋".to_string()),
            shortcut: None,
            category: CommandCategory::Sales,
            keywords: vec!["quote", "proposal", "estimate"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "quotation".to_string(),
                action: "create".to_string(),
            },
        },
        
        // Finance commands
        Command {
            id: "record_payment".to_string(),
            label: "Record Payment".to_string(),
            description: "Record a payment from a client".to_string(),
            icon: Some("💰".to_string()),
            shortcut: Some("Ctrl+P".to_string()),
            category: CommandCategory::Finance,
            keywords: vec!["payment", "receive", "collection", "gcash", "bank"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "payment".to_string(),
                action: "create".to_string(),
            },
        },
        Command {
            id: "add_expense".to_string(),
            label: "Add Expense".to_string(),
            description: "Record a business expense".to_string(),
            icon: Some("💸".to_string()),
            shortcut: Some("Ctrl+E".to_string()),
            category: CommandCategory::Finance,
            keywords: vec!["expense", "spent", "purchase", "bought"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "expense".to_string(),
                action: "create".to_string(),
            },
        },
        Command {
            id: "bank_reconciliation".to_string(),
            label: "Bank Reconciliation".to_string(),
            description: "Reconcile bank statements with records".to_string(),
            icon: Some("🏦".to_string()),
            shortcut: None,
            category: CommandCategory::Finance,
            keywords: vec!["bank", "reconcile", "statement"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "bank".to_string(),
                action: "reconcile".to_string(),
            },
        },
        
        // Tax commands
        Command {
            id: "vat_return".to_string(),
            label: "Generate VAT Return".to_string(),
            description: "Create monthly VAT return (BIR Form 2550M)".to_string(),
            icon: Some("📊".to_string()),
            shortcut: Some("Ctrl+V".to_string()),
            category: CommandCategory::Tax,
            keywords: vec!["vat", "tax", "bir", "2550m", "return"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "tax".to_string(),
                action: "vat_return".to_string(),
            },
        },
        Command {
            id: "ewt_report".to_string(),
            label: "Generate EWT Report".to_string(),
            description: "Generate Expanded Withholding Tax report".to_string(),
            icon: Some("📑".to_string()),
            shortcut: None,
            category: CommandCategory::Tax,
            keywords: vec!["ewt", "withholding", "tax", "2307"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "tax".to_string(),
                action: "ewt_report".to_string(),
            },
        },
        Command {
            id: "quarterly_tax".to_string(),
            label: "Quarterly Income Tax".to_string(),
            description: "File quarterly income tax (Form 1701Q)".to_string(),
            icon: Some("💼".to_string()),
            shortcut: None,
            category: CommandCategory::Tax,
            keywords: vec!["quarterly", "income", "1701q"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "tax".to_string(),
                action: "quarterly_return".to_string(),
            },
        },
        
        // Contact commands
        Command {
            id: "new_client".to_string(),
            label: "New Client".to_string(),
            description: "Add a new client to the system".to_string(),
            icon: Some("👤".to_string()),
            shortcut: None,
            category: CommandCategory::Contacts,
            keywords: vec!["client", "customer", "add", "new"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "contacts".to_string(),
                action: "create_client".to_string(),
            },
        },
        Command {
            id: "new_supplier".to_string(),
            label: "New Supplier".to_string(),
            description: "Add a new supplier/vendor".to_string(),
            icon: Some("🏢".to_string()),
            shortcut: None,
            category: CommandCategory::Contacts,
            keywords: vec!["supplier", "vendor", "provider"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "contacts".to_string(),
                action: "create_supplier".to_string(),
            },
        },
        
        // Report commands
        Command {
            id: "profit_loss".to_string(),
            label: "Profit & Loss Report".to_string(),
            description: "Generate income statement".to_string(),
            icon: Some("📈".to_string()),
            shortcut: None,
            category: CommandCategory::Reports,
            keywords: vec!["profit", "loss", "income", "statement", "pnl"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "reports".to_string(),
                action: "profit_loss".to_string(),
            },
        },
        Command {
            id: "balance_sheet".to_string(),
            label: "Balance Sheet".to_string(),
            description: "Generate balance sheet report".to_string(),
            icon: Some("⚖️".to_string()),
            shortcut: None,
            category: CommandCategory::Reports,
            keywords: vec!["balance", "sheet", "assets", "liabilities"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "reports".to_string(),
                action: "balance_sheet".to_string(),
            },
        },
        Command {
            id: "cash_flow".to_string(),
            label: "Cash Flow Statement".to_string(),
            description: "Generate cash flow report".to_string(),
            icon: Some("💵".to_string()),
            shortcut: None,
            category: CommandCategory::Reports,
            keywords: vec!["cash", "flow", "liquidity"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "reports".to_string(),
                action: "cash_flow".to_string(),
            },
        },
        
        // Navigation commands
        Command {
            id: "go_dashboard".to_string(),
            label: "Go to Dashboard".to_string(),
            description: "Navigate to main dashboard".to_string(),
            icon: Some("🏠".to_string()),
            shortcut: Some("Ctrl+D".to_string()),
            category: CommandCategory::Navigation,
            keywords: vec!["dashboard", "home", "main"].into_iter().map(String::from).collect(),
            action: CommandAction::Navigate("/dashboard".to_string()),
        },
        Command {
            id: "go_settings".to_string(),
            label: "Settings".to_string(),
            description: "Open settings and preferences".to_string(),
            icon: Some("⚙️".to_string()),
            shortcut: Some("Ctrl+,".to_string()),
            category: CommandCategory::Settings,
            keywords: vec!["settings", "preferences", "config"].into_iter().map(String::from).collect(),
            action: CommandAction::Navigate("/settings".to_string()),
        },
    ]
}

/// Provider for Philippine-specific commands
pub fn philippine_commands() -> Vec<Command> {
    vec![
        Command {
            id: "senior_discount".to_string(),
            label: "Apply Senior/PWD Discount".to_string(),
            description: "Apply 20% discount with VAT exemption".to_string(),
            icon: Some("👴".to_string()),
            shortcut: None,
            category: CommandCategory::Sales,
            keywords: vec!["senior", "pwd", "discount", "20%"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "discount".to_string(),
                action: "senior_pwd".to_string(),
            },
        },
        Command {
            id: "bir_2307".to_string(),
            label: "Generate Form 2307".to_string(),
            description: "Certificate of Creditable Tax Withheld at Source".to_string(),
            icon: Some("📜".to_string()),
            shortcut: None,
            category: CommandCategory::Tax,
            keywords: vec!["2307", "certificate", "withholding"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "tax".to_string(),
                action: "form_2307".to_string(),
            },
        },
        Command {
            id: "gcash_payment".to_string(),
            label: "Record GCash Payment".to_string(),
            description: "Record payment received via GCash".to_string(),
            icon: Some("📱".to_string()),
            shortcut: None,
            category: CommandCategory::Finance,
            keywords: vec!["gcash", "mobile", "payment"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "payment".to_string(),
                action: "gcash".to_string(),
            },
        },
        Command {
            id: "maya_payment".to_string(),
            label: "Record Maya Payment".to_string(),
            description: "Record payment received via Maya/PayMaya".to_string(),
            icon: Some("📱".to_string()),
            shortcut: None,
            category: CommandCategory::Finance,
            keywords: vec!["maya", "paymaya", "mobile"].into_iter().map(String::from).collect(),
            action: CommandAction::Plugin {
                plugin: "payment".to_string(),
                action: "maya".to_string(),
            },
        },
    ]
}