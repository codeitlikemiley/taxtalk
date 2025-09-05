# VSCode Rust Analyzer Linked Projects Rules

## Cargo New Project Requirements

### Mandatory VSCode Configuration Update
- EVERY time you create a new Rust project with `cargo new`, you MUST update `.vscode/settings.json`
- Add the new project path to the `rust-analyzer.linkedProjects` array
- This ensures proper IDE support and cross-project references

### Required .vscode/settings.json Structure
```json
{
    "rust-analyzer.linkedProjects": [
        "./core/Cargo.toml",
        "./server/Cargo.toml", 
        "./plugins/invoice/Cargo.toml",
        "./plugins/company/Cargo.toml",
        "./plugins/crm/Cargo.toml"
    ]
}
```

### Project Creation Workflow
1. Run `cargo new [project-name]` or `cargo new --lib [project-name]`
2. IMMEDIATELY update `.vscode/settings.json`
3. Add `"./[project-path]/Cargo.toml"` to the `rust-analyzer.linkedProjects` array
4. Save the settings file
5. Reload VSCode window if necessary for changes to take effect

### Multi-Crate Workspace Support
- For workspace projects, link the workspace root `Cargo.toml`
- For standalone crates in the project, link individual `Cargo.toml` files
- Always use relative paths starting with `"./"`

### Plugin-Specific Rules
When creating new plugins in the `plugins/` directory:
- Plugin path format: `"./plugins/[plugin-name]/Cargo.toml"`
- Maintain alphabetical order in the linkedProjects array
- Include both library and binary plugin types

### Example Updates for Different Project Types

#### Adding a New Plugin
```json
{
    "rust-analyzer.linkedProjects": [
        "./core/Cargo.toml",
        "./server/Cargo.toml",
        "./plugins/company/Cargo.toml",
        "./plugins/crm/Cargo.toml",
        "./plugins/inventory/Cargo.toml",  // <- NEW PLUGIN
        "./plugins/invoice/Cargo.toml"
    ]
}
```

#### Adding a Utility Crate
```json
{
    "rust-analyzer.linkedProjects": [
        "./core/Cargo.toml",
        "./server/Cargo.toml",
        "./shared/Cargo.toml",             // <- NEW SHARED LIB
        "./plugins/company/Cargo.toml",
        "./plugins/crm/Cargo.toml",
        "./plugins/invoice/Cargo.toml"
    ]
}
```

#### Adding Test or Tool Crates
```json
{
    "rust-analyzer.linkedProjects": [
        "./core/Cargo.toml",
        "./server/Cargo.toml",
        "./tools/plugin-generator/Cargo.toml",  // <- NEW TOOL
        "./tests/integration/Cargo.toml",       // <- NEW TESTS
        "./plugins/company/Cargo.toml",
        "./plugins/crm/Cargo.toml",
        "./plugins/invoice/Cargo.toml"
    ]
}
```

### Additional VSCode Settings for Rust Development
Include these additional settings for optimal Rust development experience:

```json
{
    "rust-analyzer.linkedProjects": [
        // ... project paths
    ],
    "rust-analyzer.cargo.target": "wasm32-wasip2",
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.extraArgs": ["--target-dir", "target/analyzer"],
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.imports.granularity.group": "module"
}
```

### File Creation Template
When creating `.vscode/settings.json` for the first time:

```json
{
    "rust-analyzer.linkedProjects": [
        "./core/Cargo.toml"
    ],
    "rust-analyzer.cargo.target": "wasm32-wasip2",
    "rust-analyzer.cargo.features": "all", 
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.procMacro.enable": true,
    "files.watcherExclude": {
        "**/target/**": true,
        "**/node_modules/**": true
    }
}
```

### Error Prevention Rules
- NEVER forget to update `.vscode/settings.json` after `cargo new`
- ALWAYS use relative paths starting with `"./"`
- ALWAYS include the full path to `Cargo.toml`, not just the directory
- MAINTAIN alphabetical or logical ordering in the array
- CHECK that the JSON syntax is valid after each update

### Integration with Project Structure
For the Crux + WASM plugin architecture, maintain this pattern:

```json
{
    "rust-analyzer.linkedProjects": [
        "./core/Cargo.toml",              // Crux + Wasmtime runtime
        "./server/Cargo.toml",            // Axum server
        "./shared/Cargo.toml",            // Shared types (if created)
        "./plugins/[plugin-name]/Cargo.toml"  // Each plugin
    ]
}
```

### Workspace vs Individual Projects
- If using Cargo workspace: link the root `Cargo.toml`
- If using individual projects: link each `Cargo.toml` separately
- For our architecture, prefer individual project linking for better plugin isolation

### Troubleshooting
If Rust Analyzer isn't working properly:
1. Check `.vscode/settings.json` syntax is valid JSON
2. Verify all paths exist and point to actual `Cargo.toml` files
3. Reload VSCode window (`Ctrl+Shift+P` → "Developer: Reload Window")
4. Check Rust Analyzer output panel for errors

### Performance Considerations
- Linking too many projects can slow down Rust Analyzer
- Consider excluding test-only or tool crates if performance is an issue
- Use `"rust-analyzer.cargo.target"` to focus on specific targets
- Set `"rust-analyzer.checkOnSave.extraArgs": ["--target-dir", "target/analyzer"]` to avoid conflicts