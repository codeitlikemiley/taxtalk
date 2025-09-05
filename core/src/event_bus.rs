// Event bus for inter-plugin communication
use crate::tokenizer::CommandMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub plugin_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub metadata: CommandMetadata,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub type EventSender = mpsc::UnboundedSender<Event>;
pub type EventReceiver = mpsc::UnboundedReceiver<Event>;

pub struct EventBus {
    subscribers: HashMap<String, Vec<EventSender>>,
    sender: EventSender,
    receiver: Option<EventReceiver>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Self {
            subscribers: HashMap::new(),
            sender,
            receiver: Some(receiver),
        }
    }

    pub fn subscribe(&mut self, plugin_id: String) -> EventReceiver {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.subscribers
            .entry(plugin_id)
            .or_insert_with(Vec::new)
            .push(sender);
        receiver
    }

    pub async fn dispatch_event(&self, event: Event) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // For now, just publish the event and return a success message
        self.publish(event.clone());
        Ok(serde_json::json!({
            "status": "success",
            "event_id": event.id.to_string()
        }))
    }

    pub fn publish(&self, event: Event) {
        if let Some(subscribers) = self.subscribers.get(&event.plugin_id) {
            for subscriber in subscribers {
                let _ = subscriber.send(event.clone());
            }
        }

        // Also send to all subscribers for broadcast events
        for subscribers in self.subscribers.values() {
            for subscriber in subscribers {
                let _ = subscriber.send(event.clone());
            }
        }
    }

    pub fn get_sender(&self) -> EventSender {
        self.sender.clone()
    }

    pub async fn run(mut self) {
        if let Some(mut receiver) = self.receiver.take() {
            while let Some(event) = receiver.recv().await {
                self.publish(event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::CommandMetadata;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_event_subscription() {
        let mut event_bus = EventBus::new();
        let mut receiver = event_bus.subscribe("test-plugin".to_string());

        let event = Event {
            id: Uuid::new_v4(),
            plugin_id: "test-plugin".to_string(),
            event_type: "test".to_string(),
            payload: serde_json::json!({"key": "value"}),
            metadata: CommandMetadata {
                user_context: None,
                timestamp: chrono::Utc::now(),
                confidence: 1.0,
            },
            timestamp: chrono::Utc::now(),
        };

        event_bus.publish(event.clone());

        let received = timeout(Duration::from_secs(1), receiver.recv()).await;
        assert!(received.is_ok());
        assert_eq!(received.unwrap().unwrap().event_type, "test");
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let mut event_bus = EventBus::new();
        let mut receiver1 = event_bus.subscribe("plugin1".to_string());
        let mut receiver2 = event_bus.subscribe("plugin1".to_string());

        let event = Event {
            id: Uuid::new_v4(),
            plugin_id: "plugin1".to_string(),
            event_type: "broadcast".to_string(),
            payload: serde_json::json!({"message": "hello"}),
            metadata: CommandMetadata {
                user_context: None,
                timestamp: chrono::Utc::now(),
                confidence: 1.0,
            },
            timestamp: chrono::Utc::now(),
        };

        event_bus.publish(event);

        let received1 = timeout(Duration::from_secs(1), receiver1.recv()).await;
        let received2 = timeout(Duration::from_secs(1), receiver2.recv()).await;

        assert!(received1.is_ok());
        assert!(received2.is_ok());
    }
}