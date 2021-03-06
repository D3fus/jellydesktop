use xdg;
use serde::{Deserialize, Serialize};
use std::fs::File;
use serde_json;
use std::path::Path;
use std::io::BufReader;
use std::io::Write;
use crate::app::{server, error};

fn default_server() -> Vec<server::Server> {
    Vec::new()
}

fn default_last_active() -> i32 {
    -1
}

fn default_auto_play_ep() -> bool {
    true
}

fn default_auto_play_mv() -> bool {
    false
}

fn default_mpv_volume() -> i32 {
    100
}

fn default_mpv_full_screen() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(default = "default_server")]
    pub server_list: Vec<server::Server>,
    #[serde(default = "default_last_active")]
    pub last_active_server: i32,
    #[serde(default = "default_auto_play_ep")]
    pub auto_play_episode: bool,
    #[serde(default = "default_auto_play_mv")]
    pub auto_play_movie: bool,
    #[serde(default = "default_mpv_volume")]
    pub mpv_volume: i32,
    #[serde(default = "default_mpv_full_screen")]
    pub mpv_full_screen: bool
}

impl ConfigFile {
    pub fn load_or_create() -> Result<ConfigFile, error::Error> {
        let dir = get_config_dir();
        if !Path::new(&dir).exists() {
            create_config_file()?;
        }
        let file = File::open(&dir).unwrap();
        let reader = BufReader::new(file);
        let j: ConfigFile = match serde_json::from_reader(reader) {
            Ok(j) => j,
            Err(_e) => return Err(error::Error::error("Error reading config file"))
        };
        Ok(j)
    }

    pub fn get_server_list(&self) -> Vec<server::Server> {
        self.server_list.clone()
    }

    pub fn get_config(&self) -> Config {
        Config {
            auto_play_episode: self.auto_play_episode,
            auto_play_movie: self.auto_play_movie,
            mpv_volume: self.mpv_volume,
            mpv_full_screen: self.mpv_full_screen
        }
    }

    pub fn save(conf: &Config, server: Vec<server::Server>, last: i32) -> Result<(), error::Error> {
        let conf_file = ConfigFile {
            server_list: server,
            last_active_server: last,
            auto_play_episode: conf.auto_play_episode,
            auto_play_movie: conf.auto_play_movie,
            mpv_volume: conf.mpv_volume,
            mpv_full_screen: conf.mpv_full_screen
        };
        let data = serde_json::to_string_pretty(&conf_file).unwrap();
        let dir = get_config_dir();
        let mut file = match File::create(&dir) {
            Ok(file) => file,
            Err(_e) => return Err(error::Error::error("Error by opening config file"))
        };
        match file.write_all(data.as_bytes()) {
            Ok(()) => Ok(()),
            Err(_e) => Err(error::Error::error("Error by writing to config file"))
        }
    }
}

fn get_base_dir() -> xdg::BaseDirectories {
    xdg::BaseDirectories::with_prefix("jellydesktop").unwrap()
}

fn get_config_dir() -> std::path::PathBuf {
    get_base_dir().place_config_file("config.json")
                .expect("cannot create configuration directory")
}

fn create_config_file() -> Result<(), error::Error> {
    let dir = get_config_dir();
    let mut file = match File::create(&dir) {
        Ok(f) => {f},
        Err(_e) => {return Err(error::Error::error("Error creating config file"))}
    };
    match file.write_all(b"{}") {
        Ok(()) => {Ok(())},
        Err(_e) => {Err(error::Error::error("Error writing to config file"))}
    }

}

pub struct Config {
    pub auto_play_episode: bool,
    pub auto_play_movie: bool,
    pub mpv_volume: i32,
    pub mpv_full_screen: bool
}

impl Config {
    pub fn empty() -> Config {
        Config {
            auto_play_episode: false,
            auto_play_movie: false,
            mpv_volume: 0,
            mpv_full_screen: false
        }
    }

    pub fn input(&mut self, c: char) {
        let mut temp = self.mpv_volume.to_string();
        if temp.len() < 3 {
            match c as u32 {
                48..= 57 => {
                    temp.push(c);
                    self.mpv_volume = temp.parse().unwrap();
                },
                _ => {}
            }
        }
    }

    pub fn del(&mut self) {
        if self.mpv_volume > 0 {
            let mut temp = self.mpv_volume.to_string();
            if temp.len() > 1 {
                temp.truncate(temp.len() -1);
                self.mpv_volume = temp.parse().unwrap();
            } else {
                self.mpv_volume = 0;
            }
        }
    }

    pub fn save(&self, server: Vec<server::Server>, last: i32) -> Result<(), error::Error> {
        ConfigFile::save(self, server, last)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), error::Error> {
        let conf = ConfigFile::load_or_create()?;
        self.mpv_volume = conf.mpv_volume;
        self.mpv_full_screen = conf.mpv_full_screen;
        self.auto_play_movie = conf.auto_play_movie;
        self.auto_play_episode = conf.auto_play_episode;
        Ok(())
    }
}

