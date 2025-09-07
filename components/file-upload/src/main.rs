use leptos::prelude::*;
use taxtalk_file_upload::{FileUpload, FileUploadMode, UploadedFile};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (mode_state, set_mode) = signal(FileUploadMode::Single);
    let mode = Signal::derive(move || mode_state.get());
    let (uploaded_files, set_uploaded_files) = signal(Vec::new());
    
    view! {
        <div class="container">
            <h1>"FileUpload Component Demo"</h1>
            
            <div class="example-section">
                <div class="example-title">"Mode Selector"</div>
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
                    on_upload=Callback::new(move |files: Vec<UploadedFile>| {
                        set_uploaded_files.set(files.clone());
                        leptos::logging::log!("Files uploaded: {:?}", files);
                    })
                />
                
                <Show
                    when=move || !uploaded_files.get().is_empty()
                    fallback=|| view! { <></> }
                >
                    <div style="margin-top: 20px; padding: 10px; background: #f0f0f0; border-radius: 4px;">
                        <strong>"Uploaded Files:"</strong>
                        <ul>
                            {move || uploaded_files.get().into_iter().map(|file| {
                                view! {
                                    <li>{format!("{} ({:.1} KB)", file.name, file.size as f64 / 1024.0)}</li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                </Show>
            </div>
        </div>
    }
}