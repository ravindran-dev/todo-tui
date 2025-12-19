use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Priority};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let accent = if app.neon_theme { Color::Cyan } else { Color::White };
    let highlight = if app.neon_theme { Color::Green } else { Color::Gray };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(size);

    let todos = if app.search_mode {
        app.filtered_todos()
    } else {
        app.todos.iter().collect()
    };

    let items: Vec<ListItem> = todos
        .iter()
        .map(|t| {
            let status = if t.completed { "✔" } else { " " };
            let (p, color) = match t.priority {
                Priority::High => ("H", Color::Red),
                Priority::Medium => ("M", Color::Yellow),
                Priority::Low => ("L", Color::Cyan),
            };

            let style = if t.completed {
                Style::default().fg(Color::Magenta).add_modifier(Modifier::DIM)
            } else {
                Style::default().fg(color)
            };

            ListItem::new(format!("[{}][{}] {}", status, p, t.title)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    " Todo List ",
                    Style::default().fg(accent).add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Black)
                .fg(highlight)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let input_title = if app.editing {
        "  Edit Todo "
    } else if app.search_mode {
        "  Search "
    } else if app.input_mode {
        " 󰀤 Add Todo "
    } else {
        " a Add | e Edit | d Delete | p Priority | s Search | t Theme | ? Help "
    };

    let input = Paragraph::new(if app.search_mode {
        app.search_query.as_str()
    } else {
        app.input.as_str()
    })
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .title(Spans::from(Span::styled(
                input_title,
                Style::default().fg(accent).add_modifier(Modifier::BOLD),
            )))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent)),
    );

    f.render_widget(input, chunks[1]);

    if app.input_mode || app.search_mode {
        let len = if app.search_mode {
            app.search_query.len()
        } else {
            app.input.len()
        };

        f.set_cursor(chunks[1].x + len as u16 + 1, chunks[1].y + 1);
    }

    if app.show_help {
        draw_help_popup(f, accent);
    }

    if app.confirm_delete {
        draw_confirm_popup(f, accent);
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

fn draw_help_popup<B: Backend>(f: &mut Frame<B>, accent: Color) {
    let area = centered_rect(60, 45, f.size());

    let help = vec![
        Spans::from(Span::styled(
            " KEYBINDINGS ",
            Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
        )),
        Spans::from(""),
        Spans::from(" a        Add new todo"),
        Spans::from(" e        Edit selected todo"),
        Spans::from(" d        Delete selected todo"),
        Spans::from(""),
        Spans::from(" p        Change priority"),
        Spans::from(" /        Search todos"),
        Spans::from(" t        Toggle theme"),
        Spans::from(""),
        Spans::from(" ↑ / ↓    Navigate todos"),
        Spans::from(" Space    Toggle completed"),
        Spans::from(""),
        Spans::from(" ?        Toggle help"),
        Spans::from(" q        Quit application"),
        Spans::from(""),
        Spans::from(Span::styled(
            " Press Esc or q to close ",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let block = Paragraph::new(help)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    " Help ",
                    Style::default()
                        .fg(accent)
                        .add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent)),
        );

    f.render_widget(Clear, area);
    f.render_widget(block, area);
}

fn draw_confirm_popup<B: Backend>(f: &mut Frame<B>, accent: Color) {
    let area = centered_rect(50, 30, f.size());

    let block = Paragraph::new("Delete this todo?\n\n[y] Yes    [n] No")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    " Confirm ",
                    Style::default().fg(accent).add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent)),
        );

    f.render_widget(Clear, area);
    f.render_widget(block, area);
}

