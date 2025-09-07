# FileUpload Component

A flexible file upload component for Leptos with drag-and-drop support, file type validation, and multiple upload modes.

## Installation

```toml
[dependencies]
taxtalk-file-upload = "0.1.0"
```

## Usage

```rust
use leptos::prelude::*;
use taxtalk_file_upload::{FileUpload, FileUploadMode, UploadedFile};

#[component]
fn MyApp() -> impl IntoView {
    let (mode, set_mode) = signal(FileUploadMode::Single);
    
    view! {
        <FileUpload
            mode=mode
            on_upload=Callback::new(move |files: Vec<UploadedFile>| {
                // Handle uploaded files
                log!("Files uploaded: {:?}", files);
            })
            max_size=10_485_760  // 10MB
            max_files=5
        />
    }
}
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `mode` | `Signal<FileUploadMode>` | - | Upload mode (Single, Multiple, Image, Document, Receipt) |
| `on_upload` | `Callback<Vec<UploadedFile>, ()>` | - | Callback when files are uploaded |
| `max_size` | `usize` | 10,485,760 (10MB) | Maximum file size in bytes |
| `max_files` | `usize` | 5 | Maximum number of files (for Multiple mode) |

## Upload Modes

- **Single**: Accept a single file of any type
- **Multiple**: Accept multiple files of any type
- **Image**: Accept only image files (JPG, PNG, GIF)
- **Document**: Accept only document files (PDF, DOC, XLS)
- **Receipt**: Accept receipts (images or PDF)

## Features

- 🎯 Drag and drop support
- 📁 Click to browse files
- ✅ File type validation
- 📏 File size validation
- 👁️ File preview with icons
- ❌ Remove uploaded files
- 🎨 Customizable via CSS
- 📱 Responsive design

## Development

### Run Examples

```bash
cd components/file-upload
trunk serve --open
```

### Run Tests

```bash
cargo test
wasm-pack test --headless --firefox
```

## Styling

The component uses CSS classes with the `tui-fu-` prefix to avoid conflicts. You can override these styles in your application:

```css
.tui-fu-dropzone {
    /* Custom dropzone styles */
}

.tui-fu-file {
    /* Custom file preview styles */
}
```

## License

MIT