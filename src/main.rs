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
                match key.code {
                    // Quit
                    KeyCode::Char('q') if !app.input_mode => break,

                    // ----- INPUT MODE -----
                    KeyCode::Enter if app.input_mode => app.submit_input(),
                    KeyCode::Esc if app.input_mode => {
                        app.input.clear();
                        app.input_mode = false;
                    }
                    KeyCode::Backspace if app.input_mode => {
                        app.input.pop();
                    }
                    KeyCode::Char(c) if app.input_mode => {
                        app.input.push(c);
                    }

                    // ----- NORMAL MODE -----
                    KeyCode::Char('a') if !app.input_mode => app.start_input(),
                    KeyCode::Char('d') if !app.input_mode => app.delete(),
                    KeyCode::Char(' ') if !app.input_mode => app.toggle(),
                    KeyCode::Down if !app.input_mode => app.next(),
                    KeyCode::Up if !app.input_mode => app.previous(),

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
