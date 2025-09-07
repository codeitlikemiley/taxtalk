use serde_json::Value;
use taxtalk_ui_core::ComponentType;

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

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    GreaterThan,
    LessThan,
    In,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionAction {
    Show,
    Hide,
    Require,
    Disable,
}

// Example usage for payment method form
pub fn create_payment_form_fields() -> Vec<FormField> {
    vec![
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
            default_value: Some(serde_json::json!("cash")),
        },
        FormField {
            name: "reference_number".into(),
            label: "Reference Number".into(),
            component: ComponentType::TextInput,
            required: false,
            condition: Some(FieldCondition {
                when: "payment_method".into(),
                operator: ConditionOperator::In,
                value: serde_json::json!(["gcash", "maya", "bank_transfer"]),
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
                value: serde_json::json!("cash"),
                action: ConditionAction::Require,
            }),
            default_value: None,
        },
    ]
}