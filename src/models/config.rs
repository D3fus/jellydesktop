use crate::app;
use serde::{Deserialize, Serialize};
use std::fs::File;
use serde_json;
use std::path::Path;
use std::io::BufReader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  #[serde(default = "default_count")]
  pub count: i32,
  #[serde(default = "default_server")]
  pub server: Vec<app::ServerList>,
  #[serde(default = "default_auto_play_episode")]
  pub auto_play_episode: bool,
  #[serde(default = "default_auto_play_movie")]
  pub auto_play_movie: bool,
  #[serde(default = "default_mpv_volume")]
  pub mpv_volume: String,
  #[serde(skip)]
  pub active: usize
}

fn default_count() -> i32 {
  0 as i32
}

fn default_server() -> Vec<app::ServerList> {
  vec![]
}

fn default_auto_play_episode() -> bool {
  true
}

fn default_auto_play_movie() -> bool {
  false
}

fn default_mpv_volume() -> String {
  String::from("100")
}

impl Config {
  pub fn read() -> Config {
      if !Path::new("config.json").exists() {
          File::create("config.json");
      }
      let file = File::open("config.json").unwrap();
      let reader = BufReader::new(file);
      let j: Config = serde_json::from_reader(reader).unwrap();
      j
  }

  pub fn add_server(&mut self, server: app::ServerList) -> Result<(), std::io::Error> {
      let s = self.server.iter().position(|s| s.name == server.name);
      if s.is_none() {
          self.server.push(server);
          serde_json::to_writer(&File::create("config.json")?, &self);
      }
      Ok(())
  }

  pub fn update(self) -> Result<(), std::io::Error> {
    serde_json::to_writer(&File::create("config.json")?, &self);
    Ok(())
  }

  fn on_enter(&mut self, app: &mut app::App){
    match self.active {
      0 => {
        app.input_mode = !app.input_mode;
        app.config.mpv_volume = self.mpv_volume.clone();
        self.clone().update();
      },
      1 => {
        self.auto_play_episode = !self.auto_play_episode;
        app.config.auto_play_episode = self.auto_play_episode;
        self.clone().update();
      },
      2 => {
        self.auto_play_movie = !self.auto_play_movie;
        app.config.auto_play_movie = self.auto_play_movie;
        self.clone().update();
      },
      _ => {}
    }
  }

  fn add_to_volume(&mut self, c: char) {
    self.mpv_volume.push(c);
  }

  pub fn on_key(&mut self, c: char, app: &mut app::App) {
    if app.input_mode {
      match c as u32 {
        48..= 57 => {
          self.add_to_volume(c);
        },
        10 => {
          self.on_enter(app);
        },
        _ => {}
      }
    } else {
      match c {
        '\n' => {
          self.on_enter(app);
        },
        'j' => {
          self.on_down();
        },
        'k' => {
          self.on_up();
        },
        'c' => {
          app.show_config = false;
        },
        'q' => {
          app.should_quit = true;
        }
        _ => {}
      }
    }
  }

  pub fn on_backspace(&mut self, app: &mut app::App) {
    if app.input_mode && self.mpv_volume.len() > 0{
      self.mpv_volume.truncate(self.mpv_volume.len() -1)
    }
  }

  pub fn on_up(&mut self) {
    if self.active > 0 {
      self.active -= 1;
    } else {
      self.active = 2;
    }
  }

  pub fn on_down(&mut self) {
    if self.active == 2 {
      self.active = 0;
    } else {
      self.active += 1;
    }
  }
}
