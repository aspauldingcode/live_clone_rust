pub mod components;

use iced::widget::{Column, Container, Row};
use iced::{Element, Length};

use self::components::{
    arrangement::ArrangementView,
    browser::BrowserView,
    device::DeviceView,
    mixer::MixerView,
    session::SessionView,
    timeline::TimelineView,
    transport::TransportBar,
    Component,
};

#[derive(Debug, Clone)]
pub enum Message {
    ArrangementMessage(components::arrangement::Message),
    BrowserMessage(components::browser::Message),
    DeviceMessage(components::device::Message),
    MixerMessage(components::mixer::Message),
    SessionMessage(components::session::Message),
    TimelineMessage(components::timeline::Message),
}

pub struct MainWindow {
    arrangement: ArrangementView,
    browser: BrowserView,
    device: DeviceView,
    mixer: MixerView,
    session: SessionView,
    timeline: TimelineView,
    transport: TransportBar,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            arrangement: ArrangementView::new(),
            browser: BrowserView::new(),
            device: DeviceView::new(),
            mixer: MixerView::new(),
            session: SessionView::new(),
            timeline: TimelineView::new(),
            transport: TransportBar::new(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let main_content = Column::new()
            .push(self.transport.view())
            .push(
                Row::new()
                    .push(self.browser.view().map(Message::BrowserMessage))
                    .push(
                        Column::new()
                            .push(self.timeline.view().map(Message::TimelineMessage))
                            .push(
                                Row::new()
                                    .push(self.session.view().map(Message::SessionMessage))
                                    .push(self.arrangement.view().map(Message::ArrangementMessage))
                            )
                            .push(
                                Row::new()
                                    .push(self.device.view().map(Message::DeviceMessage))
                                    .push(self.mixer.view().map(Message::MixerMessage))
                            )
                    )
            );

        Container::new(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ArrangementMessage(msg) => self.arrangement.update(msg),
            Message::BrowserMessage(msg) => self.browser.update(msg),
            Message::DeviceMessage(msg) => self.device.update(msg),
            Message::MixerMessage(msg) => self.mixer.update(msg),
            Message::SessionMessage(msg) => self.session.update(msg),
            Message::TimelineMessage(msg) => self.timeline.update(msg),
        }
    }
} 