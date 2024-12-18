use iced::{
    widget::Text,
    Element,
};
use crate::gui::components::Component;

#[derive(Debug, Clone)]
pub enum Message {
    // Empty for now
}

pub struct ArrangementView {
    // Empty for now
}

impl Component for ArrangementView {
    type Message = Message;

    fn new() -> Self {
        ArrangementView {}
    }

    fn view(&self) -> Element<Message> {
        Text::new("Arrangement View").into()
    }

    fn update(&mut self, _message: Message) {
        // Empty for now
    }
} 