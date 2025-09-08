use crate::types::*;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

/// Search engine for commands
pub struct CommandSearchEngine {
    matcher: SkimMatcherV2,
    config: CommandPaletteConfig,
}

impl CommandSearchEngine {
    pub fn new(config: CommandPaletteConfig) -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            config,
        }
    }
    
    /// Search commands based on query
    pub fn search(&self, commands: &[Command], query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            // Return all commands with default score
            return commands
                .iter()
                .take(self.config.max_results)
                .map(|cmd| SearchResult {
                    command: cmd.clone(),
                    score: 1.0,
                    matched_fields: vec![],
                })
                .collect();
        }
        
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        for command in commands {
            let mut best_score = 0i64;
            let mut matched_fields = Vec::new();
            
            if self.config.enable_fuzzy_search {
                // Fuzzy match on label (highest priority)
                if let Some(score) = self.matcher.fuzzy_match(&command.label, query) {
                    if score > best_score {
                        best_score = score;
                        matched_fields.push("label".to_string());
                    }
                }
                
                // Fuzzy match on description
                if let Some(score) = self.matcher.fuzzy_match(&command.description, query) {
                    if score > best_score / 2 {
                        best_score = best_score.max(score);
                        if !matched_fields.contains(&"description".to_string()) {
                            matched_fields.push("description".to_string());
                        }
                    }
                }
                
                // Fuzzy match on keywords
                for keyword in &command.keywords {
                    if let Some(score) = self.matcher.fuzzy_match(keyword, query) {
                        if score > best_score / 2 {
                            best_score = best_score.max(score);
                            if !matched_fields.contains(&"keywords".to_string()) {
                                matched_fields.push("keywords".to_string());
                            }
                        }
                    }
                }
                
                // Fuzzy match on category
                let category_str = command.category.to_string();
                if let Some(score) = self.matcher.fuzzy_match(&category_str, query) {
                    if score > best_score / 3 {
                        best_score = best_score.max(score / 2);
                        if !matched_fields.contains(&"category".to_string()) {
                            matched_fields.push("category".to_string());
                        }
                    }
                }
            } else {
                // Exact substring matching
                if command.label.to_lowercase().contains(&query_lower) {
                    best_score = 100;
                    matched_fields.push("label".to_string());
                } else if command.description.to_lowercase().contains(&query_lower) {
                    best_score = 50;
                    matched_fields.push("description".to_string());
                } else if command.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower)) {
                    best_score = 30;
                    matched_fields.push("keywords".to_string());
                } else if command.category.to_string().to_lowercase().contains(&query_lower) {
                    best_score = 20;
                    matched_fields.push("category".to_string());
                }
            }
            
            if best_score > 0 {
                results.push(SearchResult {
                    command: command.clone(),
                    score: best_score as f64,
                    matched_fields,
                });
            }
        }
        
        // Sort by score (highest first)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        // Limit results
        results.truncate(self.config.max_results);
        
        results
    }
    
    /// Parse natural language query
    pub fn parse_nlp(&self, query: &str) -> Option<NLPResult> {
        // This is a simplified NLP parser - in production, you'd use an actual NLP service
        
        let query_lower = query.to_lowercase();
        let mut parameters = Vec::new();
        let mut confidence = 0.0;
        let mut matched_command = None;
        
        // Check for invoice-related queries
        if query_lower.contains("invoice") || query_lower.contains("bill") {
            confidence = 0.7;
            matched_command = Some("create_invoice");
            
            // Extract amount if present
            if let Some(amount) = extract_amount(&query_lower) {
                parameters.push(NLPParameter {
                    name: "Amount".to_string(),
                    value: format!("₱{}", amount),
                    param_type: "currency".to_string(),
                    editable: true,
                });
                confidence += 0.1;
            }
            
            // Extract client name if present
            if query_lower.contains(" for ") || query_lower.contains(" to ") {
                let words: Vec<&str> = query.split_whitespace().collect();
                if let Some(pos) = words.iter().position(|&w| w.to_lowercase() == "for" || w.to_lowercase() == "to") {
                    if pos < words.len() - 1 {
                        let client = words[pos + 1..].join(" ");
                        parameters.push(NLPParameter {
                            name: "Client".to_string(),
                            value: client,
                            param_type: "entity".to_string(),
                            editable: true,
                        });
                        confidence += 0.1;
                    }
                }
            }
        }
        
        // Check for payment-related queries
        else if query_lower.contains("payment") || query_lower.contains("pay") || query_lower.contains("received") {
            confidence = 0.65;
            matched_command = Some("record_payment");
            
            if let Some(amount) = extract_amount(&query_lower) {
                parameters.push(NLPParameter {
                    name: "Amount".to_string(),
                    value: format!("₱{}", amount),
                    param_type: "currency".to_string(),
                    editable: true,
                });
                confidence += 0.15;
            }
        }
        
        // Check for expense-related queries
        else if query_lower.contains("expense") || query_lower.contains("spent") || query_lower.contains("bought") {
            confidence = 0.6;
            matched_command = Some("add_expense");
            
            if let Some(amount) = extract_amount(&query_lower) {
                parameters.push(NLPParameter {
                    name: "Amount".to_string(),
                    value: format!("₱{}", amount),
                    param_type: "currency".to_string(),
                    editable: true,
                });
                confidence += 0.2;
            }
        }
        
        // Check for VAT/tax-related queries
        else if query_lower.contains("vat") || query_lower.contains("tax return") || query_lower.contains("bir") {
            confidence = 0.75;
            matched_command = Some("vat_return");
            
            // Extract month if present
            for month in &["january", "february", "march", "april", "may", "june", 
                          "july", "august", "september", "october", "november", "december"] {
                if query_lower.contains(month) {
                    parameters.push(NLPParameter {
                        name: "Month".to_string(),
                        value: capitalize_first(month),
                        param_type: "date".to_string(),
                        editable: true,
                    });
                    confidence += 0.1;
                    break;
                }
            }
        }
        
        if matched_command.is_some() && confidence > 0.5 {
            Some(NLPResult {
                command: None, // Will be filled by the component
                confidence,
                parameters,
                raw_text: query.to_string(),
            })
        } else {
            None
        }
    }
}

/// Extract amount from text
fn extract_amount(text: &str) -> Option<String> {
    // Look for patterns like "1000", "1,000", "1k", "₱1000"
    let patterns = [
        (r"\d+k", 1000.0),
        (r"\d+m", 1000000.0),
    ];
    
    // First check for k/m suffixes
    for (pattern, multiplier) in patterns {
        if let Some(captures) = regex_lite::Regex::new(pattern).ok()?.find(text) {
            let num_str = captures.as_str().trim_end_matches(char::is_alphabetic);
            if let Ok(num) = num_str.parse::<f64>() {
                return Some(format!("{:.2}", num * multiplier));
            }
        }
    }
    
    // Then check for regular numbers
    if let Some(captures) = regex_lite::Regex::new(r"\d+,?\d*").ok()?.find(text) {
        let num_str = captures.as_str().replace(',', "");
        if let Ok(_) = num_str.parse::<f64>() {
            return Some(num_str);
        }
    }
    
    None
}

/// Capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

// Simple regex replacement since we can't use full regex in WASM
mod regex_lite {
    pub struct Regex {
        pattern: String,
    }
    
    impl Regex {
        pub fn new(pattern: &str) -> Result<Self, ()> {
            Ok(Self {
                pattern: pattern.to_string(),
            })
        }
        
        pub fn find(&self, text: &str) -> Option<Match> {
            // Very simplified pattern matching for demo
            if self.pattern.contains(r"\d+k") {
                // Find number followed by k
                let bytes = text.as_bytes();
                for i in 0..bytes.len() {
                    if bytes[i].is_ascii_digit() {
                        let mut j = i;
                        while j < bytes.len() && bytes[j].is_ascii_digit() {
                            j += 1;
                        }
                        if j < bytes.len() && bytes[j] == b'k' {
                            return Some(Match {
                                text: text[i..=j].to_string(),
                            });
                        }
                    }
                }
            } else if self.pattern.contains(r"\d+,?\d*") {
                // Find number with optional comma
                let bytes = text.as_bytes();
                for i in 0..bytes.len() {
                    if bytes[i].is_ascii_digit() {
                        let mut j = i;
                        while j < bytes.len() && (bytes[j].is_ascii_digit() || bytes[j] == b',') {
                            j += 1;
                        }
                        return Some(Match {
                            text: text[i..j].to_string(),
                        });
                    }
                }
            }
            None
        }
    }
    
    pub struct Match {
        text: String,
    }
    
    impl Match {
        pub fn as_str(&self) -> &str {
            &self.text
        }
    }
}