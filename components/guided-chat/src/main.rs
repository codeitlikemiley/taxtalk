use leptos::prelude::*;
use guided_chat::*;

fn main() {
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 2rem;">
            <style>{include_str!("styles.css")}</style>
            
            <div style="max-width: 1200px; margin: 0 auto;">
                <div style="text-align: center; color: white; margin-bottom: 3rem;">
                    <h1 style="font-size: 3rem; font-weight: bold; margin-bottom: 1rem;">
                        "Guided Chat Demo"
                    </h1>
                    <p style="font-size: 1.25rem; opacity: 0.9;">
                        "Step-by-step workflows for TaxTalk"
                    </p>
                </div>
                
                <div style="background: white; border-radius: 1rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1); overflow: hidden;">
                    <SimpleGuidedChat />
                </div>
                
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-top: 2rem;">
                    <div style="background: white; padding: 1.5rem; border-radius: 0.75rem;">
                        <h3 style="color: #1e40af; font-weight: 600; margin-bottom: 0.5rem;">
                            "🚀 Step-by-Step"
                        </h3>
                        <p style="color: #6b7280; font-size: 0.875rem;">
                            "Guided workflows for complex processes"
                        </p>
                    </div>
                    
                    <div style="background: white; padding: 1.5rem; border-radius: 0.75rem;">
                        <h3 style="color: #14532d; font-weight: 600; margin-bottom: 0.5rem;">
                            "✅ Validation"
                        </h3>
                        <p style="color: #6b7280; font-size: 0.875rem;">
                            "Real-time input validation"
                        </p>
                    </div>
                    
                    <div style="background: white; padding: 1.5rem; border-radius: 0.75rem;">
                        <h3 style="color: #78350f; font-weight: 600; margin-bottom: 0.5rem;">
                            "🇵🇭 Philippine Rules"
                        </h3>
                        <p style="color: #6b7280; font-size: 0.875rem;">
                            "VAT, EWT, BIR compliance"
                        </p>
                    </div>
                    
                    <div style="background: white; padding: 1.5rem; border-radius: 0.75rem;">
                        <h3 style="color: #831843; font-weight: 600; margin-bottom: 0.5rem;">
                            "📊 Progress"
                        </h3>
                        <p style="color: #6b7280; font-size: 0.875rem;">
                            "Visual progress tracking"
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}