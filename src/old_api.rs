use reqwest;
use crate::app::{ServerList, CreateServer, App};
use std::collections::HashMap;
use crate::util;
use reqwest::Error;
use crate::models::{user, query, config};
use crate::app;
use std::time::SystemTime;
use chrono::DateTime;
use chrono::offset::Local;

pub fn format_header(device_id: &String, token: Option<String>) -> Vec<String> {
  let mut header = format!(
    "MediaBrowser Client=jellydesktop, Device=Jellydesktop, DeviceId={}, Version=0.0.1",
    device_id);
  match token {
    Some(t) => {
      header = format!("{}, Token={}", header, t);
    },
    None => {}
  };
  vec!["X-Emby-Authorization".to_string(), header]
}

pub async fn login(
  server: &CreateServer,
  app: &mut App,
  mut config: config::Config) -> Result<(), String> {
  let mut login = HashMap::new();
  let device_id = util::generate_device_id();
  login.insert("username".to_string(), &server.username);
  login.insert("pw".to_string(), &server.password);
  let uri = format!("{}{}", &server.uri, String::from("/Users/AuthenticateByName"));

  let h = format_header(&device_id, None);
  let client = reqwest::Client::new();
  let res = match client.post(&uri)
    .json(&login)
    .header(&h[0], &h[1])
    .send()
    .await {
      Ok(r) => r,
      Err(e) => return  Err(String::from("Error while requesting data"))
    };

  if res.status() == 200 {
    let user: user::Authentication = match res.json().await {
      Ok(u) => u,
      Err(_e) => return Err(String::from("Error while parsing data"))
    };
    //let create = &app.server_state.servers[app.server_state.servers.len() -1];
    //&app.server_state.servers.pop();
    let mut new_server = app::ServerList::new(user, server, device_id);
    config.add_server(new_server.clone())?;
    app.select_window = "Server select".to_string();
    new_server.get_server_view().await;
    app.server_state.servers.push(new_server.clone());
    //app.server_state.servers.push(create);
  } else {
    return Err(res.text().await.unwrap())
  }

  Ok(())
  
}

pub async fn get_view(server: &mut ServerList) -> Result<(), String> {
  let user = server.user.clone().unwrap();
  let uri = format!("{}/Users/{}/Views", server.uri, &user.User.Id);

  let h = format_header(&server.device_id, Some(user.AccessToken));
  let client = reqwest::Client::new();
  let res = match client.get(&uri)
    .header(&h[0], &h[1])
    .send()
    .await {
      Ok(r) => r,
      Err(_e) => return Err(String::from("Error while requesting data"))
    };

  if res.status() == 200 {
    let j: query::QueryResult = match res.json().await {
      Ok(j) => j,
      Err(_e) => return Err(String::from("Error while parsing data"))
    };
    server.add_view(j);
  } else {
    return Err(res.text().await.unwrap())
  }
  Ok(())
}

pub async fn get_item(server: &mut ServerList, item: &query::BaseItem) -> Result<(), String> {
  let user = server.user.clone().unwrap();
  let h = format_header(&server.device_id, Some(user.AccessToken));
  let uri = format!("{}/Users/{}/Items?ParentId={}", server.uri, &user.User.Id, item.clone().Id);

  let client = reqwest::Client::new();
  let res = match client.get(&uri)
    .header(&h[0], &h[1])
    .send()
    .await {
      Ok(r) => r,
      Err(_e) => return Err(String::from("Error while requesting data"))
    };

  if res.status() == 200 {
    let mut j: query::QueryResult = match res.json().await {
      Ok(j) => j,
      Err(_e) => return Err(String::from("Error parsing responde"))
    };
    j.Items.sort_by(util::compere_items);
    server.add_list(j);
  }else{
    return Err(res.text().await.unwrap())
  }
  Ok(())
}

//#[tokio::main]
pub async fn has_played(
  uri: &String,
  user: user::Authentication,
  item_id: &String,
  device_id: &String) -> Result<(), Error> {

  let time = SystemTime::now();
  let time: DateTime<Local> = time.into();
  let time = time.format("%Y%m%d%H%M%S");
  let h = format_header(&device_id, Some(user.AccessToken));
  let uri = format!(
    "{}/Users/{}/PlayedItems/{}?DatePlayed={}",
    uri,
    &user.User.Id,
    item_id,
    time
  );

  let client = reqwest::Client::new();
  let res = client.post(&uri)
    .header(&h[0], &h[1])
    .header("Content-Length", "0")
    .send()
    .await?;
  Ok(())
}
