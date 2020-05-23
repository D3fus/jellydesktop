use tui::layout::{Layout, Constraint, Alignment, Direction};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::Frame;
use crate::util;
use crate::app::{app};

pub fn draw_config<B: Backend>(frame: &mut Frame<B>, app: &mut app::App) {
    let title = "config";
    let mut block = Block::default()
        .title(&title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    let mid_chunk = Layout::default()
        .constraints(util::calc_mid(frame.size(), 'y', 8))
        .split(frame.size());
    let mid_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(util::calc_mid(mid_chunk[1], 'x', 27))
        .split(mid_chunk[1]);
    frame.render(&mut block, mid_chunk[1]);

    let text_chunk = Layout::default()
        .constraints([Constraint::Length(4), Constraint::Length(1)].as_ref())
        .margin(1)
        .split(mid_chunk[1]);

    let text = vec![
        Text::styled(" Mpv volume:        ", app.cursor_color(0, &title)),
        Text::raw(app.config.mpv_volume.to_string()),
        Text::styled("\n Mpv fullscreen:    ", app.cursor_color(1, &title)),
        util::format_bool(app.config.mpv_full_screen),
        Text::styled("\n Autoplay episode:  ", app.cursor_color(2, &title)),
        util::format_bool(app.config.auto_play_episode),
        Text::styled("\n Autoplay movie:    ", app.cursor_color(3, &title)),
        util::format_bool(app.config.auto_play_movie)
    ];
    let mut p = Paragraph::new(text.iter());
    frame.render(&mut p, text_chunk[0]);

    let button_text = vec![
        Text::styled("<SAVE>    ", app.cursor_color(4, &title)),
        Text::styled("    <CANCEL>", app.cursor_color(5, &title))
    ];
    let mut button_p = Paragraph::new(button_text.iter()).alignment(Alignment::Center);
    frame.render(&mut button_p, text_chunk[1]);
}
