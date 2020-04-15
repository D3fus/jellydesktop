struct SessionInfo {
  PlayState: PlayerStateInfo,
  AdditionalUsers: Vec<AdditionalUsersInfo>,	
  Capabilities	ClientCapabilities{...}
  RemoteEndPoint	string
  PlayableMediaTypes	[string]
  PlaylistItemId	string
  Id	string
  ServerId	string
  UserId	string
  UserName	string
  UserPrimaryImageTag	string
  Client	string
  LastActivityDate	string($date-time)
  DeviceName	string
  DeviceType	string
  NowPlayingItem	BaseItemDto{...}
  DeviceId	string
  ApplicationVersion	string
  AppIconUrl	string
  SupportedCommands	[string]
  TranscodingInfo	TranscodingInfo{...}
  SupportsRemoteControl	boolean
}

struct PlayerStateInfo {
  PositionTicks: Option<i64>, 
  CanSeek: bool,
  IsPaused: bool,
  IsMuted: bool,
  VolumeLevel: Option<i32>,
  AudioStreamIndex: Option<i32>,
  SubtitleStreamIndex: Option<i32>,
  MediaSourceId: String,
  PlayMethod: String,
  RepeatMode: String
}

struct AdditionalUsersInfo {
  UserId: String,
  UserName: String,
  UserInternalId: i64
}

struct ClientCapabilities {
  PlayableMediaTypes: Vec<String>,
  SupportedCommands: Vec<String>,
  SupportsMediaControl: bool,
  PushToken: String,
  PushTokenType: String,
  SupportsPersistentIdentifier: bool,
  SupportsSync: bool,
  DeviceProfile	Dlna.DeviceProfile{...}
  IconUrl	string
  AppId	string
}

struct DeviceProfile {
  Name: String,
  Id: String,
  Identification	Dlna.DeviceIdentification{...}
  FriendlyName	string
  Manufacturer	string
  ManufacturerUrl	string
  ModelName	string
  ModelDescription	string
  ModelNumber	string
  ModelUrl	string
  SerialNumber	string
  EnableAlbumArtInDidl	boolean
  EnableSingleAlbumArtLimit	boolean
  EnableSingleSubtitleLimit	boolean
  SupportedMediaTypes	string
  UserId	string
  AlbumArtPn	string
  MaxAlbumArtWidth	integer($int32)
  MaxAlbumArtHeight	integer($int32)
  MaxIconWidth	integer($int32, nullable)
  MaxIconHeight	integer($int32, nullable)
  MaxStreamingBitrate	integer($int64, nullable)
  MaxStaticBitrate	integer($int64, nullable)
  MusicStreamingTranscodingBitrate	integer($int32, nullable)
  MaxStaticMusicBitrate	integer($int32, nullable)
  SonyAggregationFlags	string
  ProtocolInfo	string
  TimelineOffsetSeconds	integer($int32)
  RequiresPlainVideoItems	boolean
  RequiresPlainFolders	boolean
  EnableMSMediaReceiverRegistrar	boolean
  IgnoreTranscodeByteRangeRequests	boolean
  XmlRootAttributes	[Dlna.XmlAttribute{...}]
  DirectPlayProfiles	[Dlna.DirectPlayProfile{...}]
  TranscodingProfiles	[Dlna.TranscodingProfile{...}]
  ContainerProfiles	[Dlna.ContainerProfile{...}]
  CodecProfiles	[Dlna.CodecProfile{...}]
  ResponseProfiles	[Dlna.ResponseProfile{...}]
  SubtitleProfiles	[Dlna.SubtitleProfile{...}]
}

struct DeviceIdentification {
  FriendlyName: String,
  ModelNumber: String,
  SerialNumber: String,
  ModelName: String,
  ModelDescription: String,
  DeviceDescription: String,
  ModelUrl: String,
  Manufacturer: String,
  ManufacturerUrl: String,

  Headers, String
}
