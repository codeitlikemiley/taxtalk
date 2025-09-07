use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{File, HtmlInputElement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub description: String,
    pub quantity: f64,
    pub price: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedReceipt {
    pub vendor: Option<String>,
    pub amount: Option<f64>,
    pub date: Option<String>,
    pub or_number: Option<String>,
    pub tin: Option<String>,
    pub items: Vec<LineItem>,
    pub confidence: f32,
    pub raw_text: Option<String>,
}

impl ScannedReceipt {
    pub fn new() -> Self {
        Self {
            vendor: None,
            amount: None,
            date: None,
            or_number: None,
            tin: None,
            items: vec![],
            confidence: 0.0,
            raw_text: None,
        }
    }
}

// Philippine receipt patterns
const OR_PATTERN: &str = r"OR\s*NO\.?\s*:?\s*(\d{10})";
const TIN_PATTERN: &str = r"TIN\s*:?\s*(\d{3}-\d{3}-\d{3}-\d{3})";
const AMOUNT_PATTERN: &str = r"TOTAL\s*:?\s*₱?\s*([\d,]+\.?\d*)";
const VAT_PATTERN: &str = r"VAT\s*\(12%\)\s*:?\s*₱?\s*([\d,]+\.?\d*)";

#[component]
pub fn ReceiptScanner(
    on_scan: Callback<ScannedReceipt, ()>,
    #[prop(default = true)] auto_enhance: bool,
    #[prop(default = true)] extract_line_items: bool,
) -> impl IntoView {
    let (scanning, set_scanning) = signal(false);
    let (preview_url, set_preview_url) = signal(Option::<String>::None);
    let (scanned_data, set_scanned_data) = signal(ScannedReceipt::new());
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (processing_status, set_processing_status) = signal("Ready to scan".to_string());
    
    let file_input_ref = NodeRef::<leptos::html::Input>::new();
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    
    let handle_file_select = move |e: web_sys::Event| {
        if let Some(input) = e.target() {
            let input: HtmlInputElement = input.dyn_into().unwrap();
            if let Some(files) = input.files() {
                if let Some(file) = files.item(0) {
                    process_image_file(
                        file,
                        set_scanning,
                        set_preview_url,
                        set_scanned_data,
                        set_error_message,
                        set_processing_status,
                        canvas_ref,
                        auto_enhance,
                        extract_line_items,
                        on_scan,
                    );
                }
            }
        }
    };
    
    let trigger_file_select = move |_: leptos::ev::MouseEvent| {
        if let Some(input) = file_input_ref.get() {
            let _ = input.click();
        }
    };
    
    let retry_scan = move |_| {
        set_scanned_data.set(ScannedReceipt::new());
        set_preview_url.set(None);
        set_error_message.set(None);
        set_processing_status.set("Ready to scan".to_string());
        if let Some(input) = file_input_ref.get() {
            let _ = input.click();
        }
    };
    
    let confirm_data = move |_| {
        on_scan.run(scanned_data.get());
        // Reset for next scan
        set_scanned_data.set(ScannedReceipt::new());
        set_preview_url.set(None);
        set_processing_status.set("Ready to scan".to_string());
    };
    
    let update_field = move |field: String, value: String| {
        let mut data = scanned_data.get();
        match field.as_str() {
            "vendor" => data.vendor = Some(value),
            "amount" => data.amount = value.parse::<f64>().ok(),
            "date" => data.date = Some(value),
            "or_number" => data.or_number = Some(value),
            "tin" => data.tin = Some(value),
            _ => {}
        }
        set_scanned_data.set(data);
    };
    
    view! {
        <div class="receipt-scanner-component">
            // Hidden file input
            <input
                type="file"
                class="hidden"
                accept="image/*,.pdf"
                node_ref=file_input_ref
                on:change=handle_file_select
            />
            
            // Hidden canvas for image processing
            <canvas
                class="hidden"
                node_ref=canvas_ref
            />
            
            // Scanner UI
            <div class="border-2 border-dashed border-gray-300 rounded-lg p-6">
                {move || if preview_url.get().is_some() {
                    view! {
                        <div class="scanned-receipt">
                            // Image preview
                            <div class="mb-4">
                                <img
                                    src=preview_url.get().unwrap_or_default()
                                    alt="Receipt preview"
                                    class="max-w-full h-auto rounded shadow-lg"
                                />
                            </div>
                            
                            // Processing status
                            <div class="mb-4 text-center">
                                <p class="text-sm text-gray-600">
                                    {move || processing_status.get()}
                                </p>
                                {move || if scanning.get() {
                                    view! {
                                        <div class="mt-2">
                                            <div class="inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500"></div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}
                            </div>
                            
                            // Extracted data form
                            <div class="extracted-data bg-gray-50 rounded p-4">
                                <h3 class="text-lg font-semibold mb-3">Extracted Information</h3>
                                
                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">
                                            "Vendor/Store"
                                        </label>
                                        <input
                                            type="text"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                                            value=move || scanned_data.get().vendor.unwrap_or_default()
                                            on:change=move |e| {
                                                let val = event_target_value(&e);
                                                update_field("vendor".to_string(), val)
                                            }
                                        />
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">
                                            "Total Amount"
                                        </label>
                                        <input
                                            type="number"
                                            step="0.01"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                                            value=move || scanned_data.get().amount.map(|a| a.to_string()).unwrap_or_default()
                                            on:change=move |e| {
                                                let val = event_target_value(&e);
                                                update_field("amount".to_string(), val)
                                            }
                                        />
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">
                                            "OR Number"
                                        </label>
                                        <input
                                            type="text"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                                            value=move || scanned_data.get().or_number.unwrap_or_default()
                                            on:change=move |e| {
                                                let val = event_target_value(&e);
                                                update_field("or_number".to_string(), val)
                                            }
                                        />
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">
                                            "Date"
                                        </label>
                                        <input
                                            type="date"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                                            value=move || scanned_data.get().date.unwrap_or_default()
                                            on:change=move |e| {
                                                let val = event_target_value(&e);
                                                update_field("date".to_string(), val)
                                            }
                                        />
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">
                                            "TIN"
                                        </label>
                                        <input
                                            type="text"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                                            placeholder="000-000-000-000"
                                            value=move || scanned_data.get().tin.unwrap_or_default()
                                            on:change=move |e| {
                                                let val = event_target_value(&e);
                                                update_field("tin".to_string(), val)
                                            }
                                        />
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">
                                            "Confidence"
                                        </label>
                                        <div class="text-sm">
                                            <span class=move || {
                                                let conf = scanned_data.get().confidence;
                                                if conf > 0.8 {
                                                    "text-green-600 font-semibold"
                                                } else if conf > 0.5 {
                                                    "text-yellow-600"
                                                } else {
                                                    "text-red-600"
                                                }
                                            }>
                                                {move || format!("{:.0}%", scanned_data.get().confidence * 100.0)}
                                            </span>
                                            " accuracy"
                                        </div>
                                    </div>
                                </div>
                                
                                // Line items if extracted
                                {move || if !scanned_data.get().items.is_empty() {
                                    view! {
                                        <div class="mt-4">
                                            <h4 class="text-sm font-medium text-gray-700 mb-2">Line Items</h4>
                                            <div class="text-xs border rounded overflow-hidden">
                                                <table class="min-w-full">
                                                    <thead class="bg-gray-100">
                                                        <tr>
                                                            <th class="px-2 py-1 text-left">Item</th>
                                                            <th class="px-2 py-1 text-right">Qty</th>
                                                            <th class="px-2 py-1 text-right">Price</th>
                                                            <th class="px-2 py-1 text-right">Total</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {scanned_data.get().items.iter().map(|item| {
                                                            view! {
                                                                <tr class="border-t">
                                                                    <td class="px-2 py-1">{item.description.clone()}</td>
                                                                    <td class="px-2 py-1 text-right">{item.quantity}</td>
                                                                    <td class="px-2 py-1 text-right">{format!("{:.2}", item.price)}</td>
                                                                    <td class="px-2 py-1 text-right">{format!("{:.2}", item.total)}</td>
                                                                </tr>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}
                                
                                // Action buttons
                                <div class="mt-4 flex gap-2">
                                    <button
                                        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                                        on:click=confirm_data
                                        disabled=scanning
                                    >
                                        "Confirm & Use"
                                    </button>
                                    <button
                                        class="px-4 py-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400 transition-colors"
                                        on:click=retry_scan
                                        disabled=scanning
                                    >
                                        "Scan Again"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="text-center">
                            <svg class="mx-auto h-12 w-12 text-gray-400 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z"></path>
                            </svg>
                            <p class="text-lg font-medium text-gray-900 mb-2">
                                "Scan Receipt"
                            </p>
                            <p class="text-sm text-gray-500 mb-4">
                                "Take a photo or upload an image of your receipt"
                            </p>
                            <div class="flex gap-2 justify-center">
                                <button
                                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                                    on:click=trigger_file_select
                                >
                                    "📷 Choose Photo"
                                </button>
                            </div>
                            <p class="text-xs text-gray-400 mt-3">
                                "Supports: SM, 7-Eleven, Mercury Drug, and other Philippine stores"
                            </p>
                        </div>
                    }.into_any()
                }}
            </div>
            
            // Error message
            {move || error_message.get().map(|msg| view! {
                <div class="mt-2 p-2 bg-red-100 border border-red-300 rounded text-red-700 text-sm">
                    {msg}
                </div>
            })}
        </div>
    }
}

fn process_image_file(
    file: File,
    set_scanning: WriteSignal<bool>,
    set_preview_url: WriteSignal<Option<String>>,
    set_scanned_data: WriteSignal<ScannedReceipt>,
    set_error_message: WriteSignal<Option<String>>,
    set_processing_status: WriteSignal<String>,
    _canvas_ref: NodeRef<leptos::html::Canvas>,
    _auto_enhance: bool,
    extract_line_items: bool,
    on_scan: Callback<ScannedReceipt, ()>,
) {
    set_scanning.set(true);
    set_processing_status.set("Processing image...".to_string());
    
    // Create preview URL (in real app, would use FileReader API)
    let preview = format!("preview_{}", file.name());
    set_preview_url.set(Some(preview));
    
    // Simulate OCR processing
    set_timeout(
        move || {
            set_processing_status.set("Extracting text...".to_string());
            
            // Simulate extracted data based on common Philippine receipts
            let mut receipt = ScannedReceipt::new();
            
            // Simulate different store patterns
            let store_patterns = vec![
                ("SM", "SM SUPERMARKET", "123-456-789-000"),
                ("7-ELEVEN", "7-ELEVEN PHILIPPINES", "234-567-890-000"),
                ("MERCURY", "MERCURY DRUG", "345-678-901-000"),
            ];
            
            let random_store = store_patterns[0]; // In real app, would be from OCR
            receipt.vendor = Some(random_store.1.to_string());
            receipt.tin = Some(random_store.2.to_string());
            receipt.or_number = Some(format!("{:010}", 1234567890));
            receipt.amount = Some(1234.56);
            receipt.date = Some("2024-01-15".to_string());
            receipt.confidence = 0.85;
            
            if extract_line_items {
                receipt.items = vec![
                    LineItem {
                        description: "GROCERY ITEM 1".to_string(),
                        quantity: 2.0,
                        price: 50.00,
                        total: 100.00,
                    },
                    LineItem {
                        description: "GROCERY ITEM 2".to_string(),
                        quantity: 1.0,
                        price: 75.50,
                        total: 75.50,
                    },
                ];
            }
            
            receipt.raw_text = Some("Sample OCR text...".to_string());
            
            set_scanned_data.set(receipt.clone());
            set_scanning.set(false);
            set_processing_status.set("Extraction complete!".to_string());
            
            // Auto-confirm if high confidence
            if receipt.confidence > 0.9 {
                on_scan.run(receipt);
            }
        },
        std::time::Duration::from_millis(2000),
    );
}

// Mock OCR function (in real app, would call actual OCR service)
async fn process_with_ocr(_image_data: Vec<u8>) -> Result<String, String> {
    // Simulate OCR API call
    Ok("Sample OCR text from receipt...".to_string())
}

// Extract fields from OCR text using regex
fn extract_receipt_fields(text: &str) -> ScannedReceipt {
    let mut receipt = ScannedReceipt::new();
    receipt.raw_text = Some(text.to_string());
    
    // Extract OR number
    if let Some(captures) = regex::Regex::new(OR_PATTERN).ok()
        .and_then(|re| re.captures(text)) {
        receipt.or_number = captures.get(1).map(|m| m.as_str().to_string());
    }
    
    // Extract TIN
    if let Some(captures) = regex::Regex::new(TIN_PATTERN).ok()
        .and_then(|re| re.captures(text)) {
        receipt.tin = captures.get(1).map(|m| m.as_str().to_string());
    }
    
    // Extract amount
    if let Some(captures) = regex::Regex::new(AMOUNT_PATTERN).ok()
        .and_then(|re| re.captures(text)) {
        if let Some(amount_str) = captures.get(1) {
            receipt.amount = amount_str.as_str()
                .replace(",", "")
                .parse::<f64>()
                .ok();
        }
    }
    
    // Set confidence based on fields found
    let mut confidence: f32 = 0.0;
    if receipt.or_number.is_some() { confidence += 0.3; }
    if receipt.tin.is_some() { confidence += 0.3; }
    if receipt.amount.is_some() { confidence += 0.3; }
    receipt.confidence = confidence.min(0.95);
    
    receipt
}