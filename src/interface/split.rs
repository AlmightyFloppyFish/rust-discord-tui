use crate::discord::message::{Embed, Message};
use tui::widgets::Text;

pub struct ChatPane {
    dg_channel_id: String,
    msg_buffer: Vec<Message>,
    // Should i have this?: link_history: Vec<String>,
}

impl ChatPane {
    // TODO:
    // I think i should do message preloading here
    pub fn new(channel_id: &str) -> Self {
        ChatPane {
            dg_channel_id: channel_id.to_owned(),
            msg_buffer: Vec::new(),
        }
    }

    pub fn new_dummy(channel_id: &str) -> Self {
        ChatPane {
            dg_channel_id: channel_id.to_owned(),
            msg_buffer: vec![
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
            ],
        }
    }

    // Call .render() in scope after to include positionals
    pub fn get_tui_text(&self) -> Vec<Text> {
        let mut text_buf = Vec::with_capacity(&self.msg_buffer.len() * 5);

        for m in self.msg_buffer.iter() {
            text_buf.push(m.author());
            text_buf.push(Text::raw(": "));
            text_buf.push(Text::raw(&m.content));
            text_buf.push(m.embed());
        }
        return text_buf;
    }
}
