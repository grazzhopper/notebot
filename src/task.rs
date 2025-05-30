use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub priority: String,
    pub due_date: Option<String>,
    pub completed: bool,
    pub created_at: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_task_serialization() {
        let task = Task {
            id: 1,
            description: "Test task".to_string(),
            priority: "medium".to_string(),
            due_date: Some("2025-06-01".to_string()),
            completed: false,
            created_at: SystemTime::now(),
        };
        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(task.id, deserialized.id);
        assert_eq!(task.description, deserialized.description);
        assert_eq!(task.priority, deserialized.priority);
        assert_eq!(task.due_date, deserialized.due_date);
        assert_eq!(task.completed, deserialized.completed);
    }
}
