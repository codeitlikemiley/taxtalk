use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Message type in the chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    User,
    Assistant,
    System,
    Error,
    Success,
}

/// Chat message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub message_type: MessageType,
    pub content: String,
    pub timestamp: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub actions: Vec<MessageAction>,
    pub step_id: Option<String>,
}

/// Action button in a message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageAction {
    pub id: String,
    pub label: String,
    pub action_type: ActionType,
    pub value: Option<String>,
    pub style: ActionStyle,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    Submit,
    Cancel,
    Skip,
    Back,
    Custom(String),
    QuickReply(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionStyle {
    Primary,
    Secondary,
    Success,
    Danger,
    Link,
}

/// Workflow step definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub input_type: InputType,
    pub validation: Option<ValidationConfig>,
    pub required: bool,
    pub depends_on: Vec<String>,
    pub quick_replies: Vec<QuickReply>,
    pub help_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InputType {
    Text,
    Number,
    Currency,
    Date,
    Select(Vec<SelectOption>),
    MultiSelect(Vec<SelectOption>),
    EntityRef(String), // Entity type
    Boolean,
    File,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuickReply {
    pub label: String,
    pub value: String,
    pub style: Option<ActionStyle>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub custom_validator: Option<String>,
}

/// Workflow state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowState {
    pub workflow_id: String,
    pub current_step: Option<String>,
    pub completed_steps: Vec<String>,
    pub collected_data: HashMap<String, serde_json::Value>,
    pub is_complete: bool,
    pub error: Option<String>,
}

/// Chat configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuidedChatConfig {
    pub placeholder: String,
    pub welcome_message: Option<String>,
    pub completion_message: Option<String>,
    pub show_timestamps: bool,
    pub show_step_progress: bool,
    pub allow_back_navigation: bool,
    pub auto_scroll: bool,
    pub enable_markdown: bool,
    pub max_messages: Option<usize>,
    pub typing_indicator_delay: u32,
}

impl Default for GuidedChatConfig {
    fn default() -> Self {
        Self {
            placeholder: "Type your response...".to_string(),
            welcome_message: Some("Welcome! Let's get started.".to_string()),
            completion_message: Some("All done! Thank you.".to_string()),
            show_timestamps: true,
            show_step_progress: true,
            allow_back_navigation: true,
            auto_scroll: true,
            enable_markdown: true,
            max_messages: Some(100),
            typing_indicator_delay: 500,
        }
    }
}

/// Workflow definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub on_complete: Option<String>, // Callback function name
}

/// Philippine-specific workflows
impl Workflow {
    pub fn invoice_workflow() -> Self {
        Self {
            id: "create_invoice".to_string(),
            name: "Create Invoice".to_string(),
            description: "Step-by-step invoice creation".to_string(),
            steps: vec![
                WorkflowStep {
                    id: "client".to_string(),
                    name: "Select Client".to_string(),
                    prompt: "Who is this invoice for?".to_string(),
                    input_type: InputType::EntityRef("client".to_string()),
                    validation: None,
                    required: true,
                    depends_on: vec![],
                    quick_replies: vec![],
                    help_text: Some("Search for existing client or create new".to_string()),
                },
                WorkflowStep {
                    id: "items".to_string(),
                    name: "Add Items".to_string(),
                    prompt: "What products or services are you invoicing?".to_string(),
                    input_type: InputType::EntityRef("product".to_string()),
                    validation: None,
                    required: true,
                    depends_on: vec!["client".to_string()],
                    quick_replies: vec![],
                    help_text: Some("Add one or more items to the invoice".to_string()),
                },
                WorkflowStep {
                    id: "amount".to_string(),
                    name: "Total Amount".to_string(),
                    prompt: "What's the total amount?".to_string(),
                    input_type: InputType::Currency,
                    validation: Some(ValidationConfig {
                        min_value: Some(0.01),
                        max_value: None,
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        custom_validator: None,
                    }),
                    required: true,
                    depends_on: vec!["items".to_string()],
                    quick_replies: vec![
                        QuickReply {
                            label: "₱1,000".to_string(),
                            value: "1000".to_string(),
                            style: None,
                        },
                        QuickReply {
                            label: "₱5,000".to_string(),
                            value: "5000".to_string(),
                            style: None,
                        },
                        QuickReply {
                            label: "₱10,000".to_string(),
                            value: "10000".to_string(),
                            style: None,
                        },
                    ],
                    help_text: Some("Enter amount in Philippine Pesos".to_string()),
                },
                WorkflowStep {
                    id: "vat".to_string(),
                    name: "VAT Type".to_string(),
                    prompt: "Is this VAT inclusive or exclusive?".to_string(),
                    input_type: InputType::Select(vec![
                        SelectOption {
                            value: "inclusive".to_string(),
                            label: "VAT Inclusive".to_string(),
                            description: Some("Price includes 12% VAT".to_string()),
                        },
                        SelectOption {
                            value: "exclusive".to_string(),
                            label: "VAT Exclusive".to_string(),
                            description: Some("VAT will be added on top".to_string()),
                        },
                        SelectOption {
                            value: "exempt".to_string(),
                            label: "VAT Exempt".to_string(),
                            description: Some("No VAT applies".to_string()),
                        },
                    ]),
                    validation: None,
                    required: true,
                    depends_on: vec!["amount".to_string()],
                    quick_replies: vec![],
                    help_text: Some("Philippine VAT is 12%".to_string()),
                },
                WorkflowStep {
                    id: "due_date".to_string(),
                    name: "Due Date".to_string(),
                    prompt: "When is payment due?".to_string(),
                    input_type: InputType::Date,
                    validation: None,
                    required: false,
                    depends_on: vec!["vat".to_string()],
                    quick_replies: vec![
                        QuickReply {
                            label: "Due on receipt".to_string(),
                            value: "0".to_string(),
                            style: None,
                        },
                        QuickReply {
                            label: "Net 30".to_string(),
                            value: "30".to_string(),
                            style: None,
                        },
                        QuickReply {
                            label: "Net 60".to_string(),
                            value: "60".to_string(),
                            style: None,
                        },
                    ],
                    help_text: Some("Leave blank for due on receipt".to_string()),
                },
            ],
            on_complete: Some("create_invoice".to_string()),
        }
    }

    pub fn payment_workflow() -> Self {
        Self {
            id: "record_payment".to_string(),
            name: "Record Payment".to_string(),
            description: "Record a payment received".to_string(),
            steps: vec![
                WorkflowStep {
                    id: "invoice_or_client".to_string(),
                    name: "Payment For".to_string(),
                    prompt: "Is this payment for a specific invoice or from a client?".to_string(),
                    input_type: InputType::Select(vec![
                        SelectOption {
                            value: "invoice".to_string(),
                            label: "Specific Invoice".to_string(),
                            description: Some("Payment for an existing invoice".to_string()),
                        },
                        SelectOption {
                            value: "client".to_string(),
                            label: "Client Payment".to_string(),
                            description: Some("General payment from client".to_string()),
                        },
                    ]),
                    validation: None,
                    required: true,
                    depends_on: vec![],
                    quick_replies: vec![],
                    help_text: None,
                },
                WorkflowStep {
                    id: "payment_amount".to_string(),
                    name: "Amount".to_string(),
                    prompt: "How much was received?".to_string(),
                    input_type: InputType::Currency,
                    validation: Some(ValidationConfig {
                        min_value: Some(0.01),
                        max_value: None,
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        custom_validator: None,
                    }),
                    required: true,
                    depends_on: vec!["invoice_or_client".to_string()],
                    quick_replies: vec![],
                    help_text: Some("Enter amount in Philippine Pesos".to_string()),
                },
                WorkflowStep {
                    id: "payment_method".to_string(),
                    name: "Payment Method".to_string(),
                    prompt: "How was the payment made?".to_string(),
                    input_type: InputType::Select(vec![
                        SelectOption {
                            value: "cash".to_string(),
                            label: "Cash".to_string(),
                            description: None,
                        },
                        SelectOption {
                            value: "check".to_string(),
                            label: "Check".to_string(),
                            description: None,
                        },
                        SelectOption {
                            value: "bank_transfer".to_string(),
                            label: "Bank Transfer".to_string(),
                            description: None,
                        },
                        SelectOption {
                            value: "gcash".to_string(),
                            label: "GCash".to_string(),
                            description: None,
                        },
                        SelectOption {
                            value: "maya".to_string(),
                            label: "Maya".to_string(),
                            description: None,
                        },
                    ]),
                    validation: None,
                    required: true,
                    depends_on: vec!["payment_amount".to_string()],
                    quick_replies: vec![],
                    help_text: None,
                },
                WorkflowStep {
                    id: "ewt".to_string(),
                    name: "Withholding Tax".to_string(),
                    prompt: "Was there any withholding tax (EWT)?".to_string(),
                    input_type: InputType::Currency,
                    validation: Some(ValidationConfig {
                        min_value: Some(0.0),
                        max_value: None,
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        custom_validator: None,
                    }),
                    required: false,
                    depends_on: vec!["payment_method".to_string()],
                    quick_replies: vec![
                        QuickReply {
                            label: "No EWT".to_string(),
                            value: "0".to_string(),
                            style: None,
                        },
                        QuickReply {
                            label: "2% (Goods)".to_string(),
                            value: "2%".to_string(),
                            style: None,
                        },
                        QuickReply {
                            label: "5% (Services)".to_string(),
                            value: "5%".to_string(),
                            style: None,
                        },
                    ],
                    help_text: Some("Expanded Withholding Tax if applicable".to_string()),
                },
            ],
            on_complete: Some("record_payment".to_string()),
        }
    }
}