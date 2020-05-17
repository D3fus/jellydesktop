use tui::layout::{Layout, Constraint, Rect, Alignment, Direction};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::Frame;
use crate::app::error;
use crate::util;

pub fn draw_error<B: Backend>(frame: &mut Frame<B>, error: &mut error::Error) {
    let chunks = Layout::default()
        .constraints(util::calc_mid(frame.size(), 'y', 4))
        .split(frame.size());
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(util::calc_mid(chunks[1], 'x', error.error.len() as u16 + 7))
        .split(chunks[1]);
    let mut block = Block::default()
        .title("error")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    frame.render(&mut block, chunks[1]);
    let margin = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .margin(1)
        .split(chunks[1]);
    let text = vec![
        Text::raw(error.error.clone())
    ];
    let mut p = Paragraph::new(text.iter()).alignment(Alignment::Center);
    frame.render(&mut p, margin[0])
}
