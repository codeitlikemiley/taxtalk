use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::token_schema::TokenSchema;
use super::validator::ProvidedToken;

/// Represents a guided session for collecting tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidedSession {
    pub id: String,
    pub command: String,
    pub plugin: String,
    pub action: String,
    pub state: SessionState,
    pub collected_tokens: HashMap<String, ProvidedToken>,
    pub remaining_required: Vec<String>,
    pub remaining_optional: Vec<String>,
    pub conversation_history: Vec<ConversationEntry>,
    pub current_prompt: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// State of a guided session
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    New,
    Collecting,
    Confirming,
    Ready,
    Executing,
    Completed,
    Cancelled,
    Expired,
}

/// Entry in conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEntry {
    pub role: ConversationRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub token_collected: Option<String>,
}

/// Role in conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationRole {
    User,
    Assistant,
    System,
}

/// Session manager for handling multiple active sessions
pub struct SessionManager {
    sessions: HashMap<String, GuidedSession>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
    
    /// Create a new session
    pub fn create_session(
        &mut self,
        command: String,
        plugin: String,
        action: String,
        schema: &TokenSchema,
    ) -> Result<GuidedSession> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        // Extract required and optional token names
        let remaining_required: Vec<String> = schema
            .required_tokens
            .iter()
            .map(|t| t.name.clone())
            .collect();
            
        let remaining_optional: Vec<String> = schema
            .optional_tokens
            .iter()
            .map(|t| t.name.clone())
            .collect();
        
        let session = GuidedSession {
            id: id.clone(),
            command,
            plugin,
            action,
            state: SessionState::New,
            collected_tokens: HashMap::new(),
            remaining_required,
            remaining_optional,
            conversation_history: vec![],
            current_prompt: None,
            created_at: now,
            updated_at: now,
            expires_at: now + chrono::Duration::hours(1), // 1 hour expiry
        };
        
        self.sessions.insert(id.clone(), session.clone());
        Ok(session)
    }
    
    /// Get a session by ID
    pub fn get_session(&self, id: &str) -> Option<&GuidedSession> {
        self.sessions.get(id)
    }
    
    /// Get a mutable session by ID
    pub fn get_session_mut(&mut self, id: &str) -> Option<&mut GuidedSession> {
        self.sessions.get_mut(id)
    }
    
    /// Submit a token value to a session
    pub fn submit_token(
        &mut self,
        session_id: &str,
        token_name: String,
        value: ProvidedToken,
    ) -> Result<()> {
        let session = self.sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
        
        // Add to collected tokens
        session.collected_tokens.insert(token_name.clone(), value.clone());
        
        // Remove from remaining required
        session.remaining_required.retain(|t| t != &token_name);
        
        // Remove from remaining optional
        session.remaining_optional.retain(|t| t != &token_name);
        
        // Update conversation history
        session.conversation_history.push(ConversationEntry {
            role: ConversationRole::User,
            content: value.raw_value,
            timestamp: Utc::now(),
            token_collected: Some(token_name.clone()),
        });
        
        // Update session state
        if session.remaining_required.is_empty() {
            if session.state == SessionState::Collecting {
                session.state = SessionState::Confirming;
            }
        } else {
            session.state = SessionState::Collecting;
        }
        
        session.updated_at = Utc::now();
        
        Ok(())
    }
    
    /// Get next prompt for a session
    pub fn get_next_prompt(&mut self, session_id: &str, schema: &TokenSchema) -> Result<Option<String>> {
        let session = self.sessions
            .get(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
        
        // If we have remaining required tokens, prompt for the first one
        if let Some(next_token) = session.remaining_required.first() {
            if let Some(token_def) = schema.get_token(next_token) {
                return Ok(Some(token_def.prompt.clone()));
            }
        }
        
        // If no required tokens left but state is confirming, ask about optional
        if session.state == SessionState::Confirming && !session.remaining_optional.is_empty() {
            return Ok(Some(format!(
                "All required information collected. Would you like to provide any optional information? Available: {}",
                session.remaining_optional.join(", ")
            )));
        }
        
        Ok(None)
    }
    
    /// Mark session as ready to execute
    pub fn mark_ready(&mut self, session_id: &str) -> Result<()> {
        let session = self.sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
        
        if !session.remaining_required.is_empty() {
            return Err(anyhow::anyhow!("Cannot mark ready - required tokens missing"));
        }
        
        session.state = SessionState::Ready;
        session.updated_at = Utc::now();
        
        Ok(())
    }
    
    /// Execute a session (convert to final command)
    pub fn execute_session(&mut self, session_id: &str) -> Result<HashMap<String, serde_json::Value>> {
        let session = self.sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
        
        if session.state != SessionState::Ready {
            return Err(anyhow::anyhow!("Session not ready for execution"));
        }
        
        session.state = SessionState::Executing;
        
        // Build final token values
        let mut final_tokens = HashMap::new();
        for (name, token) in &session.collected_tokens {
            final_tokens.insert(name.clone(), token.parsed_value.clone());
        }
        
        session.state = SessionState::Completed;
        session.updated_at = Utc::now();
        
        Ok(final_tokens)
    }
    
    /// Cancel a session
    pub fn cancel_session(&mut self, session_id: &str) -> Result<()> {
        let session = self.sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
        
        session.state = SessionState::Cancelled;
        session.updated_at = Utc::now();
        
        Ok(())
    }
    
    /// Clean up expired sessions
    pub fn cleanup_expired(&mut self) {
        let now = Utc::now();
        self.sessions.retain(|_, session| {
            !(session.expires_at < now && session.state != SessionState::Completed)
        });
    }
    
    /// Get all active sessions
    pub fn get_active_sessions(&self) -> Vec<&GuidedSession> {
        self.sessions
            .values()
            .filter(|s| {
                matches!(
                    s.state,
                    SessionState::New | SessionState::Collecting | SessionState::Confirming | SessionState::Ready
                )
            })
            .collect()
    }
    
    /// Get session progress
    pub fn get_progress(&self, session_id: &str) -> Result<SessionProgress> {
        let session = self.sessions
            .get(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
        
        let total_required = session.collected_tokens.len() + session.remaining_required.len();
        let collected = session.collected_tokens.len();
        
        Ok(SessionProgress {
            session_id: session_id.to_string(),
            state: session.state.clone(),
            collected_count: collected,
            required_count: total_required,
            optional_count: session.remaining_optional.len(),
            percentage: if total_required > 0 {
                (collected as f32 / total_required as f32 * 100.0) as u32
            } else {
                100
            },
        })
    }
}

/// Progress information for a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionProgress {
    pub session_id: String,
    pub state: SessionState,
    pub collected_count: usize,
    pub required_count: usize,
    pub optional_count: usize,
    pub percentage: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let mut manager = SessionManager::new();
        let schema = TokenSchema::invoice_create();
        
        let session = manager.create_session(
            "create invoice for Juan".to_string(),
            "invoice".to_string(),
            "create".to_string(),
            &schema,
        ).unwrap();
        
        assert_eq!(session.state, SessionState::New);
        assert_eq!(session.remaining_required.len(), 3); // client, amount, items
        assert_eq!(session.remaining_optional.len(), 3); // due_date, notes, tax_exempt
    }
    
    #[test]
    fn test_token_submission() {
        let mut manager = SessionManager::new();
        let schema = TokenSchema::invoice_create();
        
        let session = manager.create_session(
            "create invoice".to_string(),
            "invoice".to_string(),
            "create".to_string(),
            &schema,
        ).unwrap();
        
        let session_id = session.id.clone();
        
        // Submit client token
        manager.submit_token(
            &session_id,
            "client".to_string(),
            ProvidedToken {
                raw_value: "Juan Cruz".to_string(),
                parsed_value: serde_json::json!("Juan Cruz"),
                resolved_entity: None,
            },
        ).unwrap();
        
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.remaining_required.len(), 2); // amount, items left
        assert_eq!(session.state, SessionState::Collecting);
    }
}