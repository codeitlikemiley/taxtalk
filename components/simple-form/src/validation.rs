use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validation result
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }
    
    pub fn error_message(&self) -> Option<&str> {
        match self {
            ValidationResult::Invalid(msg) => Some(msg),
            ValidationResult::Valid => None,
        }
    }
}

/// Validation rule types
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ValidationRule {
    Required,
    MinLength { min: usize },
    MaxLength { max: usize },
    Pattern { regex: String, message: String },
    Email,
    Url,
    Number { min: Option<f64>, max: Option<f64> },
    Integer { min: Option<i64>, max: Option<i64> },
    Date { min: Option<String>, max: Option<String> },
    Custom { validator: String, message: String },
    PhoneNumber,
    TaxId,
    Currency { min: Option<f64>, max: Option<f64> },
}

/// Field validator
pub struct FieldValidator {
    rules: Vec<ValidationRule>,
    custom_validators: HashMap<String, Box<dyn Fn(&str) -> bool>>,
}

impl FieldValidator {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            custom_validators: HashMap::new(),
        }
    }
    
    pub fn required(mut self) -> Self {
        self.rules.push(ValidationRule::Required);
        self
    }
    
    pub fn min_length(mut self, min: usize) -> Self {
        self.rules.push(ValidationRule::MinLength { min });
        self
    }
    
    pub fn max_length(mut self, max: usize) -> Self {
        self.rules.push(ValidationRule::MaxLength { max });
        self
    }
    
    pub fn pattern(mut self, regex: &str, message: &str) -> Self {
        self.rules.push(ValidationRule::Pattern {
            regex: regex.to_string(),
            message: message.to_string(),
        });
        self
    }
    
    pub fn email(mut self) -> Self {
        self.rules.push(ValidationRule::Email);
        self
    }
    
    pub fn url(mut self) -> Self {
        self.rules.push(ValidationRule::Url);
        self
    }
    
    pub fn number(mut self, min: Option<f64>, max: Option<f64>) -> Self {
        self.rules.push(ValidationRule::Number { min, max });
        self
    }
    
    pub fn integer(mut self, min: Option<i64>, max: Option<i64>) -> Self {
        self.rules.push(ValidationRule::Integer { min, max });
        self
    }
    
    pub fn date(mut self, min: Option<String>, max: Option<String>) -> Self {
        self.rules.push(ValidationRule::Date { min, max });
        self
    }
    
    pub fn phone_number(mut self) -> Self {
        self.rules.push(ValidationRule::PhoneNumber);
        self
    }
    
    pub fn tax_id(mut self) -> Self {
        self.rules.push(ValidationRule::TaxId);
        self
    }
    
    pub fn currency(mut self, min: Option<f64>, max: Option<f64>) -> Self {
        self.rules.push(ValidationRule::Currency { min, max });
        self
    }
    
    pub fn custom(mut self, name: &str, message: &str, validator: Box<dyn Fn(&str) -> bool>) -> Self {
        self.rules.push(ValidationRule::Custom {
            validator: name.to_string(),
            message: message.to_string(),
        });
        self.custom_validators.insert(name.to_string(), validator);
        self
    }
    
    pub fn validate(&self, value: &str) -> ValidationResult {
        for rule in &self.rules {
            let result = self.validate_rule(value, rule);
            if !result.is_valid() {
                return result;
            }
        }
        ValidationResult::Valid
    }
    
    fn validate_rule(&self, value: &str, rule: &ValidationRule) -> ValidationResult {
        match rule {
            ValidationRule::Required => {
                if value.trim().is_empty() {
                    ValidationResult::Invalid("This field is required".to_string())
                } else {
                    ValidationResult::Valid
                }
            }
            
            ValidationRule::MinLength { min } => {
                if value.len() < *min {
                    ValidationResult::Invalid(format!("Must be at least {} characters", min))
                } else {
                    ValidationResult::Valid
                }
            }
            
            ValidationRule::MaxLength { max } => {
                if value.len() > *max {
                    ValidationResult::Invalid(format!("Must be at most {} characters", max))
                } else {
                    ValidationResult::Valid
                }
            }
            
            ValidationRule::Pattern { regex, message } => {
                if let Ok(re) = Regex::new(regex) {
                    if re.is_match(value) {
                        ValidationResult::Valid
                    } else {
                        ValidationResult::Invalid(message.clone())
                    }
                } else {
                    ValidationResult::Invalid("Invalid pattern".to_string())
                }
            }
            
            ValidationRule::Email => {
                // Basic email validation
                let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
                if email_regex.is_match(value) {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Invalid("Invalid email address".to_string())
                }
            }
            
            ValidationRule::Url => {
                // Basic URL validation
                let url_regex = Regex::new(r"^(https?://)?([a-zA-Z0-9.-]+)(\.[a-zA-Z]{2,})(:[0-9]+)?(/.*)?$").unwrap();
                if url_regex.is_match(value) {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Invalid("Invalid URL".to_string())
                }
            }
            
            ValidationRule::Number { min, max } => {
                match value.parse::<f64>() {
                    Ok(num) => {
                        if let Some(min_val) = min {
                            if num < *min_val {
                                return ValidationResult::Invalid(format!("Must be at least {}", min_val));
                            }
                        }
                        if let Some(max_val) = max {
                            if num > *max_val {
                                return ValidationResult::Invalid(format!("Must be at most {}", max_val));
                            }
                        }
                        ValidationResult::Valid
                    }
                    Err(_) => ValidationResult::Invalid("Must be a valid number".to_string()),
                }
            }
            
            ValidationRule::Integer { min, max } => {
                match value.parse::<i64>() {
                    Ok(num) => {
                        if let Some(min_val) = min {
                            if num < *min_val {
                                return ValidationResult::Invalid(format!("Must be at least {}", min_val));
                            }
                        }
                        if let Some(max_val) = max {
                            if num > *max_val {
                                return ValidationResult::Invalid(format!("Must be at most {}", max_val));
                            }
                        }
                        ValidationResult::Valid
                    }
                    Err(_) => ValidationResult::Invalid("Must be a valid integer".to_string()),
                }
            }
            
            ValidationRule::Date { min, max } => {
                // Basic date validation (YYYY-MM-DD format)
                let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
                if !date_regex.is_match(value) {
                    return ValidationResult::Invalid("Invalid date format (use YYYY-MM-DD)".to_string());
                }
                
                if let Some(min_date) = min {
                    if value < min_date {
                        return ValidationResult::Invalid(format!("Date must be after {}", min_date));
                    }
                }
                if let Some(max_date) = max {
                    if value > max_date {
                        return ValidationResult::Invalid(format!("Date must be before {}", max_date));
                    }
                }
                ValidationResult::Valid
            }
            
            ValidationRule::PhoneNumber => {
                // Philippine phone number format
                let phone_regex = Regex::new(r"^(\+63|0)?9\d{9}$").unwrap();
                if phone_regex.is_match(value) {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Invalid("Invalid phone number".to_string())
                }
            }
            
            ValidationRule::TaxId => {
                // Philippine TIN format (###-###-###-###)
                let tin_regex = Regex::new(r"^\d{3}-?\d{3}-?\d{3}-?\d{3}$").unwrap();
                if tin_regex.is_match(value) {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Invalid("Invalid TIN format".to_string())
                }
            }
            
            ValidationRule::Currency { min, max } => {
                // Remove currency symbols and commas
                let cleaned = value.replace(['₱', '$', ','], "");
                match cleaned.parse::<f64>() {
                    Ok(amount) => {
                        if let Some(min_val) = min {
                            if amount < *min_val {
                                return ValidationResult::Invalid(format!("Must be at least ₱{:.2}", min_val));
                            }
                        }
                        if let Some(max_val) = max {
                            if amount > *max_val {
                                return ValidationResult::Invalid(format!("Must be at most ₱{:.2}", max_val));
                            }
                        }
                        ValidationResult::Valid
                    }
                    Err(_) => ValidationResult::Invalid("Invalid currency amount".to_string()),
                }
            }
            
            ValidationRule::Custom { validator, message } => {
                if let Some(custom_fn) = self.custom_validators.get(validator) {
                    if custom_fn(value) {
                        ValidationResult::Valid
                    } else {
                        ValidationResult::Invalid(message.clone())
                    }
                } else {
                    ValidationResult::Valid
                }
            }
        }
    }
}

/// Form validator for multiple fields
pub struct FormValidator {
    field_validators: HashMap<String, FieldValidator>,
}

impl FormValidator {
    pub fn new() -> Self {
        Self {
            field_validators: HashMap::new(),
        }
    }
    
    pub fn add_field(&mut self, name: &str, validator: FieldValidator) {
        self.field_validators.insert(name.to_string(), validator);
    }
    
    pub fn validate_field(&self, field_name: &str, value: &str) -> ValidationResult {
        if let Some(validator) = self.field_validators.get(field_name) {
            validator.validate(value)
        } else {
            ValidationResult::Valid
        }
    }
    
    pub fn validate_all(&self, values: &HashMap<String, String>) -> HashMap<String, ValidationResult> {
        let mut results = HashMap::new();
        
        for (field_name, validator) in &self.field_validators {
            let value = values.get(field_name).map(|s| s.as_str()).unwrap_or("");
            results.insert(field_name.clone(), validator.validate(value));
        }
        
        results
    }
    
    pub fn is_valid(&self, values: &HashMap<String, String>) -> bool {
        self.validate_all(values)
            .values()
            .all(|result| result.is_valid())
    }
}

/// Common validators for Philippine context
pub mod ph_validators {
    use super::FieldValidator;
    
    pub fn tin_validator() -> FieldValidator {
        FieldValidator::new()
            .required()
            .tax_id()
    }
    
    pub fn phone_validator() -> FieldValidator {
        FieldValidator::new()
            .required()
            .phone_number()
    }
    
    pub fn peso_amount_validator(min: Option<f64>, max: Option<f64>) -> FieldValidator {
        FieldValidator::new()
            .required()
            .currency(min, max)
    }
    
    pub fn business_name_validator() -> FieldValidator {
        FieldValidator::new()
            .required()
            .min_length(3)
            .max_length(100)
    }
    
    pub fn address_validator() -> FieldValidator {
        FieldValidator::new()
            .required()
            .min_length(10)
            .max_length(200)
    }
}