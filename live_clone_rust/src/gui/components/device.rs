use iced::{
    widget::Text,
    Element,
};
use crate::gui::components::Component;

#[derive(Debug, Clone)]
pub enum Message {
    // Empty for now
}

pub struct DeviceView {
    // Empty for now
}

impl Component for DeviceView {
    type Message = Message;

    fn new() -> Self {
        DeviceView {}
    }

    fn view(&self) -> Element<Message> {
        Text::new("Device View").into()
    }

    fn update(&mut self, _message: Message) {
        // Empty for now
    }
} 