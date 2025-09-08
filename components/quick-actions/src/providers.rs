use crate::types::*;
use chrono::{Local, Duration, Datelike};
use std::collections::HashMap;

/// Provider for Philippine business-specific quick actions
pub struct PhilippineBusinessProvider;

impl PhilippineBusinessProvider {
    pub fn new() -> Self {
        Self
    }
}

impl QuickActionProvider for PhilippineBusinessProvider {
    fn get_actions(&self, context: &str, _query: &str) -> Vec<QuickAction> {
        match context {
            "amount" | "money" | "payment_amount" => vec![
                QuickAction {
                    id: "amount_1k".to_string(),
                    label: "₱1,000".to_string(),
                    value: "1000".to_string(),
                    icon: Some("💵".to_string()),
                    color: Some("green".to_string()),
                    description: Some("One thousand pesos".to_string()),
                    keyboard_shortcut: Some("1".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "amount_5k".to_string(),
                    label: "₱5,000".to_string(),
                    value: "5000".to_string(),
                    icon: Some("💵".to_string()),
                    color: Some("green".to_string()),
                    description: Some("Five thousand pesos".to_string()),
                    keyboard_shortcut: Some("2".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "amount_10k".to_string(),
                    label: "₱10,000".to_string(),
                    value: "10000".to_string(),
                    icon: Some("💵".to_string()),
                    color: Some("green".to_string()),
                    description: Some("Ten thousand pesos".to_string()),
                    keyboard_shortcut: Some("3".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "amount_25k".to_string(),
                    label: "₱25,000".to_string(),
                    value: "25000".to_string(),
                    icon: Some("💵".to_string()),
                    color: Some("green".to_string()),
                    description: Some("Twenty-five thousand pesos".to_string()),
                    keyboard_shortcut: Some("4".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "amount_50k".to_string(),
                    label: "₱50,000".to_string(),
                    value: "50000".to_string(),
                    icon: Some("💵".to_string()),
                    color: Some("green".to_string()),
                    description: Some("Fifty thousand pesos".to_string()),
                    keyboard_shortcut: Some("5".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "amount_100k".to_string(),
                    label: "₱100,000".to_string(),
                    value: "100000".to_string(),
                    icon: Some("💰".to_string()),
                    color: Some("gold".to_string()),
                    description: Some("One hundred thousand pesos".to_string()),
                    keyboard_shortcut: Some("6".to_string()),
                    metadata: HashMap::new(),
                },
            ],
            
            "payment_method" | "method" => vec![
                QuickAction {
                    id: "method_cash".to_string(),
                    label: "Cash".to_string(),
                    value: "cash".to_string(),
                    icon: Some("💰".to_string()),
                    color: Some("green".to_string()),
                    description: Some("Cash payment".to_string()),
                    keyboard_shortcut: Some("C".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "method_bank".to_string(),
                    label: "Bank Transfer".to_string(),
                    value: "bank_transfer".to_string(),
                    icon: Some("🏦".to_string()),
                    color: Some("blue".to_string()),
                    description: Some("Direct bank transfer".to_string()),
                    keyboard_shortcut: Some("B".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "method_gcash".to_string(),
                    label: "GCash".to_string(),
                    value: "gcash".to_string(),
                    icon: Some("📱".to_string()),
                    color: Some("blue".to_string()),
                    description: Some("GCash mobile payment".to_string()),
                    keyboard_shortcut: Some("G".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "method_maya".to_string(),
                    label: "Maya".to_string(),
                    value: "maya".to_string(),
                    icon: Some("📱".to_string()),
                    color: Some("purple".to_string()),
                    description: Some("Maya/PayMaya payment".to_string()),
                    keyboard_shortcut: Some("M".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "method_check".to_string(),
                    label: "Check".to_string(),
                    value: "check".to_string(),
                    icon: Some("📝".to_string()),
                    color: Some("gray".to_string()),
                    description: Some("Check payment".to_string()),
                    keyboard_shortcut: Some("K".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "method_card".to_string(),
                    label: "Credit Card".to_string(),
                    value: "credit_card".to_string(),
                    icon: Some("💳".to_string()),
                    color: Some("indigo".to_string()),
                    description: Some("Credit/Debit card".to_string()),
                    keyboard_shortcut: Some("R".to_string()),
                    metadata: HashMap::new(),
                },
            ],
            
            "due_date" | "date" | "deadline" => vec![
                QuickAction {
                    id: "date_today".to_string(),
                    label: "Today".to_string(),
                    value: Local::now().format("%Y-%m-%d").to_string(),
                    icon: Some("📅".to_string()),
                    color: Some("red".to_string()),
                    description: Some("Due today".to_string()),
                    keyboard_shortcut: Some("T".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "date_tomorrow".to_string(),
                    label: "Tomorrow".to_string(),
                    value: (Local::now() + Duration::days(1))
                        .format("%Y-%m-%d")
                        .to_string(),
                    icon: Some("📅".to_string()),
                    color: Some("orange".to_string()),
                    description: Some("Due tomorrow".to_string()),
                    keyboard_shortcut: Some("M".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "date_7days".to_string(),
                    label: "7 Days".to_string(),
                    value: (Local::now() + Duration::days(7))
                        .format("%Y-%m-%d")
                        .to_string(),
                    icon: Some("📅".to_string()),
                    color: Some("yellow".to_string()),
                    description: Some("Due in one week".to_string()),
                    keyboard_shortcut: Some("7".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "date_15days".to_string(),
                    label: "15 Days".to_string(),
                    value: (Local::now() + Duration::days(15))
                        .format("%Y-%m-%d")
                        .to_string(),
                    icon: Some("📅".to_string()),
                    color: Some("blue".to_string()),
                    description: Some("Due in 15 days".to_string()),
                    keyboard_shortcut: Some("F".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "date_30days".to_string(),
                    label: "30 Days".to_string(),
                    value: (Local::now() + Duration::days(30))
                        .format("%Y-%m-%d")
                        .to_string(),
                    icon: Some("📅".to_string()),
                    color: Some("green".to_string()),
                    description: Some("Due in 30 days".to_string()),
                    keyboard_shortcut: Some("3".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "date_eom".to_string(),
                    label: "End of Month".to_string(),
                    value: {
                        let now = Local::now();
                        let next_month = if now.month() == 12 {
                            now.with_year(now.year() + 1)
                                .unwrap()
                                .with_month(1)
                                .unwrap()
                        } else {
                            now.with_month(now.month() + 1).unwrap()
                        };
                        (next_month - Duration::days(1))
                            .format("%Y-%m-%d")
                            .to_string()
                    },
                    icon: Some("📅".to_string()),
                    color: Some("purple".to_string()),
                    description: Some("End of current month".to_string()),
                    keyboard_shortcut: Some("E".to_string()),
                    metadata: HashMap::new(),
                },
            ],
            
            "tax" | "vat" | "withholding" => vec![
                QuickAction {
                    id: "tax_vat12".to_string(),
                    label: "12% VAT".to_string(),
                    value: "vat_12".to_string(),
                    icon: Some("🧾".to_string()),
                    color: Some("blue".to_string()),
                    description: Some("Standard VAT rate".to_string()),
                    keyboard_shortcut: Some("V".to_string()),
                    metadata: HashMap::from([
                        ("rate".to_string(), "0.12".to_string()),
                        ("type".to_string(), "vat".to_string()),
                    ]),
                },
                QuickAction {
                    id: "tax_ewt_goods".to_string(),
                    label: "2% EWT (Goods)".to_string(),
                    value: "ewt_goods_2".to_string(),
                    icon: Some("📦".to_string()),
                    color: Some("orange".to_string()),
                    description: Some("Withholding tax for goods".to_string()),
                    keyboard_shortcut: Some("2".to_string()),
                    metadata: HashMap::from([
                        ("rate".to_string(), "0.02".to_string()),
                        ("type".to_string(), "ewt".to_string()),
                    ]),
                },
                QuickAction {
                    id: "tax_ewt_services".to_string(),
                    label: "10% EWT (Services)".to_string(),
                    value: "ewt_services_10".to_string(),
                    icon: Some("💼".to_string()),
                    color: Some("purple".to_string()),
                    description: Some("Withholding tax for professional services".to_string()),
                    keyboard_shortcut: Some("1".to_string()),
                    metadata: HashMap::from([
                        ("rate".to_string(), "0.10".to_string()),
                        ("type".to_string(), "ewt".to_string()),
                    ]),
                },
                QuickAction {
                    id: "tax_exempt".to_string(),
                    label: "Tax Exempt".to_string(),
                    value: "tax_exempt".to_string(),
                    icon: Some("🆓".to_string()),
                    color: Some("green".to_string()),
                    description: Some("No tax applicable".to_string()),
                    keyboard_shortcut: Some("X".to_string()),
                    metadata: HashMap::from([
                        ("rate".to_string(), "0".to_string()),
                        ("type".to_string(), "exempt".to_string()),
                    ]),
                },
                QuickAction {
                    id: "tax_zero_rated".to_string(),
                    label: "Zero-Rated".to_string(),
                    value: "zero_rated".to_string(),
                    icon: Some("0️⃣".to_string()),
                    color: Some("gray".to_string()),
                    description: Some("0% VAT (exports)".to_string()),
                    keyboard_shortcut: Some("Z".to_string()),
                    metadata: HashMap::from([
                        ("rate".to_string(), "0".to_string()),
                        ("type".to_string(), "zero_rated".to_string()),
                    ]),
                },
            ],
            
            "discount" => vec![
                QuickAction {
                    id: "discount_senior".to_string(),
                    label: "Senior Citizen (20%)".to_string(),
                    value: "senior_20".to_string(),
                    icon: Some("👴".to_string()),
                    color: Some("purple".to_string()),
                    description: Some("20% discount + VAT exempt".to_string()),
                    keyboard_shortcut: Some("S".to_string()),
                    metadata: HashMap::from([
                        ("rate".to_string(), "0.20".to_string()),
                        ("vat_exempt".to_string(), "true".to_string()),
                    ]),
                },
                QuickAction {
                    id: "discount_pwd".to_string(),
                    label: "PWD (20%)".to_string(),
                    value: "pwd_20".to_string(),
                    icon: Some("♿".to_string()),
                    color: Some("blue".to_string()),
                    description: Some("20% discount + VAT exempt".to_string()),
                    keyboard_shortcut: Some("P".to_string()),
                    metadata: HashMap::from([
                        ("rate".to_string(), "0.20".to_string()),
                        ("vat_exempt".to_string(), "true".to_string()),
                    ]),
                },
                QuickAction {
                    id: "discount_volume".to_string(),
                    label: "Volume Discount".to_string(),
                    value: "volume".to_string(),
                    icon: Some("📦".to_string()),
                    color: Some("orange".to_string()),
                    description: Some("Bulk purchase discount".to_string()),
                    keyboard_shortcut: Some("V".to_string()),
                    metadata: HashMap::new(),
                },
                QuickAction {
                    id: "discount_promo".to_string(),
                    label: "Promotional".to_string(),
                    value: "promo".to_string(),
                    icon: Some("🎁".to_string()),
                    color: Some("pink".to_string()),
                    description: Some("Special promotion".to_string()),
                    keyboard_shortcut: Some("R".to_string()),
                    metadata: HashMap::new(),
                },
            ],
            
            _ => vec![],
        }
    }
    
    fn get_groups(&self, context: &str) -> Vec<ActionGroup> {
        match context {
            "tax" => vec![
                ActionGroup {
                    id: "vat_group".to_string(),
                    label: "VAT Options".to_string(),
                    icon: Some("🧾".to_string()),
                    priority: 1,
                    actions: vec![
                        QuickAction {
                            id: "vat_12".to_string(),
                            label: "12% VAT".to_string(),
                            value: "vat_12".to_string(),
                            icon: Some("🧾".to_string()),
                            color: Some("blue".to_string()),
                            description: Some("Standard rate".to_string()),
                            keyboard_shortcut: None,
                            metadata: HashMap::new(),
                        },
                        QuickAction {
                            id: "vat_0".to_string(),
                            label: "Zero-Rated".to_string(),
                            value: "zero_rated".to_string(),
                            icon: Some("0️⃣".to_string()),
                            color: Some("gray".to_string()),
                            description: Some("Exports".to_string()),
                            keyboard_shortcut: None,
                            metadata: HashMap::new(),
                        },
                    ],
                },
                ActionGroup {
                    id: "ewt_group".to_string(),
                    label: "Withholding Tax".to_string(),
                    icon: Some("💼".to_string()),
                    priority: 2,
                    actions: vec![
                        QuickAction {
                            id: "ewt_2".to_string(),
                            label: "2% (Goods)".to_string(),
                            value: "ewt_goods_2".to_string(),
                            icon: Some("📦".to_string()),
                            color: Some("orange".to_string()),
                            description: None,
                            keyboard_shortcut: None,
                            metadata: HashMap::new(),
                        },
                        QuickAction {
                            id: "ewt_10".to_string(),
                            label: "10% (Services)".to_string(),
                            value: "ewt_services_10".to_string(),
                            icon: Some("💼".to_string()),
                            color: Some("purple".to_string()),
                            description: None,
                            keyboard_shortcut: None,
                            metadata: HashMap::new(),
                        },
                    ],
                },
            ],
            _ => vec![],
        }
    }
    
    fn can_provide(&self, context: &str) -> bool {
        matches!(
            context,
            "amount" | "money" | "payment_amount" | 
            "payment_method" | "method" |
            "due_date" | "date" | "deadline" |
            "tax" | "vat" | "withholding" |
            "discount"
        )
    }
}