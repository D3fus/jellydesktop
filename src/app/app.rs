use tui::style::{Color, Style};
use crate::app::{server, error, config, create_server};
use crate::api;

pub struct App {
    pub active_server: i32,
    pub server_list: Vec<server::Server>,
    pub error: error::Error,
    pub config: config::Config,
    pub create_server: create_server::CreateServer,
    //pub user_view: Vec<>,
    //pub user_list: Vec<>,
    pub active_window: String,
    pub cursor: usize,
    pub input_mode: bool,
    pub quit: bool
}

impl App {
    pub fn new() -> App {
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
        let active_window = match last_active_server {
            -1 => {String::from("server create")},
            _ => {server_list[last_active_server as usize].get_name()}
        };
        App {
            active_server: last_active_server,
            server_list: server_list,
            config: conf,
            create_server: create_server::CreateServer::new(),
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

    pub fn cursor_color(&self, item: usize) -> Style {
        let mut color: Color;
        if self.cursor == item {
            color = Color::Blue;
        } else {
            color = Color::White;
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
                    config::ConfigFile::safe(&self.config, self.server_list.clone(), self.active_server);
                },
                Err(error) => self.error = error
            };
        } else {
            self.error = error::Error::error("You have to fill out the fields");
        }
    }

    pub fn login() {

    }

    pub async fn on_key(&mut self, c: char) {
        if !self.input_mode {
            match c {
                'q' => self.quit = !self.quit,
                'k' => self.on_key_up(),
                'j' => self.on_key_down(),
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
                            //TODO change to server if exist
                            self.quit = !self.quit;
                        },
                        _ => {}
                    }
                },
                _ => {}
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
            _ => {}
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
            _ => {}
        }
    }

    pub fn on_key_left() {

    }

    pub fn on_key_right() {

    }

    pub fn on_window_up() {

    }

    pub fn on_window_down() {

    }

    pub fn on_window_left() {

    }

    pub fn on_window_right() {

    }
}
