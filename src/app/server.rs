use crate::util;
use crate::models::{user};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub token: String
}

impl User {
    pub fn empty() -> User {
        User {
            id: String::from(""),
            username: String::from(""),
            token: String::from("")
        }
    }

    pub fn add_user(&mut self, user: String) {
        self.username = user;
    }

    pub fn add_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn add_id(&mut self, id: String) {
        self.id = id;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub uri: String,
    pub name: String,
    pub user: User,
    pub uuid: String
}

impl Server {
    pub fn new(uri: String, user: User) -> Server {
        let uuid = util::generate_device_id();
        let name = util::server_uri_to_name(&uri);
        Server {
            uri,
            name,
            user,
            uuid
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_data_from_login(&mut self, user: user::Authentication) {
        self.user.add_user(user.User.Name);
        self.user.add_token(user.AccessToken);
        self.user.add_id(user.User.Id);
    }
}
