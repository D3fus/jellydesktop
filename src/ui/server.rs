use tui::layout::{Layout, Constraint, Rect};
use tui::backend::Backend;
use tui::Frame;
use crate::app::{app};

pub fn draw_server<B: Backend>(frame: &mut Frame<B>, app: &mut app::App) {
    let chunks = Layout::default()
        .constraints(
            [Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3)].as_ref())
        .split(frame.size());
    draw_server_tab(frame, app, chunks[0])
}

fn draw_server_tab<B: Backend>(frame: &mut Frame<B>, app: &mut app::App, area: Rect) {
}

fn draw_server_status<B: Backend>(app: app::App, area: Rect) {

}
