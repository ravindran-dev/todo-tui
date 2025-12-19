mod app;
mod storage;
mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let todos = storage::load();
    let mut app = App::new(todos);

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if app.confirm_delete {
                    match key.code {
                        KeyCode::Char('y') => {
                            app.delete();
                            app.confirm_delete = false;
                        }
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.confirm_delete = false;
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.show_help {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => app.show_help = false,
                        _ => {}
                    }
                    continue;
                }

                if app.search_mode {
                    match key.code {
                        KeyCode::Esc => app.search_mode = false,
                        KeyCode::Backspace => {
                            app.search_query.pop();
                        }
                        KeyCode::Char(c) => app.search_query.push(c),
                        _ => {}
                    }
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') if !app.input_mode => break,

                    KeyCode::Enter if app.input_mode && app.editing => app.submit_edit(),
                    KeyCode::Enter if app.input_mode => app.submit_input(),
                    KeyCode::Esc if app.input_mode => {
                        app.input.clear();
                        app.input_mode = false;
                        app.editing = false;
                    }
                    KeyCode::Backspace if app.input_mode => {
                        app.input.pop();
                    }
                    KeyCode::Char(c) if app.input_mode => {
                        app.input.push(c);
                    }

                    KeyCode::Char('a') => app.start_input(),
                    KeyCode::Char('e') => app.start_edit(),
                    KeyCode::Char('d') => app.confirm_delete = true,
                    KeyCode::Char(' ') => app.toggle(),
                    KeyCode::Char('p') => app.cycle_priority(),
                    KeyCode::Char('s') => app.start_search(),
                    KeyCode::Char('t') => app.toggle_theme(),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('?') => app.toggle_help(),

                    _ => {}
                }

                storage::save(&app.todos);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

