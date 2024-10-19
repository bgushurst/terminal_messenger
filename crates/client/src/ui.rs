use crate::app::{App, CurrentScreen};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

mod chat;
mod disconnected;
mod exiting;
mod help;
mod login;
mod set_user;

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::LoggingIn => login::render_login(frame, app),
        CurrentScreen::Main => chat::render_chat(frame, app),
        CurrentScreen::ComposingMessage => chat::render_chat(frame, app),
        CurrentScreen::HelpMenu => help::render_help(frame),
        CurrentScreen::Exiting => exiting::render_exiting(frame),
        CurrentScreen::Disconnected => disconnected::render_disconnected(frame),
        CurrentScreen::SetUser => set_user::render_set_user(frame, app),
        _ => {} // Handle other screens if needed
    }
}

// Helper function to create a centered rect
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
