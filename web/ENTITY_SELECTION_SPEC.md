# Entity Selection & Tagged Input Specification

## Overview
This document specifies the implementation of an enhanced entity selection system that supports both single and multiple entity selection, along with a mixed text/tag input field for the main chat interface.

## 1. Component Architecture

### 1.1 Core Components

#### A. `MultiSelectEntityCombobox` Component
**Purpose**: Allow users to select multiple entities with tag-like interface
**Location**: `/web/src/components/multi_select_combobox.rs`

**Features**:
- [x] Search and filter entities
- [ ] Display selected entities as removable tags
- [ ] Support for maximum selection limit (from manifest)
- [ ] Keyboard navigation (Arrow keys, Enter to select, Backspace to remove)
- [ ] "Done" button and Cmd+Enter shortcut to confirm
- [ ] Prevent duplicate selections
- [ ] Show/hide already selected items from search results

**Props**:
```rust
#[component]
pub fn MultiSelectEntityCombobox(
    entity_type: String,
    prompt: String,
    max_selections: Option<usize>, // None = unlimited, Some(n) = max n items
    initial_selections: Vec<Entity>,
    on_confirm: WriteSignal<Vec<Entity>>, // Called when Done clicked
    on_cancel: WriteSignal<bool>,
)
```

#### B. `SingleSelectEntityCombobox` Component (Existing, needs refactor)
**Purpose**: Select exactly one entity
**Location**: `/web/src/components/entity_combobox.rs`

**Enhancements needed**:
- [ ] Consistent UI with MultiSelect variant
- [ ] Better keyboard navigation
- [ ] Visual consistency with multi-select

#### C. `MixedInput` Component
**Purpose**: Main chat input that supports both text and entity tags
**Location**: `/web/src/components/mixed_input.rs`

**Features**:
- [ ] Mix plain text with entity tags
- [ ] Show entity tags inline as chips
- [ ] Support @ mentions to trigger entity selection
- [ ] Backspace to delete tags when cursor is after them
- [ ] Click on tag to edit/remove
- [ ] Maintain cursor position correctly

**Props**:
```rust
#[component]
pub fn MixedInput(
    value: Signal<String>,
    set_value: WriteSignal<String>,
    entities: Signal<Vec<EntityToken>>,
    set_entities: WriteSignal<Vec<EntityToken>>,
    on_submit: WriteSignal<String>,
    placeholder: String,
)
```

### 1.2 Data Structures

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityToken {
    pub id: String,
    pub entity_type: String,
    pub display_name: String,
    pub position: usize, // Position in the input text
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug)]
pub struct SelectionState {
    pub selected: Vec<Entity>,
    pub search_query: String,
    pub filtered_results: Vec<Entity>,
    pub focused_index: usize,
}
```

## 2. User Interaction Flow

### 2.1 Multi-Select Flow

1. **Opening the modal**
   - User types `@product` in main input
   - MultiSelectEntityCombobox opens

2. **Searching and selecting**
   - User types "laptop" → Shows filtered results
   - User presses Enter or clicks → Adds "Laptop Computer" as tag
   - Search field clears, showing remaining products
   - Already selected "Laptop Computer" is hidden from results

3. **Managing selections**
   - Selected items show as tags with × button
   - Click × or press Backspace when focused to remove
   - Visual indicator when max selections reached

4. **Completing selection**
   - Click "Done" button or press Cmd+Enter
   - Returns to main chat with entities added to input

### 2.2 Single-Select Flow (existing, simplified)
1. User types `@client`
2. SingleSelectEntityCombobox opens
3. User searches and selects one client
4. Modal closes immediately, entity added to input

### 2.3 Mixed Input Behavior

```
User types: "Create invoice for @[ABC Corp] with @[Laptop] and @[Office Chair] total 50000"
                                  ^^^^^^^^^^      ^^^^^^^^      ^^^^^^^^^^^^^
                                  Entity tag      Entity tag    Entity tag
```

## 3. Visual Design

### 3.1 Multi-Select Combobox Layout

```
┌─────────────────────────────────────────┐
│ Select Product                          │
│ Select products for the invoice         │
├─────────────────────────────────────────┤
│ Selected (2/5):                         │
│ [Laptop Computer ×] [Office Chair ×]    │
├─────────────────────────────────────────┤
│ 🔍 Search products...                   │
├─────────────────────────────────────────┤
│ □ Printer Paper                         │
│   A4 size, 500 sheets                   │
│ □ Consulting Service                    │
│   Professional services per hour        │
├─────────────────────────────────────────┤
│                    [Cancel] [Done (2)]   │
└─────────────────────────────────────────┘
```

### 3.2 Mixed Input Field

```
┌─────────────────────────────────────────┐
│ Create invoice for [ABC Corp ×] with    │
│ [Laptop ×] [Office Chair ×] total |     │
└─────────────────────────────────────────┘
```

## 4. Implementation Tasks

### Phase 1: Multi-Select Combobox ✅ COMPLETED
- [x] Create `MultiSelectEntityCombobox` component
- [x] Implement tag display for selected items
- [x] Add selection management (add/remove)
- [x] Implement max selection limit
- [x] Hide selected items from search results
- [x] Add "Done" button and Cmd+Enter handler
- [x] Add selection counter (2/5 selected)

### Phase 2: Refactor Single-Select
- [x] Update UI to match multi-select styling
- [x] Ensure consistent keyboard navigation
- [x] Test integration with existing code

### Phase 3: Mixed Input Component ✅ COMPLETED
- [x] Create `MixedInput` component using textarea with tags
- [x] Implement entity tag rendering above input
- [x] Handle keyboard navigation (backspace to remove)
- [x] Implement @ mention detection and positioning
- [x] Handle backspace/delete for tags
- [x] Maintain proper text/entity serialization

### Phase 4: Integration ✅ COMPLETED
- [x] Update `guided_chat.rs` to use new components
- [x] Modify entity selection logic for cardinality
- [x] Detect cardinality from metadata and show appropriate selector
- [x] Handle both single and multi-entity submission

### Phase 5: Polish & Testing
- [ ] Keyboard accessibility (Tab navigation, ARIA labels)
- [ ] Loading states and error handling
- [ ] Performance optimization for large entity lists
- [ ] Cross-browser testing
- [ ] Mobile responsiveness

## 5. Technical Considerations

### 5.1 State Management
- Use Leptos signals for reactive state
- Maintain separate state for selected items and search results
- Sync state between parent and child components

### 5.2 Keyboard Shortcuts
- **Enter**: Select focused item
- **Backspace**: Remove last tag (when search is empty)
- **Arrow Up/Down**: Navigate results
- **Escape**: Cancel selection
- **Cmd/Ctrl+Enter**: Confirm multi-selection
- **Tab**: Navigate between elements

### 5.3 Performance
- Virtualize long lists (>100 items)
- Debounce search input (200ms)
- Lazy load entity details
- Cache search results

### 5.4 Accessibility
- ARIA labels for screen readers
- Focus management
- Keyboard-only navigation
- High contrast mode support

## 6. API Integration

### 6.1 Manifest Enhancement
```json
{
  "tokens": {
    "product": {
      "type": "entity",
      "entity_type": "product",
      "cardinality": "multiple",
      "max_selections": 10,
      "required": false
    },
    "client": {
      "type": "entity", 
      "entity_type": "client",
      "cardinality": "single",
      "required": true
    }
  }
}
```

### 6.2 Entity Selection Response
```rust
#[derive(Serialize, Deserialize)]
struct EntitySelectionComplete {
    token_type: String,
    selected_entities: Vec<String>, // Entity IDs
    cardinality: String,
}
```

## 7. Testing Scenarios

### 7.1 Multi-Select Tests
- [ ] Select maximum allowed items
- [ ] Try to exceed maximum
- [ ] Remove items and re-add
- [ ] Search with no results
- [ ] Keyboard-only selection
- [ ] Very long entity names

### 7.2 Mixed Input Tests
- [ ] Type text before, between, and after tags
- [ ] Delete tags with backspace
- [ ] Copy/paste with tags
- [ ] Undo/redo operations
- [ ] Very long input with many tags

## 8. Success Criteria

1. **User Experience**
   - Intuitive tag-based multi-selection
   - Clear visual feedback for selections
   - Smooth keyboard navigation
   - Fast search and filtering

2. **Technical**
   - No memory leaks
   - <100ms response time for searches
   - Proper cleanup on unmount
   - Consistent state management

3. **Accessibility**
   - WCAG 2.1 AA compliance
   - Full keyboard navigation
   - Screen reader support

## 9. Future Enhancements

- [ ] Drag and drop to reorder tags
- [ ] Bulk operations (select all, clear all)
- [ ] Recent selections memory
- [ ] Smart suggestions based on context
- [ ] Inline entity preview on hover
- [ ] Custom entity renderers

## 10. Implementation Summary

### Completed Components

1. **MultiSelectEntityCombobox** (`/web/src/components/multi_select_combobox.rs`)
   - Full tag-based multi-selection with visual feedback
   - Maximum selection limits with counter
   - Search filtering that hides already selected items
   - Keyboard navigation and Cmd+Enter confirmation
   - Color-coded entity type badges

2. **MixedInput** (`/web/src/components/mixed_input.rs`)
   - Textarea with entity tags displayed above
   - Backspace to remove tags when input is empty
   - @ mention detection for triggering entity selection
   - Visual tag display with remove buttons

3. **Integration Updates** (`/web/src/components/guided_chat.rs`)
   - Automatic cardinality detection from entity metadata
   - Conditional rendering of single vs multi-select combobox
   - Support for comma-separated entity IDs for multiple selections
   - Proper state management for both selection types

### Key Features Implemented

- **Multi-Select Flow**: Users can select multiple products/entities, see them as tags, remove individually, and confirm all at once
- **Visual Feedback**: Selection counter shows "Selected (3/10)", disabled state when max reached
- **Keyboard Navigation**: Full keyboard support with Arrow keys, Enter, Escape, and Cmd+Enter
- **Smart Filtering**: Already selected items are automatically hidden from search results
- **Entity Tags**: Color-coded tags with entity type-specific colors (blue for clients, purple for products, etc.)

---

**Status**: ✅ COMPLETED
**Last Updated**: 2024-12-06
**Owner**: Development Team