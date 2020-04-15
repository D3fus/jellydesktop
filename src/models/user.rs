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

struct UserConfiguration {
  AudioLanguagePreference: String,
  PlayDefaultAudioTrack: bool,
  SubtitleLanguagePreference: String,
  DisplayMissingEpisodes: bool,
  GroupedFolders: Vec<String>,
  SubtitleMode: String,
  DisplayCollectionsView: bool,
  EnableLocalPassword: bool,
  OrderedViews: Vec<String>,
  LatestItemsExcludes: Vec<String>,
  MyMediaExcludes: Vec<String>,
  HidePlayedInLatest: bool,
  RememberAudioSelections: bool,
  RememberSubtitleSelections: bool,
  EnableNextEpisodeAutoPlay: bool
}

struct UserPolicy {
  IsAdministrator: bool,
  IsHidden: bool,
  IsHiddenRemotely: bool,
  IsDisabled: bool,
  MaxParentalRating: i32,
  BlockedTags: Vec<String>,
  EnableUserPreferenceAccess: bool,
  AccessSchedules: Vec<AccessSchedules>,
  BlockUnratedItems: Vec<String>,
  EnableRemoteControlOfOtherUsers: bool,
  EnableSharedDeviceControl: bool,
  EnableRemoteAccess: bool,
  EnableLiveTvManagement: bool,
  EnableLiveTvAccess: bool,
  EnableMediaPlayback: bool,
  EnableAudioPlaybackTranscoding: bool,
  EnableVideoPlaybackTranscoding: bool,
  EnablePlaybackRemuxing: bool,
  EnableContentDeletion: bool, 
  EnableContentDeletionFromFolders: Vec<String>,
  EnableContentDownloading: bool,
  EnableSubtitleDownloading: bool,
  EnableSubtitleManagement: bool,
  EnableSyncTranscoding: bool,
  EnableMediaConversion: bool,
  EnabledDevices: Vec<String>,
  EnableAllDevices: bool,
  EnabledChannels: Vec<String>,
  EnableAllChannels: bool,
  EnabledFolders: Vec<String>,
  EnableAllFolders: bool,
  InvalidLoginAttemptCount: i32,
  EnablePublicSharing: bool,
  BlockedMediaFolders: Vec<String>,
  BlockedChannels: Vec<String>,
  RemoteClientBitrateLimit: i32,
  AuthenticationProviderId: String,
  ExcludedSubFolders: Vec<String>,
  DisablePremiumFeatures: bool
}

struct AccessSchedules {
  DayOfWeek: String,
  StartHour: f64,
  EndHour: f64
}
