use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use std::{error::Error, time::Duration};
use termion::{event::Key, input::MouseTerminal, screen::AlternateScreen};

mod app;
mod event;
mod ui;

use event::{Events, Event, Config};
use app::{App, ServerList};

fn main() -> Result<(), Box<dyn Error>> {
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
    let mut server = App::new("Server select", vec![ServerList {name: "f", uri: "f", user_name: "f", user_token: "f"}]);
    loop {
      terminal.draw(|mut f| ui::draw(&mut f, &mut server))?;
      match events.next()? {
        Event::Input(key) => match key {
          Key::Char(c) => {
            server.on_key(c);
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
