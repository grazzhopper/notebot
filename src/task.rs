use crate::config;
use serde::{Deserialize, Serialize};
use std::fs;
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

pub fn add_task(
    description: String,
    priority: String,
    due_date: Option<String>,
) -> anyhow::Result<()> {
    let tasks_path = config::get_tasks_path()?;
    let content = fs::read_to_string(&tasks_path).unwrap_or_else(|_| String::from("[]"));
    let mut tasks: Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    tasks.push(Task {
        id,
        description: description.clone(),
        priority,
        due_date,
        completed: false,
        created_at: SystemTime::now(),
    });
    fs::write(&tasks_path, serde_json::to_string_pretty(&tasks)?)?;
    println!("Task added: {}", description);
    Ok(())
}

pub fn list_tasks() -> anyhow::Result<()> {
    let tasks_path = config::get_tasks_path()?;
    let content = fs::read_to_string(&tasks_path).unwrap_or_else(|_| String::from("[]"));
    let tasks: Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }
    for task in tasks {
        let status = if task.completed { "[x]" } else { "[ ]" };
        println!(
            "{} ID: {} | {} | Priority: {} | Due: {} | Created: {:?}",
            status,
            task.id,
            task.description,
            task.priority,
            task.due_date.unwrap_or("None".to_string()),
            task.created_at
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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

    #[test]
    fn test_add_task() {
        config::init_storage().unwrap();
        // Reset tasks.json to ensure clean state
        let tasks_path = config::get_tasks_path().unwrap();
        fs::write(&tasks_path, "[]").unwrap();
        add_task(
            "Test task".to_string(),
            "medium".to_string(),
            Some("2025-06-01".to_string()),
        )
        .unwrap();
        let content = fs::read_to_string(&tasks_path).unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&content).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Test task");
        assert_eq!(tasks[0].priority, "medium");
        assert_eq!(tasks[0].due_date, Some("2025-06-01".to_string()));
        assert_eq!(tasks[0].completed, false);
    }

    #[test]
    fn test_list_tasks_empty() {
        config::init_storage().unwrap();
        // Reset tasks.json
        let tasks_path = config::get_tasks_path().unwrap();
        fs::write(&tasks_path, "[]").unwrap();
        list_tasks().unwrap();
    }

    #[test]
    fn test_list_tasks_with_tasks() {
        config::init_storage().unwrap();
        // Reset tasks.json
        let tasks_path = config::get_tasks_path().unwrap();
        fs::write(&tasks_path, "[]").unwrap();
        add_task(
            "Test task".to_string(),
            "medium".to_string(),
            Some("2025-06-01".to_string()),
        )
        .unwrap();
        list_tasks().unwrap();
    }
}
