use ratatui::{prelude::*, widgets::*};

use crate::state::{App, Focus};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_username(f, app, chunks[1]);
    draw_password(f, app, chunks[2]);
}

fn draw_username<B: Backend>(f: &mut Frame<B>, app: &App, rect: Rect) {
    let style = if matches!(app.focus(), Focus::Username) {
        f.set_cursor(rect.x + app.cursor_position() as u16 + 1, rect.y + 1);
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    let input = Paragraph::new(app.username())
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(style)
                .title("Username"),
        );
    f.render_widget(input, rect);
}

fn draw_password<B: Backend>(f: &mut Frame<B>, app: &App, rect: Rect) {
    let style = if matches!(app.focus(), Focus::Password) {
        f.set_cursor(rect.x + app.cursor_position() as u16 + 1, rect.y + 1);
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    let input = Paragraph::new("*".repeat(app.password().len()))
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(style)
                .title("Password"),
        );
    f.render_widget(input, rect);
}
