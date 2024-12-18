use iced::{
    widget::{Container, Text, Column},
    Element, Length,
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

    fn view(&mut self) -> Element<Message> {
        Container::new(
            Column::new()
                .push(Text::new("Browser"))
        )
        .width(Length::Units(200))
        .height(Length::Fill)
        .into()
    }

    fn update(&mut self, _message: Message) {
        // Empty for now
    }
} 