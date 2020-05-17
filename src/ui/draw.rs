use tui::backend::Backend;
use tui::Frame;
use crate::app::{app, error};
use crate::ui::error::draw_error;
use crate::ui::server::draw_server;
use crate::ui::create_server::draw_create_server;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut app::App) {
    if app.has_error() {
        draw_error(frame, &mut app.error)
    } else {
        match app.active_server {
            -1 => draw_create_server(frame, app),
            _ => draw_server(frame, app)
        }
    }
}
