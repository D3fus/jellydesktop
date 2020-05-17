use reqwest;
use std::collections::HashMap;
use hostname;
use crate::app::{error, server, app, create_server};
use crate::models::{user};

fn get_header(server: &server::Server) -> Vec<String> {
    let host = match hostname::get() {
        Ok(host) => host.into_string().unwrap(),
        Err(err) => String::from("localhost")
    };
    let mut header = format!(
        "MediaBrowser Client=jellydesktop, Device={}, DeviceId={}, Version=0.0.1",
        host, server.uuid);
    let empty = String::from("");
    match &server.user.token {
        _ => {
        header = format!("{}, Token={}", header, server.user.token);
        },
        empty => {}
    };
    vec!["X-Emby-Authorization".to_string(), header]
}

async fn get(uri: String, server: &server::Server) -> Result<reqwest::Response, error::Error> {
    let header = get_header(server);

    let client = reqwest::Client::new();
    match client.get(&uri)
        .header(&header[0], &header[1])
        .send()
        .await {
            Ok(res) => Ok(res),
            Err(_e) => Err(error::Error::error("Error while requesting data"))
        }
}

async fn post(
    uri: String,
    server: &server::Server,
    data: HashMap<String, &String>) -> Result<reqwest::Response, error::Error> {

    let header = get_header(server);

    let client = reqwest::Client::new();
    match client.post(&uri)
        .json(&data)
        .header(&header[0], &header[1])
        .send()
        .await {
            Ok(res) => Ok(res),
            Err(_e) => Err(error::Error::error("Error while requesting data"))
        }
}

pub async fn login(
    server_data: &create_server::CreateServer,
    server: &mut server::Server) -> Result<(), error::Error> {

    let mut login = HashMap::new();
    login.insert(String::from("username"), &server_data.username);
    login.insert(String::from("pw"), &server_data.password);

    let uri = format!("{}{}", &server_data.uri, String::from("/Users/AuthenticateByName"));
    let res = post(uri, &server, login).await?;
    match res.json::<user::Authentication>().await {
        Ok(user) => {
            server.get_data_from_login(user);
            Ok(())
        },
        Err(_e) => Err(error::Error::error("Error while parsing response"))
    }
}
