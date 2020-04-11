use tui::widgets::{Widget, Block, Tabs, Row, Table, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::Frame;
use crate::app::{App, ServerList, CreateServer};

pub fn draw<B: Backend>(f: &mut Frame<B>, server: &mut App) {
  let chunks = Layout::default()
    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
    .split(f.size());
  let names = server.clone().get_server_names();
  let color = server.clone().window_focused(server.title);
  let mut tabs = Tabs::default()
    .block(
      Block::default()
      .borders(Borders::ALL)
      .title(server.title)
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
      let mut serv = server.server_state.servers[draw];
      draw_server(f, &mut serv, chunks[1])
    }
  }
}

fn draw_add_server<B>(f: &mut Frame<B>, server: &mut App, area: Rect)
  where 
      B: Backend,
{
  let chunks = Layout::default()
    .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
    .margin(1)
    .split(area);
  let color = server.clone().window_focused("add server +");
  let mut block = Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(color))
    .title("Create new Server");
  f.render(&mut block, area);

  let create = server.create.clone();
  let text = [
    Text::styled("Server URL: ", Style::default().fg(create.clone().high("uri"))),
    Text::raw(create.username),
    Text::styled("\nUsername: ", Style::default().fg(create.clone().high("username"))),
    Text::raw(".."),
    Text::styled("\nPassword: ", Style::default().fg(create.high("password"))),
    Text::raw("..")
  ];
  let mut p = Paragraph::new(text.iter()).wrap(true);
  f.render(&mut p, chunks[0])
}

fn draw_server<B>(f: &mut Frame<B>, server: &mut ServerList, area: Rect)
  where
      B: Backend,
{
  let chunks = Layout::default()
    .constraints([Constraint::Length(2), Constraint::Length(3)].as_ref())
    .margin(1)
    .split(area);
  let mut block = Block::default().borders(Borders::ALL).title(server.name);
  f.render(&mut block, area);
}

