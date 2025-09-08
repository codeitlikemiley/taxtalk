use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, HtmlInputElement, DataTransfer, FileList};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FileUploadMode {
    Single,
    Multiple,
    Image,
    Document,
    Receipt,
}

#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub id: String,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub preview_url: Option<String>,
    pub upload_progress: f32,
}

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
        <div class="file-upload-component">
            // Hidden file input
            <input
                type="file"
                class="hidden"
                accept=accept_types
                multiple=multiple
                node_ref=file_input_ref
                on:change=handle_file_select
            />
            
            // Drop zone
            <div
                class=move || {
                    format!(
                        "border-2 border-dashed rounded-lg p-8 text-center transition-colors {}",
                        if is_dragging.get() {
                            "border-blue-500 bg-blue-50"
                        } else {
                            "border-gray-300 hover:border-gray-400"
                        }
                    )
                }
                on:drop=handle_drop
                on:dragover=handle_drag_over
                on:dragleave=handle_drag_leave
                on:click=trigger_file_select
            >
                <div class="text-gray-600">
                    <svg class="mx-auto h-12 w-12 text-gray-400 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                    </svg>
                    <p class="text-lg font-medium">
                        {move || if is_dragging.get() {
                            "Drop files here"
                        } else {
                            "Drop files here or click to browse"
                        }}
                    </p>
                    <p class="text-sm text-gray-500 mt-2">
                        {move || match mode.get() {
                            FileUploadMode::Image => "Images only (JPG, PNG, GIF)".to_string(),
                            FileUploadMode::Document => "Documents only (PDF, DOC, XLS)".to_string(),
                            FileUploadMode::Receipt => "Receipts (Images or PDF)".to_string(),
                            FileUploadMode::Single => "Single file only".to_string(),
                            FileUploadMode::Multiple => format!("Up to {} files", max_files),
                        }}
                    </p>
                    <p class="text-xs text-gray-400 mt-1">
                        {format!("Maximum file size: {}MB", max_size / 1_048_576)}
                    </p>
                </div>
            </div>
            
            // Error message
            {move || error_message.get().map(|msg| view! {
                <div class="mt-2 p-2 bg-red-100 border border-red-300 rounded text-red-700 text-sm">
                    {msg}
                </div>
            })}
            
            // Uploaded files preview
            {move || if !files.get().is_empty() {
                view! {
                    <div class="mt-4">
                        <h3 class="text-sm font-medium text-gray-700 mb-2">
                            "Uploaded Files:"
                        </h3>
                        <div class="grid grid-cols-3 gap-3">
                            {files.get().into_iter().map(|file| {
                                let file_id = file.id.clone();
                                view! {
                                    <div class="relative group border border-gray-200 rounded-lg p-3 hover:shadow-md transition-shadow">
                                        <div class="text-center">
                                            {if file.mime_type.starts_with("image/") {
                                                view! {
                                                    <div class="h-16 w-16 mx-auto mb-2 bg-gray-100 rounded flex items-center justify-center">
                                                        "🖼"
                                                    </div>
                                                }.into_any()
                                            } else if file.mime_type.contains("pdf") {
                                                view! {
                                                    <div class="h-16 w-16 mx-auto mb-2 bg-red-100 rounded flex items-center justify-center">
                                                        "📄"
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="h-16 w-16 mx-auto mb-2 bg-blue-100 rounded flex items-center justify-center">
                                                        "📁"
                                                    </div>
                                                }.into_any()
                                            }}
                                            <p class="text-xs text-gray-600 truncate">
                                                {file.name.clone()}
                                            </p>
                                            <p class="text-xs text-gray-400">
                                                {format!("{:.1} KB", file.size as f64 / 1024.0)}
                                            </p>
                                        </div>
                                        <button
                                            class="absolute top-1 right-1 bg-red-500 text-white rounded-full w-5 h-5 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity"
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