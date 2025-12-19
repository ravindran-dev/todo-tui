use crate::app::Todo;
use std::fs;
use std::path::Path;

const FILE: &str = "todos.json";

pub fn load() -> Vec<Todo> {
    if Path::new(FILE).exists() {
        let data = fs::read_to_string(FILE).unwrap();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        vec![]
    }
}

pub fn save(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).unwrap();
    fs::write(FILE, data).unwrap();
}
