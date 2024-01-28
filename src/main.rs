use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use state::App;
use std::io;

mod config;
mod state;
mod ui;

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // run app
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        loop {
            let event = event::read()?;
            if matches!(event, Event::Resize(_, _)) {
                break;
            }

            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Release {
                    continue;
                }

                match key.code {
                    KeyCode::Enter | KeyCode::Tab | KeyCode::Down => app.next_input(),
                    KeyCode::BackTab | KeyCode::Up => app.prev_input(),
                    #[cfg(debug_assertions)]
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(to_insert) => app.enter_char(to_insert),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Esc => app.reset(),
                    _ => continue,
                };
            }

            break;
        }
    }
}
