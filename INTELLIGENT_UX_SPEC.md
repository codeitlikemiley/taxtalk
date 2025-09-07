# Intelligent Command Input UX Specification

## Executive Summary
Transform the current rigid guided session flow into an intelligent, flexible command input system that supports:
- Quick action buttons for common tokens
- @ token autocomplete with inline forms
- Natural language parsing with entity matching
- Inline confirmations with smart defaults
- Mini-session mode for focused input

## Current Problems
1. **Rigid Flow**: Users must answer questions sequentially
2. **Poor Discovery**: Users don't know what tokens are available
3. **Inefficient Input**: Too many steps for simple commands
4. **No Intelligence**: System doesn't understand natural language
5. **Modal Hell**: Too many popups disrupting flow

## Proposed Solution

### 1. Three Input Modes

#### A. Quick Actions Mode
- **Location**: `web/src/components/guided_chat.rs:500-534`
- Display clickable quick action buttons based on missing tokens
- When clicked, enter mini-session mode for that specific token
- Example buttons:
  - "Select Client" → Opens inline entity selector
  - "Enter Amount" → Shows numeric input with validation
  - "Add Items" → Opens inline item builder

#### B. @ Token Autocomplete Mode
- **Location**: `web/src/components/autocomplete_simple.rs:153-187`
- Type @ to see available tokens for current context
- Tab/Enter to select token
- Shows appropriate input widget inline:
  - `@client` → Entity selector dropdown
  - `@amount` → Numeric input with currency
  - `@date` → Date picker
  - `@items` → Inline item builder

#### C. Natural Language Mode
- **Location**: New file `core/src/nlp/parser.rs`
- Parse complete sentences like "invoice for ABC Corp 5000 pesos"
- Extract entities and values intelligently
- Confirm understanding with user

### 2. Component Architecture

#### A. Quick Action Bar Component
```rust
// web/src/components/quick_actions.rs
#[component]
pub fn QuickActionBar(
    missing_tokens: Signal<Vec<MissingToken>>,
    on_action_click: WriteSignal<ActiveToken>,
) -> impl IntoView {
    // Render buttons for each missing token
    // Click enters mini-session for that token
}
```

#### B. Token Input Widget
```rust
// web/src/components/token_input.rs
#[component]
pub fn TokenInput(
    token: MissingToken,
    on_submit: WriteSignal<TokenValue>,
    on_cancel: WriteSignal<bool>,
) -> impl IntoView {
    // Renders appropriate input based on token.input_type
    // Inline, non-modal interface
}
```

#### C. NLP Parser Service
```rust
// core/src/nlp/parser.rs
pub struct NLPParser {
    tokenizer: Tokenizer,
    entity_matcher: EntityMatcher,
}

impl NLPParser {
    pub fn parse_command(&self, text: &str) -> ParsedCommand {
        // Extract tokens from natural language
        // Match entities against database
        // Return structured command
    }
}
```

### 3. Implementation Plan

#### Phase 1: Quick Actions (2 days)
**Files to modify:**
- `web/src/components/guided_chat.rs:132-260` - Add quick action handling
- `web/src/components/mod.rs:7` - Register new components
- Create `web/src/components/quick_actions.rs` - Quick action bar
- Create `web/src/components/token_input.rs` - Inline token inputs

**Key changes:**
1. Replace text prompts with action buttons
2. Implement mini-session mode
3. Add inline input widgets

#### Phase 2: @ Token System (2 days)
**Files to modify:**
- `web/src/components/autocomplete_simple.rs:153-187` - Enhance detection
- `web/src/components/guided_chat.rs:261-287` - Handle @ tokens
- `core/src/validation/token_schema.rs:63-150` - Add token hints

**Key changes:**
1. Show available tokens on @ press
2. Tab/Enter completion
3. Inline form rendering

#### Phase 3: NLP Parser (3 days)
**Files to create:**
- `core/src/nlp/mod.rs` - NLP module
- `core/src/nlp/parser.rs` - Natural language parser
- `core/src/nlp/entity_matcher.rs` - Fuzzy entity matching
- `server/src/api/nlp.rs` - NLP endpoints

**Key changes:**
1. Parse natural language commands
2. Fuzzy match entities
3. Confirmation prompts

#### Phase 4: Inline Confirmations (1 day)
**Files to modify:**
- `web/src/components/guided_chat.rs:695-743` - Add confirmation
- Create `web/src/components/confirmation.rs` - Inline confirmation

**Key changes:**
1. Show parsed understanding
2. Y/N quick confirm
3. Edit before submit

### 4. User Flow Examples

#### Example 1: Quick Actions
```
User: create invoice
Bot: [Quick Actions Bar]
     [Select Client] [Enter Amount] [Add Items] [Set Due Date]
User: *clicks [Select Client]*
Bot: [Inline Client Selector - shows top 5 + search]
User: *selects "ABC Corporation"*
Bot: ✓ Client: ABC Corporation
     [Enter Amount] [Add Items] [Set Due Date]
```

#### Example 2: @ Token Autocomplete
```
User: create invoice @cli[TAB]
Bot: [Autocomplete: @client]
User: @client ABC[ENTER]
Bot: [Shows matches: ABC Corporation, ABC Trading]
User: *selects ABC Corporation*
Bot: create invoice @client ABC Corporation @amount [cursor here]
```

#### Example 3: Natural Language
```
User: invoice ABC Corp 5000 pesos due next week
Bot: I understand you want to create:
     • Invoice for: ABC Corporation ✓
     • Amount: ₱5,000.00
     • Due date: December 13, 2024
     
     [Confirm] [Edit] [Cancel]
User: [ENTER or clicks Confirm]
Bot: ✅ Invoice #INV-2024-001 created
```

### 5. Technical Implementation Details

#### A. Token Detection Enhancement
```rust
// core/src/validation/validator.rs:169-210
pub fn detect_token_context(text: &str, cursor: usize) -> TokenContext {
    // Detect if user is typing a token
    // Return available tokens for context
    // Support tab completion
}
```

#### B. Mini-Session State
```rust
// web/src/components/guided_chat.rs:85-95
#[derive(Clone, Debug)]
struct MiniSession {
    token: MissingToken,
    input_mode: InputMode,
    partial_value: Option<String>,
}
```

#### C. Entity Fuzzy Matching
```rust
// core/src/nlp/entity_matcher.rs
pub fn fuzzy_match_entity(
    text: &str, 
    entity_type: &str,
    threshold: f32
) -> Vec<EntityMatch> {
    // Use Levenshtein distance
    // Consider phonetic matching
    // Return ranked matches
}
```

### 6. Database Schema Updates

#### A. Token Hints Table
```sql
-- migrations/004_token_hints.sql
CREATE TABLE token_hints (
    id TEXT PRIMARY KEY,
    plugin_id TEXT NOT NULL,
    action TEXT NOT NULL,
    token_name TEXT NOT NULL,
    hint_text TEXT NOT NULL,
    example_value TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### B. Common Phrases Table
```sql
-- migrations/005_common_phrases.sql
CREATE TABLE common_phrases (
    id TEXT PRIMARY KEY,
    phrase TEXT NOT NULL,
    token_mapping TEXT NOT NULL, -- JSON
    usage_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 7. API Endpoints

#### A. NLP Parse Endpoint
```rust
// server/src/main.rs:890-920
async fn parse_natural_language(
    Json(payload): Json<NLPRequest>
) -> Result<Json<ParsedCommand>, StatusCode> {
    // Parse natural language
    // Return structured command
}
```

#### B. Token Hints Endpoint
```rust
// server/src/main.rs:921-950
async fn get_token_hints(
    Query(params): Query<TokenHintParams>
) -> Result<Json<Vec<TokenHint>>, StatusCode> {
    // Return available tokens for context
}
```

### 8. UI Component Styling

#### A. Quick Action Buttons
```css
/* web/styles/quick_actions.css */
.quick-action-btn {
    @apply px-4 py-2 bg-blue-100 hover:bg-blue-200 
           text-blue-700 rounded-lg transition-all
           border border-blue-300 cursor-pointer;
}
```

#### B. Inline Token Input
```css
/* web/styles/token_input.css */
.token-input-inline {
    @apply inline-flex items-center bg-gray-50 
           rounded-lg px-3 py-1 mx-1 border-2 
           border-blue-400 focus-within:border-blue-500;
}
```

### 9. Testing Strategy

#### A. Unit Tests
- Test NLP parser with various phrases
- Test entity fuzzy matching
- Test token detection

#### B. Integration Tests
- Test quick action flow
- Test @ token completion
- Test natural language understanding

#### C. E2E Tests
- Complete invoice creation via quick actions
- Complete invoice creation via natural language
- Mixed mode input

### 10. Success Metrics

1. **Input Speed**: 70% reduction in keystrokes
2. **Error Rate**: 50% fewer validation errors
3. **Discovery**: 80% of users find tokens without help
4. **Completion**: 90% task completion rate
5. **Satisfaction**: 4.5+ star user rating

## Implementation Priority

1. **High Priority** (Week 1)
   - Quick action buttons
   - @ token autocomplete
   - Basic inline inputs

2. **Medium Priority** (Week 2)
   - NLP parser
   - Fuzzy entity matching
   - Inline confirmations

3. **Low Priority** (Week 3)
   - Advanced NLP features
   - Learning from usage
   - Personalization

## Files to Modify/Create

### Core Changes
1. `web/src/components/guided_chat.rs` - Main chat component
2. `web/src/components/quick_actions.rs` - NEW: Quick action bar
3. `web/src/components/token_input.rs` - NEW: Inline token inputs
4. `web/src/components/confirmation.rs` - NEW: Inline confirmation
5. `core/src/nlp/mod.rs` - NEW: NLP module
6. `core/src/nlp/parser.rs` - NEW: Natural language parser
7. `core/src/nlp/entity_matcher.rs` - NEW: Fuzzy entity matching
8. `server/src/api/nlp.rs` - NEW: NLP API endpoints
9. `core/src/validation/validator.rs` - Enhanced token detection
10. `web/src/components/autocomplete_simple.rs` - Enhanced @ detection

### Database Changes
1. `migrations/004_token_hints.sql` - Token hints table
2. `migrations/005_common_phrases.sql` - Common phrases table

### Configuration
1. `core/src/validation/token_schema.rs` - Add token metadata
2. `server/src/main.rs` - Add new endpoints

## Next Steps

1. Review and approve this specification
2. Create detailed tasks for each component
3. Begin with Phase 1: Quick Actions
4. Iterate based on user feedback
5. Deploy incrementally

This specification provides a complete overhaul of the command input system, making it more intelligent, efficient, and user-friendly.