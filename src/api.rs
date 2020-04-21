use reqwest;
use crate::app::{ServerList, CreateServer, App};
use std::collections::HashMap;
use crate::util;
use reqwest::Error;
use crate::models::{user, query};
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

pub async fn login(server: &CreateServer, app: &mut App) -> Result<(), Error> {
  let mut login = HashMap::new();
  let device_id = util::generate_device_id();
  login.insert("username".to_string(), &server.username);
  login.insert("pw".to_string(), &server.password);
  let uri = format!("{}{}", &server.uri, String::from("/Users/AuthenticateByName"));

  let h = format_header(&device_id, None);
  let client = reqwest::Client::new();
  let res = client.post(&uri)
    .json(&login)
    .header(&h[0], &h[1])
    .send()
    .await?;

  if res.status() == 200 {
    let user: user::Authentication = res.json().await?;
    //let create = &app.server_state.servers[app.server_state.servers.len() -1];
    //&app.server_state.servers.pop();
    let mut new_server = app::ServerList::new(user, server, device_id);
    util::add_server_to_config(&new_server);
    app.select_window = "Server select".to_string(); 
    new_server.get_server_view().await;
    app.server_state.servers.push(new_server.clone());
    //app.server_state.servers.push(create);
  } else {
   //error 
  }

  Ok(())
  
}

pub async fn get_view(server: &mut ServerList) -> Result<(), Error> {
  let user = server.user.clone().unwrap();
  let uri = format!("{}/Users/{}/Views", server.uri, &user.User.Id);

  let h = format_header(&server.device_id, Some(user.AccessToken));
  let client = reqwest::Client::new();
  let res = client.get(&uri)
    .header(&h[0], &h[1])
    .send()
    .await?;

  if res.status() == 200 {
    let j: query::QueryResult = res.json().await?;
    server.add_view(j);
  } else {
    println!("{:?}", res.text().await?);
  }
  Ok(())
}

pub async fn get_item(server: &mut ServerList, item: &query::BaseItem) -> Result<(), Error> {
  let user = server.user.clone().unwrap();
  let h = format_header(&server.device_id, Some(user.AccessToken));
  let uri = format!("{}/Users/{}/Items?ParentId={}", server.uri, &user.User.Id, item.clone().Id);

  let client = reqwest::Client::new();
  let res = client.get(&uri)
    .header(&h[0], &h[1])
    .send()
    .await?;

  if res.status() == 200 {
    let mut j: query::QueryResult = res.json().await?;
    j.Items.sort_by(util::compere_items);
    server.add_list(j);
  }else{
    println!("{:?}", res.text().await);
  }
  Ok(())
}

#[tokio::main]
pub async fn has_played(server: ServerList, item: query::BaseItem) -> Result<(), Error> {
  let time = SystemTime::now();
  let time: DateTime<Local> = time.into();
  let time = time.format("%Y%m%d%H%M%S");
  let user = server.user.clone().unwrap();
  let h = format_header(&server.device_id, Some(user.AccessToken));
  let uri = format!(
    "{}/Users/{}/PlayedItems/{}?DatePlayed={}",
    server.uri,
    &user.User.Id,
    item.clone().Id,
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
