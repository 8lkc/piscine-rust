mod err;
use err::{ParseErr, ReadErr};

pub use serde_json; // Use serde_json for JSON parsing
pub use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use serde_json::Value; // Import Value from serde_json

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = File::open(path).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        let json_value: Value = serde_json::from_str(&contents).map_err(|e| {
            Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>
        })?;

        // Check if tasks are empty
        let tasks = json_value["tasks"]
            .as_array()
            .ok_or_else(|| Box::new(ParseErr::Empty) as Box<dyn Error>)?;

        let title = json_value["title"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        // Parse tasks
        let tasks: Vec<Task> = tasks.iter().filter_map(|task| {
            let id = task.get("id")?.as_u64()? as u32;
            let description = task.get("description")?.as_str()?.to_string();
            let level = task.get("level")?.as_u64()? as u32;

            Some(Task { id, description, level })
        }).collect();

        Ok(TodoList { title, tasks })
    }
}
