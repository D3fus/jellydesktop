use tui::layout::{Layout, Constraint, Alignment, Direction};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::Frame;
use crate::app::{app};
use crate::util;

pub fn draw_help<B: Backend>(frame: &mut Frame<B>, app: &mut app::App) {
    let block = Block::default()
        .title("help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    let mid_chunk = Layout::default()
        .constraints(util::calc_mid(frame.size(), 'y', 25))
        .split(frame.size());
    let mid_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(util::calc_mid(mid_chunk[1], 'x', 50))
        .split(mid_chunk[1]);
    let text = vec![
        Text::raw("--Navigate--"),
        Text::raw("\ncursor:"),
        Text::raw("\n j: down"),
        Text::raw("\n k: up"),
        Text::raw("\n h: left"),
        Text::raw("\n l: right"),
        Text::raw("\n\nwindow:"),
        Text::raw("\n shift + j: down"),
        Text::raw("\n shift + k: up"),
        Text::raw("\n shift + h: left"),
        Text::raw("\n shift + l: right"),
        Text::raw("\n\nEnter: change inpude mode or load contend"),
        Text::raw("\np: Play all not watched items in this folder"),
        Text::raw("\nshift + p: Play all items in this folder"),
        Text::raw("\n\n--Autoplay--"),
        Text::raw("\ns: stop autoplay"),
        Text::raw("\nn: play next")
    ];
    let mut p = Paragraph::new(text.iter()).block(block);
    frame.render(&mut p, mid_chunk[1]);
}
