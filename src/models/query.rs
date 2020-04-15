#![allow(non_snake_case)]
use serde::{Deserialize};
use serde;

#[derive(Deserialize, Debug, Clone)]
pub struct QueryResult {
  pub Items: Vec<BaseItem>,	
  pub TotalRecordCount: i32
}

#[derive(Deserialize, Debug, Clone)]
pub struct BaseItem {
  pub Name: String,
  pub ServerId: String,
  pub Id: String,
  pub Etag: Option<String>,
  pub DateCreated: Option<String>,
  pub CanDelete: Option<bool>,
  pub CanDownload: Option<bool>,
  pub SortName: Option<String>,
  pub ExternalUrls: Option<Vec<ExternalUrl>>,
  pub Path: Option<String>,
  pub EnableMediaSourceDisplay: Option<bool>,
  pub Taglines: Option<Vec<String>>,
  pub Genres: Option<Vec<String>>,
  pub PlayAccess: Option<String>,
  pub RemoteTrailers: Option<Vec<ExternalUrl>>,
  #[serde(skip_deserializing)]
  pub ProviderIds: Option<String>,
  pub IsFolder: Option<bool>,
  pub ParentId: Option<String>,
  pub Type: String,
  #[serde(skip_deserializing)]
  pub People: Option<String>,
  #[serde(skip_deserializing)]
  pub Studios: Option<String>,
  #[serde(skip_deserializing)]
  pub GenreItems: Option<String>,
  pub LocalTrailerCount: Option<i32>,
  pub UserData: UserItemData,
  pub ChildCount: Option<i32>,
  pub SpecialFeatureCount: Option<i32>,
  pub DisplayPreferencesId: Option<String>,
  pub Tags: Option<Vec<String>>,
  pub PrimaryImageAspectRatio: Option<f64>,
  #[serde(skip_deserializing)]
  pub ImageTags: Option<String>,
  pub BackdropImageTags: Vec<String>,
  pub ScreenshotImageTags: Option<Vec<String>>,
  pub LocationType: String,
  pub LockedFields: Option<Vec<String>>,
  pub LockData: Option<bool>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct ExternalUrl {
  pub Name: String,
  pub Url: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserItemData {
  pub Rating: Option<f64>,
  pub PlayedPercentage: Option<f64>,
  pub UnplayedItemCount: Option<i32>,
  pub PlaybackPositionTicks: i64,
  pub PlayCount: i32,
  pub IsFavorite: bool,
  pub Likes: Option<bool>,
  pub LastPlayedDate: Option<String>,
  pub Played: bool,
  pub Key: String,
  #[serde(skip_deserializing)]
  pub ItemId: Option<String> 
}
