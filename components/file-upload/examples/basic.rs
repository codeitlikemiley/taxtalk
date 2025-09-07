use leptos::prelude::*;
use taxtalk_file_upload::{FileUpload, FileUploadMode};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (mode, set_mode) = signal(FileUploadMode::Single);
    let (uploaded_files, set_uploaded_files) = signal(Vec::new());
    
    view! {
        <div class="container">
            <h1>"FileUpload Component Examples"</h1>
            
            <div class="example-section">
                <div class="example-title">"Single File Upload"</div>
                <FileUpload
                    mode=Signal::derive(move || FileUploadMode::Single)
                    on_upload=Callback::new(move |files| {
                        set_uploaded_files.set(files.clone());
                        leptos::logging::log!("Single file uploaded: {:?}", files);
                    })
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Multiple File Upload"</div>
                <FileUpload
                    mode=Signal::derive(move || FileUploadMode::Multiple)
                    on_upload=Callback::new(move |files| {
                        leptos::logging::log!("Multiple files uploaded: {:?}", files);
                    })
                    max_files=10
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Image Only Upload"</div>
                <FileUpload
                    mode=Signal::derive(move || FileUploadMode::Image)
                    on_upload=Callback::new(move |files| {
                        leptos::logging::log!("Images uploaded: {:?}", files);
                    })
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Document Upload"</div>
                <FileUpload
                    mode=Signal::derive(move || FileUploadMode::Document)
                    on_upload=Callback::new(move |files| {
                        leptos::logging::log!("Documents uploaded: {:?}", files);
                    })
                    max_size=20_971_520  // 20MB
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Receipt Scanner"</div>
                <FileUpload
                    mode=Signal::derive(move || FileUploadMode::Receipt)
                    on_upload=Callback::new(move |files| {
                        leptos::logging::log!("Receipts uploaded: {:?}", files);
                    })
                />
            </div>
            
            <div class="example-section">
                <div class="example-title">"Mode Switcher Example"</div>
                <div style="margin-bottom: 15px;">
                    <label>"Select Mode: "</label>
                    <select on:change=move |e| {
                        let value = event_target_value(&e);
                        let new_mode = match value.as_str() {
                            "single" => FileUploadMode::Single,
                            "multiple" => FileUploadMode::Multiple,
                            "image" => FileUploadMode::Image,
                            "document" => FileUploadMode::Document,
                            "receipt" => FileUploadMode::Receipt,
                            _ => FileUploadMode::Single,
                        };
                        set_mode.set(new_mode);
                    }>
                        <option value="single">"Single"</option>
                        <option value="multiple">"Multiple"</option>
                        <option value="image">"Image"</option>
                        <option value="document">"Document"</option>
                        <option value="receipt">"Receipt"</option>
                    </select>
                </div>
                <FileUpload
                    mode=mode
                    on_upload=Callback::new(move |files| {
                        set_uploaded_files.set(files.clone());
                        leptos::logging::log!("Files uploaded with mode {:?}: {:?}", mode.get(), files);
                    })
                />
                
                {move || if !uploaded_files.get().is_empty() {
                    view! {
                        <div style="margin-top: 20px; padding: 10px; background: #f0f0f0; border-radius: 4px;">
                            <strong>"Uploaded Files:"</strong>
                            <ul>
                                {uploaded_files.get().into_iter().map(|file| {
                                    view! {
                                        <li>{format!("{} ({:.1} KB)", file.name, file.size as f64 / 1024.0)}</li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }}
            </div>
        </div>
    }
}