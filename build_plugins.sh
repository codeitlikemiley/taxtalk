#!/bin/bash

# Build all plugins for WASM
echo "Building TaxTalk plugins..."

# Add WASM target if not already added
rustup target add wasm32-wasip2 2>/dev/null

# Build each plugin
for plugin_dir in plugins/*/; do
    if [ -f "$plugin_dir/Cargo.toml" ]; then
        plugin_name=$(basename "$plugin_dir")
        echo "Building plugin: $plugin_name"
        
        cd "$plugin_dir"
        cargo build --target wasm32-wasip2 --release
        
        # Check if build succeeded
        if [ $? -eq 0 ]; then
            echo "✓ $plugin_name built successfully"
        else
            echo "✗ Failed to build $plugin_name"
        fi
        
        cd - > /dev/null
    fi
done

echo "Plugin build complete!"