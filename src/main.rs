use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use std::{error::Error, time::Duration};
use termion::{event::Key, input::MouseTerminal, screen::AlternateScreen};

mod app;
mod api;
mod event;
mod util;
mod ui;
mod models;

use event::{Events, Event, Config};
use app::{App, Player};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let events = Events::with_config(Config {
      tick_rate: Duration::from_millis(200),
      ..Config::default()
    });
    let mut server = App::new("Server select".to_string(), vec![]);
    let mut player = Player::new();
    if util::exists_config() {
      let serv = util::read_server_from_config();
      for s in serv.server {
        server.server_state.servers.push(s.clone());
      } 
    }
    loop {
      if server.clone().to_play() {
        player.add(&server.clone().get_dawn_server());
        server.del_to_play();
      }
      player.play_if_ready().await;
      terminal.draw(|mut f| ui::draw(&mut f, &mut server, &mut player))?;
      match events.next()? {
        Event::Input(key) => match key {
          Key::Char(c) => {
            server.on_key(c).await;
          },
          Key::Left => {
            server.on_left();
          },
          Key::Right => {
            server.on_right();
          },
          Key::Up => {
            server.on_up();
          },
          Key::Down => {
            server.on_down();
          },
          Key::Backspace => {
            server.on_backspace();
          },
          _ => {}
        },
        Event::Tick => {
          
        }
      };

      if server.should_quit {
        break;
      }

    }

    Ok(())
}
