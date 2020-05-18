use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use std::{error::Error, time::Duration};
use termion::{event::Key, input::MouseTerminal, screen::AlternateScreen};

mod app;
mod util;
mod event;
mod ui;
mod api;

//mod app;
//mod api;
//mod event;
//mod util;
//mod ui;
mod models;

use event::{Events, Event, Config};
use app::app::App;
//use models::config;

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

    let mut app = App::new().await;
    loop {
        terminal.draw(|mut frame| ui::draw::draw(&mut frame, &mut app))?;
        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => {
                    app.on_key(c).await;
                },
                Key::Up => {
                    if !app.input_mode {
                        app.on_key_up();
                    }
                },
                Key::Down => {
                    if !app.input_mode {
                        app.on_key_down();
                    }
                },
                Key::Backspace => {
                    app.on_key_backspace();
                }
                _ => {}
            }
            _ => {}
        }
        if app.quit {
            //TODO safe config bevore quiting
            break
        }
    }

    //let mut config = match config::Config::read() {
    //  Ok(c) => c,
    //  Err(e) => {
    //    return error(&e, &mut terminal, &events);
    //  }
    //};
    //let mut server = App::new("Server select".to_string(), &config);
    //let mut player = Player::new(false, config.mpv_volume.clone());

    //loop {
    //  if server.clone().to_play() {
    //    player.auto_play = server.clone().is_auto_play();
    //    player.add(&server.clone().get_dawn_server());
    //    server.del_to_play();
    //  }
    //  player.play_if_ready().await;

    //  terminal.draw(|mut f| ui::draw(&mut f, &mut server, &mut player, &mut config))?;
    //  match events.next()? {
    //    Event::Input(key) => match key {
    //      Key::Char(c) => {
    //        player.on_key(c);
    //        if server.show_config {
    //          match config.on_key(c, &mut server) {
    //            Ok(()) => {},
    //            Err(e) => {server.error = e}
    //          };
    //        } else {
    //          server.on_key(c, config.clone()).await;
    //        }
    //      },
    //      Key::Left => {
    //        server.on_left();
    //      },
    //      Key::Right => {
    //        server.on_right();
    //      },
    //      Key::Up => {
    //        if server.show_config {
    //          config.on_up();
    //        } else {
    //          server.on_up();
    //        }
    //      },
    //      Key::Down => {
    //        if server.show_config {
    //          config.on_down()
    //        } else {
    //          server.on_down();
    //        }
    //      },
    //      Key::Backspace => {
    //        if server.show_config {
    //          config.on_backspace(&mut server);
    //        }else {
    //          server.on_backspace();
    //        }
    //      },
    //      _ => {}
    //    },
    //    Event::Tick => {
    //
    //    }
    //  };

    //  if server.should_quit {
    //    break;
    //  }

    //}

    Ok(())
}

//use tui::backend::Backend;
//
//fn error<B: Backend>(error: &String, t: &mut Terminal<B>, e: &Events) -> Result<(), Box<dyn Error>>{
//  loop {
//    t.draw(|mut f| ui::draw_error(&mut f, error))?;
//    match e.next()? {
//      Event::Input(key) => match key {
//        Key::Char(c) => {
//          match c {
//            'q' => {break},
//            '\n' => {break},
//            _ => {}
//          }
//        },
//        _ => {}
//      },
//      _ => {}
//    }
//  }
//  Ok(())
//}
