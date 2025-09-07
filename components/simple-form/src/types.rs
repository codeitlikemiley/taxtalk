use serde::{Deserialize, Serialize};
use crate::validation::ValidationRule;

/// Field input types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Email,
    Password,
    Number,
    Integer,
    Currency,
    Date,
    DateTime,
    Time,
    Boolean,
    Select,
    MultiSelect,
    Textarea,
    File,
    Phone,
    Url,
    Color,
    Range,
}

/// Field configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldConfig {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub required: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub autofocus: bool,
    pub validation_rules: Vec<ValidationRule>,
    pub options: Option<Vec<SelectOption>>,
    pub default_value: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: Option<String>,
    pub rows: Option<u32>,
    pub cols: Option<u32>,
}

impl FieldConfig {
    pub fn new(name: &str, label: &str, field_type: FieldType) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            field_type,
            placeholder: None,
            help_text: None,
            required: false,
            disabled: false,
            readonly: false,
            autofocus: false,
            validation_rules: Vec::new(),
            options: None,
            default_value: None,
            min: None,
            max: None,
            step: None,
            rows: None,
            cols: None,
        }
    }
    
    pub fn placeholder(mut self, text: &str) -> Self {
        self.placeholder = Some(text.to_string());
        self
    }
    
    pub fn help_text(mut self, text: &str) -> Self {
        self.help_text = Some(text.to_string());
        self
    }
    
    pub fn required(mut self) -> Self {
        self.required = true;
        if !self.validation_rules.iter().any(|r| matches!(r, ValidationRule::Required)) {
            self.validation_rules.push(ValidationRule::Required);
        }
        self
    }
    
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
    
    pub fn autofocus(mut self) -> Self {
        self.autofocus = true;
        self
    }
    
    pub fn validation(mut self, rule: ValidationRule) -> Self {
        self.validation_rules.push(rule);
        self
    }
    
    pub fn options(mut self, opts: Vec<SelectOption>) -> Self {
        self.options = Some(opts);
        self
    }
    
    pub fn default_value(mut self, value: &str) -> Self {
        self.default_value = Some(value.to_string());
        self
    }
    
    pub fn min(mut self, value: &str) -> Self {
        self.min = Some(value.to_string());
        self
    }
    
    pub fn max(mut self, value: &str) -> Self {
        self.max = Some(value.to_string());
        self
    }
    
    pub fn step(mut self, value: &str) -> Self {
        self.step = Some(value.to_string());
        self
    }
    
    pub fn rows(mut self, count: u32) -> Self {
        self.rows = Some(count);
        self
    }
    
    pub fn cols(mut self, count: u32) -> Self {
        self.cols = Some(count);
        self
    }
}

/// Select option
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            disabled: false,
        }
    }
    
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Form configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormConfig {
    pub title: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<FieldConfig>,
    pub submit_text: String,
    pub cancel_text: String,
    pub show_cancel: bool,
    pub validate_on_blur: bool,
    pub validate_on_change: bool,
    pub show_required_indicator: bool,
}

impl FormConfig {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            fields: Vec::new(),
            submit_text: "Submit".to_string(),
            cancel_text: "Cancel".to_string(),
            show_cancel: true,
            validate_on_blur: true,
            validate_on_change: false,
            show_required_indicator: true,
        }
    }
    
    pub fn title(mut self, text: &str) -> Self {
        self.title = Some(text.to_string());
        self
    }
    
    pub fn description(mut self, text: &str) -> Self {
        self.description = Some(text.to_string());
        self
    }
    
    pub fn field(mut self, field: FieldConfig) -> Self {
        self.fields.push(field);
        self
    }
    
    pub fn submit_text(mut self, text: &str) -> Self {
        self.submit_text = text.to_string();
        self
    }
    
    pub fn cancel_text(mut self, text: &str) -> Self {
        self.cancel_text = text.to_string();
        self
    }
    
    pub fn hide_cancel(mut self) -> Self {
        self.show_cancel = false;
        self
    }
    
    pub fn validate_on_change(mut self) -> Self {
        self.validate_on_change = true;
        self
    }
    
    pub fn no_validate_on_blur(mut self) -> Self {
        self.validate_on_blur = false;
        self
    }
    
    pub fn hide_required_indicator(mut self) -> Self {
        self.show_required_indicator = false;
        self
    }
}

impl Default for FormConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Form submission result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormData {
    pub values: std::collections::HashMap<String, String>,
}

/// Common form presets for Philippine context
pub mod ph_forms {
    use super::*;
    use crate::validation::ValidationRule;
    
    pub fn client_form() -> FormConfig {
        FormConfig::new()
            .title("New Client")
            .description("Enter client information")
            .field(
                FieldConfig::new("name", "Business Name", FieldType::Text)
                    .required()
                    .placeholder("ABC Corporation")
                    .validation(ValidationRule::MinLength { min: 3 })
                    .validation(ValidationRule::MaxLength { max: 100 })
            )
            .field(
                FieldConfig::new("tin", "TIN", FieldType::Text)
                    .required()
                    .placeholder("###-###-###-###")
                    .help_text("Tax Identification Number")
                    .validation(ValidationRule::TaxId)
            )
            .field(
                FieldConfig::new("address", "Business Address", FieldType::Textarea)
                    .required()
                    .rows(3)
                    .validation(ValidationRule::MinLength { min: 10 })
            )
            .field(
                FieldConfig::new("phone", "Contact Number", FieldType::Phone)
                    .required()
                    .placeholder("09#########")
                    .validation(ValidationRule::PhoneNumber)
            )
            .field(
                FieldConfig::new("email", "Email Address", FieldType::Email)
                    .placeholder("contact@example.com")
                    .validation(ValidationRule::Email)
            )
    }
    
    pub fn invoice_form() -> FormConfig {
        FormConfig::new()
            .title("New Invoice")
            .description("Create a sales invoice")
            .field(
                FieldConfig::new("invoice_no", "Invoice Number", FieldType::Text)
                    .required()
                    .placeholder("INV-2024-0001")
            )
            .field(
                FieldConfig::new("client", "Client", FieldType::Select)
                    .required()
                    .options(vec![
                        SelectOption::new("", "Select a client"),
                        SelectOption::new("1", "ABC Corporation"),
                        SelectOption::new("2", "XYZ Industries"),
                    ])
            )
            .field(
                FieldConfig::new("date", "Invoice Date", FieldType::Date)
                    .required()
                    .default_value(&chrono::Local::now().format("%Y-%m-%d").to_string())
            )
            .field(
                FieldConfig::new("due_date", "Due Date", FieldType::Date)
                    .required()
            )
            .field(
                FieldConfig::new("amount", "Amount", FieldType::Currency)
                    .required()
                    .placeholder("₱0.00")
                    .validation(ValidationRule::Currency { min: Some(0.01), max: None })
            )
            .field(
                FieldConfig::new("vat_type", "VAT Type", FieldType::Select)
                    .required()
                    .options(vec![
                        SelectOption::new("inclusive", "VAT Inclusive"),
                        SelectOption::new("exclusive", "VAT Exclusive"),
                        SelectOption::new("exempt", "VAT Exempt"),
                        SelectOption::new("zero", "Zero Rated"),
                    ])
            )
    }
    
    pub fn payment_form() -> FormConfig {
        FormConfig::new()
            .title("Record Payment")
            .description("Record a payment transaction")
            .field(
                FieldConfig::new("payment_date", "Payment Date", FieldType::Date)
                    .required()
                    .default_value(&chrono::Local::now().format("%Y-%m-%d").to_string())
            )
            .field(
                FieldConfig::new("amount", "Amount", FieldType::Currency)
                    .required()
                    .placeholder("₱0.00")
                    .validation(ValidationRule::Currency { min: Some(0.01), max: None })
            )
            .field(
                FieldConfig::new("payment_method", "Payment Method", FieldType::Select)
                    .required()
                    .options(vec![
                        SelectOption::new("cash", "Cash"),
                        SelectOption::new("check", "Check"),
                        SelectOption::new("bank_transfer", "Bank Transfer"),
                        SelectOption::new("gcash", "GCash"),
                        SelectOption::new("maya", "Maya"),
                        SelectOption::new("credit_card", "Credit Card"),
                    ])
            )
            .field(
                FieldConfig::new("reference", "Reference Number", FieldType::Text)
                    .placeholder("Check/Transaction number")
            )
            .field(
                FieldConfig::new("notes", "Notes", FieldType::Textarea)
                    .rows(2)
                    .placeholder("Optional notes")
            )
    }
}