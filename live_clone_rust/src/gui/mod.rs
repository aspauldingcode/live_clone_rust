use iced::{
    widget::{button, Container, Row},
    Element, Length, Renderer, Application,
};

pub struct MainWindow {
    playing: bool,
    bpm: f64,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePlay,
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (
            MainWindow {
                playing: false,
                bpm: 120.0,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Live Clone")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::TogglePlay => {
                self.playing = !self.playing;
                // TODO: Start/stop metronome
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> Element<Message> {
        let play_button = button(if self.playing { "Stop" } else { "Play" })
            .on_press(Message::TogglePlay);

        let controls = Row::new()
            .spacing(20)
            .push(play_button);

        Container::new(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
} 