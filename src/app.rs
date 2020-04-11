use tui::style::{Color};


#[derive(Debug, Clone)]
pub struct ServerState<'a> {
  pub servers: Vec<ServerList<'a>>,
  pub index: usize,
  pub draw: usize
}

impl<'a> ServerState<'a> {
  pub fn new(mut server: Vec<ServerList<'a>>) -> ServerState {
    server.push(ServerList {
      uri: "",
      name: "add server +",
      user_name: "",
      user_token: ""
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
    self.draw == self.servers.len() - 1
  }
}

#[derive(Debug, Copy, Clone)]
pub struct ServerList<'a> {
  pub uri: &'a str,
  pub name: &'a str,
  pub user_name: &'a str,
  pub user_token: &'a str
}

#[derive(Debug, Clone)]
pub struct App<'a> {
  pub title: &'a str,
  pub server_state: ServerState<'a>,
  pub should_quit: bool,
  pub input_mode: bool,
  pub input: &'a str,
  pub select_window: &'a str,
  pub create: CreateServer<'a>
}

impl<'a> App<'a> {
  pub fn new(title: &'a str, servers: Vec<ServerList<'a>>) -> App<'a> {
    App {
      title: title,
      server_state: ServerState::new(servers),
      should_quit: false,
      input_mode: false,
      input: "",
      select_window: "Server select",
      create: CreateServer::new()
    }
  }

  pub fn on_key(&mut self, c: char) {
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
      '\n' => {
        self.on_enter();
      }
      _ => {}
    }
  }

  fn on_enter(&mut self) {
    if self.select_window == "Server selcet" {
      let i = self.server_state.index;
      self.server_state.draw = i;
    }else if self.select_window == "add server +" {
      //self.create.on_enter(self);
    }
  }

  pub fn win_up(&mut self) {
    if self.select_window != "Server select" {

    }
  }

  pub fn win_down(&mut self) {
    if self.select_window == "Server select" {
      let i = self.server_state.draw;
      let tit = self.server_state.servers[i].name;
      self.select_window = tit;
    }
  }

  pub fn on_up(&mut self) {}

  pub fn on_down(&mut self) {
    if self.select_window == "add server +"{
      self.create.on_down();
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

  pub fn get_server_names(self) -> Vec<&'a str> {
    let mut names: Vec<&'a str> = vec![];
    for s in self.server_state.servers {
      names.push(s.name);
    }
    names
  }

  pub fn window_focused(self, name: &'a str) -> Color {
    if self.select_window == name {
      Color::Blue
    } else {
      Color::White
    }
  } 
}

#[derive(Debug, Clone)]
pub struct CreateServer<'a> {
  pub uri: &'a str,
  pub username: &'a str,
  pub password: &'a str,
  pub active: &'a str
}

impl<'a> CreateServer<'a> {
  pub fn new() -> CreateServer<'a> {
    CreateServer {
      uri: "",
      username: "",
      password: "",
      active: "uri"
    } 
  }

  pub fn high(self, name: &'a str) -> Color {
    if self.active == name {
      Color::Blue
    } else {
      Color::White
    }
  }

  pub fn on_down(&mut self) {
    let u = vec!["uri", "username", "password"];
    let mut index = u.iter().position(|&r| r == self.active).unwrap();
    if index == u.len() -1 {
      index = 0;
    } else {
      index += 1;
    }
    self.active = u[index];
  }

  pub fn on_enter(&mut self, app: &mut App) {
    app.input_mode = true;
  }
  
}
