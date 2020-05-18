use tui::layout::{Layout, Constraint, Alignment, Rect, Direction};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::style::{Color, Style};
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
    draw_server_tab(frame, app, chunks[0]);
    draw_server_status(frame, app, chunks[2]);

    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)].as_ref())
        .split(chunks[1]);
    draw_server_view(frame, app, mid_chunks[0]);
    draw_server_list(frame, app, mid_chunks[1]);
}

fn draw_server_view<B: Backend>(frame: &mut Frame<B>, app: &mut app::App, area: Rect) {
    let server_name = app.active_server_name();
    let mut block = Block::default()
        .title(&server_name)
        .borders(Borders::ALL)
        .border_style(app.window_color(&server_name));
    let text: Vec<Text> = app.user_view.iter().enumerate().map(|(i, item)| {
        let t = format!("{}\n", item.name);
        Text::styled(t, app.cursor_color(i, &server_name))
    }).collect();
    let mut p = Paragraph::new(text.iter()).block(block);
    frame.render(&mut p, area);
}

fn draw_server_list<B: Backend>(frame: &mut Frame<B>, app: &mut app::App, area: Rect) {
    let mut block = Block::default()
        .title(&app.active_list)
        .borders(Borders::ALL)
        .border_style(app.window_color(&app.active_list));
    //frame.render(&mut block, area);
    let text: Vec<Text> = app.user_list.iter().enumerate().map(|(i, item)| {
        let t = format!("{}\n", item.name);
        Text::styled(t, app.cursor_color(i, &app.active_list))
    }).collect();
    let mut p = Paragraph::new(text.iter()).block(block);
    frame.render(&mut p, area);
}

fn draw_server_tab<B: Backend>(frame: &mut Frame<B>, app: &mut app::App, area: Rect) {
    let title = "servers";
    let mut block = Block::default()
        .title(&title)
        .borders(Borders::ALL)
        .border_style(app.window_color(&title));
    frame.render(&mut block, area);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(14)].as_ref())
        .margin(1)
        .horizontal_margin(2)
        .split(area);

    let server_name_list = app.get_servers_name();
    let mut server_text = Vec::new();
    for (i, item) in server_name_list.iter().enumerate() {
        server_text.push(Text::styled(item, app.cursor_color(i, &title)));
        if i < server_name_list.len() -1 {
            server_text.push(Text::raw(" | "));
        }
    }
    let mut server = Paragraph::new(server_text.iter());
    frame.render(&mut server, chunks[0]);

    let add_server = vec![
        Text::styled("| add Server +", app.cursor_color(server_name_list.len(), title))
    ];
    let mut add_server = Paragraph::new(add_server.iter()).wrap(true).alignment(Alignment::Right);
    frame.render(&mut add_server, chunks[1]);
}

fn draw_server_status<B: Backend>(frame: &mut Frame<B>, app: &mut app::App, area: Rect) {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    frame.render(&mut block, area);
}
