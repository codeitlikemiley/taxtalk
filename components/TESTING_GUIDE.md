# TaxTalk Component Library Testing Guide

## Overview
This guide documents all completed components, how to test them, and their key features. Each component runs independently on its own port for isolated testing.

## Quick Start
```bash
# Navigate to any component directory
cd components/{component-name}

# Run the demo
trunk serve

# Component will be available at http://localhost:{port}
```

## Completed Components

### 1. ✅ Entity-Tag Component
**Location:** `/components/entity-tag`  
**Port:** 8081  
**Purpose:** Visual entity tags with type-based colors and removal capability

**How to Test:**
```bash
cd components/entity-tag
trunk serve
# Open http://localhost:8081
```

**Features to Test:**
- Color-coded tags by entity type (client, supplier, product, etc.)
- Hover effects and animations
- Remove button functionality
- Disabled state styling
- Multiple tag layouts

---

### 2. ✅ Token-Autocomplete Component  
**Location:** `/components/token-autocomplete`  
**Port:** 8082  
**Purpose:** Smart autocomplete for tokenized commands with filtering

**How to Test:**
```bash
cd components/token-autocomplete
trunk serve
# Open http://localhost:8082
```

**Features to Test:**
- Type in the main input field to see autocomplete suggestions
- Use the dedicated filter input in the dropdown
- Press Enter to select highlighted item
- Use Ctrl+N/P for navigation (arrow keys disabled by design)
- Multiple token selection
- Category-based filtering

**Known Issues Fixed:**
- Dropdown now properly closes after selection
- Enter key works without modifier keys
- Filter input auto-focuses when dropdown opens

---

### 3. ✅ Entity-Combobox Component
**Location:** `/components/entity-combobox`  
**Port:** 8084  
**Purpose:** Searchable dropdown with API integration and caching

**How to Test:**
```bash
cd components/entity-combobox
trunk serve
# Open http://localhost:8084
```

**Features to Test:**
- Search filtering as you type
- Mock API with simulated loading states
- Cache expiry (60 seconds by default)
- Keyboard navigation (arrow keys, Enter, Escape)
- Required vs optional field validation
- Loading spinner during API calls
- Recent selections tracking

---

### 4. ✅ Simple-Form Component
**Location:** `/components/simple-form`  
**Port:** 8085  
**Purpose:** Comprehensive form builder with Philippine-specific validation

**How to Test:**
```bash
cd components/simple-form
trunk serve
# Open http://localhost:8085
```

**Features to Test:**
- **Basic Form:** Try all field types (text, email, number, select, textarea, checkbox)
- **Client Registration:** Philippine TIN validation, phone number format
- **Invoice Form:** Currency fields, VAT calculations
- **Payment Form:** Amount validation, payment method selection

**Validation Tests:**
- Enter invalid email format
- Try Philippine TIN: 123-456-789-000 (valid format)
- Phone: +639171234567 or 09171234567 (both valid)
- Currency: Must be positive numbers
- Submit with missing required fields

---

### 5. ✅ Command-Entity-Input Component
**Location:** `/components/command-entity-input`  
**Port:** 8086  
**Purpose:** Real-time command parsing with entity selection

**How to Test:**
```bash
cd components/command-entity-input
trunk serve
# Open http://localhost:8086
```

**Features to Test:**
- **Command Detection:** Type "create", "update", "delete", "list", "search", "generate"
- **Entity Selection:** Type "@" after a command to see entity selector
- **Mode Switching:** Automatically switches between Command and AI modes
- **Visual Feedback:** Color-coded tokens for commands, entities, parameters
- **Validation:** Some commands require entities (e.g., "create" needs an entity)

**Example Inputs:**
- `create invoice` - Will show validation error (needs entity)
- `update @` - Shows entity selector
- `list invoices` - Valid command
- `how do I create an invoice?` - AI mode
- `delete @client` - Command with entity selection

---

### 6. ✅ Mixed-Input Component
**Location:** `/components/mixed-input`  
**Port:** 8087  
**Purpose:** Multi-entity coordination with mixed text and entity inputs

**How to Test:**
```bash
cd components/mixed-input
trunk serve
# Open http://localhost:8087
```

**Features to Test:**
- **Entity Tags:** Visual tags above the textarea
- **@ Mentions:** Type @ to trigger entity selection callback
- **Mixed Content:** Combine plain text with entity references
- **Character Limit:** Second input has 200 character limit with counter
- **Entity Removal:** Click X on tags or use Backspace when input is empty
- **Multi-line Support:** Shift+Enter for new lines, Enter to submit

**Sample Workflow:**
1. Type "Send invoice to @" 
2. Entity selector would appear (simulated in demo)
3. Add multiple entities
4. See visual tags with type-based colors
5. Submit with Enter key

---

### 7. ✅ Dual-Mode-Input Component
**Location:** `/components/dual-mode-input`  
**Port:** 8088  
**Purpose:** Smart input with automatic mode switching and state preservation

**How to Test:**
```bash
cd components/dual-mode-input
trunk serve
# Open http://localhost:8088
```

**Features to Test:**
- **Mode Toggle:** Click the mode toggle button to switch between AI and Token modes
- **Auto-Switch:** Type "create", "invoice", "pay", or "report" in AI mode to auto-switch
- **Tokenization:** In token mode, build commands with visual tags
- **Entity Selection:** Type @ after a command to add entities
- **Command History:** Use Cmd+↑/↓ to navigate through history
- **State Persistence:** Second input saves mode preference to localStorage
- **Command Aliases:** Try "bill" (alias for "invoice"), "new" (alias for "create")

**Keyboard Shortcuts:**
- `Cmd/Ctrl + Enter`: Submit
- `Cmd/Ctrl + ↑/↓`: Navigate history
- `Escape`: Clear command or close selector
- `Shift + Enter`: New line in AI mode

**Three Demo Configurations:**
1. Basic (no persistence, no history)
2. Full featured (history + persistence)
3. Starting in Token mode

---

## Testing Commands Summary

### Run All Components
```bash
# In separate terminals:
cd components/entity-tag && trunk serve &
cd components/token-autocomplete && trunk serve &
cd components/entity-combobox && trunk serve &
cd components/simple-form && trunk serve &
cd components/command-entity-input && trunk serve &
cd components/mixed-input && trunk serve &
cd components/dual-mode-input && trunk serve &
```

### Build All Components
```bash
# From components directory
for dir in */; do
    echo "Building $dir"
    cd "$dir" && trunk build && cd ..
done
```

## Component Dependencies
All components use:
- Leptos 0.7 (reactive framework)
- WASM target (runs in browser)
- Trunk (build tool)
- TaxTalk UI Core (shared utilities)

## Common Issues & Solutions

### Port Already in Use
Each component has a unique port (8081-8088). If a port is busy:
```bash
# Kill the process using the port
lsof -ti:8081 | xargs kill -9
```

### Build Errors
```bash
# Clean and rebuild
trunk clean
cargo clean
trunk build
```

### Component Not Loading
- Check browser console for errors
- Ensure `trunk serve` is running
- Try hard refresh (Cmd+Shift+R)

### 8. ✅ Multi-Select-Combobox Component
**Location:** `/components/multi-select-combobox`  
**Port:** 8089  
**Purpose:** Multi-selection dropdown with virtual scrolling for large datasets

**How to Test:**
```bash
cd components/multi-select-combobox
trunk serve
# Open http://localhost:8089
```

**Features to Test:**
- Multiple selection with visual tags
- Virtual scrolling for 100+ items (automatically enabled for lists > 50 items)
- Selection limits (optional max selections)
- Keyboard navigation (arrow keys, Enter, Escape, Backspace)
- Search/filter functionality
- Initial pre-selected items
- Clear all button
- Tag removal with X button

**Key Interactions:**
- Type to filter available options
- Click or Enter to add selections
- Backspace removes last selection when input is empty
- Click X on tags to remove specific selections
- Clear button removes all selections

---

## Next Components to Build (Pending)
- [ ] Smart-Suggestions (ML-powered suggestions)
- [ ] Quick-Actions (contextual action menu)
- [ ] Command-Palette (global command interface)
- [ ] Inline-Confirmation (inline validation states)
- [ ] Chat Component (real-time messaging)
- [ ] Guided-Chat (conversation flow management)

## Component Features Matrix

| Component | Keyboard Nav | API Integration | Validation | State Persistence | Philippine Context |
|-----------|-------------|-----------------|------------|-------------------|-------------------|
| Entity-Tag | ❌ | ❌ | ❌ | ❌ | ✅ |
| Token-Autocomplete | ✅ | ❌ | ❌ | ❌ | ✅ |
| Entity-Combobox | ✅ | ✅ | ✅ | ✅ | ✅ |
| Simple-Form | ✅ | ❌ | ✅ | ❌ | ✅ |
| Command-Entity-Input | ✅ | ❌ | ✅ | ❌ | ✅ |
| Mixed-Input | ✅ | ❌ | ❌ | ❌ | ✅ |
| Dual-Mode-Input | ✅ | ❌ | ✅ | ✅ | ✅ |
| Multi-Select-Combobox | ✅ | ❌ | ✅ | ❌ | ✅ |

## Performance Benchmarks
All components target:
- < 100ms initial render
- < 16ms re-render (60fps)
- < 50KB bundle size per component
- < 10MB memory footprint

## Documentation
Each component includes:
- README.md with API documentation
- Inline code comments
- Demo page with examples
- Type definitions
- CSS with customization variables

---

**Note:** All components are built with Philippine business context in mind, including TIN validation, peso currency formatting, and local business terminology.