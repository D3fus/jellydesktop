use tui::style::{Color};
use crate::api;
use crate::models::{user, query, config};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::process::Child;

pub struct Player {
  pub player: Child,
  pub list: Vec<String>,
  pub user: Option<user::Authentication>,
  pub start_playing: bool,
  pub uri: String,
  pub device_id: String,
  pub auto_play: bool,
  pub volume: String,
  pub time_out: usize,
}

impl Player {
  pub fn new(auto_play: bool, volume: String) -> Player {
      Player {
        player: Command::new("echo").spawn().unwrap(),
        list: vec![],
        user: None,
        uri: String::from(""),
        device_id: String::from(""),
        start_playing: false,
        auto_play: auto_play,
        volume: volume,
        time_out: 0,
      }
    }

  pub fn on_key(&mut self, c: char) {
    match c {
      's' => {
        self.stop_auto_play();
      },
      'n' => {
        self.skip_auto_play();
      },
      _ => {}
    }
  }

  pub fn skip_auto_play(&mut self) {
    self.time_out = 0;
  }

    pub fn add(&mut self, server: &ServerList) {
      self.user = server.user.clone();
      if self.auto_play {
        self.list = server.to_play.clone();
      } else {
        self.list = vec![server.to_play[0].clone()];
      }
      self.uri = server.uri.clone();
      self.device_id = server.device_id.clone();
    }

  pub async fn play_if_ready(&mut self) {
    if !self.start_playing && self.list.len() > 0 {
      self.start_playing = true;
    } else if self.start_playing && self.list.len() > 0{
      if self.is_fin_playing() {
        if self.time_out == 0 {
          self.play().await;
        } else {
          self.time_out -= 1;
        }
      }
    }

  }

  pub fn stop_auto_play(&mut self) {
    self.list = vec![];
    self.time_out = 0;
  }

  pub async fn play(&mut self) {
    let base = format!(
      "{}/Items/{}/Download?api_key={}",
      self.uri,
      self.list[0],
      self.user.clone().unwrap().AccessToken
    );
    //TODO error if not installed
    self.player = Command::new("mpv")
      .args(&[
        base,
        "--really-quiet".to_string(),
        format!("--volume={}", self.volume)
      ])
      .spawn()
      .unwrap();
    let re = api::has_played(&self.uri, self.user.clone().unwrap(), &self.list[0], &self.device_id).await;
    match re {
      Ok(_e) => {},
      Err(e) => {println!("{:?}", e)}
    }
    self.list.remove(0);
    self.time_out = 100;
  }

  pub fn is_fin_playing(&mut self) -> bool {
    match self.player.try_wait() {
      Ok(Some(_s)) => {true},
      Ok(None) => {false}
      Err(_e) => {false},
    }
  }
}

  #[derive(Debug, Clone)]
  pub struct ServerState {
    pub servers: Vec<ServerList>,
    pub index: usize,
    pub draw: usize
  }

  impl ServerState {
    pub fn new(mut server: Vec<ServerList>) -> ServerState {
      let mut x = vec![ServerList {
        uri: "".to_string(),
        name: "add server +".to_string(),
        user: None,
        device_id: "".to_string(),
        list: None,
        view: None,
        list_name: String::from(""),
        active_view: 0,
        active_list: 0,
        to_play: vec![]
      }];
      for serv in server{
        x.push(serv);
      }
      ServerState {
        servers: x, index: 0, draw: 0
      }
    }

    pub fn next(&mut self) {
      if self.index < self.servers.len() -1 {
        self.index += 1;
      }
    }

    pub fn previous(&mut self) {
      if self.index > 0 {
        self.index -= 1;
      }
    }

    pub fn is_add(self) -> bool {
      self.draw == 0
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct ServerList {
    pub uri: String,
    pub name: String,
    pub user: Option<user::Authentication>,
    pub device_id: String,
    #[serde(skip)]
    pub list: Option<query::QueryResult>,
    #[serde(skip)]
    pub view: Option<query::QueryResult>,
    #[serde(skip)]
    pub list_name: String,
    #[serde(skip)]
    pub active_view: usize,
    #[serde(skip)]
    pub active_list: usize,
    #[serde(skip)]
    pub to_play: Vec<String>
  }

  impl ServerList {
    pub fn new(user: user::Authentication, server: &CreateServer, id: String) -> ServerList {
      let name = server.uri.split("https://").collect();
      ServerList {
        uri: server.uri.clone(),
        name: name,
        user: Some(user),
        device_id: id,
        list: None,
        view: None,
        list_name: String::from(""),
        active_view: 0,
        active_list: 0,
        to_play: vec![]
      }
    }

    pub async fn get_server_view(&mut self) -> Result<(), String> {
      api::get_view(self).await?;
      Ok(())
    }

    pub fn add_view(&mut self, view: query::QueryResult) {
      self.view = Some(view);
    }

    pub fn add_list(&mut self, list: query::QueryResult) {
      self.list = Some(list);
    }

    pub fn is_active_view(&mut self, name: &String) -> Color {
      if name == &self.view.as_ref().unwrap().Items[self.active_view].Name {
        Color::Blue
      } else {
        Color::White
      }
    }

    pub fn is_active_list(&mut self, name: &String) -> Color {
      if name == &self.list.as_ref().unwrap().Items[self.active_list].Name {
        Color::Blue
      } else {
        Color::White
      }
    }

    pub fn on_up_view(&mut self) {
      if self.active_view == 0 {
        self.active_view = self.view.as_ref().unwrap().Items.len() -1;
      } else {
        self.active_view -= 1;
      }
    }

    pub fn on_down_view(&mut self) {
      if self.active_view == self.view.as_ref().unwrap().Items.len() -1 {
        self.active_view = 0;
      } else {
        self.active_view += 1;
      }
    }

    pub fn on_up_list(&mut self) {
      if self.active_list == 0 {
        self.active_list = self.list.as_ref().unwrap().Items.len() -1;
      } else {
        self.active_list -= 1;
      }
    }

    pub fn on_down_list(&mut self) {
      if self.active_list == self.list.as_ref().unwrap().Items.len() -1 {
        self.active_list = 0;
      } else {
        self.active_list += 1;
      }
    }

    pub async fn on_enter_view(&mut self) -> Result<(), String> {
        let item = &self.view.clone().unwrap().Items[self.active_view];
        api::get_item(self, item).await?;
        self.active_list = 0;
        self.list_name = item.Name.clone();
        Ok(())
    }

    pub async fn on_enter_list(&mut self) -> Result<(), String> {
      let t = &self.list.clone().unwrap().Items[self.active_list].Type;
      if t == "Movie" || t == "Episode" {
        for item in self.list.clone().unwrap().Items {
          let p = self.list.clone().unwrap().Items.iter().position(|i| i.Name == item.Name).unwrap();
          if p >= self.active_list{
            self.to_play.push(item.Id);
          }
        }
      } else {
        let item = &self.list.clone().unwrap().Items[self.active_list];
        api::get_item(self, item).await?;
        self.active_list = 0;
      }
      Ok(())
    }
  }

#[derive(Clone)]
pub struct AppConfig {
  pub auto_play_episode: bool,
  pub auto_play_movie: bool,
  pub mpv_volume: String
}

impl AppConfig {
  pub fn new(c: &config::Config) -> AppConfig {
    AppConfig {
      auto_play_movie: c.auto_play_movie,
      auto_play_episode: c.auto_play_episode,
      mpv_volume: c.mpv_volume.clone()
    }
  }
}

  #[derive(Clone)]
  pub struct App {
    pub title: String,
    pub server_state: ServerState,
    pub should_quit: bool,
    pub input_mode: bool,
    pub select_window: String,
    pub create: CreateServer,
    pub show_help: bool,
    pub show_config: bool,
    pub config: AppConfig,
    pub error: String,
  }

  impl App {
    pub fn new(title: String, config: &config::Config) -> App {
      let servers = config.server.clone();
      App {
        title: title,
        server_state: ServerState::new(servers),
        should_quit: false,
        input_mode: false,
        select_window: "Server select".to_string(),
        create: CreateServer::new(),
        show_help: false,
        show_config: false,
        config: AppConfig::new(config),
        error: String::from("")
      }
    }

    pub fn is_auto_play(self) -> bool {
      let t = self.server_state.servers[self.server_state.draw].list.clone();
      match t {
        Some(l) => {
          let x = l.Items[0].Type.clone();
          if x == "Movie" {
            self.config.auto_play_movie
          }else if x == "Episode" {
            self.config.auto_play_episode
          }else {
            false
          }
        },
        None => false
      }
    }

    pub fn show_server(self) -> bool {
      if self.show_config || self.show_help{
        false
      } else {
        true
      }
    }

    pub async fn on_key(&mut self, c: char, config: config::Config) {
      if self.input_mode {
        if &self.select_window == "add server +" {
          match c {
            '\n' => {
              self.on_enter(config).await;
            },
            _ => {
              self.create.input(c);
            }
          }
        }
      } else {
        match c {
          //quit app
          'q' => {
            self.should_quit = true;
          },
          //navigation in window
          'l' => {
            self.on_right();
          },
          'h' => {
            self.on_left();
          },
          'k' => {
            if !self.show_config{
              self.on_up();
            }
          },
          'j' => {
            if !self.show_config{
              self.on_down();
            }
          },
          //navigate windows
          'J' => {
            self.win_down();
          },
          'K' => {
            self.win_up();
          },
          'H' => {
            self.win_left();
          },
          'L' => {
            self.win_right();
          },
          //set whatchted or unwatched
          'w' => {
           
          },
          //open config/help
          '?' => {
            self.change_draw_mode('?');
          },
          'c' => {
            self.change_draw_mode('c');
          },
          //turn auto play on/off
          'a' => {
            self.change_auto_play();
          }
          //enter
          '\n' => {
            if !self.show_config{
              self.on_enter(config).await;
            }
          }
          _ => {}
        }
      }
    }

    pub fn change_auto_play(&mut self) {
      let list = self.clone().get_dawn_server().list.clone();
      match list {
        Some(l) => {
          let t = l.Items[0].Type.clone();
          if t == "Episode" {
            self.config.auto_play_episode = !self.config.auto_play_episode;
          }
          if t == "Movie" {
            self.config.auto_play_movie = !self.config.auto_play_movie;
          }
        },
        None => {}
      }
    }

    pub fn change_draw_mode(&mut self, c: char) {
      if c == '?' {
        self.show_config = false;
        self.show_help = !self.show_help;
      } else {
        self.show_help = false;
        self.show_config = !self.show_config;
      }
    }

    async fn on_enter(&mut self, config: config::Config) {
      if &self.select_window == "Server select" {
        let i = self.server_state.index;
        self.server_state.draw = i;
        if self.server_state.servers[self.server_state.draw].name != "add server +" {
          //api::get_view(self.server_state.servers[self.server_state.draw], self);
          self.server_state.servers[self.server_state.draw].get_server_view().await;
          self.select_window = self.server_state.servers[self.server_state.draw].name.clone();
        }
      }else if &self.select_window == "add server +" {
        let t = self.create.clone().on_enter(self.input_mode, self, config).await;
        match t {
          Some(b) => {
            self.input_mode = b;
          },
          None => {}
        }
      }else if &self.select_window == &self.server_state.servers[self.server_state.draw].name {
        self.server_state.servers[self.server_state.draw].on_enter_view().await;
        self.select_window = self.server_state.servers[self.server_state.draw].list_name.clone();
      } else if &self.select_window == &self.server_state.servers[self.server_state.draw].list_name {
        self.server_state.servers[self.server_state.draw].on_enter_list().await;
      }
    }

    pub fn on_backspace(&mut self) {
      if self.select_window == "add server +" {
        if self.input_mode {
          self.create.del_input();
        }
      }
    }

    pub fn win_up(&mut self) {
      if self.select_window != "Server select" {
        self.select_window = String::from("Server select");
      }
    }

    pub fn win_down(&mut self) {
      if self.select_window == "Server select" {
        let i = self.server_state.draw;
        let tit = &self.server_state.servers[i].name;
        self.select_window = tit.to_string();
      }
    }

    pub fn win_left(&mut self) {
      let server = &self.server_state.servers[self.server_state.draw];
      if self.select_window == server.list_name {
        self.select_window = server.name.clone();
      }
    }

    pub fn win_right(&mut self) {
      let server = &self.server_state.servers[self.server_state.draw];
      if self.select_window == server.name {
        self.select_window = server.list_name.clone();
      }
    }

    pub fn on_up(&mut self) {
      let server = &self.server_state.servers[self.server_state.draw];
      if self.select_window == server.name {
        self.server_state.servers[self.server_state.draw].on_up_view();
      } else if self.select_window == server.list_name {
        self.server_state.servers[self.server_state.draw].on_up_list();
      }
    }

    pub fn on_down(&mut self) {
      let server = &self.server_state.servers[self.server_state.draw];
      if self.select_window == "add server +"{
        self.create.on_down();
      } else if self.select_window == self.server_state.servers[self.server_state.draw].name {
        self.server_state.servers[self.server_state.draw].on_down_view();
      } else if self.select_window == server.list_name {
        self.server_state.servers[self.server_state.draw].on_down_list();
      }
    }

    pub fn on_right(&mut self) {
      if self.select_window == "Server select" {
        self.server_state.next();
      }
    }

    pub fn on_left(&mut self) {
      if self.select_window == "Server select" {
        self.server_state.previous();
      }
    }

    pub fn get_server_names(self) -> Vec<String> {
      let mut names: Vec<String> = vec![];
      for s in self.server_state.servers {
        names.push(s.name);
      }
      names
    }

    pub fn window_focused(self, name: &String) -> Color {
      if &self.select_window == name {
        Color::Blue
      } else {
        Color::White
      }
    }

    pub fn get_dawn_server(self) -> ServerList {
    self.server_state.servers[self.server_state.draw].clone()
  }

  pub fn del_to_play(&mut self) {
    self.server_state.servers[self.server_state.draw].to_play = vec![];
  }

    pub fn to_play(self) -> bool {
      self.server_state.servers[self.server_state.draw].to_play.clone().len() > 0
    }
}

#[derive(Debug, Clone)]
pub struct CreateServer{
  pub uri: String,
  pub username: String, 
  pub password: String,
  pub active: i32
}

impl CreateServer {
  pub fn new() -> CreateServer {
    CreateServer {
      uri: "".to_string(),
      username: "".to_string(),
      password: "".to_string(),
      active: 0 
    } 
  }

  pub fn high(self, name: &str) -> Color {
    let u = vec!["uri", "username", "password", "ok", "cancel"];
    let i = u.iter().position(|&r| r == name).unwrap();
    if self.active as usize == i {
      Color::Blue
    } else {
      Color::White
    }
  }

  pub fn on_down(&mut self) {
    let u = vec!["uri", "username", "password", "ok", "cancel"];
    let mut index = self.active;
    if index as usize == u.len() -1 {
      index = 0;
    } else {
      index += 1;
    }
    self.active = index;
  }

  pub fn del_input(&mut self) {
    match self.active {
      0 => {
        if self.uri.len() > 0 {
          self.uri.truncate(self.uri.len() - 1);
        }
      },
      1 => {
        if self.username.len() > 0 {
          self.username.truncate(self.username.len() - 1);
        }
      },
      2 => {
        if self.password.len() > 0 {
          self.password.truncate(self.password.len() - 1);
        }
      },
      _ => {}
    }
  }

  pub fn input(&mut self, c: char) {
    match self.active {
      0 => {
        self.uri.push(c);
      },
      1 => {
        self.username.push(c);
      },
      2 => {
        self.password.push(c);
      },
      _ => {}
    }
  }

  pub async fn on_enter(mut self, input: bool, app: &mut App, config: config::Config) -> Option<bool> {
    if self.active == 3{
      if !self.uri.contains("https://"){
        self.uri = format!("https://{}", self.uri);
      }
      let x = api::login(&self, app, config).await;
      match x {
        Ok(d) => {},
        Err(e) => {}
      }
      None
    } else {
      Some(!input)
    }
  }
  
}
