use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use tauri::{Manager, Window};
use watcher::{Event, RecursiveMode, Watcher};

pub fn start_watcher(window: Window) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch("/", RecursiveMode::Recursive).unwrap();

    let manager = Manager::default();
    let runtime = manager.runtime();

    runtime.spawn(async move {
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Event::Create(path) = event {
                        let path_string = path.to_str().unwrap().to_string();
                        let message = format!("File created: {}", path_string);
                        window.emit("file-created", message).unwrap();
                        // Run your virus scan function here
                    }
                }
                Err(e) => {
                    println!("Watch error: {:?}", e);
                    break;
                }
            }
        }
    });

    thread::spawn(move || {
        loop {
            std::thread::park();
        }
    });
}
