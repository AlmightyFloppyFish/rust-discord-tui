use interface::input::{Mode, ModeSwitcher};
use interface::split::ChatPane;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use termion::event::{Event, Key};
use termion::input::TermRead;

mod discord;
mod interface;

use interface::Tui;

struct Session {
    // history: RwLock<Vec<Message>>,
    settings: RwLock<HashMap<String, String>>,
    tui: Mutex<Tui>,
    discord: RwLock<i32>,

    // This contains all panes
    chat_panes: Vec<RwLock<ChatPane>>,
    active_pane: RwLock<u8>,

    // I can just read from Insert(T).last() to see if it should switch
    mode: ModeSwitcher,
}

impl Session {
    fn new() -> Result<Self, io::Error> {
        Ok(Session {
            tui: Mutex::new(interface::Tui::new()),
            settings: RwLock::new(HashMap::new()),
            chat_panes: vec![RwLock::new(ChatPane::new("TEST"))],
            active_pane: RwLock::new(0),
            discord: RwLock::new(0),
            mode: ModeSwitcher::new(""),
        })
    }

    fn dummy_new() -> Self {
        Session {
            tui: Mutex::new(interface::Tui::new()),
            settings: RwLock::new(HashMap::new()),
            mode: ModeSwitcher::new(""),
            chat_panes: vec![
                RwLock::new(ChatPane::new_dummy("TEST")),
                RwLock::new(ChatPane::new_dummy("TEST2")),
                RwLock::new(ChatPane::new_dummy("TEST3")),
                RwLock::new(ChatPane::new_dummy("TEST4")),
            ],
            active_pane: RwLock::new(0),
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
            // Backdoor escape, remember to remove
            Event::Key(Key::Char('ö')) => break,
            Event::Key(Key::Esc) => {
                s.mode.set_mode(Mode::Insert);
                s.mode.exec_buffer.write().unwrap().clear();
            }
            Event::Key(Key::Backspace) => {
                match s.mode.is_exec() {
                    true => s.mode.exec_buffer.write().unwrap().pop(),
                    false => s.mode.insert_buffer.write().unwrap().pop(),
                };
            }
            Event::Key(Key::Ctrl(direction)) => match direction {
                'h' => {
                    let mut active = s.active_pane.write().unwrap();
                    if *active > 0 {
                        *active -= 1;
                    }
                }
                'l' => {
                    let mut active = s.active_pane.write().unwrap();
                    if *active < (s.chat_panes.len() - 1) as u8 {
                        *active += 1;
                    };
                }
                _ => (),
            },

            // Might seem weird but this is actually have you do Key::Enter in termion
            Event::Key(Key::Char('\n')) => {
                match s.mode.is_exec() {
                    true => {
                        // Check validity
                        //
                        // Do command if valid
                        //
                        // Else notify user of it being invalid and allow him to continue
                        panic!("");
                    }
                    false => {
                        // Send message
                    }
                }
            }
            Event::Key(Key::Char(c)) => match s.mode.is_exec() {
                true => s.mode.exec_buffer.write().unwrap().push(c),
                false => {
                    if c == ':' && s.mode.is_escaping() {
                        s.mode.escape();
                    } else {
                        s.mode.insert_buffer.write().unwrap().push(c)
                    }
                }
            },
            _ => {}
        }
        s.update().unwrap();
    }

    s.tui.lock().unwrap().terminal.show_cursor().unwrap();
    s.tui.lock().unwrap().clear();
}
