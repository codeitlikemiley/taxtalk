use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// A suggestion provided by the ML engine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suggestion {
    pub id: String,
    pub text: String,
    pub description: Option<String>,
    pub confidence: f32, // 0.0 to 1.0
    pub category: SuggestionCategory,
    pub metadata: HashMap<String, serde_json::Value>,
    pub action: Option<SuggestionAction>,
}

/// Categories of suggestions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SuggestionCategory {
    Command,        // Command to execute
    Entity,         // Entity to select
    Value,          // Value to input
    TimeBased,      // Time-sensitive suggestion
    Contextual,     // Based on current context
    Historical,     // Based on past behavior
    Philippine,     // Philippine-specific (BIR, VAT, etc.)
}

/// Actions that can be triggered by accepting a suggestion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestionAction {
    pub action_type: String,
    pub payload: serde_json::Value,
}

/// User interaction for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInteraction {
    pub timestamp: DateTime<Utc>,
    pub context: String,
    pub input: String,
    pub selected_suggestion: Option<String>,
    pub rejected_suggestions: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Pattern detected by the ML engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: String,
    pub pattern_type: PatternType,
    pub frequency: f32,
    pub confidence: f32,
    pub context_keys: Vec<String>,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Sequence,       // A follows B
    TimeOfDay,      // Happens at specific times
    DayOfWeek,      // Happens on specific days
    MonthEnd,       // Month-end activities
    Periodic,       // Repeating patterns
    Contextual,     // Context-dependent
}

/// Configuration for the ML engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLConfig {
    pub max_suggestions: usize,
    pub min_confidence: f32,
    pub learning_rate: f32,
    pub pattern_threshold: f32,
    pub history_limit: usize,
    pub enable_time_patterns: bool,
    pub enable_philippine_context: bool,
}

impl Default for MLConfig {
    fn default() -> Self {
        Self {
            max_suggestions: 5,
            min_confidence: 0.3,
            learning_rate: 0.1,
            pattern_threshold: 0.5,
            history_limit: 1000,
            enable_time_patterns: true,
            enable_philippine_context: true,
        }
    }
}

/// ML Engine state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLState {
    pub patterns: Vec<Pattern>,
    pub interaction_history: Vec<UserInteraction>,
    pub frequency_map: HashMap<String, f32>,
    pub context_associations: HashMap<String, Vec<String>>,
    pub last_update: DateTime<Utc>,
}

impl Default for MLState {
    fn default() -> Self {
        Self {
            patterns: Vec::new(),
            interaction_history: Vec::new(),
            frequency_map: HashMap::new(),
            context_associations: HashMap::new(),
            last_update: Utc::now(),
        }
    }
}

/// Philippine business context
#[derive(Debug, Clone)]
pub struct PhilippineContext {
    pub current_month: u32,
    pub is_month_end: bool,
    pub is_quarter_end: bool,
    pub is_year_end: bool,
    pub upcoming_deadlines: Vec<Deadline>,
}

#[derive(Debug, Clone)]
pub struct Deadline {
    pub name: String,
    pub date: DateTime<Utc>,
    pub category: String,
}

impl PhilippineContext {
    pub fn new() -> Self {
        let now = Utc::now();
        let month = now.format("%m").to_string().parse::<u32>().unwrap_or(1);
        let day = now.format("%d").to_string().parse::<u32>().unwrap_or(1);
        
        Self {
            current_month: month,
            is_month_end: day >= 25,
            is_quarter_end: (month % 3 == 0) && day >= 25,
            is_year_end: month == 12 && day >= 20,
            upcoming_deadlines: Self::get_upcoming_deadlines(month, day),
        }
    }
    
    fn get_upcoming_deadlines(month: u32, day: u32) -> Vec<Deadline> {
        let mut deadlines = Vec::new();
        
        // VAT filing (20th of following month)
        if day >= 15 {
            deadlines.push(Deadline {
                name: "VAT Return (Form 2550M)".to_string(),
                date: Utc::now(),
                category: "BIR".to_string(),
            });
        }
        
        // Withholding tax (10th of following month)
        if day >= 5 && day <= 10 {
            deadlines.push(Deadline {
                name: "Withholding Tax (Form 1601C)".to_string(),
                date: Utc::now(),
                category: "BIR".to_string(),
            });
        }
        
        // Quarterly income tax
        if month % 3 == 0 && day >= 20 {
            deadlines.push(Deadline {
                name: "Quarterly Income Tax (1701Q)".to_string(),
                date: Utc::now(),
                category: "BIR".to_string(),
            });
        }
        
        deadlines
    }
}