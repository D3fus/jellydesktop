use std::process::Command;
use std::process::Child;
use crate::app::server;

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

//TODO use mpvlib
//pub fn play<'a>(uri: String, playing: &Arc<Mutex<Test>>, sender: std::sync::mpsc::Sender<Test<'a>>) {
//    thread::spawn(move ||{
//        let mut t = Test::new();
//        t.playing = &true;
//        sender.send(t).unwrap();
//        let mut mpv = mpv::MpvHandlerBuilder::new().expect("aaa");
//        mpv.set_option("osc",true).unwrap();
//        let mut mpv = mpv.build();
//        match mpv {
//            Ok(mut m) => {
//                m.command_async(&["loadfile", &uri], 5).unwrap();
//                m.set_property("loop","1").unwrap();
//                m.set_property("speed",1.0).unwrap();
//                m.wait_event(0.0);
//                'main: loop {
//                    while let Some(event) = m.wait_event(0.0) {
//                        // even if you don't do anything with the events, it is still necessary to empty
//                        // the event loop
//                        match event {
//                            // Shutdown will be triggered when the window is explicitely closed,
//                            // while Idle will be triggered when the queue will end
//                            mpv::Event::Shutdown | mpv::Event::Idle => {
//                                break 'main;
//                                t.playing = &false;
//                                sender.send(t).unwrap();
//                            }
//                            _ => {}
//                        };
//                    }
//                }
//            },
//            Err(e) => println!("{:?}", e)
//        }
//    });
//}
