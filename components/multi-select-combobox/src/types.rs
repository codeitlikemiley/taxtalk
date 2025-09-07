use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub entity_type: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl Entity {
    pub fn new(entity_type: &str, name: &str) -> Self {
        Self {
            id: format!("{}_{}", entity_type, chrono::Utc::now().timestamp_millis()),
            entity_type: entity_type.to_string(),
            name: name.to_string(),
            display_name: name.to_string(),
            description: None,
            metadata: None,
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
}

#[derive(Debug, Clone)]
pub struct SelectionState {
    pub selected: Vec<Entity>,
    pub selected_ids: HashSet<String>,
    pub max_selections: Option<usize>,
}

impl SelectionState {
    pub fn new(max_selections: Option<usize>) -> Self {
        Self {
            selected: Vec::new(),
            selected_ids: HashSet::new(),
            max_selections,
        }
    }
    
    pub fn with_initial(mut self, entities: Vec<Entity>) -> Self {
        for entity in entities {
            self.selected_ids.insert(entity.id.clone());
            self.selected.push(entity);
        }
        self
    }
    
    pub fn can_add_more(&self) -> bool {
        match self.max_selections {
            Some(max) => self.selected.len() < max,
            None => true,
        }
    }
    
    pub fn add(&mut self, entity: Entity) -> bool {
        if self.can_add_more() && !self.selected_ids.contains(&entity.id) {
            self.selected_ids.insert(entity.id.clone());
            self.selected.push(entity);
            true
        } else {
            false
        }
    }
    
    pub fn remove(&mut self, entity_id: &str) -> bool {
        if self.selected_ids.remove(entity_id) {
            self.selected.retain(|e| e.id != entity_id);
            true
        } else {
            false
        }
    }
    
    pub fn clear(&mut self) {
        self.selected.clear();
        self.selected_ids.clear();
    }
    
    pub fn count(&self) -> usize {
        self.selected.len()
    }
    
    pub fn is_selected(&self, entity_id: &str) -> bool {
        self.selected_ids.contains(entity_id)
    }
}

// Virtual scrolling support
#[derive(Debug, Clone)]
pub struct VirtualListState {
    pub total_items: usize,
    pub item_height: f64,
    pub container_height: f64,
    pub scroll_top: f64,
    pub visible_start: usize,
    pub visible_end: usize,
    pub overscan: usize, // Number of items to render outside visible area
}

impl VirtualListState {
    pub fn new(item_height: f64, container_height: f64) -> Self {
        Self {
            total_items: 0,
            item_height,
            container_height,
            scroll_top: 0.0,
            visible_start: 0,
            visible_end: 0,
            overscan: 3,
        }
    }
    
    pub fn update_items(&mut self, total: usize) {
        self.total_items = total;
        self.calculate_visible_range();
    }
    
    pub fn update_scroll(&mut self, scroll_top: f64) {
        self.scroll_top = scroll_top;
        self.calculate_visible_range();
    }
    
    pub fn update_container_height(&mut self, height: f64) {
        self.container_height = height;
        self.calculate_visible_range();
    }
    
    fn calculate_visible_range(&mut self) {
        let visible_count = (self.container_height / self.item_height).ceil() as usize;
        let start_index = (self.scroll_top / self.item_height).floor() as usize;
        
        self.visible_start = start_index.saturating_sub(self.overscan);
        self.visible_end = (start_index + visible_count + self.overscan).min(self.total_items);
    }
    
    pub fn total_height(&self) -> f64 {
        self.total_items as f64 * self.item_height
    }
    
    pub fn offset_y(&self) -> f64 {
        self.visible_start as f64 * self.item_height
    }
}