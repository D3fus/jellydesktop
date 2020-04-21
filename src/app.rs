use tui::style::{Color};
use crate::api;
use crate::models::{user, query};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::process::Child;

pub struct Player {
  pub player: Child,
  pub list: Vec<String>,
  pub user: Option<user::Authentication>,
  pub start_playing: bool,
  pub uri: String,
  pub auto_play: bool,
  pub time_out: usize
}

impl Player {
  pub fn new() -> Player {
      Player {
        player: Command::new("echo").spawn().unwrap(),
        list: vec![],
        user: None,
        uri: String::from(""),
        start_playing: false,
        auto_play: false,
        time_out: 0
      }
    }

    pub fn add(&mut self, server: &ServerList) {
      self.user = server.user.clone();
      self.list = server.to_play.clone();
      self.uri = server.uri.clone();
      if self.list.len() > 1 {
        self.auto_play = true;
      }
    }

  pub fn play_if_ready(&mut self) {
    if !self.start_playing && self.list.len() > 0 {
      self.play();
      self.start_playing = true;
    } else if self.start_playing && self.list.len() > 0{
      if self.is_fin_playing() {
        if self.time_out == 0 {
          self.play();
        } else {
          self.time_out -= 1;
        }
      }
    }
  }

  pub fn play(&mut self) {
    let base = format!(
      "{}/Items/{}/Download?api_key={}",
      self.uri,
      self.list[0],
      self.user.clone().unwrap().AccessToken
    );
    //TODO error if not installed
    self.player = Command::new("mpv")
      .args(&[base, "--really-quiet".to_string()])
      .spawn()
      .unwrap();
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
      server.push(ServerList {
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
      });
      ServerState {
        servers: server, index: 0, draw: 0
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

    pub async fn get_server_view(&mut self) {
      //TODO login error
      let t = api::get_view(self).await;
      match t {
        Ok(u) => {},
        Err(e) => {println!("{:#?}", e);}
      }
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

    pub async fn on_enter_view(&mut self) {
        let item = &self.view.clone().unwrap().Items[self.active_view];
        let re = api::get_item(self, item).await;
        match re {
          Ok(e) => {},
          Err(e) => {println!("{:?}", e)}
        };
        self.active_list = 0;
        self.list_name = item.Name.clone();
    }

    pub async fn on_enter_list(&mut self) {
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
        let re = api::get_item(self, item).await;
        match re {
          Ok(e) => {},
          Err(e) => {println!("{:?}", e)}
        };
        self.active_list = 0;
      }
    }
  }

  #[derive(Debug, Clone)]
  pub struct App {
    pub title: String,
    pub server_state: ServerState,
    pub should_quit: bool,
    pub input_mode: bool,
    pub select_window: String,
    pub create: CreateServer,
  }

  impl App {
    pub fn new(title: String, servers: Vec<ServerList>) -> App {
      App {
        title: title,
        server_state: ServerState::new(servers),
        should_quit: false,
        input_mode: false,
        select_window: "Server select".to_string(),
        create: CreateServer::new(),
      }
    }

    pub async fn on_key(&mut self, c: char) {
      if self.input_mode {
        if &self.select_window == "add server +" {
          match c {
            '\n' => {
              self.on_enter().await;
            },
            _ => {
              self.create.input(c);
            }
          }
        }
      } else {
        match c {
          'q' => {
            self.should_quit = true;
          },
          'l' => {
            self.on_right();
          },
          'h' => {
            self.on_left();
          },
          'k' => {
            self.on_up();
          },
          'j' => {
            self.on_down();
          },
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
          '?' => {
            //TODO help page
          },
          'c' => {
            //TODO config page
          },
          '\n' => {
            self.on_enter().await;
          }
          _ => {}
        }
      }
    }

    async fn on_enter(&mut self) {
      if &self.select_window == "Server select" {
        let i = self.server_state.index;
        self.server_state.draw = i;
        if self.server_state.servers[self.server_state.draw].name != "add server +" {
          //api::get_view(self.server_state.servers[self.server_state.draw], self);
          self.server_state.servers[self.server_state.draw].get_server_view().await;
          self.select_window = self.server_state.servers[self.server_state.draw].name.clone();
        }
      }else if &self.select_window == "add server +" {
        let t = self.create.clone().on_enter(self.input_mode, self).await;
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

  pub async fn on_enter(mut self, input: bool, app: &mut App) -> Option<bool> {
    if self.active == 3{
      if !self.uri.contains("https://"){
        self.uri = format!("https://{}", self.uri);
      }
      let x = api::login(&self, app).await;
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
