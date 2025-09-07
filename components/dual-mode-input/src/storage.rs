use gloo_storage::{LocalStorage, Storage};
use crate::types::{InputMode, TokenizedInput};

const STORAGE_KEY_MODE: &str = "dual_mode_input_mode";
const STORAGE_KEY_HISTORY: &str = "dual_mode_input_history";
const MAX_HISTORY: usize = 50;

pub struct InputStorage;

impl InputStorage {
    pub fn save_mode(mode: &InputMode) {
        let _ = LocalStorage::set(STORAGE_KEY_MODE, mode);
    }
    
    pub fn load_mode() -> Option<InputMode> {
        LocalStorage::get(STORAGE_KEY_MODE).ok()
    }
    
    pub fn save_to_history(input: &str) {
        let mut history: Vec<String> = LocalStorage::get(STORAGE_KEY_HISTORY).unwrap_or_default();
        
        // Don't save duplicates
        if history.first() == Some(&input.to_string()) {
            return;
        }
        
        history.insert(0, input.to_string());
        
        // Keep only the latest MAX_HISTORY items
        if history.len() > MAX_HISTORY {
            history.truncate(MAX_HISTORY);
        }
        
        let _ = LocalStorage::set(STORAGE_KEY_HISTORY, history);
    }
    
    pub fn get_history() -> Vec<String> {
        LocalStorage::get(STORAGE_KEY_HISTORY).unwrap_or_default()
    }
    
    pub fn clear_history() {
        let _: Result<(), _> = LocalStorage::delete(STORAGE_KEY_HISTORY);
    }
}