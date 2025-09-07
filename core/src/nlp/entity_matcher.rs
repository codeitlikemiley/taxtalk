use serde::{Deserialize, Serialize};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMatch {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub score: f32,
    pub matched_text: String,
}

pub struct EntityMatcher {
    matcher: SkimMatcherV2,
}

impl EntityMatcher {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }
    
    pub fn fuzzy_match_entity(
        &self,
        text: &str,
        entity_type: &str,
        entities: Vec<(String, String)>, // (id, name) pairs
        threshold: f32,
    ) -> Vec<EntityMatch> {
        let mut matches = Vec::new();
        
        for (id, name) in entities {
            if let Some(score) = self.matcher.fuzzy_match(&name.to_lowercase(), &text.to_lowercase()) {
                let normalized_score = (score as f32) / 100.0; // Normalize to 0-1 range
                
                if normalized_score >= threshold {
                    matches.push(EntityMatch {
                        id: id.clone(),
                        name: name.clone(),
                        entity_type: entity_type.to_string(),
                        score: normalized_score,
                        matched_text: text.to_string(),
                    });
                }
            }
            
            // Also check for partial matches
            let name_parts: Vec<&str> = name.split_whitespace().collect();
            let text_parts: Vec<&str> = text.split_whitespace().collect();
            
            // Check if all text parts match some name parts
            let mut partial_match = true;
            let mut total_score = 0.0;
            let mut match_count = 0;
            
            for text_part in &text_parts {
                let mut best_score: f32 = 0.0;
                for name_part in &name_parts {
                    if let Some(score) = self.matcher.fuzzy_match(
                        &name_part.to_lowercase(), 
                        &text_part.to_lowercase()
                    ) {
                        best_score = best_score.max(score as f32);
                    }
                }
                
                if best_score > 50.0 {
                    total_score += best_score;
                    match_count += 1;
                } else {
                    partial_match = false;
                    break;
                }
            }
            
            if partial_match && match_count > 0 {
                let avg_score = (total_score / (match_count as f32)) / 100.0;
                if avg_score >= threshold && !matches.iter().any(|m| m.id == id) {
                    matches.push(EntityMatch {
                        id,
                        name,
                        entity_type: entity_type.to_string(),
                        score: avg_score * 0.9, // Slightly penalize partial matches
                        matched_text: text.to_string(),
                    });
                }
            }
        }
        
        // Sort by score descending
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        matches
    }
    
    pub fn find_best_match(
        &self,
        text: &str,
        entity_type: &str,
        entities: Vec<(String, String)>,
    ) -> Option<EntityMatch> {
        self.fuzzy_match_entity(text, entity_type, entities, 0.5)
            .into_iter()
            .next()
    }
    
    /// Extract entity references from text, considering common patterns
    pub fn extract_entity_references(&self, text: &str) -> Vec<(String, String)> {
        let mut references = Vec::new();
        
        // Patterns for entity extraction
        let patterns = vec![
            (r"(?i)(?:for|to|client:?)\s+([A-Z][A-Za-z0-9\s&,.-]+)", "client"),
            (r"(?i)(?:from|supplier:?|vendor:?)\s+([A-Z][A-Za-z0-9\s&,.-]+)", "supplier"),
            (r"(?i)(?:invoice|inv|si)[\s#-]*([\w-]+\d+)", "invoice"),
            (r"(?i)(?:payment|pay|or)[\s#-]*([\w-]+\d+)", "payment"),
        ];
        
        for (pattern_str, entity_type) in patterns {
            if let Ok(re) = regex::Regex::new(pattern_str) {
                for cap in re.captures_iter(text) {
                    if let Some(match_group) = cap.get(1) {
                        let extracted = match_group.as_str().trim();
                        references.push((entity_type.to_string(), extracted.to_string()));
                    }
                }
            }
        }
        
        references
    }
    
    /// Check if text likely contains an entity name
    pub fn is_likely_entity_name(&self, text: &str) -> bool {
        // Check for patterns that suggest entity names
        let indicators = vec![
            // Has capital letters (proper nouns)
            text.chars().any(|c| c.is_uppercase()),
            // Contains common business suffixes
            text.to_lowercase().contains("corp") ||
            text.to_lowercase().contains("inc") ||
            text.to_lowercase().contains("llc") ||
            text.to_lowercase().contains("ltd") ||
            text.to_lowercase().contains("company") ||
            text.to_lowercase().contains("enterprise") ||
            text.to_lowercase().contains("trading") ||
            text.to_lowercase().contains("store"),
            // Has multiple words (likely a full name)
            text.split_whitespace().count() >= 2,
            // Not purely numeric
            !text.chars().all(|c| c.is_numeric() || c.is_whitespace()),
        ];
        
        indicators.iter().filter(|&&x| x).count() >= 2
    }
}

impl Default for EntityMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fuzzy_match() {
        let matcher = EntityMatcher::new();
        
        let entities = vec![
            ("1".to_string(), "ABC Corporation".to_string()),
            ("2".to_string(), "XYZ Trading".to_string()),
            ("3".to_string(), "Juan dela Cruz".to_string()),
        ];
        
        let matches = matcher.fuzzy_match_entity("ABC Corp", "client", entities, 0.5);
        
        assert!(!matches.is_empty());
        assert_eq!(matches[0].name, "ABC Corporation");
    }
    
    #[test]
    fn test_extract_entity_references() {
        let matcher = EntityMatcher::new();
        
        let text = "create invoice for ABC Corporation with payment from Juan";
        let references = matcher.extract_entity_references(text);
        
        assert!(references.iter().any(|(t, _)| t == "client"));
    }
    
    #[test]
    fn test_is_likely_entity_name() {
        let matcher = EntityMatcher::new();
        
        assert!(matcher.is_likely_entity_name("ABC Corporation"));
        assert!(matcher.is_likely_entity_name("Juan dela Cruz"));
        assert!(!matcher.is_likely_entity_name("12345"));
        assert!(!matcher.is_likely_entity_name("amount"));
    }
}