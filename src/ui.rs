use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(size);

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|t| {
            let status = if t.completed { "✔" } else { " " };
            let style = if t.completed {
                Style::default().fg(Color::Magenta).add_modifier(Modifier::DIM)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("[{}] {}", status, t.title)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    " Todo List ",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Black)
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let input_title = if app.input_mode {
        " Add Todo (Enter = save | Esc = cancel) "
    } else {
        " Press 'a' to add a todo | ? Help "
    };

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    input_title,
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(input, chunks[1]);

    if app.input_mode {
        f.set_cursor(
            chunks[1].x + app.input.len() as u16 + 1,
            chunks[1].y + 1,
        );
    }

    if app.show_help {
        draw_help_popup(f);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_help_popup<B: Backend>(f: &mut Frame<B>) {
    let area = centered_rect(70, 60, f.size());

    let help = vec![
        Spans::from(Span::styled(
            "Todo TUI – Keybindings",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Spans::from(""),
        Spans::from("a        → Add new todo"),
        Spans::from("Enter    → Save todo"),
        Spans::from("Esc      → Cancel / Close popup"),
        Spans::from("↑ / ↓    → Navigate todos"),
        Spans::from("Space    → Toggle complete"),
        Spans::from("d        → Delete todo"),
        Spans::from("?        → Toggle help"),
        Spans::from("q        → Quit"),
        Spans::from(""),
        Spans::from(Span::styled(
            "Press Esc or q to close",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let block = Paragraph::new(help)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    " 📖 Help ",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(Clear, area);
    f.render_widget(block, area);
}
