use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="showcase-home">
            <div class="showcase-hero">
                <h1>"TaxTalk UI Components"</h1>
                <p>"A comprehensive component library for building modern business applications"</p>
            </div>
            
            <div class="showcase-features">
                <h2>"Features"</h2>
                <div class="showcase-feature-grid">
                    <div class="showcase-feature">
                        <span class="showcase-feature-icon">"🎨"</span>
                        <h3>"Styled Components"</h3>
                        <p>"Pre-styled components with customizable CSS"</p>
                    </div>
                    <div class="showcase-feature">
                        <span class="showcase-feature-icon">"🧩"</span>
                        <h3>"Modular Design"</h3>
                        <p>"Each component in its own crate for isolation"</p>
                    </div>
                    <div class="showcase-feature">
                        <span class="showcase-feature-icon">"🚀"</span>
                        <h3>"Performance"</h3>
                        <p>"Optimized for WASM with minimal bundle size"</p>
                    </div>
                    <div class="showcase-feature">
                        <span class="showcase-feature-icon">"📱"</span>
                        <h3>"Responsive"</h3>
                        <p>"Works seamlessly on desktop and mobile"</p>
                    </div>
                </div>
            </div>
            
            <div class="showcase-components">
                <h2>"Available Components"</h2>
                <div class="showcase-component-list">
                    <A href="/file-upload" class="showcase-component-card">
                        <h3>"File Upload"</h3>
                        <p>"Drag & drop file upload with validation"</p>
                        <span class="showcase-component-status showcase-status-ready">"Ready"</span>
                    </A>
                    
                    <div class="showcase-component-card showcase-component-disabled">
                        <h3>"Dynamic Table"</h3>
                        <p>"Data table with computed columns"</p>
                        <span class="showcase-component-status showcase-status-wip">"In Progress"</span>
                    </div>
                    
                    <div class="showcase-component-card showcase-component-disabled">
                        <h3>"Conditional Form"</h3>
                        <p>"Forms with dynamic field visibility"</p>
                        <span class="showcase-component-status showcase-status-wip">"In Progress"</span>
                    </div>
                    
                    <div class="showcase-component-card showcase-component-disabled">
                        <h3>"Receipt Scanner"</h3>
                        <p>"Philippine receipt OCR scanner"</p>
                        <span class="showcase-component-status showcase-status-wip">"In Progress"</span>
                    </div>
                </div>
            </div>
            
            <div class="showcase-usage">
                <h2>"Installation"</h2>
                <pre class="showcase-code">
                    <code>
"[dependencies]
taxtalk-ui = \"0.1.0\""
                    </code>
                </pre>
                
                <h2>"Quick Start"</h2>
                <pre class="showcase-code">
                    <code>
"use leptos::prelude::*;
use taxtalk_ui::{{FileUpload, FileUploadMode}};

#[component]
fn App() -> impl IntoView {{
    let mode = Signal::derive(|| FileUploadMode::Single);
    
    view! {{
        <FileUpload
            mode=mode
            on_upload=|files| {{
                log!(\"Files: {{:?}}\", files);
            }}
        />
    }}
}}"
                    </code>
                </pre>
            </div>
        </div>
    }
}