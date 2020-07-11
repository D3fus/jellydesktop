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

mod models;

use event::{Events, Event, Config};
use app::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let mut events = Events::with_config(Config {
      tick_rate: Duration::from_millis(200),
      ..Config::default()
    });
    events.disable_exit_key();

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
                    app.on_key_backspace().await;
                }
                _ => {}
            }
            _ => {}
        }
        if app.player.ready_to_play() {
            if app.player.auto_play_timeout == 0 {
                app.mark_as_seen().await;
            }
            app.player.play(app.config.mpv_volume, app.config.mpv_full_screen);
        }
        if app.quit {
            //TODO safe config bevore quiting
            break
        }
    }

    Ok(())
}

