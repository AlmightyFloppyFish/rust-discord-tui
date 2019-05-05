use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;

mod discord;
mod interface;

use discord::message::{Embed, Message};
use interface::Tui;

struct Session {
    history: RwLock<Vec<Message>>,
    settings: RwLock<HashMap<String, String>>,
    tui: Mutex<Tui>,
    discord: RwLock<i32>,
}

impl Session {
    fn new() -> Result<Self, io::Error> {
        Ok(Session {
            tui: Mutex::new(interface::Tui::new()),
            settings: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
            discord: RwLock::new(0),
        })
    }

    fn dummy_new() -> Self {
        Session {
            tui: Mutex::new(interface::Tui::new()),
            settings: RwLock::new(HashMap::new()),
            history: RwLock::new(vec![
                Message::new(
                    "Bertill",
                    "Here's a link!",
                    Embed::Link(String::from("https://www.some-domain.com/")),
                ),
                Message::new(
                    "Bertill",
                    "faef egkajhg lakgh klv ka glr glvagnr an eafg a",
                    Embed::Nothing,
                ),
                Message::new("greg", "fjkfda gfarg?", Embed::Nothing),
                Message::new(
                    "greg",
                    "Here's a video",
                    Embed::Video(String::from("https://youtube.com/4832348")),
                ),
                Message::new(
                    "Some User",
                    "Here's an image",
                    Embed::Image(String::from("https://www.imgur.com/awefa")),
                ),
            ]),
            discord: RwLock::new(0),
        }
    }
}

fn main() {
    let s = Arc::new(Session::dummy_new());

    {
        // Setup
        match s.tui.lock() {
            Ok(mut t) => {
                t.clear();
                t.terminal.hide_cursor().unwrap();
            }
            Err(_) => (),
        }
    }

    let timed = s.clone();
    thread::spawn(move || loop {
        timed.update().unwrap();
        thread::sleep(Duration::from_millis(1000));
    });

    let stdin = io::stdin();
    for c in stdin.events() {
        let event = c.unwrap();
        match event {
            Event::Key(Key::Char('q')) => break,
            _ => {}
        }
        s.update().unwrap();
    }

    s.tui.lock().unwrap().clear();
    s.tui.lock().unwrap().terminal.show_cursor().unwrap();
}
