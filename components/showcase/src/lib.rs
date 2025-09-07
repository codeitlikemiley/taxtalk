use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod pages;
use pages::{Home, FileUploadPage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Html attr:lang="en" attr:dir="ltr" />
        <Title text="TaxTalk UI Component Showcase" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        
        <Router>
            <div class="showcase-app">
                <nav class="showcase-nav">
                    <div class="showcase-nav-brand">
                        <h1>"TaxTalk UI"</h1>
                        <span class="showcase-nav-version">"v0.1.0"</span>
                    </div>
                    <ul class="showcase-nav-menu">
                        <li><A href="/">"Home"</A></li>
                        <li><A href="/file-upload">"File Upload"</A></li>
                        // <li><A href="/table">"Dynamic Table"</A></li>
                        // <li><A href="/form">"Conditional Form"</A></li>
                        // <li><A href="/scanner">"Receipt Scanner"</A></li>
                    </ul>
                </nav>
                
                <main class="showcase-main">
                    <Routes fallback=|| view! { <div>"404 - Page not found"</div> }>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/file-upload") view=FileUploadPage />
                        // <Route path=path!("/table") view=TablePage />
                        // <Route path=path!("/form") view=FormPage />
                        // <Route path=path!("/scanner") view=ScannerPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}