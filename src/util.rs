use uuid::Uuid;
use crate::models::query;
use std::cmp::Ordering;

pub fn format_pw(pw: String) -> String{
  (0..pw.len()).map(|_| "*").collect()
}

pub fn generate_device_id() -> String {
  Uuid::new_v4().to_string()
}

//pub fn exists_config() -> bool {
//  Path::new("config.json").exists()
//}
//
//pub fn read_server_from_config() -> Serv {
//  let file = File::open("config.json").unwrap();
//  let reader = BufReader::new(file);
//  let j = serde_json::from_reader(reader);
//  j.unwrap()
//}
//
//pub fn add_server_to_config(server: &app::ServerList) -> Result<(), std::io::Error> {
//  let j = {
//    if Path::new("config.json").exists() {
//      let mut x = read_server_from_config();
//      let u = x.server.iter().position(|ser| { ser.name == server.name});
//      match u {
//        Some(s) => {
//          if x.server[s].user.clone().unwrap().User.Name != server.user.clone().unwrap().User.Name {
//            x.server.push(server.clone());
//          }
//          x
//        },
//        None => {x.server.push(server.clone());x}
//      }
//    } else {
//      Serv {
//        count: 1,
//        server: vec!(server.clone()),
//        auto_play: true
//      }
//    }
//  };
//  let buf = Vec::new();
//  let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
//  let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
//  j.serialize(&mut ser).unwrap();
//  serde_json::to_writer(&File::create("config.json")?, &j);
//  Ok(())
//}

pub fn compere_items(a: &query::BaseItem, b: &query::BaseItem) -> Ordering {
  if a.IndexNumber.is_some() {
    if a.IndexNumber.unwrap() < b.IndexNumber.unwrap(){
      return Ordering::Less;
    }
    if a.IndexNumber.unwrap() > b.IndexNumber.unwrap() {
      return Ordering::Greater;
    }
  }
 return Ordering::Equal;
}
