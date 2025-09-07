use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, HtmlInputElement, FileList};
use uuid::Uuid;
use crate::types::*;

#[component]
pub fn FileUpload(
    mode: Signal<FileUploadMode>,
    on_upload: Callback<Vec<UploadedFile>, ()>,
    #[prop(default = 10_485_760)] max_size: usize, // 10MB default
    #[prop(default = 5)] max_files: usize,
) -> impl IntoView {
    let (files, set_files) = signal(Vec::<UploadedFile>::new());
    let (is_dragging, set_is_dragging) = signal(false);
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let file_input_ref = NodeRef::<leptos::html::Input>::new();
    
    // Determine accept attribute based on mode
    let accept_types = move || match mode.get() {
        FileUploadMode::Image => "image/*",
        FileUploadMode::Document => ".pdf,.doc,.docx,.xls,.xlsx",
        FileUploadMode::Receipt => "image/*,.pdf",
        _ => "*",
    };
    
    let multiple = move || matches!(mode.get(), FileUploadMode::Multiple);
    
    let handle_files = move |file_list: FileList| {
        let mut new_files = files.get();
        let file_count = file_list.length();
        
        // Validate file count
        if new_files.len() + file_count as usize > max_files {
            set_error_message.set(Some(format!("Maximum {} files allowed", max_files)));
            return;
        }
        
        for i in 0..file_count {
            if let Some(file) = file_list.item(i) {
                // Validate file size
                if file.size() as usize > max_size {
                    set_error_message.set(Some(format!("File {} exceeds maximum size of {}MB", 
                        file.name(), max_size / 1_048_576)));
                    continue;
                }
                
                // Validate file type based on mode
                let mime_type = file.type_();
                let is_valid = match mode.get() {
                    FileUploadMode::Image => mime_type.starts_with("image/"),
                    FileUploadMode::Document => {
                        mime_type.contains("pdf") || 
                        mime_type.contains("word") || 
                        mime_type.contains("sheet")
                    },
                    FileUploadMode::Receipt => {
                        mime_type.starts_with("image/") || mime_type.contains("pdf")
                    },
                    _ => true,
                };
                
                if !is_valid {
                    set_error_message.set(Some(format!("Invalid file type: {}", file.name())));
                    continue;
                }
                
                // Create preview URL for images
                let preview_url = if mime_type.starts_with("image/") {
                    // In a real app, we'd use FileReader API to create data URL
                    // For now, we'll use a placeholder
                    Some(format!("preview_{}", file.name()))
                } else {
                    None
                };
                
                let uploaded_file = UploadedFile {
                    id: Uuid::new_v4().to_string(),
                    name: file.name(),
                    size: file.size() as usize,
                    mime_type,
                    preview_url,
                    upload_progress: 0.0,
                };
                
                new_files.push(uploaded_file);
            }
        }
        
        set_files.set(new_files.clone());
        set_error_message.set(None);
        
        // Trigger callback
        on_upload.run(new_files);
    };
    
    let handle_drop = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        set_is_dragging.set(false);
        
        if let Some(data_transfer) = e.data_transfer() {
            if let Some(files) = data_transfer.files() {
                handle_files(files);
            }
        }
    };
    
    let handle_drag_over = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        set_is_dragging.set(true);
    };
    
    let handle_drag_leave = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        set_is_dragging.set(false);
    };
    
    let handle_file_select = move |e: web_sys::Event| {
        if let Some(input) = e.target() {
            let input: HtmlInputElement = input.dyn_into().unwrap();
            if let Some(files) = input.files() {
                handle_files(files);
            }
        }
    };
    
    let trigger_file_select = move |_| {
        if let Some(input) = file_input_ref.get() {
            let _ = input.click();
        }
    };
    
    let remove_file = move |id: String| {
        let mut current_files = files.get();
        current_files.retain(|f| f.id != id);
        set_files.set(current_files.clone());
        on_upload.run(current_files);
    };
    
    view! {
        <div class="tui-fu-component">
            // Hidden file input
            <input
                type="file"
                class="tui-fu-hidden"
                accept=accept_types
                multiple=multiple
                node_ref=file_input_ref
                on:change=handle_file_select
            />
            
            // Drop zone
            <div
                class=move || {
                    format!(
                        "tui-fu-dropzone {}",
                        if is_dragging.get() {
                            "tui-fu-dragging"
                        } else {
                            ""
                        }
                    )
                }
                on:drop=handle_drop
                on:dragover=handle_drag_over
                on:dragleave=handle_drag_leave
                on:click=trigger_file_select
            >
                <div class="tui-fu-content">
                    <svg class="tui-fu-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                    </svg>
                    <p class="tui-fu-title">
                        {move || if is_dragging.get() {
                            "Drop files here"
                        } else {
                            "Drop files here or click to browse"
                        }}
                    </p>
                    <p class="tui-fu-subtitle">
                        {move || match mode.get() {
                            FileUploadMode::Image => "Images only (JPG, PNG, GIF)".to_string(),
                            FileUploadMode::Document => "Documents only (PDF, DOC, XLS)".to_string(),
                            FileUploadMode::Receipt => "Receipts (Images or PDF)".to_string(),
                            FileUploadMode::Single => "Single file only".to_string(),
                            FileUploadMode::Multiple => format!("Up to {} files", max_files),
                        }}
                    </p>
                    <p class="tui-fu-info">
                        {format!("Maximum file size: {}MB", max_size / 1_048_576)}
                    </p>
                </div>
            </div>
            
            // Error message
            {move || error_message.get().map(|msg| view! {
                <div class="tui-fu-error">
                    {msg}
                </div>
            })}
            
            // Uploaded files preview
            {move || if !files.get().is_empty() {
                view! {
                    <div class="tui-fu-preview">
                        <h3 class="tui-fu-preview-title">
                            "Uploaded Files:"
                        </h3>
                        <div class="tui-fu-preview-grid">
                            {files.get().into_iter().map(|file| {
                                let file_id = file.id.clone();
                                view! {
                                    <div class="tui-fu-file">
                                        <div class="tui-fu-file-content">
                                            {if file.mime_type.starts_with("image/") {
                                                view! {
                                                    <div class="tui-fu-file-icon tui-fu-image">
                                                        "🖼"
                                                    </div>
                                                }.into_any()
                                            } else if file.mime_type.contains("pdf") {
                                                view! {
                                                    <div class="tui-fu-file-icon tui-fu-pdf">
                                                        "📄"
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="tui-fu-file-icon tui-fu-document">
                                                        "📁"
                                                    </div>
                                                }.into_any()
                                            }}
                                            <p class="tui-fu-file-name">
                                                {file.name.clone()}
                                            </p>
                                            <p class="tui-fu-file-size">
                                                {format!("{:.1} KB", file.size as f64 / 1024.0)}
                                            </p>
                                        </div>
                                        <button
                                            class="tui-fu-file-remove"
                                            on:click=move |e| {
                                                e.stop_propagation();
                                                remove_file(file_id.clone());
                                            }
                                        >
                                            "×"
                                        </button>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </div>
    }
}