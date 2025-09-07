use anyhow::Result;

// Use the same Token enum as router
use crate::router::Token;

pub struct Tokenizer {}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>> {
        // For now, simple tokenization
        // In the real implementation, this would use the factory's pattern matching
        let words: Vec<String> = input.split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        let mut tokens = Vec::new();
        
        for word in words {
            // Try to create appropriate token types
            if word.starts_with('@') {
                // Entity reference
                tokens.push(Token::EntityRef {
                    entity_type: "unknown".to_string(),
                    identifier: word.trim_start_matches('@').to_string(),
                });
            } else if word.parse::<f64>().is_ok() {
                // Amount
                tokens.push(Token::Amount {
                    value: word.parse()?,
                    currency: "PHP".to_string(),
                });
            } else if is_verb(&word) {
                // Verb/Command
                tokens.push(Token::Verb(word.clone()));
            } else {
                // Generic text
                tokens.push(Token::Text(word));
            }
        }
        
        Ok(tokens)
    }
}

fn is_verb(word: &str) -> bool {
    matches!(word.to_lowercase().as_str(),
        "create" | "add" | "new" | "invoice" | "pay" | "payment" | 
        "receive" | "refund" | "register" | "update" | "list" |
        "bill" | "charge" | "collect"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokenization() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("create invoice for @client 1000").unwrap();
        assert_eq!(tokens.len(), 5);
    }
}