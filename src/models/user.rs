#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use serde;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authentication {
  pub User: User,
  #[serde(skip_deserializing)]
  pub SessionInfo: Option<String>,
  pub AccessToken: String,
  pub ServerId: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub Name: String,
  pub ServerId: String,
  pub Id: String,
  pub HasPassword: bool,
  pub HasConfiguredPassword: bool,	
  pub HasConfiguredEasyPassword: bool,	
  pub EnableAutoLogin: Option<bool>,	
  pub LastLoginDate: Option<String>,	
  pub LastActivityDate: Option<String>,
  #[serde(skip_deserializing)]
  pub Configuration: Option<String>,
  #[serde(skip_deserializing)]
  pub Policy: Option<String>,
  pub PrimaryImageAspectRatio: Option<f64>
}
