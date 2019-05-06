use crate::Session;
use std::io;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::*;
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

pub mod input;
pub mod split;

pub struct Tui {
    pub terminal: Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
}

impl Session {
    pub fn update(&self) -> Result<(), io::Error> {
        self.tui.lock().unwrap().terminal.draw(|mut f| {
            let is_insert = self.mode.is_insert();
            /* Scope:
             *  I want atleast one vsplit to be a thing to begin with
             *  The selected one should have borders, this will allow me to leave
             *  the input field unchanged between split switches.
             *
             *  I want some dialog to open up when you escape.
             *  I'll probably make the escape text `::` and not `:`
             *
             *  For now this kind of resembles MVC
             *
             *  I need to create an abstraction and have a struct for "chat pane". And then gen
             *  them by iterating.
             */
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(96), Constraint::Percentage(4)].as_ref())
                .split(f.size());
            let mut total_splits = self.chat_panes.len();
            if total_splits < 1 {
                total_splits = 1
            };
            let mut split_space = vec![Constraint::Min(20)];
            for _ in 0..total_splits {
                split_space.push(Constraint::Percentage(100 / total_splits as u16));
            }
            let inner = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(split_space.as_ref())
                .split(outer[0]);
            // Text box
            if is_insert {
                Paragraph::new([Text::raw(&*self.mode.insert_buffer.read().unwrap())].iter())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" Mode: Insert "),
                    )
                    .wrap(false)
                    .render(&mut f, outer[1]);
            } else {
                Paragraph::new(
                    [Text::styled(
                        &*self.mode.exec_buffer.read().unwrap(),
                        Style::default().fg(Color::Blue),
                    )]
                    .iter(),
                )
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Mode: Exec   ")
                        .title_style(Style::default().fg(Color::Blue)),
                )
                .wrap(false)
                .render(&mut f, outer[1]);
            }
            // Sidebar
            Block::default()
                .borders(Borders::ALL)
                .render(&mut f, inner[0]);

            let mut count = 1 as usize;
            for (pane_id, pane) in self.chat_panes.iter().enumerate() {
                Paragraph::new(pane.read().unwrap().get_tui_text().iter())
                    .block(Block::default().borders(Borders::ALL).border_style(
                        if *self.active_pane.read().unwrap() == pane_id as u8 {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default().fg(Color::Reset)
                        },
                    ))
                    .wrap(true)
                    .render(&mut f, inner[count]);
                count += 1;
            }
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
