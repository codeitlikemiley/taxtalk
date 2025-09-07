use leptos::prelude::*;
use taxtalk_conditional_form::{
    ConditionalForm, FormField, FieldCondition, 
    ConditionOperator, ConditionAction, create_payment_form_fields
};
use taxtalk_ui_core::ComponentType;
use serde_json::Value;
use std::collections::HashMap;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Payment form example
    let (payment_fields_state, _) = signal(create_payment_form_fields());
    let payment_fields = Signal::derive(move || payment_fields_state.get());
    
    // Dynamic form example with conditions
    let dynamic_fields = vec![
        FormField {
            name: "user_type".into(),
            label: "User Type".into(),
            component: ComponentType::Select(vec![
                "individual".into(),
                "company".into(),
            ]),
            required: true,
            condition: None,
            default_value: Some(serde_json::json!("individual")),
        },
        FormField {
            name: "company_name".into(),
            label: "Company Name".into(),
            component: ComponentType::TextInput,
            required: false,
            condition: Some(FieldCondition {
                when: "user_type".into(),
                operator: ConditionOperator::Equals,
                value: serde_json::json!("company"),
                action: ConditionAction::Show,
            }),
            default_value: None,
        },
        FormField {
            name: "tin".into(),
            label: "TIN".into(),
            component: ComponentType::TextInput,
            required: false,
            condition: Some(FieldCondition {
                when: "user_type".into(),
                operator: ConditionOperator::Equals,
                value: serde_json::json!("company"),
                action: ConditionAction::Require,
            }),
            default_value: None,
        },
        FormField {
            name: "amount".into(),
            label: "Amount".into(),
            component: ComponentType::AmountInput,
            required: true,
            condition: None,
            default_value: Some(serde_json::json!(0.0)),
        },
        FormField {
            name: "senior_discount".into(),
            label: "Apply Senior Discount".into(),
            component: ComponentType::Boolean,
            required: false,
            condition: Some(FieldCondition {
                when: "amount".into(),
                operator: ConditionOperator::GreaterThan,
                value: serde_json::json!(0.0),
                action: ConditionAction::Show,
            }),
            default_value: Some(serde_json::json!(false)),
        },
    ];
    
    let (dynamic_fields_state, _) = signal(dynamic_fields);
    let dynamic_fields_signal = Signal::derive(move || dynamic_fields_state.get());
    
    let (submitted_data, set_submitted_data) = signal(None::<HashMap<String, Value>>);
    
    view! {
        <div class="container">
            <h1>"ConditionalForm Component Demo"</h1>
            
            <div class="example-section">
                <div class="example-title">"Payment Form Example"</div>
                <ConditionalForm
                    fields=payment_fields
                    on_submit=Callback::new(move |data: HashMap<String, Value>| {
                        set_submitted_data.set(Some(data.clone()));
                        leptos::logging::log!("Payment form submitted: {:?}", data);
                    })
                    inline=false
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Dynamic Conditional Form"</div>
                <ConditionalForm
                    fields=dynamic_fields_signal
                    on_submit=Callback::new(move |data: HashMap<String, Value>| {
                        set_submitted_data.set(Some(data.clone()));
                        leptos::logging::log!("Dynamic form submitted: {:?}", data);
                    })
                    inline=false
                />
            </div>
            
            <Show
                when=move || submitted_data.get().is_some()
                fallback=|| ()
            >
                <div class="example-section">
                    <div class="example-title">"Submitted Data"</div>
                    <pre style="background: #f0f0f0; padding: 10px; border-radius: 4px; overflow-x: auto;">
                        {move || {
                            serde_json::to_string_pretty(&submitted_data.get())
                                .unwrap_or_else(|_| "Error serializing data".to_string())
                        }}
                    </pre>
                </div>
            </Show>
        </div>
    }
}