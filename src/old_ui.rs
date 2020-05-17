use tui::widgets::{Block, Gauge, Tabs, Row, Table, Borders, Paragraph, Text};
use tui::layout::{Layout, Alignment, Constraint, Direction, Rect};
use tui::style::{Color, Modifier, Style};
use tui::backend::Backend;
use tui::Frame;
use crate::app::{App, ServerList, Player};
use crate::models::config;
use crate::util;

pub fn draw_error<B: Backend>(f: &mut Frame<B>, error: &String) {
  let chunks = Layout::default()
    .constraints([Constraint::Percentage(40), Constraint::Percentage(20), Constraint::Percentage(40)].as_ref())
    .split(f.size());
  let mid_chunk = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(20), Constraint::Percentage(60), Constraint::Percentage(20)].as_ref())
    .split(chunks[1]);
  let block = Block::default()
    .borders(Borders::ALL)
    .title("Error")
    .border_style(Style::default().fg(Color::Red));
  let text = vec![Text::styled(error, Style::default().fg(Color::Red))];
  let mut p = Paragraph::new(text.iter())
    .wrap(true)
    .block(block)
    .alignment(Alignment::Center);
  f.render(&mut p, mid_chunk[1]);
}

pub fn draw<B: Backend>(f: &mut Frame<B>, server: &mut App, player: &mut Player, config: &mut config::Config) {
  if server.clone().show_server() {
    let chunks = Layout::default()
      .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
      .split(f.size());
    let names = server.clone().get_server_names();
    let color = server.clone().window_focused(&server.title);
    let mut tabs = Tabs::default()
      .block(
        Block::default()
        .borders(Borders::ALL)
        .title(&server.title)
        .border_style(Style::default().fg(color))
      )
      .titles(&names)
      .select(server.server_state.index)
      .style(Style::default().fg(Color::White))
      .highlight_style(Style::default().fg(Color::Blue));
    f.render(&mut tabs, chunks[0]);
    draw_status_bar(f, server, player, chunks[2]);
    match server.server_state.clone().is_add() {
      true => {draw_add_server(f, server, chunks[1])},
      false => {
        let draw = server.server_state.draw;
        let serv = &mut server.clone().server_state.servers[draw];
        draw_server(f, serv, server, chunks[1], player)
      }
    }
  } else {
    if server.show_help {
      draw_help_page(f, server)
    } else {
      draw_config_page(f, server, config, player)
    }
  }
}

fn format_bool_to_text<'b>(b: bool) -> Text<'b> {
  if b {
    Text::styled("ON", Style::default().fg(Color::Green))
  } else {
    Text::styled("OFF", Style::default().fg(Color::Red))
  }
}

fn get_style<'b>(position: usize, p2: usize) -> Style {
  if position == p2{
    Style::default().fg(Color::Blue)
  } else {
    Style::default().fg(Color::White)
  }
}

fn draw_config_page<B>(f: &mut Frame<B>, app: &mut App, config: &mut config::Config, player: &mut Player)
where
  B: Backend,
{
  let block = Block::default()
    .borders(Borders::ALL)
    .title("config")
    .border_style(Style::default().fg(Color::Blue));
  let text = [
    Text::styled("Mpv Volume: ", get_style(config.active, 0)),
    Text::raw(config.mpv_volume.clone().to_string()),
    Text::styled("\nAuto play Episode: ", get_style(config.active, 1)),
    format_bool_to_text(config.auto_play_episode.clone()),
    Text::styled("\nAuto play Movie: ", get_style(config.active, 2)),
    format_bool_to_text(config.auto_play_movie.clone())
  ];
  let mut p = Paragraph::new(text.iter()).wrap(false).block(block);
  f.render(&mut p, f.size())
}

fn draw_help_page<B>(f: &mut Frame<B>, app: &mut App)
where
  B: Backend,
{
  let chunk = Layout::default()
    .constraints([Constraint::Percentage(100)].as_ref())
    .split(f.size());
  let color = app.clone().window_focused(&"help".to_string());
  let mut block = Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(color))
    .title("help");
  f.render(&mut block, chunk[0]);
}

fn draw_status_bar<B>(f: &mut Frame<B>, app: &mut App, player: &mut Player, area: Rect)
where
  B: Backend,
{
  let mut block = Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::White));
  f.render(&mut block, area);
  let chunks = Layout::default()
    .constraints([Constraint::Percentage(33),Constraint::Percentage(33),Constraint::Percentage(33)].as_ref())
    .margin(1)
    .direction(Direction::Horizontal)
    .split(area);

  let text_left = [Text::raw("Config = c")];
  let mut p_left = Paragraph::new(text_left.iter()).wrap(false);

  let text_mid = [
    Text::raw("Autoplay = "),
    format_bool_to_text(app.clone().is_auto_play()),
    Text::raw(" | Volume = "),
    Text::styled(player.volume.to_string(), Style::default().fg(Color::Cyan))
  ];
  let mut p_mid = Paragraph::new(text_mid.iter()).wrap(false).alignment(Alignment::Center);

  let text_right = [Text::raw("Help = ?")];
  let mut p_right = Paragraph::new(text_right.iter()).wrap(false).alignment(Alignment::Right);

  f.render(&mut p_left, chunks[0]);
  f.render(&mut p_mid, chunks[1]);
  f.render(&mut p_right, chunks[2]);
}

fn draw_add_server<B>(f: &mut Frame<B>, server: &mut App, area: Rect)
  where 
      B: Backend,
{
  let chunks = Layout::default()
    .constraints([Constraint::Length(5), Constraint::Length(3)].as_ref())
    .margin(1)
    .split(area);
  let color = server.clone().window_focused(&"add server +".to_string());
  let mut block = Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(color))
    .title("Create new Server");
  f.render(&mut block, area);

  let create = server.create.clone();
  let text = [
    Text::styled("Server URL: ", Style::default().fg(create.clone().high("uri"))),
    Text::raw(create.clone().uri),
    Text::styled("\nUsername: ", Style::default().fg(create.clone().high("username"))),
    Text::raw(create.clone().username),
    Text::styled("\nPassword: ", Style::default().fg(create.clone().high("password"))),
    Text::raw(util::format_pw(create.clone().password)),
    Text::styled("\n\n<OK> ", Style::default().fg(create.clone().high("ok"))),
    Text::styled("<CANCEL> ", Style::default().fg(create.high("cancel"))),
  ];
  let mut p = Paragraph::new(text.iter()).wrap(true);
  f.render(&mut p, chunks[0])
}

fn draw_server<B>(f: &mut Frame<B>, server: &mut ServerList, app: &mut App, area: Rect, player: &mut Player)
  where
      B: Backend,
{
  let color_view = app.clone().window_focused(&server.name);
  let color_list = app.clone().window_focused(&server.list_name);
  let chunks = Layout::default()
    .constraints([Constraint::Length(25), Constraint::Length(3)].as_ref())
    .direction(Direction::Horizontal)
    .split(area);
  let mut view_block = Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(color_view))
    .title(&server.name);
  let mut list_block = Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(color_list))
    .title(&server.list_name);
  f.render(&mut view_block, chunks[0]);
  f.render(&mut list_block, chunks[1]);
  let view_chunk = Layout::default()
    .constraints([Constraint::Percentage(100), Constraint::Length(3)].as_ref())
    .margin(1)
    .split(chunks[0]);


  let view = server.view.clone();
  match view {
    Some(l) => {
      let text: Vec<Text> = l.Items.iter().map(|item| {
        let t = format!("{}\n", item.Name);
        Text::styled(t, Style::default().fg(server.clone().is_active_view(&item.Name)))
      }).collect();
      let mut p = Paragraph::new(text.iter()).wrap(true);
      f.render(&mut p, view_chunk[0])
    },
    None => {}
  }

  let list = server.list.clone();
  match list {
    Some(l) => {
      let mut rows = Vec::new();
      for item in l.Items {
        let color = server.clone().is_active_list(&item.Name);
        let to_play: String = {
          if item.UserData.Played {
            String::from("✓")
          } else {
            let play = item.UserData.UnplayedItemCount.clone();
            match play {
              Some(p) => {p.to_string()},
              None => {String::from("")}
            }
          }
        };
        let name;
        if item.IndexNumber.is_some(){
          name = format!("{}. {}", item.IndexNumber.clone().unwrap(), item.Name);
        } else {
          name = item.Name
        }
        rows.push(Row::StyledData(vec![name, to_play].into_iter(), Style::default().fg(color)));
      }
      let h = chunks[0].height as usize -5;
      if server.active_list > h {
        for i in 0..server.active_list as usize - h{
          rows.remove(0);
        }
      }
      let mut t = Table::new(["name", "to watcht/watcht"].iter(), rows.into_iter())
        .block(list_block)
        .widths(&[Constraint::Percentage(70), Constraint::Percentage(30)])
        .style(Style::default().fg(Color::White))
        .column_spacing(1);
      if player.list.len() == 0 || !player.is_fin_playing() || !player.auto_play || player.time_out == 0 {
        f.render(&mut t, chunks[1]);
      } else {
        let list_chunk = Layout::default()
          .constraints([Constraint::Percentage(90), Constraint::Percentage(30)].as_ref())
          .split(chunks[1]);
        let mut cancel_block = Block::default()
          .borders(Borders::ALL)
          .title("Auto Play");
        f.render(&mut cancel_block, list_chunk[1]);

        let a_chunk = Layout::default()
          .constraints([Constraint::Length(1), Constraint::Length(3)].as_ref())
          .margin(2)
          .split(list_chunk[1]);
        let label = format!("{} sec", (player.time_out as f32 * 0.2).ceil());
        let mut gauge = Gauge::default()
          .style(
            Style::default()
              .fg(Color::Magenta)
              .bg(Color::Black)
              .modifier(Modifier::ITALIC | Modifier::BOLD),
          )
        .label(&label)
        .ratio((player.time_out as f64 - 100.0) * -0.01);

        f.render(&mut gauge, a_chunk[0]);

        let s = app.server_state.servers[app.server_state.draw].clone();
        let text = format!(
          "Next playing: {}. {}",
          (s.active_list + 2).to_string(),
          s.list.unwrap().Items[s.active_list +1].Name);
        let text = [
          Text::raw(text),
          Text::raw("\n (s)top | play (n)ext")
        ];
        let mut p = Paragraph::new(text.iter()).wrap(false).alignment(Alignment::Center);
        f.render(&mut p, a_chunk[1]);

        f.render(&mut t, list_chunk[0]);
      }
    },
    None => {}
  }
}
