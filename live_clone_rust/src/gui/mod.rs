pub mod components;

use iced::widget::{Column, Container, Row};
use iced::{Application, Command, Element, Length};

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
#[allow(dead_code)]
pub enum Message {
    Transport(components::transport::TransportMessage),
    ArrangementMessage(components::arrangement::Message),
    BrowserMessage(components::browser::Message),
    DeviceMessage(components::device::Message),
    MixerMessage(components::mixer::Message),
    SessionMessage(components::session::Message),
    TimelineMessage(components::timeline::Message),
}

#[allow(dead_code)]
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

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Transport(msg) => self.transport.update(msg),
            Message::ArrangementMessage(msg) => self.arrangement.update(msg),
            Message::BrowserMessage(msg) => self.browser.update(msg),
            Message::DeviceMessage(msg) => self.device.update(msg),
            Message::MixerMessage(msg) => self.mixer.update(msg),
            Message::SessionMessage(msg) => self.session.update(msg),
            Message::TimelineMessage(msg) => self.timeline.update(msg),
        }
    }
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Live Clone")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.update(message);
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
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
} 