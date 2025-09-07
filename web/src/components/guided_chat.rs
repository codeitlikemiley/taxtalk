use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::ev;
use leptos::wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use crate::config::api_url;
use crate::components::simple_form::{SimpleForm, ItemsForm};
use crate::components::autocomplete_simple::{AutocompletePopup, Entity, detect_mention, FilledToken};
use crate::components::entity_combobox::EntityCombobox;
use crate::components::multi_select_combobox::MultiSelectEntityCombobox;
use crate::components::quick_actions::QuickActionBar;
use crate::components::command_palette::CommandPalette;
use crate::components::entity_tag::{EntityTag, EntityToken};
use crate::components::inline_confirmation::{InlineConfirmation, ParsedField};
use crate::components::mixed_input::MixedInput;
use serde_json::json;
use web_sys::HtmlTextAreaElement;

#[derive(Clone, Debug)]
enum MessageStatus {
    Sent,
    Delivered,
    Error,
}

#[derive(Clone, Debug)]
struct ChatMessage {
    id: String,
    content: String,
    is_user: bool,
    timestamp: String,
    status: MessageStatus,
}

// Validation structures
#[derive(Clone, Debug, Serialize)]
struct ValidateCommand {
    command: String,
    session_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct ValidateResponse {
    valid: bool,
    action: Option<String>,
    entity: Option<String>,
    plugin: Option<String>,
    missing_required: Vec<MissingToken>,
    missing_optional: Vec<MissingToken>,
    provided_tokens: serde_json::Value,
    defaults_applied: serde_json::Value,
    session: Option<SessionInfo>,
}

#[derive(Clone, Debug, Deserialize)]
struct MissingToken {
    name: String,
    prompt: String,
    #[serde(rename = "type")]
    token_type: String,
    input_type: String,
}

#[derive(Clone, Debug, Deserialize)]
struct SessionInfo {
    id: String,
    status: String,
    progress: u32,
    total_required: usize,
    next_prompt: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
struct SubmitToken {
    token: String,
    value: serde_json::Value,
}

#[derive(Clone, Debug, Serialize)]
struct ExecuteCommand {
    command: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ExecuteResponse {
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<String>,
}

// Session state
#[derive(Clone, Debug)]
struct GuidedSession {
    id: String,
    command: String,
    progress: u32,
    total_required: usize,
    current_token: Option<String>,
    current_prompt: Option<String>,
    missing_required: Vec<MissingToken>,
    missing_optional: Vec<MissingToken>,
    collected_tokens: Vec<(String, String)>,
}

#[component]
pub fn GuidedChat() -> impl IntoView {
    let (messages, set_messages) = signal::<Vec<ChatMessage>>(vec![
        ChatMessage {
            id: "welcome".to_string(),
            content: "Hi! I'm TaxTalk, your AI bookkeeping assistant. I can help you with:\n\n• Creating invoices and recording payments\n• Managing VAT and tax calculations\n• Generating BIR-compliant reports\n• Tracking expenses and income\n\nJust type your command in natural language!".to_string(),
            is_user: false,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            status: MessageStatus::Delivered,
        }
    ]);
    
    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (active_session, set_active_session) = signal::<Option<GuidedSession>>(None);
    let (show_form, set_show_form) = signal(false);
    let (show_items_form, set_show_items_form) = signal(false);
    let (current_form_token, set_current_form_token) = signal::<Option<MissingToken>>(None);
    let (form_submit_value, set_form_submit_value) = signal::<Option<String>>(None);
    let (items_submit_value, set_items_submit_value) = signal::<Option<Vec<serde_json::Value>>>(None);
    let (show_entity_combobox, set_show_entity_combobox) = signal(false);
    let (entity_combobox_type, set_entity_combobox_type) = signal(String::new());
    let (entity_combobox_prompt, set_entity_combobox_prompt) = signal(String::new());
    let (entity_combobox_token, set_entity_combobox_token) = signal(String::new());
    let (entity_combobox_initial_query, set_entity_combobox_initial_query) = signal(String::new());
    let (entity_selected, set_entity_selected) = signal::<Option<Entity>>(None);
    let (entity_combobox_cancelled, set_entity_combobox_cancelled) = signal(false);
    let (entity_combobox_cardinality, set_entity_combobox_cardinality) = signal(String::from("single"));
    let (entity_combobox_max_selections, set_entity_combobox_max_selections) = signal::<Option<usize>>(None);
    let (multi_entity_selected, set_multi_entity_selected) = signal::<Vec<Entity>>(vec![]);
    
    // Autocomplete state
    let (show_autocomplete, set_show_autocomplete) = signal(false);
    let (autocomplete_query, set_autocomplete_query) = signal(String::new());
    let (autocomplete_type, set_autocomplete_type) = signal::<Option<String>>(None);
    let (autocomplete_position, set_autocomplete_position) = signal((0.0, 0.0));
    let (cursor_position, set_cursor_position) = signal(0);
    let (selected_entity, set_selected_entity) = signal::<Option<Entity>>(None);
    let (autocomplete_selected, set_autocomplete_selected) = signal::<Option<Entity>>(None);
    let input_ref = NodeRef::<leptos::html::Textarea>::new();
    
    // Command palette and smart features
    let (show_command_palette, set_show_command_palette) = signal(false);
    let (show_inline_confirmation, set_show_inline_confirmation) = signal(false);
    let (parsed_fields, _set_parsed_fields) = signal::<Vec<ParsedField>>(vec![]);
    // Entity tokens in the input
    let (entity_tokens, set_entity_tokens) = signal::<Vec<EntityToken>>(vec![]);
    let (removed_token_id, set_removed_token_id) = signal::<Option<String>>(None);
    let (autocomplete_locked, set_autocomplete_locked) = signal(false);
    let (_use_quick_actions, _set_use_quick_actions) = signal(false);
    let (_mini_session_mode, _set_mini_session_mode) = signal(false);
    let (_quick_action_selected, _set_quick_action_selected) = signal::<Option<String>>(None);
    let (_show_custom_input, _set_show_custom_input) = signal(false);
    let (on_palette_execute, set_on_palette_execute) = signal::<Option<String>>(None);
    let (_on_confirmation_confirm, set_on_confirmation_confirm) = signal(false);
    let (_on_confirmation_edit, set_on_confirmation_edit) = signal::<Option<String>>(None);
    let (_on_confirmation_cancel, set_on_confirmation_cancel) = signal(false);
    let (_smart_command_submit, _set_smart_command_submit) = signal::<Option<String>>(None);
    let (token_selected, set_token_selected) = signal::<Option<String>>(None);
    
    let send_message = move |_| {
        let content = input_value.get();
        if content.trim().is_empty() {
            return;
        }
        
        // Add user message
        let user_message = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.clone(),
            is_user: true,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            status: MessageStatus::Sent,
        };
        
        set_messages.update(|msgs| msgs.push(user_message));
        set_input_value.set(String::new());
        set_is_loading.set(true);
        
        // Check if we have an active session
        let session = active_session.get();
        
        spawn_local(async move {
            if let Some(sess) = session {
                // Submit token to existing session
                if let Some(current_token) = sess.current_token.clone() {
                    // Check if this is an entity token and user typed an entity name or ID
                    let is_entity_token = matches!(current_token.as_str(), 
                        "client" | "supplier" | "customer" | "product" | "employee");
                    
                    if is_entity_token {
                        // Check if user typed @entity_type pattern - if so, show combobox
                        if content.starts_with("@") {
                            // User typed @client or similar - show combobox for that type
                            set_entity_combobox_type.set(current_token.clone());
                            set_entity_combobox_prompt.set(format!("Select a {}", current_token));
                            set_entity_combobox_token.set(current_token);
                            set_show_entity_combobox.set(true);
                            set_is_loading.set(false);
                        } else {
                            // Try to find the entity by name
                            match search_entity_by_name(&content, &current_token).await {
                                Ok(Some(entity)) => {
                                    // Found entity, submit its ID
                                    submit_session_token(&sess.id, &current_token, entity.id).await;
                                    continue_session(sess.id.clone(), set_messages, set_active_session, set_is_loading, 
                                        set_show_entity_combobox, set_entity_combobox_type, 
                                        set_entity_combobox_prompt, set_entity_combobox_token, sess.command.clone()).await;
                                },
                                _ => {
                                    // Entity not found or ambiguous - show combobox with the query
                                    set_entity_combobox_type.set(current_token.clone());
                                    set_entity_combobox_prompt.set(format!("Please select a {} for '{}'", current_token, content));
                                    set_entity_combobox_token.set(current_token);
                                    set_entity_combobox_initial_query.set(content.clone());
                                    set_show_entity_combobox.set(true);
                                    set_is_loading.set(false);
                                }
                            }
                        }
                    } else {
                        // Regular token, submit as-is
                        submit_session_token(&sess.id, &current_token, content.clone()).await;
                        continue_session(sess.id.clone(), set_messages, set_active_session, set_is_loading,
                            set_show_entity_combobox, set_entity_combobox_type, 
                            set_entity_combobox_prompt, set_entity_combobox_token, sess.command.clone()).await;
                    }
                }
            } else {
                // Validate new command
                let validation = validate_command(content.clone(), None).await;
                
                match validation {
                    Ok(resp) => {
                        if resp.valid {
                            // Command is complete, execute it
                            execute_validated_command(content, set_messages, set_is_loading).await;
                        } else {
                            // Start guided session
                            if let Some(session_info) = resp.session {
                                let guided_session = GuidedSession {
                                    id: session_info.id,
                                    command: content,
                                    progress: session_info.progress,
                                    total_required: session_info.total_required,
                                    current_token: resp.missing_required.first().map(|t| t.name.clone()),
                                    current_prompt: resp.missing_required.first().map(|t| t.prompt.clone()),
                                    missing_required: resp.missing_required.clone(),
                                    missing_optional: resp.missing_optional,
                                    collected_tokens: vec![],
                                };
                                
                                // Check if first token needs a form
                                if let Some(first_token) = resp.missing_required.first() {
                                    // Don't show entity combobox immediately - show prompt first
                                    if first_token.input_type == "Form" || first_token.name == "items" {
                                        // Special handling for invoice items
                                        if first_token.name == "items" {
                                            set_show_items_form.set(true);
                                        } else {
                                            set_current_form_token.set(Some(first_token.clone()));
                                            set_show_form.set(true);
                                        }
                                    } else {
                                        // Show chat prompt
                                        let bot_message = ChatMessage {
                                            id: uuid::Uuid::new_v4().to_string(),
                                            content: format!("📝 {}\n\n{}", 
                                                format_session_progress(&guided_session),
                                                first_token.prompt
                                            ),
                                            is_user: false,
                                            timestamp: chrono::Local::now().format("%H:%M").to_string(),
                                            status: MessageStatus::Delivered,
                                        };
                                        set_messages.update(|msgs| msgs.push(bot_message));
                                    }
                                }
                                
                                set_active_session.set(Some(guided_session));
                                set_is_loading.set(false);
                            }
                        }
                    },
                    Err(e) => {
                        let error_msg = ChatMessage {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: format!("⚠️ Error: {}", e),
                            is_user: false,
                            timestamp: chrono::Local::now().format("%H:%M").to_string(),
                            status: MessageStatus::Error,
                        };
                        set_messages.update(|msgs| msgs.push(error_msg));
                        set_is_loading.set(false);
                    }
                }
            }
        });
    };
    
    let handle_input = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        set_input_value.set(value.clone());
        
        // Always check for @ mentions for better UX
        if let Ok(target) = ev.target().unwrap().dyn_into::<HtmlTextAreaElement>() {
            let cursor_pos = target.selection_start().ok().flatten().unwrap_or(0) as usize;
            set_cursor_position.set(cursor_pos);
            
            if let Some((query, entity_type)) = detect_mention(&value, cursor_pos) {
                set_autocomplete_query.set(query);
                if entity_type == "entity_type_hint" {
                    // Show entity type suggestions
                    set_autocomplete_type.set(Some("entity_type_hint".to_string()));
                } else {
                    // Show entities of specific type
                    set_autocomplete_type.set(Some(entity_type));
                }
                set_show_autocomplete.set(true);
                
                // Position popup above the input area
                // Get textarea element to calculate position
                if let Some(element) = input_ref.get() {
                    // Position above the textarea
                    set_autocomplete_position.set((100.0, 280.0));
                } else {
                    set_autocomplete_position.set((100.0, 280.0));
                }
                
                // Lock input when @ is typed
                set_autocomplete_locked.set(true);
            } else {
                set_show_autocomplete.set(false);
                set_autocomplete_locked.set(false);
            }
        }
    };
    
    // Handle autocomplete selection
    Effect::new(move |_| {
        if let Some(entity) = autocomplete_selected.get() {
            let input_val = input_value.get();
            let cursor_pos = cursor_position.get();
            
            // Check if this is an entity type selection
            if entity.entity_type == "_entity_type" {
                // User selected an entity type, update the input to continue typing
                if let Some(at_pos) = input_val[..cursor_pos].rfind('@') {
                    let before_at = &input_val[..at_pos];
                    let after_cursor = if cursor_pos < input_val.len() {
                        &input_val[cursor_pos..]
                    } else {
                        ""
                    };
                    
                    // Get entity type name and cardinality
                    let entity_type_name = entity.name.trim_start_matches("_type_");
                    let cardinality = entity.metadata.as_ref()
                        .and_then(|m| m.get("cardinality"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("single");
                    
                    // Check if this token type is already filled (for single cardinality)
                    let already_filled = if cardinality == "single" {
                        entity_tokens.get().iter().any(|t| t.entity_type == entity_type_name)
                    } else {
                        false
                    };
                    
                    if already_filled {
                        // Show message that this token is already filled
                        let msg = ChatMessage {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: format!("⚠️ A {} has already been selected. You can only select one.", entity_type_name),
                            is_user: false,
                            timestamp: chrono::Local::now().format("%H:%M").to_string(),
                            status: MessageStatus::Delivered,
                        };
                        set_messages.update(|msgs| msgs.push(msg));
                        
                        // Clear input and reset
                        set_input_value.set(before_at.to_string() + after_cursor);
                        set_show_autocomplete.set(false);
                        set_autocomplete_locked.set(false);
                    } else {
                        // Show entity combobox for this type
                        set_show_entity_combobox.set(true);
                        set_entity_combobox_type.set(entity_type_name.to_string());
                        set_entity_combobox_prompt.set(format!("Select {}", entity_type_name));
                        set_entity_combobox_token.set(entity_type_name.to_string());
                        set_entity_combobox_initial_query.set("".to_string());
                        set_entity_combobox_cardinality.set(cardinality.to_string());
                        
                        // For products, we might want to set a max (example: 10)
                        if entity_type_name == "product" && cardinality == "multiple" {
                            set_entity_combobox_max_selections.set(Some(10));
                        } else {
                            set_entity_combobox_max_selections.set(None);
                        }
                        
                        // Remove the @ mention from input since we're showing modal
                        set_input_value.set(before_at.to_string() + after_cursor);
                        
                        // Hide autocomplete and unlock
                        set_show_autocomplete.set(false);
                        set_autocomplete_locked.set(false);
                    }
                }
            } else {
                // Actual entity selected - add to tokens
                let token = EntityToken {
                    id: entity.id.clone(),
                    entity_type: entity.entity_type.clone(),
                    display_name: entity.name.clone(),
                    metadata: entity.metadata.clone(),
                };
                
                set_entity_tokens.update(|tokens| tokens.push(token));
                
                // Clear the @ mention from input
                if let Some(at_pos) = input_val[..cursor_pos].rfind('@') {
                    let before_at = &input_val[..at_pos];
                    let after_cursor = if cursor_pos < input_val.len() {
                        &input_val[cursor_pos..]
                    } else {
                        ""
                    };
                    set_input_value.set(before_at.to_string() + after_cursor);
                }
                
                set_show_autocomplete.set(false);
                set_autocomplete_locked.set(false);
            }
            
            set_autocomplete_selected.set(None);
        }
    });
    
    let handle_keypress = move |ev: ev::KeyboardEvent| {
        // If autocomplete is locked, prevent typing except for navigation
        if autocomplete_locked.get_untracked() {
            if !matches!(ev.key().as_str(), "ArrowDown" | "ArrowUp" | "Enter" | "Tab" | "Escape") {
                ev.prevent_default();
            }
            return;
        }
        
        // Don't send message if autocomplete is open and arrow keys are pressed
        if show_autocomplete.get_untracked() && (ev.key() == "ArrowDown" || ev.key() == "ArrowUp" || ev.key() == "Escape") {
            return;
        }
        
        if ev.key() == "Enter" && !ev.shift_key() && !show_autocomplete.get_untracked() {
            ev.prevent_default();
            send_message(ev::MouseEvent::new("click").unwrap());
        } else if ev.ctrl_key() && ev.key() == "k" {
            ev.prevent_default();
            set_show_command_palette.set(true);
        } else if ev.key() == "Escape" {
            // Close any open modals
            if show_command_palette.get() {
                set_show_command_palette.set(false);
            } else if show_inline_confirmation.get_untracked() {
                set_show_inline_confirmation.set(false);
            } else if show_autocomplete.get_untracked() {
                set_show_autocomplete.set(false);
                set_autocomplete_locked.set(false);
            }
        }
    };
    
    let cancel_session = move |_| {
        set_active_session.set(None);
        set_show_form.set(false);
        
        let cancel_msg = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content: "❌ Session cancelled. You can start a new command anytime.".to_string(),
            is_user: false,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            status: MessageStatus::Delivered,
        };
        set_messages.update(|msgs| msgs.push(cancel_msg));
    };
    
    // Watch for form submission
    Effect::new(move |_| {
        if let Some(value) = form_submit_value.get() {
            if let Some(session) = active_session.get() {
                if let Some(token) = current_form_token.get() {
                    spawn_local({
                        let session_id = session.id.clone();
                        let token_name = token.name.clone();
                        async move {
                            submit_session_token(&session_id, &token_name, value).await;
                            continue_session(session_id.clone(), set_messages, set_active_session, set_is_loading,
                                set_show_entity_combobox, set_entity_combobox_type, 
                                set_entity_combobox_prompt, set_entity_combobox_token, session.command.clone()).await;
                        }
                    });
                    
                    set_show_form.set(false);
                    set_current_form_token.set(None);
                    set_form_submit_value.set(None);
                }
            }
        }
    });
    
    Effect::new(move |_| {
        if let Some(items) = items_submit_value.get() {
            if let Some(session) = active_session.get() {
                let items_json = json!(items);
                
                spawn_local({
                    let session_id = session.id.clone();
                    async move {
                        submit_session_token(&session_id, "items", items_json.to_string()).await;
                        continue_session(session_id.clone(), set_messages, set_active_session, set_is_loading,
                            set_show_entity_combobox, set_entity_combobox_type, 
                            set_entity_combobox_prompt, set_entity_combobox_token, session.command.clone()).await;
                    }
                });
                
                set_show_items_form.set(false);
                set_items_submit_value.set(None);
            }
        }
    });
    
    // Watch for entity combobox selection
    Effect::new(move |_| {
        if let Some(entity) = entity_selected.get() {
            // Create entity token
            let token = EntityToken {
                id: entity.id.clone(),
                entity_type: entity.entity_type.clone(),
                display_name: entity.name.clone(),
                metadata: entity.metadata.clone(),
            };
            
            // Add to tokens list
            set_entity_tokens.update(|tokens| tokens.push(token));
            
            // If in session, submit it
            if let Some(session) = active_session.get() {
                let token_name = entity_combobox_token.get();
                
                // Show selected entity in chat
                let msg = ChatMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    content: format!("Selected: {}", entity.name),
                    is_user: true,
                    timestamp: chrono::Local::now().format("%H:%M").to_string(),
                    status: MessageStatus::Sent,
                };
                set_messages.update(|msgs| msgs.push(msg));
                
                spawn_local({
                    let session_id = session.id.clone();
                    let entity_id = entity.id.clone();
                    let command = session.command.clone();
                    async move {
                        submit_session_token(&session_id, &token_name, entity_id).await;
                        continue_session(session_id.clone(), set_messages, set_active_session, set_is_loading,
                            set_show_entity_combobox, set_entity_combobox_type, 
                            set_entity_combobox_prompt, set_entity_combobox_token, command).await;
                    }
                });
            }
            
            set_show_entity_combobox.set(false);
            set_entity_selected.set(None);
        }
    });
    
    // Watch for multi-entity selection (for multiple cardinality)
    Effect::new(move |_| {
        let selected = multi_entity_selected.get();
        if !selected.is_empty() {
            // Add all selected entities to tokens
            for entity in selected {
                let token = EntityToken {
                    id: entity.id.clone(),
                    entity_type: entity.entity_type.clone(),
                    display_name: entity.name.clone(),
                    metadata: entity.metadata.clone(),
                };
                set_entity_tokens.update(|tokens| tokens.push(token));
            }
            
            // If in session, submit them
            if let Some(session) = active_session.get() {
                let token_name = entity_combobox_token.get();
                let entity_ids = multi_entity_selected.get().iter()
                    .map(|e| e.id.clone())
                    .collect::<Vec<String>>()
                    .join(",");
                
                // Show selected entities in chat
                let names = multi_entity_selected.get().iter()
                    .map(|e| e.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");
                let msg = ChatMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    content: format!("Selected: {}", names),
                    is_user: true,
                    timestamp: chrono::Local::now().format("%H:%M").to_string(),
                    status: MessageStatus::Sent,
                };
                set_messages.update(|msgs| msgs.push(msg));
                
                spawn_local({
                    let session_id = session.id.clone();
                    let command = session.command.clone();
                    async move {
                        submit_session_token(&session_id, &token_name, entity_ids).await;
                        continue_session(session_id.clone(), set_messages, set_active_session, set_is_loading,
                            set_show_entity_combobox, set_entity_combobox_type, 
                            set_entity_combobox_prompt, set_entity_combobox_token, command).await;
                    }
                });
            }
            
            set_show_entity_combobox.set(false);
            set_multi_entity_selected.set(vec![]);
        }
    });
    
    // Watch for entity combobox cancellation
    Effect::new(move |_| {
        if entity_combobox_cancelled.get() {
            set_show_entity_combobox.set(false);
            set_entity_combobox_type.set(String::new());
            set_entity_combobox_prompt.set(String::new());
            set_entity_combobox_token.set(String::new());
            set_entity_combobox_initial_query.set(String::new());
            set_entity_combobox_cardinality.set(String::from("single"));
            set_entity_combobox_max_selections.set(None);
            set_is_loading.set(false);
            set_entity_combobox_cancelled.set(false);
        }
    });
    
    // Global keyboard shortcuts
    Effect::new(move |_| {
        let window = web_sys::window().expect("window should exist");
        let document = window.document().expect("document should exist");
        
        let closure = wasm_bindgen::closure::Closure::<dyn FnMut(_)>::new(move |ev: web_sys::KeyboardEvent| {
            // Escape key cancels active session
            if ev.key() == "Escape" {
                if active_session.get().is_some() {
                    ev.prevent_default();
                    // Cancel the active session
                    set_active_session.set(None);
                    set_show_form.set(false);
                    set_show_items_form.set(false);
                    set_show_entity_combobox.set(false);
                    set_entity_combobox_type.set(String::new());
                    set_entity_combobox_prompt.set(String::new());
                    set_entity_combobox_token.set(String::new());
                    set_entity_combobox_initial_query.set(String::new());
                    set_is_loading.set(false);
                    
                    let cancel_msg = ChatMessage {
                        id: uuid::Uuid::new_v4().to_string(),
                        content: "❌ Session cancelled".to_string(),
                        is_user: false,
                        timestamp: chrono::Local::now().format("%H:%M").to_string(),
                        status: MessageStatus::Delivered,
                    };
                    set_messages.update(|msgs| msgs.push(cancel_msg));
                }
            }
            // Ctrl+K opens command palette
            else if ev.ctrl_key() && ev.key() == "k" {
                ev.prevent_default();
                set_show_command_palette.set(true);
            }
        });
        
        document
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("should add event listener");
        
        // Keep the closure alive
        closure.forget();
    });
    
    // Watch for command palette execution
    Effect::new(move |_| {
        if let Some(command) = on_palette_execute.get() {
            // Parse command and execute
            if command.starts_with("nlp:") {
                // Handle NLP parsed command
                let json_str = command.strip_prefix("nlp:").unwrap_or("");
                if let Ok(_parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
                    // Process the NLP command
                    set_input_value.set(format!("Processing command: {}", json_str));
                    send_message(ev::MouseEvent::new("click").unwrap());
                }
            } else {
                // Regular command execution
                let parts: Vec<&str> = command.split(':').collect();
                if parts.len() == 2 {
                    let plugin = parts[0];
                    let action = parts[1];
                    set_input_value.set(format!("{}:{}", plugin, action));
                    send_message(ev::MouseEvent::new("click").unwrap());
                }
            }
            set_on_palette_execute.set(None);
        }
    });
    
    // Watch for token removal
    Effect::new(move |_| {
        if let Some(token_id) = removed_token_id.get() {
            // Remove token from list
            set_entity_tokens.update(|tokens| {
                tokens.retain(|t| t.id != token_id);
            });
            set_removed_token_id.set(None);
        }
    });
    
    
    view! {
        <div class="flex flex-col h-full relative">
            // Session Progress Bar
            {move || if let Some(session) = active_session.get() {
                view! {
                    <div class="bg-gradient-to-r from-blue-50 to-purple-50 border-b border-blue-200 px-6 py-3">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-3">
                                <div class="w-8 h-8 bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold">
                                    {session.progress} "/" {session.total_required}
                                </div>
                                <div>
                                    <div class="text-sm font-medium text-gray-700">"Creating: " {session.command}</div>
                                    <div class="text-xs text-gray-500">"Step " {session.progress + 1} " of " {session.total_required}</div>
                                </div>
                            </div>
                            <button
                                class="px-3 py-1 text-sm text-red-600 hover:text-red-700 hover:bg-red-50 rounded-lg transition-colors"
                                on:click=cancel_session
                            >
                                "Cancel"
                            </button>
                        </div>
                        <div class="mt-2">
                            <div class="h-2 bg-gray-200 rounded-full overflow-hidden">
                                <div 
                                    class="h-full bg-gradient-to-r from-blue-500 to-purple-500 transition-all duration-300"
                                    style={format!("width: {}%", session.progress as f32 / session.total_required as f32 * 100.0)}
                                ></div>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            // Header
            <div class="bg-white border-b border-gray-200 px-6 py-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                        <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                        <h2 class="text-lg font-semibold text-gray-800">"Guided Chat Assistant"</h2>
                    </div>
                    <div class="flex items-center space-x-2">
                        <span class="text-sm text-gray-500">"AI-Powered Token Validation"</span>
                    </div>
                </div>
            </div>
            
            // Messages area
            <div class="flex-1 overflow-y-auto px-6 py-4 space-y-4 bg-gradient-to-b from-gray-50 to-white">
                <For
                    each=move || messages.get()
                    key=|msg| msg.id.clone()
                    children=move |msg| {
                        view! {
                            <div class={if msg.is_user { "flex justify-end" } else { "flex justify-start" }}>
                                <div class={if msg.is_user { "max-w-2xl" } else { "max-w-3xl" }}>
                                    <div class={if msg.is_user { 
                                        "bg-gradient-to-r from-blue-500 to-blue-600 text-white rounded-2xl rounded-br-sm px-5 py-3 shadow-lg" 
                                    } else { 
                                        "bg-white border border-gray-200 rounded-2xl rounded-bl-sm px-5 py-3 shadow-md" 
                                    }}>
                                        <div class="whitespace-pre-wrap text-sm leading-relaxed">{msg.content}</div>
                                    </div>
                                    <div class="flex items-center space-x-2 mt-1 px-2">
                                        <span class="text-xs text-gray-400">{msg.timestamp}</span>
                                        {move || match msg.status {
                                            MessageStatus::Sent => view! {
                                                <svg class="w-3 h-3 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                                    <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"/>
                                                </svg>
                                            }.into_any(),
                                            MessageStatus::Delivered => view! {
                                                <svg class="w-3 h-3 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                                                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any(),
                                            MessageStatus::Error => view! {
                                                <svg class="w-3 h-3 text-red-500" fill="currentColor" viewBox="0 0 20 20">
                                                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any(),
                                        }}
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
                
                {move || if is_loading.get() {
                    view! {
                        <div class="flex justify-start">
                            <div class="bg-white border border-gray-200 rounded-2xl rounded-bl-sm px-5 py-3 shadow-md">
                                <div class="flex space-x-2">
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
            
            // Input area
            <div class="bg-white border-t border-gray-200 px-6 py-4">
                
                // Quick action buttons for session
                {move || if let Some(session) = active_session.get() {
                    if let Some(token) = &session.current_token {
                        if let Some(prompt) = &session.current_prompt {
                            // Get token type from missing tokens  
                            let token_type = "String".to_string(); // Simplified for now
                            
                            // Create signals for QuickActionBar
                            let (quick_action_value, set_quick_action_value) = signal(None::<String>);
                            let (_quick_custom, set_quick_custom) = signal(false);
                            
                            // Watch for changes from QuickActionBar
                            Effect::new(move |_| {
                                if let Some(v) = quick_action_value.get() {
                                    set_input_value.set(v);
                                }
                            });
                            
                            view! {
                                <QuickActionBar
                                    token=token.clone()
                                    token_type=token_type
                                    prompt=prompt.clone()
                                    on_action=set_quick_action_value
                                    on_custom=set_quick_custom
                                />
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    } else {
                        view! { <div></div> }.into_any()
                    }
                } else {
                    view! { <div></div> }.into_any()
                }}
                
                <div class="flex items-end space-x-3">
                    <div class="flex-1 relative">
                        // Entity tags display
                        {move || if !entity_tokens.get().is_empty() {
                            view! {
                                <div class="flex flex-wrap gap-2 mb-2">
                                    <For
                                        each=move || entity_tokens.get()
                                        key=|token| token.id.clone()
                                        children=move |token| {
                                            view! {
                                                <EntityTag
                                                    token=token
                                                    on_remove=set_removed_token_id
                                                />
                                            }
                                        }
                                    />
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }}
                        <textarea
                            node_ref=input_ref
                            class="w-full px-4 py-3 pr-12 border border-gray-300 rounded-xl resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                            placeholder={move || if active_session.get().is_some() {
                                "Enter your answer..."
                            } else {
                                "Type your command here... Use @ to mention entities"
                            }}
                            rows="1"
                            prop:value=move || input_value.get()
                            on:input=handle_input
                            on:keydown=handle_keypress
                            disabled=move || is_loading.get() || autocomplete_locked.get()
                        />
                    </div>
                    <button
                        class="px-6 py-3 bg-gradient-to-r from-blue-500 to-blue-600 text-white rounded-xl hover:from-blue-600 hover:to-blue-700 disabled:from-gray-400 disabled:to-gray-400 transition-all shadow-lg hover:shadow-xl transform hover:-translate-y-0.5"
                        on:click=send_message
                        disabled=move || is_loading.get()
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                        </svg>
                    </button>
                </div>
                <div class="mt-2 flex items-center space-x-4 text-xs text-gray-500">
                    <span>"Press Enter to send"</span>
                    <span>"•"</span>
                    <span>"Ctrl+K for commands"</span>
                    <span>"•"</span>
                    <span>"@ for tokens"</span>
                    <span>"•"</span>
                    {move || if active_session.get().is_some() {
                        view! { <span>"Session active"</span> }.into_any()
                    } else {
                        view! { <span>"Type naturally"</span> }.into_any()
                    }}
                </div>
            </div>
            
            // Form Modal for complex tokens
            {move || {
                if let Some(token) = current_form_token.get() {
                    view! {
                        <SimpleForm
                            show=show_form.into()
                            title=format!("Provide {}", token.name)
                            token_name=token.name.clone()
                            token_type=token.token_type.clone()
                            prompt=token.prompt.clone()
                            on_submit=set_form_submit_value
                            on_cancel=set_show_form
                        />
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
            
            // Invoice Items Form
            <ItemsForm
                show=show_items_form.into()
                on_submit=set_items_submit_value
                on_cancel=set_show_items_form
            />
            
            // Entity Combobox for guided session (single or multi based on cardinality)
            {move || {
                if show_entity_combobox.get() && !entity_combobox_type.get().is_empty() {
                    let cardinality = entity_combobox_cardinality.get();
                    if cardinality == "multiple" {
                        view! {
                            <MultiSelectEntityCombobox
                                entity_type=entity_combobox_type.get()
                                prompt=entity_combobox_prompt.get()
                                max_selections=entity_combobox_max_selections.get()
                                initial_selections=vec![]
                                on_confirm=set_multi_entity_selected
                                on_cancel=set_entity_combobox_cancelled
                            />
                        }.into_any()
                    } else {
                        view! {
                            <EntityCombobox
                                entity_type=entity_combobox_type.get()
                                prompt=entity_combobox_prompt.get()
                                initial_query=entity_combobox_initial_query.get()
                                on_select=set_entity_selected
                                on_cancel=set_entity_combobox_cancelled
                            />
                        }.into_any()
                    }
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
            
            // Autocomplete Popup
            <AutocompletePopup
                show=show_autocomplete.into()
                query=autocomplete_query.into()
                entity_type=autocomplete_type.into()
                position=autocomplete_position.into()
                on_select=set_autocomplete_selected
                on_cancel=set_show_autocomplete
                command_context=input_value.into()
                cursor_pos=cursor_position.into()
                filled_tokens={Signal::derive(move || {
                    entity_tokens.get().iter().map(|t| FilledToken {
                        token_type: t.entity_type.clone(),
                        entity_id: t.id.clone(),
                        entity_type: t.entity_type.clone(),
                    }).collect::<Vec<_>>()
                })}
            />
            
            // Command Palette (Ctrl+K)
            <CommandPalette
                show=show_command_palette.into()
                on_close=set_show_command_palette
                on_execute=set_on_palette_execute
            />
            
            // Inline Confirmation for parsed commands
            {move || if show_inline_confirmation.get() && !parsed_fields.get().is_empty() {
                view! {
                    <InlineConfirmation
                        title="Confirm Command".to_string()
                        fields=parsed_fields.get()
                        message=Some("Review and confirm the parsed command".to_string())
                        on_confirm=set_on_confirmation_confirm
                        on_edit=set_on_confirmation_edit
                        on_cancel=set_on_confirmation_cancel
                    />
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}

async fn validate_command(command: String, session_id: Option<String>) -> Result<ValidateResponse, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(api_url("api/validate"))
        .json(&ValidateCommand { command, session_id })
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status().is_success() {
        response
            .json::<ValidateResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

async fn submit_session_token(session_id: &str, token: &str, value: String) {
    log::info!("Submitting token '{}' with value '{}' to session {}", token, value, session_id);
    let client = reqwest::Client::new();
    
    let _ = client
        .post(api_url(&format!("api/session/{}/token", session_id)))
        .json(&SubmitToken {
            token: token.to_string(),
            value: serde_json::json!(value),
        })
        .send()
        .await;
}

async fn search_entity_by_name(name: &str, entity_type: &str) -> Result<Option<Entity>, String> {
    let client = reqwest::Client::new();
    
    let url = api_url(&format!("api/search?q={}&type={}", name, entity_type));
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status().is_success() {
        #[derive(Deserialize)]
        struct SearchResponse {
            results: Vec<Entity>,
        }
        
        let data: SearchResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        // Return entity if exactly one match
        if data.results.len() == 1 {
            Ok(Some(data.results.into_iter().next().unwrap()))
        } else {
            Ok(None) // Ambiguous or no results
        }
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

async fn continue_session(
    session_id: String,
    set_messages: WriteSignal<Vec<ChatMessage>>,
    set_active_session: WriteSignal<Option<GuidedSession>>,
    set_is_loading: WriteSignal<bool>,
    _set_show_entity_combobox: WriteSignal<bool>,
    _set_entity_combobox_type: WriteSignal<String>,
    _set_entity_combobox_prompt: WriteSignal<String>,
    _set_entity_combobox_token: WriteSignal<String>,
    original_command: String,
) {
    // Re-validate with session_id to get updated missing tokens
    let client = reqwest::Client::new();
    
    match client
        .post(api_url("api/validate"))
        .json(&ValidateCommand { 
            command: String::new(), // Empty command with session_id to continue
            session_id: Some(session_id.clone()),
        })
        .send()
        .await
    {
        Ok(response) => {
            if let Ok(validation) = response.json::<ValidateResponse>().await {
                if !validation.missing_required.is_empty() {
                    // Continue with next token
                    let next_token = &validation.missing_required[0];
                    
                    // Update session state
                    if let Some(session_info) = validation.session {
                        log::info!("Session progress update: {}/{}", session_info.progress, session_info.total_required);
                        let guided_session = GuidedSession {
                            id: session_id,
                            command: original_command.clone(),
                            progress: session_info.progress,
                            total_required: session_info.total_required,
                            current_token: Some(next_token.name.clone()),
                            current_prompt: Some(next_token.prompt.clone()),
                            missing_required: validation.missing_required.clone(),
                            missing_optional: validation.missing_optional,
                            collected_tokens: vec![], // Would be nice to preserve these
                        };
                        
                        // Show prompt for next token
                        let bot_message = ChatMessage {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: format!("📝 Progress: {} of {}\n\n{}", 
                                session_info.progress + 1,
                                session_info.total_required,
                                next_token.prompt
                            ),
                            is_user: false,
                            timestamp: chrono::Local::now().format("%H:%M").to_string(),
                            status: MessageStatus::Delivered,
                        };
                        set_messages.update(|msgs| msgs.push(bot_message));
                        
                        set_active_session.set(Some(guided_session));
                    }
                } else {
                    // All tokens collected, execute
                    execute_session(session_id, set_messages, set_active_session, set_is_loading).await;
                }
                
                set_is_loading.set(false);
            } else {
                set_is_loading.set(false);
            }
        },
        Err(e) => {
            let error_msg = ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                content: format!("⚠️ Error continuing session: {}", e),
                is_user: false,
                timestamp: chrono::Local::now().format("%H:%M").to_string(),
                status: MessageStatus::Error,
            };
            set_messages.update(|msgs| msgs.push(error_msg));
            set_is_loading.set(false);
        }
    }
}

async fn execute_session(
    session_id: String,
    set_messages: WriteSignal<Vec<ChatMessage>>,
    set_active_session: WriteSignal<Option<GuidedSession>>,
    set_is_loading: WriteSignal<bool>,
) {
    let client = reqwest::Client::new();
    
    match client
        .post(api_url(&format!("api/session/{}/execute", session_id)))
        .send()
        .await
    {
        Ok(response) => {
            if let Ok(result) = response.json::<ExecuteResponse>().await {
                let msg = if result.success {
                    ChatMessage {
                        id: uuid::Uuid::new_v4().to_string(),
                        content: "✅ Transaction completed successfully!".to_string(),
                        is_user: false,
                        timestamp: chrono::Local::now().format("%H:%M").to_string(),
                        status: MessageStatus::Delivered,
                    }
                } else {
                    ChatMessage {
                        id: uuid::Uuid::new_v4().to_string(),
                        content: format!("❌ Error: {}", result.error.unwrap_or_else(|| "Unknown error".to_string())),
                        is_user: false,
                        timestamp: chrono::Local::now().format("%H:%M").to_string(),
                        status: MessageStatus::Error,
                    }
                };
                set_messages.update(|msgs| msgs.push(msg));
            }
        },
        Err(e) => {
            let error_msg = ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                content: format!("⚠️ Error executing session: {}", e),
                is_user: false,
                timestamp: chrono::Local::now().format("%H:%M").to_string(),
                status: MessageStatus::Error,
            };
            set_messages.update(|msgs| msgs.push(error_msg));
        }
    }
    
    set_active_session.set(None);
    set_is_loading.set(false);
}

async fn execute_validated_command(
    command: String,
    set_messages: WriteSignal<Vec<ChatMessage>>,
    set_is_loading: WriteSignal<bool>,
) {
    let client = reqwest::Client::new();
    
    match client
        .post(api_url("api/execute"))
        .json(&ExecuteCommand { command })
        .send()
        .await
    {
        Ok(response) => {
            if let Ok(result) = response.json::<ExecuteResponse>().await {
                let msg = if result.success {
                    ChatMessage {
                        id: uuid::Uuid::new_v4().to_string(),
                        content: format!("✅ {}", format_json_response(&result.data.unwrap_or(serde_json::json!({})))),
                        is_user: false,
                        timestamp: chrono::Local::now().format("%H:%M").to_string(),
                        status: MessageStatus::Delivered,
                    }
                } else {
                    ChatMessage {
                        id: uuid::Uuid::new_v4().to_string(),
                        content: format!("❌ {}", result.error.unwrap_or_else(|| "Unknown error".to_string())),
                        is_user: false,
                        timestamp: chrono::Local::now().format("%H:%M").to_string(),
                        status: MessageStatus::Error,
                    }
                };
                set_messages.update(|msgs| msgs.push(msg));
            }
        },
        Err(e) => {
            let error_msg = ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                content: format!("⚠️ Connection error: {}", e),
                is_user: false,
                timestamp: chrono::Local::now().format("%H:%M").to_string(),
                status: MessageStatus::Error,
            };
            set_messages.update(|msgs| msgs.push(error_msg));
        }
    }
    
    set_is_loading.set(false);
}

fn format_session_progress(session: &GuidedSession) -> String {
    let mut result = String::from("📝 Creating transaction\n");
    result.push_str(&format!("Progress: {} of {} fields collected\n", session.progress, session.total_required));
    
    if !session.collected_tokens.is_empty() {
        result.push_str("\n✅ Collected:\n");
        for (name, value) in &session.collected_tokens {
            result.push_str(&format!("  • {}: {}\n", name, value));
        }
    }
    
    result
}

fn format_json_response(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let mut result = String::new();
            for (key, val) in map {
                result.push_str(&format!("📌 {}: {}\n", key, format_json_value(val)));
            }
            result
        },
        _ => serde_json::to_string_pretty(value).unwrap_or_else(|_| "Success".to_string())
    }
}

fn format_json_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        _ => serde_json::to_string(value).unwrap_or_else(|_| "...".to_string())
    }
}