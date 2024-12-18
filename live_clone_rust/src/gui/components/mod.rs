pub mod arrangement;
pub mod browser;
pub mod device;
pub mod mixer;
pub mod session;
pub mod timeline;
pub mod transport;

use iced::Element;

#[allow(dead_code)]
pub trait Component {
    type Message;

    fn new() -> Self;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
} 