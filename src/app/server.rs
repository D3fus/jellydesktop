use crate::util;
use crate::models::{user, query};
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

    pub fn empty() -> Server {
        Server {
            uri: String::from(""),
            name: String::from(""),
            user: User::empty(),
            uuid: String::from("")
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerList {
    pub name: String,
    pub id: String,
    pub parent_id: String,
    pub category: String,
    pub index_nummer: i32,
    pub played: bool,
    pub unplayed: i32,
    pub runtime: i64
}

impl ServerList {
    pub fn format_from_query(query: query::QueryResult) -> Vec<ServerList> {
        query.Items.iter().map(|item| {
            let parent_id: String;
            let index: i32;
            let unplayed: i32;
            let mut runtime = -1;
            if item.ParentBackdropItemId.is_some() {
                parent_id = item.ParentBackdropItemId.clone().unwrap();
            } else {
                parent_id = String::from("");
            }
            if item.IndexNumber.is_some() {
                index = item.IndexNumber.clone().unwrap();
            } else {
                index = -1;
            }
            if item.UserData.UnplayedItemCount.is_some() {
                unplayed = item.UserData.UnplayedItemCount.unwrap();
            } else {
                unplayed = -1;
            }
            if item.RunTimeTicks.is_some() {
                runtime = item.RunTimeTicks.unwrap() / 10 ^ 7;
            }
            ServerList {
                name: item.Name.clone(),
                id: item.Id.clone(),
                parent_id: parent_id,
                category: item.Type.clone(),
                index_nummer: index,
                played: item.UserData.Played,
                unplayed: unplayed,
                runtime: runtime
            }
        }).collect()
    }
}

pub struct ServerView {
    pub name: String,
    pub id: String
}

impl ServerView {
    pub fn format_from_query(query: query::QueryResult) -> Vec<ServerView> {
        query.Items.iter().map(|item| {
            ServerView{
                name: item.Name.clone(),
                id: item.Id.clone()
            }
        }).collect()
    }
}
