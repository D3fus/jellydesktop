use std::process::Command;
use std::process::Child;
use crate::app::server;
use mpv;

pub struct Player {
    pub player: Child,
    pub server: server::Server,
    pub list: Vec<server::ServerList>,
    pub index: usize,
    pub auto_play_timeout: usize,
    pub playing: bool
}

impl Player {
    pub fn new() -> Player {
        let tmp_server = server::Server::empty();
        Player {
            player: Command::new("echo").spawn().unwrap(),
            server: tmp_server,
            list: vec![],
            index: 0,
            auto_play_timeout: 0,
            playing: false
        }
    }

    pub fn add_list(&mut self, list: Vec<server::ServerList>, index: usize, server: &server::Server) {
        self.server = server.clone();
        self.list = list;
        self.index = index;
    }

    pub fn play(&mut self, volume: i32, full: bool) {
        //TODO error if not installed
        let fullscreen = if full {
            String::from("--fullscreen")
        } else {
            String::from("")
        };
        let uri = format!("{}/Items/{}/Download?api_key={}",
                self.server.uri,
                self.list[0].id,
                self.server.user.token
            );
        self.player = Command::new("mpv")
            .args(&[
                uri,
                "--really-quiet".to_string(),
                format!("--volume={}", volume.to_string()),
                format!("--media-title={}. {}", self.list[0].index_nummer, self.list[0].name),
                fullscreen
            ])
            .spawn()
            .unwrap();
        self.list.remove(0);
        self.index += 1;
        self.auto_play_timeout = 100;
    }

    pub fn play_next(&mut self) {
        self.auto_play_timeout = 0;
    }

    pub fn stop_auto_play(&mut self) {
        self.list = vec![];
        self.auto_play_timeout = 0;
        self.playing = false;
        self.index = 0;
    }

    pub fn ready_to_play(&mut self) -> bool {
        self.is_playing();
        if !self.playing && !self.list.is_empty() {
            if self.auto_play_timeout > 0 {
                self.auto_play_timeout -= 1;
                false
            } else {
                true
            }
        } else {
            if self.list.is_empty() && self.auto_play_timeout > 0 {
                self.auto_play_timeout = 0;
            }
            false
        }
    }

    pub fn is_playing(&mut self) {
        self.playing = match self.player.try_wait() {
            Ok(Some(_s)) => {false},
            Ok(None) => {true}
            Err(_e) => {true},
        }
    }
}

pub struct MpvPlayer {
    pub mpv: mpv::MpvHandler,
    pub playlist: Vec<server::ServerList>
}

impl MpvPlayer {
    pub fn new() -> MpvPlayer {
        let mut mpv_builder = mpv::MpvHandlerBuilder::new().unwrap();
        mpv_builder.set_option("config", "").unwrap();
        mpv_builder.set_option("volume", "50").unwrap();
        let mut mpv = mpv_builder.build().unwrap();
        MpvPlayer {
            mpv: mpv,
            playlist: vec![]
        }
    }

    pub fn play_item(&mut self, item: server::ServerList) {}

    pub fn play_playlist(&mut self) {}

    pub fn add_to_playlist(&mut self, mut items: Vec<server::ServerList>) {
        self.playlist.append(&mut items);
    }

    pub fn check_if_ready(&mut self) -> bool {
        false
    }

    pub fn update_player(&mut self) {
       
    }

    pub fn close_player(&mut self) {

    }

    async fn send_play_status(&self) {

    }
}
