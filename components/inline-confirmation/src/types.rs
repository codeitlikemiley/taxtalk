use serde::{Deserialize, Serialize};
use std::fmt;

/// Validation state for the input
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ValidationState {
    /// Initial state, no validation performed
    Idle,
    /// Currently validating (async validation)
    Validating,
    /// Validation passed
    Valid(Option<String>), // Optional success message
    /// Validation failed
    Invalid(String), // Error message
    /// Warning state (valid but needs attention)
    Warning(String),
}

impl Default for ValidationState {
    fn default() -> Self {
        ValidationState::Idle
    }
}

/// Confirmation action types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConfirmAction {
    /// Delete action (dangerous)
    Delete,
    /// Save/Update action
    Save,
    /// Submit form
    Submit,
    /// Cancel/Reset action
    Cancel,
    /// Custom action
    Custom(String),
}

impl fmt::Display for ConfirmAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfirmAction::Delete => write!(f, "Delete"),
            ConfirmAction::Save => write!(f, "Save"),
            ConfirmAction::Submit => write!(f, "Submit"),
            ConfirmAction::Cancel => write!(f, "Cancel"),
            ConfirmAction::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Configuration for inline confirmation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineConfirmConfig {
    /// Show validation on blur
    pub validate_on_blur: bool,
    /// Show validation on change
    pub validate_on_change: bool,
    /// Debounce validation in ms
    pub validation_debounce: u32,
    /// Show success checkmark
    pub show_success_icon: bool,
    /// Show error icon
    pub show_error_icon: bool,
    /// Show loading spinner
    pub show_loading_spinner: bool,
    /// Require confirmation for actions
    pub require_confirmation: bool,
    /// Confirmation timeout in ms (0 = no timeout)
    pub confirmation_timeout: u32,
    /// Auto-focus on error
    pub auto_focus_on_error: bool,
    /// Shake animation on error
    pub shake_on_error: bool,
}

impl Default for InlineConfirmConfig {
    fn default() -> Self {
        Self {
            validate_on_blur: true,
            validate_on_change: false,
            validation_debounce: 300,
            show_success_icon: true,
            show_error_icon: true,
            show_loading_spinner: true,
            require_confirmation: true,
            confirmation_timeout: 3000,
            auto_focus_on_error: true,
            shake_on_error: true,
        }
    }
}

/// Validation rule types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValidationRule {
    /// Required field
    Required(String), // Error message
    /// Minimum length
    MinLength(usize, String),
    /// Maximum length
    MaxLength(usize, String),
    /// Regular expression pattern
    Pattern(String, String), // (pattern, error message)
    /// Email validation
    Email(String),
    /// URL validation
    Url(String),
    /// Number range
    Range(f64, f64, String), // (min, max, error message)
    /// Custom validation function name
    Custom(String, String), // (function_name, error message)
    /// Philippine-specific validations
    PhilippineTIN(String),
    PhilippinePhone(String),
    PhilippineZip(String),
}

/// Input type for specific validations
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Currency,
    Percentage,
    TIN,
    Date,
    Time,
    DateTime,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputType::Text => write!(f, "text"),
            InputType::Email => write!(f, "email"),
            InputType::Password => write!(f, "password"),
            InputType::Number => write!(f, "number"),
            InputType::Tel => write!(f, "tel"),
            InputType::Url => write!(f, "url"),
            InputType::Currency => write!(f, "text"),
            InputType::Percentage => write!(f, "text"),
            InputType::TIN => write!(f, "text"),
            InputType::Date => write!(f, "date"),
            InputType::Time => write!(f, "time"),
            InputType::DateTime => write!(f, "datetime-local"),
        }
    }
}

/// Confirmation state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConfirmationState {
    /// Initial state
    Idle,
    /// Awaiting confirmation
    Pending,
    /// Action confirmed
    Confirmed,
    /// Action cancelled
    Cancelled,
}

impl Default for ConfirmationState {
    fn default() -> Self {
        ConfirmationState::Idle
    }
}

/// Animation types for feedback
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AnimationType {
    None,
    Shake,
    Pulse,
    Bounce,
    Fade,
    Slide,
}

/// Feedback message with severity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackMessage {
    pub text: String,
    pub severity: FeedbackSeverity,
    pub dismissible: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FeedbackSeverity {
    Success,
    Error,
    Warning,
    Info,
}

/// Philippine-specific validation data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhilippineValidation {
    /// Valid TIN format patterns
    pub tin_patterns: Vec<String>,
    /// Valid phone number patterns
    pub phone_patterns: Vec<String>,
    /// Valid zip codes
    pub zip_codes: Vec<String>,
}

impl Default for PhilippineValidation {
    fn default() -> Self {
        Self {
            tin_patterns: vec![
                r"^\d{3}-\d{3}-\d{3}$".to_string(),      // 123-456-789
                r"^\d{3}-\d{3}-\d{3}-\d{3}$".to_string(), // 123-456-789-000
            ],
            phone_patterns: vec![
                r"^09\d{9}$".to_string(),           // 09123456789
                r"^\+639\d{9}$".to_string(),        // +639123456789
                r"^639\d{9}$".to_string(),          // 639123456789
            ],
            zip_codes: vec![
                // Sample Metro Manila zip codes
                "1000".to_string(), // Manila
                "1100".to_string(), // Quezon City
                "1200".to_string(), // Makati
                "1300".to_string(), // Pasay
                "1600".to_string(), // Pasig
                "1700".to_string(), // Parañaque
                // Add more as needed
            ],
        }
    }
}