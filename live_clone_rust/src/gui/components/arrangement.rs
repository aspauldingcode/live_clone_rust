use iced::{
    widget::{Container, Text, Column},
    Element, Length,
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

    fn view(&mut self) -> Element<Message> {
        Container::new(
            Column::new()
                .push(Text::new("Arrangement"))
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn update(&mut self, _message: Message) {
        // Empty for now
    }
} 