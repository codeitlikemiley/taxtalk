use crate::token::traits::{Token, TokenPattern};

/// Pattern implementations for matching tokens
#[derive(Debug)]
pub struct TypePattern {
    pub token_type: String,
}

impl TypePattern {
    pub fn new(token_type: impl Into<String>) -> Self {
        Self {
            token_type: token_type.into(),
        }
    }
}

impl TokenPattern for TypePattern {
    fn matches(&self, token: &dyn Token) -> bool {
        token.token_type() == self.token_type
    }
    
    fn describe(&self) -> String {
        format!("Type: {}", self.token_type)
    }
}

/// Pattern for matching multiple types
#[derive(Debug)]
pub struct MultiTypePattern {
    pub token_types: Vec<String>,
}

impl MultiTypePattern {
    pub fn new(token_types: Vec<String>) -> Self {
        Self { token_types }
    }
}

impl TokenPattern for MultiTypePattern {
    fn matches(&self, token: &dyn Token) -> bool {
        self.token_types.contains(&token.token_type().to_string())
    }
    
    fn describe(&self) -> String {
        format!("Types: {}", self.token_types.join(", "))
    }
}

/// Composite pattern for complex matching
#[derive(Debug)]
pub struct CompositePattern {
    pub patterns: Vec<Box<dyn TokenPattern>>,
    pub mode: PatternMode,
}

#[derive(Debug, Clone, Copy)]
pub enum PatternMode {
    All,  // All patterns must match
    Any,  // Any pattern must match
    None, // No pattern must match
}

impl CompositePattern {
    pub fn all(patterns: Vec<Box<dyn TokenPattern>>) -> Self {
        Self {
            patterns,
            mode: PatternMode::All,
        }
    }
    
    pub fn any(patterns: Vec<Box<dyn TokenPattern>>) -> Self {
        Self {
            patterns,
            mode: PatternMode::Any,
        }
    }
    
    pub fn none(patterns: Vec<Box<dyn TokenPattern>>) -> Self {
        Self {
            patterns,
            mode: PatternMode::None,
        }
    }
}

impl TokenPattern for CompositePattern {
    fn matches(&self, token: &dyn Token) -> bool {
        match self.mode {
            PatternMode::All => self.patterns.iter().all(|p| p.matches(token)),
            PatternMode::Any => self.patterns.iter().any(|p| p.matches(token)),
            PatternMode::None => !self.patterns.iter().any(|p| p.matches(token)),
        }
    }
    
    fn describe(&self) -> String {
        format!("{:?} of: {}", self.mode, 
            self.patterns.iter()
                .map(|p| p.describe())
                .collect::<Vec<_>>()
                .join(", "))
    }
}

/// Pattern for matching tokens by metadata
#[derive(Debug)]
pub struct MetadataPattern {
    pub source_plugin: Option<String>,
    pub min_confidence: Option<f32>,
    pub required_tags: Vec<String>,
}

impl Default for MetadataPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataPattern {
    pub fn new() -> Self {
        Self {
            source_plugin: None,
            min_confidence: None,
            required_tags: vec![],
        }
    }
    
    pub fn with_plugin(mut self, plugin: String) -> Self {
        self.source_plugin = Some(plugin);
        self
    }
    
    pub fn with_min_confidence(mut self, confidence: f32) -> Self {
        self.min_confidence = Some(confidence);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.required_tags = tags;
        self
    }
}

impl TokenPattern for MetadataPattern {
    fn matches(&self, token: &dyn Token) -> bool {
        let metadata = token.metadata();
        
        // Check source plugin
        if let Some(ref plugin) = self.source_plugin {
            if metadata.source_plugin.as_ref() != Some(plugin) {
                return false;
            }
        }
        
        // Check confidence
        if let Some(min_conf) = self.min_confidence {
            if metadata.confidence < min_conf {
                return false;
            }
        }
        
        // Check tags
        for tag in &self.required_tags {
            if !metadata.tags.contains(tag) {
                return false;
            }
        }
        
        true
    }
    
    fn describe(&self) -> String {
        let mut parts = vec![];
        
        if let Some(ref plugin) = self.source_plugin {
            parts.push(format!("plugin={plugin}"));
        }
        
        if let Some(conf) = self.min_confidence {
            parts.push(format!("confidence>={conf}"));
        }
        
        if !self.required_tags.is_empty() {
            parts.push(format!("tags={}", self.required_tags.join(",")));
        }
        
        format!("Metadata[{}]", parts.join(", "))
    }
}

/// Pattern for matching by raw text content
#[derive(Debug)]
pub struct TextPattern {
    pub pattern: regex::Regex,
}

impl TextPattern {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: regex::Regex::new(pattern)?,
        })
    }
}

impl TokenPattern for TextPattern {
    fn matches(&self, token: &dyn Token) -> bool {
        self.pattern.is_match(token.raw_text())
    }
    
    fn describe(&self) -> String {
        format!("Text matches: {}", self.pattern.as_str())
    }
}

/// Pattern for matching tokens that can compose with specific types
#[derive(Debug)]
pub struct ComposablePattern {
    pub with_types: Vec<String>,
}

impl ComposablePattern {
    pub fn new(with_types: Vec<String>) -> Self {
        Self { with_types }
    }
}

impl TokenPattern for ComposablePattern {
    fn matches(&self, token: &dyn Token) -> bool {
        // Create a dummy token of each type to check composability
        // This is a simplified check - in practice, you'd need actual instances
        for type_name in &self.with_types {
            // For now, we'll use the token's own composability logic
            // In a real implementation, you'd create instances of the target types
            if !token.can_compose_with(token) { // Placeholder logic
                return false;
            }
        }
        true
    }
    
    fn describe(&self) -> String {
        format!("Composable with: {}", self.with_types.join(", "))
    }
}