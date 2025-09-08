use crate::types::*;
use std::collections::HashMap;

/// Manages workflow steps and progression
#[derive(Clone)]
pub struct StepManager {
    workflow: Workflow,
    step_index: HashMap<String, usize>,
}

impl StepManager {
    pub fn new(workflow: Workflow) -> Self {
        let mut step_index = HashMap::new();
        for (i, step) in workflow.steps.iter().enumerate() {
            step_index.insert(step.id.clone(), i);
        }
        
        Self {
            workflow,
            step_index,
        }
    }
    
    /// Get a step by ID
    pub fn get_step(&self, step_id: &str) -> Option<&WorkflowStep> {
        self.step_index.get(step_id)
            .and_then(|&index| self.workflow.steps.get(index))
    }
    
    /// Get the next step based on current step and workflow state
    pub fn get_next_step(&self, current_step_id: &str, state: &WorkflowState) -> Option<&WorkflowStep> {
        let current_index = self.step_index.get(current_step_id)?;
        
        // Find next step that isn't already completed and whose dependencies are met
        for i in (current_index + 1)..self.workflow.steps.len() {
            let step = &self.workflow.steps[i];
            
            // Check if dependencies are met
            let deps_met = step.depends_on.iter().all(|dep| {
                state.completed_steps.contains(dep) || 
                state.collected_data.contains_key(dep)
            });
            
            if deps_met && !state.completed_steps.contains(&step.id) {
                return Some(step);
            }
        }
        
        None
    }
    
    /// Get the previous step
    pub fn get_previous_step(&self, current_step_id: &str) -> Option<&WorkflowStep> {
        let current_index = self.step_index.get(current_step_id)?;
        
        if *current_index > 0 {
            self.workflow.steps.get(current_index - 1)
        } else {
            None
        }
    }
    
    /// Check if all required steps are complete
    pub fn is_workflow_complete(&self, state: &WorkflowState) -> bool {
        self.workflow.steps.iter()
            .filter(|step| step.required)
            .all(|step| state.completed_steps.contains(&step.id))
    }
    
    /// Get list of remaining required steps
    pub fn get_remaining_steps(&self, state: &WorkflowState) -> Vec<&WorkflowStep> {
        self.workflow.steps.iter()
            .filter(|step| {
                step.required && !state.completed_steps.contains(&step.id)
            })
            .collect()
    }
    
    /// Calculate workflow progress percentage
    pub fn get_progress_percentage(&self, state: &WorkflowState) -> u32 {
        let total_required = self.workflow.steps.iter()
            .filter(|s| s.required)
            .count();
        
        if total_required == 0 {
            return 100;
        }
        
        let completed_required = self.workflow.steps.iter()
            .filter(|s| s.required && state.completed_steps.contains(&s.id))
            .count();
        
        ((completed_required as f32 / total_required as f32) * 100.0) as u32
    }
    
    /// Validate if a step can be started based on dependencies
    pub fn can_start_step(&self, step_id: &str, state: &WorkflowState) -> bool {
        if let Some(step) = self.get_step(step_id) {
            step.depends_on.iter().all(|dep| {
                state.completed_steps.contains(dep)
            })
        } else {
            false
        }
    }
    
    /// Get steps that are ready to be started
    pub fn get_available_steps(&self, state: &WorkflowState) -> Vec<&WorkflowStep> {
        self.workflow.steps.iter()
            .filter(|step| {
                !state.completed_steps.contains(&step.id) &&
                self.can_start_step(&step.id, state)
            })
            .collect()
    }
}