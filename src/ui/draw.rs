use tui::backend::Backend;
use tui::Frame;
//use tui::layout::{Layout, Direction};
//use tui::widgets::{Block, Borders, Paragraph, Text};
//use tui::style::{Color, Style};
use crate::app::{app};
use crate::ui::error::draw_error;
use crate::ui::server::draw_server;
use crate::ui::create_server::draw_create_server;
use crate::ui::config::draw_config;
use crate::ui::help::draw_help;
//use crate::util;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut app::App) {
    if app.has_error() {
        draw_error(frame, &mut app.error)
    } else {
        match app.active_page {
            3 => draw_help(frame),
            2 => draw_config(frame, app),
            1 => draw_create_server(frame, app),
            0 => draw_server(frame, app),
            _ => {}
        }
    }
    //if app.loading {
    //    println!("true ");
    //    let loading_chunk = Layout::default()
    //        .constraints(util::calc_mid(frame.size(), 'y', 4))
    //        .split(frame.size());
    //    let loading_chunk = Layout::default()
    //        .direction(Direction::Horizontal)
    //        .constraints(util::calc_mid(loading_chunk[1], 'x', 20))
    //        .split(loading_chunk[1]);
    //    let mut block = Block::default()
    //        .borders(Borders::ALL)
    //        .border_style(Style::default().fg(Color::Yellow));
    //    frame.render(&mut block, loading_chunk[1]);
    //    //let text = vec![Text::raw("Loading...")];
    //    //let mut p = Paragraph::new(text.iter()).block(block);
    //    //frame.render(&mut p, loading_chunk[1]);
    //}
}
