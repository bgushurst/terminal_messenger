// ui/help.rs
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render_help(frame: &mut Frame) {
    frame.render_widget(ratatui::widgets::Clear, frame.area());

    frame.render_widget(Clear, frame.area());
    let help_menu_block = Block::default()
        .title("Help Menu")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));
    let help_menu_text = Text::styled(
        "(q) to quit\n(n) to set username",
        Style::default().fg(Color::Red),
    );
    let help_menu_paragraph = Paragraph::new(help_menu_text)
        .block(help_menu_block)
        .wrap(Wrap { trim: false });
    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(help_menu_paragraph, area);
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
