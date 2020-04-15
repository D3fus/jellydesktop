use tui::widgets::{Widget, Block, Tabs, Row, Table, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::Frame;
use crate::app::{App, ServerList, CreateServer};
use crate::util;

pub fn draw<B: Backend>(f: &mut Frame<B>, server: &mut App) {
  let chunks = Layout::default()
    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
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
    .style(Style::default().fg(Color::Green))
    .highlight_style(Style::default().fg(Color::Yellow));
  f.render(&mut tabs, chunks[0]);
  match server.server_state.clone().is_add() {
    true => {draw_add_server(f, server, chunks[1])},
    false => {
      let draw = server.server_state.draw;
      let serv = &mut server.clone().server_state.servers[draw];
      draw_server(f, serv, server, chunks[1])
    }
  }
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

  let mut create = server.create.clone();
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

fn draw_server<B>(f: &mut Frame<B>, server: &mut ServerList, app: &mut App, area: Rect)
  where
      B: Backend,
{
  let list = server.list.clone();
  let color = app.clone().window_focused(&server.name);
  let chunks = Layout::default()
    .constraints([Constraint::Length(10), Constraint::Length(3)].as_ref())
    .margin(1)
    .split(area);
  let mut block = Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(color))
    .title(&server.name);
  f.render(&mut block, area);
  match list {
    Some(l) => {
      let text: Vec<Text> = l.Items.iter().map(|item| {
        let t = format!("{}\n", item.Name);
        Text::styled(t, Style::default().fg(server.is_active(&item.Name)))
      }).collect();
      let mut p = Paragraph::new(text.iter()).wrap(true);
      f.render(&mut p, chunks[0])
    },
    None => {}
  }
}

