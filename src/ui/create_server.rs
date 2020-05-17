use tui::layout::{Layout, Constraint, Alignment, Rect, Direction};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::backend::Backend;
use tui::Frame;
use crate::app::{app};
use crate::util;

pub fn draw_create_server<B: Backend>(frame: &mut Frame<B>, app: &mut app::App) {
    let chunks = Layout::default()
        .constraints(util::calc_mid(frame.size(), 'y', 10))
        .margin(1)
        .split(frame.size());
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(util::calc_mid(chunks[1], 'x', 50))
        .split(chunks[1]);
    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .title("Create new Server");
    frame.render(&mut block, chunks[1]);

    let margin = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Length(1)].as_ref())
        .margin(2)
        .split(chunks[1]);
    let text = [
        Text::styled("Server URL: ", app.cursor_color(0)),
        Text::raw(app.create_server.get('u')),
        Text::styled("\nUsername: ", app.cursor_color(1)),
        Text::raw(app.create_server.get('n')),
        Text::styled("\nPassword: ", app.cursor_color(2)),
        Text::raw(app.create_server.get('p')),
    ];
    let mut p = Paragraph::new(text.iter()).wrap(true);
    frame.render(&mut p, margin[0]);

    let text = [
        Text::styled("\n <OK>      ", app.cursor_color(3)),
        Text::styled("       <CANCEL>", app.cursor_color(4)),
    ];
    let mut p = Paragraph::new(text.iter()).wrap(true).alignment(Alignment::Center);
    frame.render(&mut p, margin[1]);
}
