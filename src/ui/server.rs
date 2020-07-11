use tui::layout::{Layout, Constraint, Alignment, Rect, Direction};
use tui::widgets::{Block, Row, Gauge, Table, Borders, Paragraph, Text};
use tui::style::{Color, Modifier, Style};
use tui::backend::Backend;
use tui::Frame;
use crate::app::{app};
use crate::util;

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
        .constraints([Constraint::Length(25), Constraint::Min(0)].as_ref())
        .split(chunks[1]);
    draw_server_view(frame, app, mid_chunks[0]);
    if app.player.auto_play_timeout > 0 && !app.player.playing && !app.player.list.is_empty() {
        let auto_play_chunks = Layout::default()
            .constraints([Constraint::Min(0), Constraint::Length(7)].as_ref())
            .split(mid_chunks[1]);
        draw_server_list(frame, app, auto_play_chunks[0]);
        draw_server_autoplay(frame, app, auto_play_chunks[1]);
    } else {
        draw_server_list(frame, app, mid_chunks[1]);
    }
}

fn draw_server_view<B: Backend>(frame: &mut Frame<B>, app: &mut app::App, area: Rect) {
    let server_name = app.active_server_name();
    let block = Block::default()
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

    if app.user_list.is_empty() {
        frame.render(&mut block, area);
    } else {
        let mut rows = Vec::new();
        let mut max_len = 4;
        let width = area.width as usize;
        for (i, item) in app.user_list.iter().enumerate() {
            let mut name;
            let t;
            let mut watch = String::from("");
            if item.category == "Movie" || item.category == "Episode" {
                if item.category == "Episode" {
                    name = format!("{}. {}", item.index_nummer, item.name);
                } else {
                    name = item.name.clone();
                }
                t = String::from(" ▶️");

                if item.played {
                    watch = String::from("✓");
                }
            } else {
                name = item.name.clone();
                t = String::from(" 📁");

                if item.unplayed > 0 {
                    watch = item.unplayed.to_string();
                } else {
                    watch = String::from("✓");
                }
            }
            if name.len() > max_len && name.len() + 16 < width {
                max_len = name.len() + 2;
            } else if name.len() + 16 >= width {
                name = util::format_long_name(name, width -16);
                max_len = name.len();
            }
            rows.push(Row::StyledData(
                vec![t, name, watch].into_iter(), app.cursor_color(i, &app.active_list)));
        }

        if app.cursor > area.height as usize -5 {
            let at = app.cursor - (area.height as usize - 5);
            rows = rows.split_off(at);
            //rows = row.to_vec();
        }
        let widths = vec![
            Constraint::Length(4),
            Constraint::Length(max_len as u16),
            Constraint::Min(0)
        ];
        let mut table = Table::new(["type", "name", "to watch"].iter(), rows.into_iter())
            .block(block)
            .widths(&widths)
            .style(Style::default().fg(Color::White))
            .column_spacing(1);
        frame.render(&mut table, area);
    }
}

fn draw_server_autoplay<B: Backend>(frame: &mut Frame<B>, app: &mut app::App, area: Rect) {
    let mut block = Block::default()
        .title("Autoplay")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    frame.render(&mut block, area);

    let chunks = Layout::default()
        .constraints([Constraint::Length(1), Constraint::Length(2)].as_ref())
        .margin(2)
        .split(area);

    let label = format!("{} sec", (app.player.auto_play_timeout as f32 * 0.2).ceil());
    let mut gauge = Gauge::default()
        .style(
        Style::default()
            .fg(Color::Magenta)
            .bg(Color::Black)
            .modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(&label)
        .ratio((app.player.auto_play_timeout as f64 - 100.0) * -0.01);
    frame.render(&mut gauge, chunks[0]);

    let name = if app.auto_play_list.is_empty(){
        app.user_list[app.player.index].name.clone()
    } else {
        app.auto_play_list[0].name.clone()
    };
    //TODO this is bullshit
    // fix Episode number
    let mut text = format!(
        "Next playing: {}. {}",
        //app.player.index,
        (app.player.list[0].index_nummer).to_string(),
        name);
    let width = chunks[1].width as usize;
    if text.len() >= width {
        text = util::format_long_name(text, width -4);
    }
    let text = [
        Text::raw(text),
        Text::raw("\n (s)top | play (n)ext")
    ];
    let mut p = Paragraph::new(text.iter()).wrap(false).alignment(Alignment::Center);
    frame.render(&mut p, chunks[1]);
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

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Min(0), Constraint::Length(9)].as_ref())
        .margin(1)
        .split(area);
    let config_text = vec![Text::raw(" Config = c")];
    let mut config_p = Paragraph::new(config_text.iter());
    frame.render(&mut config_p, chunks[0]);

    let autoplay = if app.user_list.is_empty() {
        false
    } else {
        if app.user_list[0].category == "Episode" {
            app.config.auto_play_episode
        } else if app.user_list[0].category == "Movie" {
            app.config.auto_play_movie
        } else {
            false
        }
    };
    let text = if chunks[1].width > 49 {
        vec!["Volume = ", " | Fullscreen = ", " | Autoplay = "]
    } else {
        vec!["V = ", " | F = ", " | A = "]
    };
    let status_text = vec![
        Text::raw(text[0]),
        Text::styled(app.config.mpv_volume.to_string(), Style::default().fg(Color::Cyan)),
        Text::raw(text[1]),
        util::format_bool(app.config.mpv_full_screen),
        Text::raw(text[2]),
        util::format_bool(autoplay)
    ];
    let mut status_p = Paragraph::new(status_text.iter()).alignment(Alignment::Center);
    frame.render(&mut status_p, chunks[1]);

    let help_text = vec![Text::raw("Help = ?")];
    let mut help_p = Paragraph::new(help_text.iter());
    frame.render(&mut help_p, chunks[2]);
}
