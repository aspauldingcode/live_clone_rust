use iced::{
    widget::{Container, Text, Column},
    Element, Length,
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

    fn view(&mut self) -> Element<Message> {
        Container::new(
            Column::new()
                .push(Text::new("Device"))
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn update(&mut self, _message: Message) {
        // Empty for now
    }
} 