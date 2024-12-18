use iced::{
    widget::Text,
    Element,
};
use crate::gui::components::Component;

#[derive(Debug, Clone)]
pub enum Message {
    // Empty for now
}

pub struct MixerView {
    // Empty for now
}

impl Component for MixerView {
    type Message = Message;

    fn new() -> Self {
        MixerView {}
    }

    fn view(&self) -> Element<Message> {
        Text::new("Mixer View").into()
    }

    fn update(&mut self, _message: Message) {
        // Empty for now
    }
} 