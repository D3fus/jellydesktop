use reqwest;
use std::collections::HashMap;
use hostname;
use std::time::Duration;
use crate::app::{error, server, create_server};
use crate::models::{user, query};

fn get_header(server: &server::Server) -> Vec<String> {
    let host = match hostname::get() {
        Ok(host) => host.into_string().unwrap(),
        Err(_err) => String::from("localhost")
    };
    let mut header = format!(
        "MediaBrowser Client=jellydesktop, Device={}, DeviceId={}, Version=0.1.0",
        host, server.uuid);
    if server.user.token != "" {
        header = format!("{}, Token={}", header, server.user.token);
    }
    vec!["X-Emby-Authorization".to_string(), header]
}

async fn get(uri: String, server: &server::Server) -> Result<reqwest::Response, error::Error> {
    let header = get_header(server);

    let client = reqwest::Client::new();
    match client.get(&uri)
        .header(&header[0], &header[1])
        .timeout(Duration::from_secs(10))
        .send()
        .await {
            Ok(res) => Ok(res),
            Err(_e) => Err(error::Error::error("Error while requesting data"))
        }
}

async fn post(
    uri: String,
    server: &server::Server,
    data: Option<HashMap<String, &String>>) -> Result<reqwest::Response, error::Error> {

    let header = get_header(server);

    let client = reqwest::Client::new();
    let client = client.post(&uri);
    let client = match data {
        Some(d) => client.json(&d),
        None => client.header("Content-Length", "0")
    };
    match client.header(&header[0], &header[1])
        .timeout(Duration::from_secs(10))
        .send()
        .await {
            Ok(res) => Ok(res),
            Err(_e) => Err(error::Error::error("Error while sending data"))
        }
}

pub async fn login(
    server_data: &create_server::CreateServer,
    server: &mut server::Server) -> Result<(), error::Error> {

    let mut login = HashMap::new();
    login.insert(String::from("username"), &server_data.username);
    login.insert(String::from("pw"), &server_data.password);

    let uri = format!("{}{}", &server_data.uri, String::from("/Users/AuthenticateByName"));
    let res = post(uri, &server, Some(login)).await?;
    match res.json::<user::Authentication>().await {
        Ok(user) => {
            server.get_data_from_login(user);
            Ok(())
        },
        Err(_e) => Err(error::Error::error("Error while parsing response"))
    }
}

pub async fn get_view(server: &server::Server) -> Result<Vec<server::ServerView>, error::Error> {
    let uri = format!("{}/Users/{}/Views", server.uri, server.user.id);
    let res = get(uri, server).await?;
    match res.json::<query::QueryResult>().await {
        Ok(query) => Ok(server::ServerView::format_from_query(query)),
        Err(_e) => Err(error::Error::error("Error while parsing response"))
    }
}

pub async fn get_items(server: &server::Server, id: &str) -> Result<Vec<server::ServerList>, error::Error> {
    let uri = format!("{}/Users/{}/Items?ParentId={}", server.uri, server.user.id, id);
    let res = get(uri, server).await?;
    match res.json::<query::QueryResult>().await {
        Ok(query) => Ok(server::ServerList::format_from_query(query)),
        Err(_e) => Err(error::Error::error("Error while parsing response"))
    }
}

pub async fn set_as_seen(server: &server::Server, item: &str) -> Result<(), error::Error> {
    let uri = format!(
        "{}/Users/{}/PlayedItems/{}",
        server.uri,
        server.user.id,
        item
    );
    post(uri, server, None).await?;
    Ok(())
}
