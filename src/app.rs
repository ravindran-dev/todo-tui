use serde::{Deserialize, Serialize};
use tui::widgets::ListState;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub priority: Priority,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub state: ListState,
    pub input: String,
    pub input_mode: bool,
    pub show_help: bool,
    pub editing: bool,
    pub search_mode: bool,
    pub search_query: String,
    pub neon_theme: bool,
    pub confirm_delete: bool,
}

impl App {
    pub fn new(todos: Vec<Todo>) -> Self {
        let mut state = ListState::default();
        if !todos.is_empty() {
            state.select(Some(0));
        }

        Self {
            todos,
            state,
            input: String::new(),
            input_mode: false,
            show_help: false,
            editing: false,
            search_mode: false,
            search_query: String::new(),
            neon_theme: true,
            confirm_delete: false,
        }
    }

    pub fn start_input(&mut self) {
        self.input.clear();
        self.input_mode = true;
        self.editing = false;
    }

    pub fn submit_input(&mut self) {
        let title = self.input.trim();
        if !title.is_empty() {
            self.todos.push(Todo {
                id: Uuid::new_v4(),
                title: title.to_string(),
                completed: false,
                priority: Priority::Medium,
            });
            self.sort_by_priority();        }
        self.input.clear();
        self.input_mode = false;
    }

    pub fn start_edit(&mut self) {
        if let Some(i) = self.state.selected() {
            self.input = self.todos[i].title.clone();
            self.input_mode = true;
            self.editing = true;
        }
    }

    pub fn submit_edit(&mut self) {
        if let Some(i) = self.state.selected() {
            let title = self.input.trim();
            if !title.is_empty() {
                self.todos[i].title = title.to_string();
            }
        }
        self.input.clear();
        self.input_mode = false;
        self.editing = false;
        self.sort_by_priority();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i + 1 < self.todos.len() => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => self.todos.len().saturating_sub(1),
        };
        self.state.select(Some(i));
    }

    pub fn toggle(&mut self) {
        if let Some(i) = self.state.selected() {
            self.todos[i].completed = !self.todos[i].completed;
        }
    }

    pub fn delete(&mut self) {
        if let Some(i) = self.state.selected() {
            self.todos.remove(i);
            if self.todos.is_empty() {
                self.state.select(None);
            } else {
                self.state.select(Some(i.min(self.todos.len() - 1)));
            }
        }
    }

    pub fn cycle_priority(&mut self) {
        if let Some(i) = self.state.selected() {
            self.todos[i].priority = match self.todos[i].priority {
                Priority::High => Priority::Medium,
                Priority::Medium => Priority::Low,
                Priority::Low => Priority::High,
            };
            self.sort_by_priority();
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn toggle_theme(&mut self) {
        self.neon_theme = !self.neon_theme;
    }

    pub fn start_search(&mut self) {
        self.search_mode = true;
        self.search_query.clear();
    }

    pub fn filtered_todos(&self) -> Vec<&Todo> {
        if self.search_query.is_empty() {
            self.todos.iter().collect()
        } else {
            self.todos
                .iter()
                .filter(|t| t.title.to_lowercase().contains(&self.search_query.to_lowercase()))
                .collect()
        }
    }
    fn priority_rank(p: &Priority) -> u8 {
        match p {
            Priority::High => 0,
            Priority::Medium => 1,
            Priority::Low => 2,
        }
    }

    fn sort_by_priority(&mut self) {
        self.todos.sort_by_key(|t| Self::priority_rank(&t.priority));

        if !self.todos.is_empty() {
            self.state.select(Some(0));
        }
    }

}

