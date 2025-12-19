use serde::{Deserialize, Serialize};
use tui::widgets::ListState;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub state: ListState,

    
    pub input: String,
    pub input_mode: bool,
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
        }
    }

    pub fn start_input(&mut self) {
        self.input.clear();
        self.input_mode = true;
    }

    pub fn submit_input(&mut self) {
        let title = self.input.trim();
        if !title.is_empty() {
            self.todos.push(Todo {
                id: Uuid::new_v4(),
                title: title.to_string(),
                completed: false,
            });
            self.state.select(Some(self.todos.len() - 1));
        }
        self.input.clear();
        self.input_mode = false;
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
            if let Some(todo) = self.todos.get_mut(i) {
                todo.completed = !todo.completed;
            }
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
}
