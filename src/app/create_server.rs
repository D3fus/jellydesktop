use crate::util;

pub struct CreateServer {
    pub uri: String,
    pub username: String,
    pub password: String,
    pub cursor: usize,
    pub cursor_blink: bool,
    pub cursor_timer: usize
}

impl CreateServer {
    pub fn new() -> CreateServer {
        CreateServer {
            uri: String::from("<https://yoururl.com>"),
            username: String::from("<UserName>"),
            password: String::from("<Password>"),
            cursor: 0,
            cursor_blink: false,
            cursor_timer: 0
        }
    }

    pub fn get(&mut self, t: char) -> String {
        let out = match t {
            'u' => {
                if self.cursor == 0 {
                    self.blink(self.uri.clone())
                } else {
                    self.uri.clone()
                }
            },
            'n' => {
                if self.cursor == 1 {
                    self.blink(self.username.clone())
                } else {
                    self.username.clone()
                }
            },
            'p' => {
                if self.password == "<Password>" {
                    self.password.clone()
                } else {
                    let for_pw = util::format_pw(self.password.clone());
                    if self.cursor == 2 {
                        self.blink(for_pw)
                    } else {
                        for_pw
                    }
                }
            },
            _ => {String::from("")}
        };
        out
    }

    fn blink (&mut self, text: String) -> String {
        let mut text = text;
        if self.cursor_blink {
            if self.cursor_timer <= 2 {
                text.push_str("█");
            } else if self.cursor_timer == 4{
                self.cursor_timer = 0;
            }
            self.cursor_timer += 1;
        }
        text
    }

    pub fn enter(&mut self, cursor: usize) {
        self.cursor = cursor;
        match cursor {
            0 => {
                if self.uri == "<https://yoururl.com>"{
                    self.uri = String::from("");
                } else if self.uri == "" {
                    self.uri = String::from("<https://yoururl.com>");
                }
                self.cursor_blink = !self.cursor_blink;
            },
            1 => {
                if self.username == "<UserName>" {
                    self.username = String::from("");
                } else if self.username == "" {
                    self.username = String::from("<UserName>");
                }
                self.cursor_blink = !self.cursor_blink;
            },
            2 => {
                if self.password == "<Password>" {
                    self.password = String::from("");
                } else if self.password == "" {
                    self.password = String::from("<Password>");
                }
                self.cursor_blink = !self.cursor_blink;
            },
            _ => {}
        }

    }

    pub fn input(&mut self, c: char) {
        match self.cursor {
            0 => self.input_uri(c),
            1 => self.input_username(c),
            2 => self.input_password(c),
            _ => {},
        }
    }

    fn input_uri(&mut self, c: char) {
        self.uri.push(c);
    }

    fn input_username(&mut self, c: char) {
        self.username.push(c);
    }

    fn input_password(&mut self, c: char) {
        self.password.push(c);
    }

    pub fn del(&mut self) {
        match self.cursor {
            0 => self.del_uri(),
            1 => self.del_username(),
            2 => self.del_password(),
            _ => {},
        }
    }

    fn del_uri(&mut self) {
        if !self.uri.is_empty() {
            self.uri.truncate(self.uri.len() - 1);
        }
    }

    fn del_username(&mut self) {
        if !self.username.is_empty() {
            self.username.truncate(self.username.len() - 1);
        }
    }

    fn del_password(&mut self) {
        if !self.password.is_empty() {
            self.password.truncate(self.password.len() - 1);
        }
    }

    pub fn changed_input(&self) -> bool {
        self.uri != "" && self.uri != "<https://yoururl.com>" &&
            self.username != "" && self.username != "<UserName>" &&
            self.password != "" && self.password != "<Password>"
    }
}
