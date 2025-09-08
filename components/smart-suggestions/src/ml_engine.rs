use crate::types::*;
use chrono::{Utc, Timelike, Datelike};
use std::collections::HashMap;

/// Simple ML engine for generating smart suggestions
#[derive(Clone)]
pub struct MLEngine {
    config: MLConfig,
    state: MLState,
    philippine_context: PhilippineContext,
}

impl MLEngine {
    pub fn new(config: MLConfig) -> Self {
        Self {
            config,
            state: MLState::default(),
            philippine_context: PhilippineContext::new(),
        }
    }
    
    pub fn with_state(config: MLConfig, state: MLState) -> Self {
        Self {
            config,
            state,
            philippine_context: PhilippineContext::new(),
        }
    }
    
    /// Generate suggestions based on current context
    pub fn generate_suggestions(&mut self, context: &str, current_input: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        // 1. Time-based suggestions
        if self.config.enable_time_patterns {
            suggestions.extend(self.generate_time_based_suggestions());
        }
        
        // 2. Context-based suggestions
        suggestions.extend(self.generate_contextual_suggestions(context, current_input));
        
        // 3. Historical pattern suggestions
        suggestions.extend(self.generate_historical_suggestions(context));
        
        // 4. Philippine-specific suggestions
        if self.config.enable_philippine_context {
            suggestions.extend(self.generate_philippine_suggestions());
        }
        
        // 5. Frequency-based suggestions
        suggestions.extend(self.generate_frequency_suggestions(current_input));
        
        // Sort by confidence and limit
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        suggestions.truncate(self.config.max_suggestions);
        
        // Filter by minimum confidence
        suggestions.retain(|s| s.confidence >= self.config.min_confidence);
        
        suggestions
    }
    
    /// Learn from user interaction
    pub fn learn(&mut self, interaction: UserInteraction) {
        // Update frequency map
        if let Some(selected) = &interaction.selected_suggestion {
            let key = format!("{}:{}", interaction.context, selected);
            let freq = self.state.frequency_map.entry(key).or_insert(0.0);
            *freq = (*freq * (1.0 - self.config.learning_rate)) + self.config.learning_rate;
        }
        
        // Update context associations
        self.update_context_associations(&interaction);
        
        // Detect patterns
        self.detect_patterns(&interaction);
        
        // Add to history
        self.state.interaction_history.push(interaction);
        
        // Limit history size
        if self.state.interaction_history.len() > self.config.history_limit {
            self.state.interaction_history.remove(0);
        }
        
        self.state.last_update = Utc::now();
    }
    
    fn generate_time_based_suggestions(&self) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        let now = Utc::now();
        let hour = now.hour();
        let day = now.day();
        
        // Morning suggestions (8-10 AM)
        if hour >= 8 && hour <= 10 {
            suggestions.push(Suggestion {
                id: "morning_sales_entry".to_string(),
                text: "Record yesterday's sales".to_string(),
                description: Some("Common morning task".to_string()),
                confidence: 0.7,
                category: SuggestionCategory::TimeBased,
                metadata: HashMap::new(),
                action: None,
            });
        }
        
        // Month-end suggestions
        if self.philippine_context.is_month_end {
            suggestions.push(Suggestion {
                id: "month_end_vat".to_string(),
                text: "Generate VAT return".to_string(),
                description: Some("Monthly VAT filing due on 20th".to_string()),
                confidence: 0.9,
                category: SuggestionCategory::TimeBased,
                metadata: HashMap::new(),
                action: Some(SuggestionAction {
                    action_type: "generate_report".to_string(),
                    payload: serde_json::json!({"report": "vat_return"}),
                }),
            });
            
            suggestions.push(Suggestion {
                id: "month_end_reconcile".to_string(),
                text: "Reconcile bank accounts".to_string(),
                description: Some("Month-end reconciliation".to_string()),
                confidence: 0.8,
                category: SuggestionCategory::TimeBased,
                metadata: HashMap::new(),
                action: None,
            });
        }
        
        // Payroll suggestions (25th-30th)
        if day >= 25 && day <= 30 {
            suggestions.push(Suggestion {
                id: "payroll_process".to_string(),
                text: "Process payroll".to_string(),
                description: Some("End of month payroll".to_string()),
                confidence: 0.85,
                category: SuggestionCategory::TimeBased,
                metadata: HashMap::new(),
                action: None,
            });
        }
        
        suggestions
    }
    
    fn generate_contextual_suggestions(&self, context: &str, input: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        let input_lower = input.to_lowercase();
        
        // Invoice context
        if context.contains("invoice") || input_lower.contains("invoice") {
            suggestions.push(Suggestion {
                id: "add_vat".to_string(),
                text: "Add 12% VAT".to_string(),
                description: Some("Standard VAT rate".to_string()),
                confidence: 0.8,
                category: SuggestionCategory::Contextual,
                metadata: HashMap::new(),
                action: Some(SuggestionAction {
                    action_type: "apply_vat".to_string(),
                    payload: serde_json::json!({"rate": 0.12}),
                }),
            });
            
            if input_lower.contains("professional") || input_lower.contains("service") {
                suggestions.push(Suggestion {
                    id: "apply_ewt".to_string(),
                    text: "Apply 10% EWT for professional services".to_string(),
                    description: Some("Expanded withholding tax".to_string()),
                    confidence: 0.75,
                    category: SuggestionCategory::Contextual,
                    metadata: HashMap::new(),
                    action: None,
                });
            }
        }
        
        // Payment context
        if context.contains("payment") || input_lower.contains("pay") {
            suggestions.push(Suggestion {
                id: "payment_gcash".to_string(),
                text: "Payment via GCash".to_string(),
                description: Some("Popular payment method".to_string()),
                confidence: 0.6,
                category: SuggestionCategory::Contextual,
                metadata: HashMap::new(),
                action: None,
            });
            
            suggestions.push(Suggestion {
                id: "payment_bank".to_string(),
                text: "Bank transfer".to_string(),
                description: Some("Direct bank payment".to_string()),
                confidence: 0.6,
                category: SuggestionCategory::Contextual,
                metadata: HashMap::new(),
                action: None,
            });
        }
        
        // Expense context
        if context.contains("expense") || input_lower.contains("bought") || input_lower.contains("purchase") {
            suggestions.push(Suggestion {
                id: "input_vat".to_string(),
                text: "Claim input VAT".to_string(),
                description: Some("For VAT-registered suppliers".to_string()),
                confidence: 0.7,
                category: SuggestionCategory::Contextual,
                metadata: HashMap::new(),
                action: None,
            });
        }
        
        suggestions
    }
    
    fn generate_historical_suggestions(&self, context: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        // Find patterns matching current context
        for pattern in &self.state.patterns {
            if pattern.context_keys.contains(&context.to_string()) && pattern.confidence >= self.config.pattern_threshold {
                // Generate suggestion based on pattern
                let suggestion = Suggestion {
                    id: format!("pattern_{}", pattern.id),
                    text: format!("Based on your pattern: {}", self.describe_pattern(pattern)),
                    description: Some(format!("Happens {:.0}% of the time", pattern.frequency * 100.0)),
                    confidence: pattern.confidence,
                    category: SuggestionCategory::Historical,
                    metadata: HashMap::new(),
                    action: None,
                };
                suggestions.push(suggestion);
            }
        }
        
        suggestions
    }
    
    fn generate_philippine_suggestions(&self) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        // BIR deadline reminders
        for deadline in &self.philippine_context.upcoming_deadlines {
            suggestions.push(Suggestion {
                id: format!("deadline_{}", deadline.name.to_lowercase().replace(" ", "_")),
                text: format!("Prepare {}", deadline.name),
                description: Some(format!("Due soon - {}", deadline.category)),
                confidence: 0.85,
                category: SuggestionCategory::Philippine,
                metadata: HashMap::new(),
                action: None,
            });
        }
        
        // Common Philippine business operations
        if self.philippine_context.is_month_end {
            suggestions.push(Suggestion {
                id: "bir_remittance".to_string(),
                text: "Remit withholding taxes to BIR".to_string(),
                description: Some("Monthly requirement".to_string()),
                confidence: 0.8,
                category: SuggestionCategory::Philippine,
                metadata: HashMap::new(),
                action: None,
            });
            
            suggestions.push(Suggestion {
                id: "sss_payment".to_string(),
                text: "Pay SSS contributions".to_string(),
                description: Some("Monthly SSS remittance".to_string()),
                confidence: 0.75,
                category: SuggestionCategory::Philippine,
                metadata: HashMap::new(),
                action: None,
            });
        }
        
        suggestions
    }
    
    fn generate_frequency_suggestions(&self, input: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        // Find frequently used items that match input
        for (key, frequency) in &self.state.frequency_map {
            if key.to_lowercase().contains(&input.to_lowercase()) && *frequency > 0.3 {
                let parts: Vec<&str> = key.split(':').collect();
                if parts.len() >= 2 {
                    suggestions.push(Suggestion {
                        id: format!("freq_{}", key.replace(':', "_")),
                        text: parts[1].to_string(),
                        description: Some(format!("Used frequently ({:.0}%)", frequency * 100.0)),
                        confidence: *frequency,
                        category: SuggestionCategory::Historical,
                        metadata: HashMap::new(),
                        action: None,
                    });
                }
            }
        }
        
        suggestions
    }
    
    fn update_context_associations(&mut self, interaction: &UserInteraction) {
        if let Some(selected) = &interaction.selected_suggestion {
            let associations = self.state.context_associations
                .entry(interaction.context.clone())
                .or_insert_with(Vec::new);
            
            if !associations.contains(selected) {
                associations.push(selected.clone());
            }
        }
    }
    
    fn detect_patterns(&mut self, interaction: &UserInteraction) {
        // Simple pattern detection based on time
        let hour = interaction.timestamp.hour();
        
        // Check if this is a time-of-day pattern
        let time_pattern_id = format!("time_{}_{}", interaction.context, hour);
        
        if let Some(pattern) = self.state.patterns.iter_mut().find(|p| p.id == time_pattern_id) {
            // Update existing pattern
            pattern.frequency = (pattern.frequency * 0.9) + 0.1;
            pattern.last_seen = interaction.timestamp;
            pattern.confidence = pattern.frequency.min(0.95);
        } else if interaction.selected_suggestion.is_some() {
            // Create new pattern
            let new_pattern = Pattern {
                id: time_pattern_id,
                pattern_type: PatternType::TimeOfDay,
                frequency: 0.1,
                confidence: 0.3,
                context_keys: vec![interaction.context.clone()],
                last_seen: interaction.timestamp,
            };
            self.state.patterns.push(new_pattern);
        }
    }
    
    fn describe_pattern(&self, pattern: &Pattern) -> String {
        match pattern.pattern_type {
            PatternType::TimeOfDay => format!("Usually done at this time"),
            PatternType::DayOfWeek => format!("Common on this day"),
            PatternType::MonthEnd => format!("Month-end activity"),
            PatternType::Sequence => format!("Follows previous action"),
            PatternType::Periodic => format!("Recurring task"),
            PatternType::Contextual => format!("Related to current context"),
        }
    }
    
    pub fn get_state(&self) -> &MLState {
        &self.state
    }
    
    pub fn set_state(&mut self, state: MLState) {
        self.state = state;
    }
}