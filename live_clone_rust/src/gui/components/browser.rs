use iced::{
    widget::Text,
    Element,
};
use crate::gui::components::Component;

#[derive(Debug, Clone)]
pub enum Message {
    // Empty for now
}

pub struct BrowserView {
    // Empty for now
}

impl Component for BrowserView {
    type Message = Message;

    fn new() -> Self {
        BrowserView {}
    }

    fn view(&self) -> Element<Message> {
        Text::new("Browser View").into()
    }

    fn update(&mut self, _message: Message) {
        // Empty for now
    }
} 