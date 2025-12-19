use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(size);

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|t| {
            let status = if t.completed { "✔" } else { " " };
            ListItem::new(format!("[{}] {}", status, t.title))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Todo List").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let input = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(if app.input_mode {
                    "Add Todo (Enter = save, Esc = cancel)"
                } else {
                    "Press 'a' to add a todo"
                }),
        );

    f.render_widget(input, chunks[1]);

    if app.input_mode {
        f.set_cursor(
            chunks[1].x + app.input.len() as u16 + 1,
            chunks[1].y + 1,
        );
    }
}
