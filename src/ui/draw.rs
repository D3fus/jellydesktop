use tui::backend::Backend;
use tui::Frame;
use crate::app::{app};
use crate::ui::error::draw_error;
use crate::ui::server::draw_server;
use crate::ui::create_server::draw_create_server;
use crate::ui::config::draw_config;
use crate::ui::help::draw_help;

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
}
