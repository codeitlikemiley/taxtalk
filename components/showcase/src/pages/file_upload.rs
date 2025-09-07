use leptos::prelude::*;
use taxtalk_ui::{FileUpload, FileUploadMode, UploadedFile};

#[component]
pub fn FileUploadPage() -> impl IntoView {
    let (mode, set_mode) = signal(FileUploadMode::Single);
    let (uploaded_files, set_uploaded_files) = signal(Vec::<UploadedFile>::new());
    let (show_code, set_show_code) = signal(false);
    
    view! {
        <div class="showcase-page">
            <div class="showcase-page-header">
                <h1>"File Upload Component"</h1>
                <p>"A flexible file upload component with drag-and-drop support"</p>
            </div>
            
            <div class="showcase-demo-section">
                <h2>"Interactive Demo"</h2>
                
                <div class="showcase-controls">
                    <label>
                        "Mode: "
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
                    </label>
                    
                    <button
                        class="showcase-btn"
                        on:click=move |_| set_show_code.update(|v| *v = !*v)
                    >
                        {move || if show_code.get() { "Hide Code" } else { "Show Code" }}
                    </button>
                </div>
                
                <div class="showcase-demo">
                    <FileUpload
                        mode=mode
                        on_upload=Callback::new(move |files| {
                            set_uploaded_files.set(files.clone());
                            leptos::logging::log!("Files uploaded: {:?}", files);
                        })
                    />
                </div>
                
                {move || if show_code.get() {
                    view! {
                        <pre class="showcase-code">
                            <code>
{format!(r#"<FileUpload
    mode=Signal::derive(|| FileUploadMode::{:?})
    on_upload=Callback::new(|files| {{
        log!("Files uploaded: {{:?}}", files);
    }})
/>"#, mode.get())}
                            </code>
                        </pre>
                    }
                } else {
                    view! { <></> }
                }}
                
                {move || if !uploaded_files.get().is_empty() {
                    view! {
                        <div class="showcase-result">
                            <h3>"Uploaded Files:"</h3>
                            <ul>
                                {uploaded_files.get().into_iter().map(|file| {
                                    view! {
                                        <li>
                                            <strong>{file.name.clone()}</strong>
                                            " - "
                                            {format!("{:.1} KB", file.size as f64 / 1024.0)}
                                            " ("
                                            {file.mime_type.clone()}
                                            ")"
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }
                } else {
                    view! { <></> }
                }}
            </div>
            
            <div class="showcase-props-section">
                <h2>"Props"</h2>
                <table class="showcase-table">
                    <thead>
                        <tr>
                            <th>"Prop"</th>
                            <th>"Type"</th>
                            <th>"Default"</th>
                            <th>"Description"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"mode"</code></td>
                            <td><code>"Signal<FileUploadMode>"</code></td>
                            <td>"-"</td>
                            <td>"Upload mode (Single, Multiple, Image, Document, Receipt)"</td>
                        </tr>
                        <tr>
                            <td><code>"on_upload"</code></td>
                            <td><code>"Callback<Vec<UploadedFile>, ()>"</code></td>
                            <td>"-"</td>
                            <td>"Callback when files are uploaded"</td>
                        </tr>
                        <tr>
                            <td><code>"max_size"</code></td>
                            <td><code>"usize"</code></td>
                            <td>"10,485,760"</td>
                            <td>"Maximum file size in bytes"</td>
                        </tr>
                        <tr>
                            <td><code>"max_files"</code></td>
                            <td><code>"usize"</code></td>
                            <td>"5"</td>
                            <td>"Maximum number of files"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            
            <div class="showcase-examples-section">
                <h2>"Examples"</h2>
                
                <div class="showcase-example">
                    <h3>"Basic Usage"</h3>
                    <pre class="showcase-code">
                        <code>
"use taxtalk_ui::{{FileUpload, FileUploadMode}};

#[component]
fn MyComponent() -> impl IntoView {{
    let mode = Signal::derive(|| FileUploadMode::Single);
    
    view! {{
        <FileUpload
            mode=mode
            on_upload=Callback::new(|files| {{
                // Handle uploaded files
            }})
        />
    }}
}}"
                        </code>
                    </pre>
                </div>
                
                <div class="showcase-example">
                    <h3>"Multiple Files with Size Limit"</h3>
                    <pre class="showcase-code">
                        <code>
"<FileUpload
    mode=Signal::derive(|| FileUploadMode::Multiple)
    on_upload=on_upload
    max_size=20_971_520  // 20MB
    max_files=10
/>"
                        </code>
                    </pre>
                </div>
            </div>
        </div>
    }
}