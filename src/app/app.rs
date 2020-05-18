use tui::style::{Color, Style};
use crate::app::{server, error, config, create_server};
use crate::api;

pub struct App {
    pub active_server: i32,
    pub server_list: Vec<server::Server>,
    pub error: error::Error,
    pub config: config::Config,
    pub create_server: create_server::CreateServer,
    pub user_view: Vec<server::ServerView>,
    pub active_list: String,
    pub user_list: Vec<server::ServerList>,
    pub active_window: String,
    pub cursor: usize,
    pub input_mode: bool,
    pub quit: bool
}

impl App {
    pub async fn new() -> App {
        let mut last_active_server: i32;
        let mut server_list: Vec<server::Server>;
        let mut err: error::Error;
        let mut conf: config::Config;
        match config::ConfigFile::load_or_create() {
            Ok(c) => {
                last_active_server = c.last_active_server;
                server_list = c.get_server_list();
                err = error::Error::error("");
                conf = c.get_config();
            },
            Err(error) => {
                last_active_server = -2;
                server_list = vec![];
                err = error;
                conf = config::Config::empty();
            }
        };
        let active_window: String;
        let user_view: Vec<server::ServerView>;
        match last_active_server {
            -1 => {
                active_window = String::from("Create new Server");
                user_view = Vec::new();
            },
            _ => {
                active_window = server_list[last_active_server as usize].get_name();
                match api::get_view(&server_list[last_active_server as usize]).await {
                    Ok(view) => user_view = view,
                    Err(error) => {
                        user_view = Vec::new();
                        err = error;
                    }
                }
            }
        }
        let active_list = String::from("");
        App {
            active_server: last_active_server,
            server_list: server_list,
            config: conf,
            create_server: create_server::CreateServer::new(),
            user_view: user_view,
            active_list: active_list,
            user_list: vec![],
            error: err,
            active_window: active_window,
            cursor: 0,
            input_mode: false,
            quit: false,
        }
    }

    pub fn has_error(&self) -> bool {
        self.error.error != ""
    }

    pub fn cursor_color(&self, item: usize, window: &str) -> Style {
        let mut color: Color;
        if self.cursor == item && self.active_window == window{
            color = Color::Blue;
        } else {
            color = Color::White;
        }
        Style::default().fg(color)
    }

    pub fn window_color(&self, item: &str) -> Style {
        let mut color = Color::White;
        if self.active_window == item {
            color = Color::Blue;
        }
        Style::default().fg(color)
    }

    async fn add_server(&mut self) {
        if self.create_server.changed_input() {
            if self.create_server.uri.chars().last().unwrap() == '/' {
                self.create_server.uri.truncate(self.create_server.uri.len() - 1);
            }
            if !self.create_server.uri.contains("http") {
                self.create_server.uri = format!("https://{}", self.create_server.uri);
            }
            let mut server = server::Server::new(self.create_server.uri.clone(), server::User::empty());
            match api::login(&self.create_server, &mut server).await {
                Ok(()) => {
                    self.server_list.push(server);
                    self.active_server = self.server_list.len() as i32 -1;
                    self.active_window = self.active_server_name();
                    match config::ConfigFile::save(
                        &self.config,
                        self.server_list.clone(),
                        self.active_server) {
                        Ok(()) => {},
                        Err(error) => self.error = error
                    };
                },
                Err(error) => self.error = error
            };
        } else {
            self.error = error::Error::error("You have to fill out the fields");
        }
    }

    pub fn login() {

    }

    fn active_cursor_window(&self) -> usize {
        if self.active_window == "servers" {
            0
        } else if self.active_window == self.active_server_name() {
            1
        } else if self.active_window == self.active_list {
            2
        } else {
            0
        }
    }

    pub async fn on_key(&mut self, c: char) {
        if !self.input_mode {
            match c {
                'q' => self.quit = !self.quit,
                'k' => self.on_key_up(),
                'j' => self.on_key_down(),
                'l' => self.on_key_right(),
                'h' => self.on_key_left(),
                'K' => self.on_window_up(),
                'J' => self.on_window_down(),
                'L' => self.on_window_right(),
                'H' => self.on_window_left(),
                '\n' => self.on_key_enter().await,
                _ => {}
            }
        } else {
            match c {
                '\n' => self.on_key_enter().await,
                _ => {
                    match self.active_server {
                        -1 => self.create_server.input(c),
                        _ => {}
                    }
                }
            }
        }
    }

    async fn on_key_enter(&mut self) {
        if self.has_error() {
            match self.active_server {
                -2 => self.quit = !self.quit,
                _ => self.error = error::Error::error("")
            }
        } else {
            match self.active_server {
                -1 => {
                    match self.cursor {
                        0..=2 => {
                            self.input_mode = !self.input_mode;
                            self.create_server.enter(self.cursor);
                        },
                        3 => {
                            self.add_server().await;
                        },
                        4 => {
                            if !self.server_list.is_empty() {
                                self.cursor = 0;
                                self.active_server = 0;
                                self.active_window = self.active_server_name();
                            }
                        },
                        _ => {}
                    }
                },
                _ => {
                    match self.active_cursor_window() {
                        0 => {
                            let len = self.server_list.len();
                            if len == self.cursor {
                                self.active_server = -1;
                                self.active_window = String::from("Create new Server");
                                self.cursor = 0;
                            }
                        },
                        1 => {
                            let item = &self.user_view[self.cursor];
                            match api::get_items(&self.server_list[self.active_server as usize], &item.id).await {
                                Ok(list) => {
                                    self.user_list = list;
                                    self.active_list = item.name.clone();
                                    self.active_window = item.name.clone();
                                    self.cursor = 0;
                                },
                                Err(error) => self.error = error
                            }
                        },
                        2 => {},
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn on_key_backspace(&mut self) {
        match self.active_server {
            -1 => {
                if self.input_mode {
                    self.create_server.del();
                }
            },
            _ => {}
        }
    }

    pub fn on_key_up(&mut self) {
        match self.active_server {
            -1 => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                } else {
                    self.cursor = 4;
                }
            },
            _ => {
                match self.active_cursor_window() {
                    1 => {
                        if self.cursor > 0 {
                            self.cursor -= 1;
                        } else {
                            self.cursor = self.user_view.len() - 1;
                        }
                    },
                    2 => {},
                    _ => {}
                }
            }
        }
    }

    pub fn on_key_down(&mut self) {
        match self.active_server {
            -1 => {
                if self.cursor == 4 {
                    self.cursor = 0;
                } else {
                    self.cursor += 1;
                }
            },
            _ => {
                match self.active_cursor_window() {
                    1 => {
                        if self.cursor == self.user_view.len() -1 {
                            self.cursor = 0;
                        } else {
                            self.cursor += 1;
                        }
                    },
                    2 => {},
                    _ => {}
                }
            }
        }
    }

    pub fn on_key_left(&mut self) {
        match self.active_server {
            -1 => {},
            _ => {match self.active_cursor_window() {
                0 => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    } else {
                        self.cursor = self.server_list.len();
                    }
                },
                _ => {}
            };}
        }
    }

    pub fn on_key_right(&mut self) {
        match self.active_server {
            -1 => {},
            _ => {match self.active_cursor_window() {
                0 => {
                    if self.cursor == self.server_list.len() {
                        self.cursor = 0;
                    } else {
                        self.cursor += 1;
                    }
                },
                _ => {}
            };}
        }
    }

    pub fn on_window_up(&mut self) {
        let server_name = self.active_server_name();
        let active_list = &self.active_list;
        match &self.active_window {
            server_name => {
                self.active_window = "servers".to_string();
                self.cursor = 0;
            },
            active_list => {
                self.active_window = "servers".to_string();
                self.cursor = 0;
            },
            _ => {}
        }
    }

    pub fn on_window_down(&mut self) {
        let servers = "servers".to_string();
        match &self.active_window {
            servers => {
                self.active_window = self.active_server_name();
                self.cursor = 0;
            },
            _ => {}
        }
    }

    pub fn on_window_left(&mut self) {
        let list = self.active_list.clone();
        match &self.active_window {
            list => {
                self.active_window = self.active_server_name();
                self.cursor = 0;
            },
            _ => {}
        }
    }

    pub fn on_window_right(&mut self) {
        let view = self.active_server_name();
        match &self.active_window {
            view => {
                self.active_window = self.active_list.clone();
                self.cursor = 0;
            },
            _ => {}
        }
    }

    pub fn get_servers_name(&self) -> Vec<String> {
        let mut name = Vec::new();
        for server in &self.server_list {
            name.push(server.name.clone());
        }
        name
    }

    pub fn active_server_name(&self) -> String {
        self.server_list[self.active_server as usize].name.clone()
    }
}
