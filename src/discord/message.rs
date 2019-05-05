use std::fmt;
use termion::color;
use tui::style::{Color, Style};
use tui::widgets::Text;

pub enum Embed {
    Link(String),
    Image(String),
    Video(String),
    Nothing,
}

pub struct Message {
    pub content: String,
    pub embed: Embed,
    pub author: String,
}

impl Message {
    pub fn new(author: &str, content: &str, embed: Embed) -> Self {
        Message {
            author: author.to_owned(),
            content: content.to_owned(),
            embed: embed,
        }
    }

    pub fn author(&self) -> Text {
        Text::styled(&self.author, Style::default().fg(Color::Green))
    }

    pub fn embed(&self) -> Text {
        let (link, color) = match &self.embed {
            Embed::Nothing => return Text::raw("\n"),
            Embed::Image(link) => (link, Color::Cyan),
            Embed::Link(link) => (link, Color::Blue),
            Embed::Video(link) => (link, Color::Red),
        };
        let mut out = String::new();
        out.push_str("\n  - ");
        out.push_str(link);
        out.push_str("\n");
        return Text::styled(out, Style::default().fg(color));
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}: {}{} {}{}",
            color::Fg(color::Red),
            self.author,
            color::Fg(color::Reset),
            self.content,
            match &self.embed {
                Embed::Nothing => String::from(""),
                Embed::Link(l) => format!("\n    - {}{}", color::Fg(color::Green), l),
                Embed::Image(l) => format!("\n    - {}{}", color::Fg(color::LightGreen), l),
                Embed::Video(l) => format!("\n    - {}{}", color::Fg(color::LightGreen), l),
            },
            color::Fg(color::Reset)
        )
    }
}
