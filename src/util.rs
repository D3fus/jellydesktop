use uuid::Uuid;
use crate::app;
use std::fs::File;
use serde_json;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::io::BufReader;

pub fn format_pw(pw: String) -> String{
  (0..pw.len()).map(|_| "*").collect()
}

pub fn generate_deviceId() -> String {
  Uuid::new_v4().to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct serv {
  pub count: i32,
  pub server: Vec<app::ServerList>,
}

pub fn exists_config() -> bool {
  Path::new("config.json").exists()
}

pub fn read_server_from_config() -> serv {
  let file = File::open("config.json").unwrap();
  let reader = BufReader::new(file);
  let j = serde_json::from_reader(reader);
  j.unwrap()
}

pub fn add_server_to_config(server: &app::ServerList) -> Result<(), std::io::Error> {
  let j = {
    if Path::new("config.json").exists() {
      let mut x = read_server_from_config();
      let u = x.server.iter().position(|ser| { ser.name == server.name});
      match u {
        Some(s) => {
          if x.server[s].user.clone().unwrap().User.Name != server.user.clone().unwrap().User.Name {
            x.server.push(server.clone());
          }
          x
        },
        None => {x.server.push(server.clone());x}
      }
    } else {
      serv {
        count: 1,
        server: vec!(server.clone())
      }
    }
  };
  let buf = Vec::new();
  let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
  let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
  j.serialize(&mut ser).unwrap();
  serde_json::to_writer(&File::create("config.json")?, &j);
  Ok(())
}
