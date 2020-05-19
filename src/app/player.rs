use mpv;
use std::thread;

pub fn play(uri: String) {
    thread::spawn(move ||{
    let mut mpv = mpv::MpvHandlerBuilder::new().expect("aaa");
    mpv.set_option("osc",true).unwrap();
    let mut mpv = mpv.build();
    match mpv {
        Ok(mut m) => {
            m.command_async(&["loadfile", &uri], 5).unwrap();
            m.set_property("loop","1").unwrap();
            m.set_property("speed",1.0).unwrap();
            m.wait_event(0.0);
            'main: loop {
                while let Some(event) = m.wait_event(0.0) {
                    // even if you don't do anything with the events, it is still necessary to empty
                    // the event loop
                    match event {
                        // Shutdown will be triggered when the window is explicitely closed,
                        // while Idle will be triggered when the queue will end
                        mpv::Event::Shutdown | mpv::Event::Idle => {
                            break 'main;
                        }
                        _ => {}
                    };
                }
            }
        },
        Err(e) => println!("{:?}", e)
    }});
}
