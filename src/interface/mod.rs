use crate::discord::message::Embed;
use crate::Session;
use std::io;
use std::sync::{Arc, Mutex};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;

pub struct Tui {
    pub terminal: Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
}

impl Session {
    pub fn update(&self) -> Result<(), io::Error> {
        self.tui.lock().unwrap().terminal.draw(|mut f| {
            /* Scope:
             *  I want atleast one vsplit to be a thing to begin with
             *  The selected one should have borders, this will allow me to leave
             *  the input field unchanged between split switches.
             *
             *  I want some dialog to open up when you escape.
             *  I'll probably make the escape text `::` and not `:`
             */
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(96), Constraint::Percentage(4)].as_ref())
                .split(f.size());
            let inner = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Min(20), Constraint::Percentage(100)].as_ref())
                .split(outer[0]);
            // Text box
            Block::default()
                .borders(Borders::ALL)
                .render(&mut f, outer[1]);
            // Sidebar
            Block::default()
                .borders(Borders::ALL)
                .render(&mut f, inner[0]);

            // -> Chat space
            // This cannot use List::new() untill pull #102 is merged, so here's the worst hack of
            // my life instead.
            let stored = self.history.read().unwrap();
            let mut text_buf = Vec::with_capacity(stored.len() * 5);

            for m in stored.iter() {
                text_buf.push(m.author());
                text_buf.push(Text::raw(": "));
                text_buf.push(Text::raw(&m.content));
                text_buf.push(m.embed());
            }

            Paragraph::new(text_buf.iter())
                .block(Block::default().borders(Borders::ALL))
                .render(&mut f, inner[1]);

            // <- Chat space
        })?;
        Ok(())
    }
}

impl Tui {
    pub fn new() -> Self {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let backend = TermionBackend::new(stdout);
        Tui {
            terminal: Terminal::new(backend).unwrap(),
        }
    }
    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
        self.terminal.flush().unwrap();
    }
}
