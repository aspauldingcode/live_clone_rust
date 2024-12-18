use iced::{
    widget::Text,
    Element,
};
use crate::gui::components::Component;

#[derive(Debug, Clone)]
pub enum Message {
    // Empty for now
}

pub struct TimelineView {
    // Empty for now
}

impl Component for TimelineView {
    type Message = Message;

    fn new() -> Self {
        TimelineView {}
    }

    fn view(&self) -> Element<Message> {
        Text::new("Timeline View").into()
    }

    fn update(&mut self, _message: Message) {
        // Empty for now
    }
} 