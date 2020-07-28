use std::process::Command;
use std::process::Child;
use crate::app::{server, config};
use libmpv;
use libmpv::FileState::{Append, AppendPlay};
use tempfile::NamedTempFile;
use std::io::{self, Write, Read};

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
    pub mpv: libmpv::Mpv,
    pub playlist: Vec<server::ServerList>,
    pub server: server::Server,
    pub finished: Vec<String>,
    pub autoplay_timer: i32,
    volume: i32
}

impl MpvPlayer {
    pub fn new(config: &config::Config) -> MpvPlayer {
        let mpv = libmpv::Mpv::new().unwrap();
        mpv.set_property("osc", "").unwrap();
        mpv.set_property("keep-open", "yes").unwrap();
        let server = server::Server::empty();
        MpvPlayer {
            mpv: mpv,
            playlist: vec![],
            server: server,
            finished: vec![],
            autoplay_timer: -1,
            volume: config.mpv_volume
        }
    }

    pub fn play_item(&mut self, item: server::ServerList) {
        let item = &self.playlist[0];
        let uri = self.format_server_uri(&item);
        let title = if item.category != "Episode" {
            format!("media-title={}", item.name)
        } else {
            format!("media-title={}. {}", item.index_nummer, item.name)
        };
        self.mpv.playlist_load_files(&[(&uri, Append, Some(&title))]).unwrap();
    }

    pub fn play_from_playlist(&mut self) {

    }

    pub fn get_next_playing(&self) -> Option<&server::ServerList> {
        if self.playlist.len() > 1 {
            Some(&self.playlist[1])
        } else {
            None
        }
    }

    pub fn show_autoplay(&self) -> bool {
        self.autoplay_timer != -1 && self.playlist.len() > 1
    }

    pub fn stop_autoplay(&mut self) {
        self.playlist = vec![];
        self.autoplay_timer = -1;
        self.mpv.playlist_clear().unwrap();
    }

    pub fn play_playlist(&mut self) {
        let mut file = NamedTempFile::new().unwrap();
        let mut m3u = String::from("#EXTM3U\n");
        for (i, item) in self.playlist.iter().enumerate() {
            let uri = self.format_server_uri(&item);
            let title = if item.category != "Episode" {
                format!("{}", item.name)
            } else {
                format!("{}. {}", item.index_nummer, item.name)
            };
            m3u = format!("{}#EXTINF: {}, {}\n{}\n", m3u, item.runtime, title, uri);
        }
        file.write_all(m3u.as_bytes()).unwrap();
        let path = file.into_temp_path();
        self.mpv.playlist_load_list(path.to_str().unwrap(), true).unwrap();
        self.mpv.set_property("volume", &*self.volume.to_string()).unwrap();
        self.mpv.unpause().unwrap();
    }


    pub fn add_to_playlist(&mut self, mut items: Vec<server::ServerList>) {
        self.playlist.append(&mut items);
    }

    pub fn check_if_ready(&mut self) -> bool {
        false
    }

    pub fn on_tick(&mut self) {
        if self.playlist.len() > 0 {
            let mut item = &mut self.playlist[0];
            match self.mpv.get_property::<i64>("percent-pos") {
                Ok(progress) => {
                    if progress >= 95 && !item.played {
                        item.played = true;
                        self.finished.push(item.id.clone());
                    }
                },
                Err(_) => {}
            };
            match self.mpv.get_property::<i64>("time-remaining") {
                Ok(timer) => {
                    if timer <= 20 {
                        self.autoplay_timer = timer as i32;
                    } else if timer == 0 {
                        self.autoplay_timer = -1;
                    }
                },
                Err(_) => {
                    self.autoplay_timer = -1;
                }
            };
        }
    }

    pub fn get_seen(&self) -> (&Vec<String>, &server::Server) {
        (&self.finished, &self.server)
    }

    pub fn clear_seen(&mut self) {
        self.finished = vec![];
    }

    pub fn open_player(&self) {
        self.mpv.set_property("force-window", "yes").unwrap();
    }

    pub fn close_player(&mut self) {

    }

    fn format_server_uri(&self, item: &server::ServerList) -> String {
        format!("{}/Items/{}/Download?api_key={}",
            self.server.uri,
            item.id,
            self.server.user.token
        )
    }

    async fn send_play_status(&self) {

    }
}
