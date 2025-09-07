use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub action: Option<String>,
    pub plugin: Option<String>,
    pub tokens: Vec<ParsedToken>,
    pub confidence: f32,
    pub original_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedToken {
    pub name: String,
    pub value: String,
    pub token_type: TokenType,
    pub position: (usize, usize), // start, end in original text
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Entity(String),     // Entity type like "client", "invoice"
    Amount,
    Date,
    Text,
    Method,
    Boolean,
    Unknown,
}

pub struct NLPParser {
    action_patterns: HashMap<String, Vec<String>>,
    amount_pattern: Regex,
    date_patterns: HashMap<String, Regex>,
    entity_keywords: HashMap<String, Vec<String>>,
}

impl NLPParser {
    pub fn new() -> Self {
        let mut action_patterns = HashMap::new();
        
        // Invoice actions
        action_patterns.insert("create_invoice".to_string(), vec![
            "create invoice".to_string(),
            "new invoice".to_string(),
            "invoice for".to_string(),
            "bill to".to_string(),
            "charge".to_string(),
        ]);
        
        // Payment actions
        action_patterns.insert("record_payment".to_string(), vec![
            "payment from".to_string(),
            "received payment".to_string(),
            "paid by".to_string(),
            "payment of".to_string(),
            "collected".to_string(),
        ]);
        
        // Expense actions
        action_patterns.insert("record_expense".to_string(), vec![
            "bought".to_string(),
            "purchased".to_string(),
            "spent".to_string(),
            "paid for".to_string(),
            "expense for".to_string(),
        ]);
        
        let amount_pattern = Regex::new(r"(?i)(?:₱|php|pesos?|p)?\s*(\d{1,3}(?:,\d{3})*(?:\.\d{2})?)\s*(?:₱|php|pesos?|p)?").unwrap();
        
        let mut date_patterns = HashMap::new();
        date_patterns.insert("today".to_string(), 
            Regex::new(r"(?i)\b(today|now|immediate)\b").unwrap());
        date_patterns.insert("tomorrow".to_string(), 
            Regex::new(r"(?i)\b(tomorrow|next day)\b").unwrap());
        date_patterns.insert("days".to_string(), 
            Regex::new(r"(?i)\b(\d+)\s*days?\b").unwrap());
        date_patterns.insert("weeks".to_string(), 
            Regex::new(r"(?i)\b(\d+)\s*weeks?\b").unwrap());
        date_patterns.insert("months".to_string(), 
            Regex::new(r"(?i)\b(\d+)\s*months?\b").unwrap());
        date_patterns.insert("specific".to_string(), 
            Regex::new(r"(?i)\b(\d{1,2}[/-]\d{1,2}[/-]\d{2,4})\b").unwrap());
        
        let mut entity_keywords = HashMap::new();
        entity_keywords.insert("client".to_string(), vec![
            "for".to_string(),
            "to".to_string(),
            "client".to_string(),
            "customer".to_string(),
        ]);
        entity_keywords.insert("supplier".to_string(), vec![
            "from".to_string(),
            "supplier".to_string(),
            "vendor".to_string(),
        ]);
        
        Self {
            action_patterns,
            amount_pattern,
            date_patterns,
            entity_keywords,
        }
    }
    
    pub fn parse_command(&self, text: &str) -> ParsedCommand {
        let lower_text = text.to_lowercase();
        let mut tokens = Vec::new();
        let mut action = None;
        let mut plugin = None;
        
        // Detect action
        for (action_name, patterns) in &self.action_patterns {
            for pattern in patterns {
                if lower_text.contains(pattern) {
                    action = Some(action_name.clone());
                    
                    // Determine plugin from action
                    plugin = Some(match action_name.as_str() {
                        "create_invoice" => "invoice",
                        "record_payment" => "payment",
                        "record_expense" => "expense",
                        _ => "unknown",
                    }.to_string());
                    break;
                }
            }
            if action.is_some() {
                break;
            }
        }
        
        // Extract amounts
        if let Some(captures) = self.amount_pattern.captures(text) {
            if let Some(amount_match) = captures.get(1) {
                let amount_str = amount_match.as_str().replace(",", "");
                if let Ok(amount) = amount_str.parse::<f64>() {
                    tokens.push(ParsedToken {
                        name: "amount".to_string(),
                        value: amount.to_string(),
                        token_type: TokenType::Amount,
                        position: (amount_match.start(), amount_match.end()),
                        confidence: 0.95,
                    });
                }
            }
        }
        
        // Extract dates
        for (date_type, pattern) in &self.date_patterns {
            if let Some(captures) = pattern.captures(&lower_text) {
                let value = match date_type.as_str() {
                    "today" => chrono::Local::now().format("%Y-%m-%d").to_string(),
                    "tomorrow" => (chrono::Local::now() + chrono::Duration::days(1))
                        .format("%Y-%m-%d").to_string(),
                    "days" => {
                        if let Some(days_match) = captures.get(1) {
                            if let Ok(days) = days_match.as_str().parse::<i64>() {
                                (chrono::Local::now() + chrono::Duration::days(days))
                                    .format("%Y-%m-%d").to_string()
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    },
                    "specific" => {
                        if let Some(date_match) = captures.get(1) {
                            date_match.as_str().to_string()
                        } else {
                            continue;
                        }
                    },
                    _ => continue,
                };
                
                if let Some(match_info) = captures.get(0) {
                    tokens.push(ParsedToken {
                        name: "due_date".to_string(),
                        value,
                        token_type: TokenType::Date,
                        position: (match_info.start(), match_info.end()),
                        confidence: 0.9,
                    });
                }
            }
        }
        
        // Extract payment methods
        let payment_methods = vec![
            ("cash", vec!["cash", "tunai"]),
            ("bank_transfer", vec!["bank transfer", "transfer", "bank"]),
            ("gcash", vec!["gcash", "g-cash"]),
            ("maya", vec!["maya", "paymaya"]),
            ("check", vec!["check", "cheque"]),
            ("credit_card", vec!["credit card", "cc", "card"]),
        ];
        
        for (method_value, keywords) in payment_methods {
            for keyword in keywords {
                if lower_text.contains(keyword) {
                    if let Some(pos) = lower_text.find(keyword) {
                        tokens.push(ParsedToken {
                            name: "method".to_string(),
                            value: method_value.to_string(),
                            token_type: TokenType::Method,
                            position: (pos, pos + keyword.len()),
                            confidence: 0.85,
                        });
                        break;
                    }
                }
            }
        }
        
        // Extract boolean flags
        if lower_text.contains("tax exempt") || lower_text.contains("no tax") {
            tokens.push(ParsedToken {
                name: "tax_exempt".to_string(),
                value: "true".to_string(),
                token_type: TokenType::Boolean,
                position: (0, 0), // Would need to find actual position
                confidence: 0.9,
            });
        }
        
        // Extract entities (simplified - would use entity_matcher in real implementation)
        // This is a placeholder - actual implementation would use fuzzy matching
        let words: Vec<&str> = text.split_whitespace().collect();
        for i in 0..words.len() {
            if i > 0 {
                let prev_word = words[i - 1].to_lowercase();
                if prev_word == "for" || prev_word == "to" || prev_word == "client" {
                    // Assume next words might be client name
                    let mut entity_name = String::new();
                    for j in i..words.len().min(i + 3) {
                        if !entity_name.is_empty() {
                            entity_name.push(' ');
                        }
                        entity_name.push_str(words[j]);
                    }
                    
                    if !entity_name.is_empty() && !entity_name.chars().all(|c| c.is_numeric()) {
                        tokens.push(ParsedToken {
                            name: "client".to_string(),
                            value: entity_name,
                            token_type: TokenType::Entity("client".to_string()),
                            position: (0, 0), // Would calculate actual position
                            confidence: 0.7,
                        });
                        break;
                    }
                }
            }
        }
        
        // Calculate overall confidence
        let confidence = if action.is_some() && !tokens.is_empty() {
            tokens.iter().map(|t| t.confidence).sum::<f32>() / tokens.len() as f32
        } else if action.is_some() {
            0.6
        } else {
            0.3
        };
        
        ParsedCommand {
            action,
            plugin,
            tokens,
            confidence,
            original_text: text.to_string(),
        }
    }
    
    pub fn extract_mixed_command(&self, text: &str) -> Vec<ParsedCommand> {
        // Split by common delimiters
        let parts: Vec<&str> = text.split([',', ';', '&'])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        
        parts.into_iter()
            .map(|part| self.parse_command(part))
            .collect()
    }
}

impl Default for NLPParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_invoice_command() {
        let parser = NLPParser::new();
        
        let result = parser.parse_command("create invoice for ABC Corp 5000 pesos due tomorrow");
        
        assert_eq!(result.action, Some("create_invoice".to_string()));
        assert_eq!(result.plugin, Some("invoice".to_string()));
        assert!(result.tokens.iter().any(|t| t.name == "amount" && t.value == "5000"));
        assert!(result.tokens.iter().any(|t| t.name == "due_date"));
        assert!(result.tokens.iter().any(|t| t.name == "client"));
    }
    
    #[test]
    fn test_parse_payment_command() {
        let parser = NLPParser::new();
        
        let result = parser.parse_command("received payment from Juan 10,000 via gcash");
        
        assert_eq!(result.action, Some("record_payment".to_string()));
        assert!(result.tokens.iter().any(|t| t.name == "amount" && t.value == "10000"));
        assert!(result.tokens.iter().any(|t| t.name == "method" && t.value == "gcash"));
    }
    
    #[test]
    fn test_parse_mixed_amounts() {
        let parser = NLPParser::new();
        
        let test_cases = vec![
            ("₱1,000", "1000"),
            ("PHP 5000", "5000"),
            ("10000 pesos", "10000"),
            ("P500.50", "500.50"),
        ];
        
        for (input, expected) in test_cases {
            let result = parser.parse_command(&format!("invoice for client {input}"));
            assert!(result.tokens.iter().any(|t| t.name == "amount" && t.value == expected),
                "Failed for input: {input}");
        }
    }
}