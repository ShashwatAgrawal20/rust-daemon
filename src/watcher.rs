use inotify::{Inotify, WatchMask};
use std::time::Duration;

pub fn watcher(directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut inotify = Inotify::init()?;
    println!("Watching: {}", directory);
    inotify.watches().add(directory, WatchMask::ALL_EVENTS)?;

    let mut buffer = [0; 4096];
    loop {
        let events = match inotify.read_events_blocking(&mut buffer) {
            Ok(events) => events,
            Err(err) => {
                eprintln!("Error reading events: {}", err);
                continue;
            }
        };

        for event in events {
            println!("Event:- {:#?}", event);
        }

        std::thread::sleep(Duration::from_millis(500));
    }
}
