use iced::{
    widget::{button, text_input, Container, Row, checkbox},
    Element, Length, Application, Command,
    keyboard,
    event,
};
use crate::audio::Metronome;
use std::time::Instant;
use tokio::time::{sleep, Duration};

pub struct MainWindow {
    playing: bool,
    metronome: Option<Metronome>,
    bpm_input: String,
    tap_times: Vec<Instant>,
    current_tap_count: usize,
    time_sig_num: String,
    time_sig_den: String,
    auto_start_on_tap: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePlay,
    BpmChanged(String),
    Tap,
    ClearOldTaps(Instant),
    TimeSignatureNumChanged(String),
    TimeSignatureDenChanged(String),
    ToggleAutoStartOnTap,
    StartMetronome,
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let metronome = Metronome::new(120.0).ok();
        
        (
            MainWindow {
                playing: false,
                metronome,
                bpm_input: "120".to_string(),
                tap_times: Vec::new(),
                current_tap_count: 0,
                time_sig_num: "4".to_string(),
                time_sig_den: "4".to_string(),
                auto_start_on_tap: true,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Live Clone")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TogglePlay => {
                self.playing = !self.playing;
                if let Some(metronome) = &mut self.metronome {
                    if self.playing {
                        metronome.start();
                    } else {
                        metronome.stop();
                    }
                }
                Command::none()
            }
            Message::BpmChanged(new_bpm) => {
                self.bpm_input = new_bpm.clone();
                if let Ok(bpm) = new_bpm.parse::<f64>() {
                    if bpm >= 20.0 && bpm <= 999.0 {
                        if let Some(metronome) = &mut self.metronome {
                            metronome.set_bpm(bpm);
                        }
                    }
                }
                Command::none()
            }
            Message::Tap => {
                let now = Instant::now();
                
                self.tap_times.retain(|&time| now.duration_since(time).as_secs_f64() <= 2.0);
                
                if let Ok(num_beats) = self.time_sig_num.parse::<usize>() {
                    if self.current_tap_count >= num_beats {
                        self.current_tap_count = 0;
                    }
                }
                
                if let Some(&last_tap) = self.tap_times.last() {
                    if now.duration_since(last_tap).as_secs_f64() > 2.0 {
                        self.current_tap_count = 0;
                    }
                }

                self.tap_times.push(now);
                self.current_tap_count += 1;

                if self.tap_times.len() >= 2 {
                    let last_tap = self.tap_times.last().unwrap();
                    let previous_tap = self.tap_times.get(self.tap_times.len() - 2).unwrap();
                    let interval = last_tap.duration_since(*previous_tap).as_secs_f64();
                    let bpm = 60.0 / interval;

                    if bpm >= 20.0 && bpm <= 999.0 {
                        self.bpm_input = format!("{:.0}", bpm);
                        if let Some(metronome) = &mut self.metronome {
                            metronome.set_bpm(bpm);
                            
                            if let Ok(num_beats) = self.time_sig_num.parse::<usize>() {
                                if self.current_tap_count == num_beats && !self.playing && self.auto_start_on_tap {
                                    self.playing = true;
                                    return Command::perform(
                                        async move {
                                            sleep(Duration::from_secs_f64(interval)).await;
                                        },
                                        |_| Message::StartMetronome
                                    );
                                }
                            }
                        }
                    }
                }

                Command::perform(
                    async move {
                        sleep(Duration::from_secs(2)).await;
                        now
                    },
                    Message::ClearOldTaps
                )
            }
            Message::StartMetronome => {
                if let Some(metronome) = &mut self.metronome {
                    metronome.start();
                }
                Command::none()
            }
            Message::ClearOldTaps(threshold) => {
                self.tap_times.retain(|&time| time >= threshold);
                if self.tap_times.is_empty() {
                    self.current_tap_count = 0;
                }
                Command::none()
            }
            Message::TimeSignatureNumChanged(num) => {
                if let Ok(n) = num.parse::<u8>() {
                    if n > 0 && n <= 16 {
                        self.time_sig_num = num;
                        if let Some(metronome) = &mut self.metronome {
                            if let Ok(d) = self.time_sig_den.parse::<u8>() {
                                metronome.set_time_signature(n, d);
                            }
                        }
                    }
                }
                Command::none()
            }
            Message::TimeSignatureDenChanged(den) => {
                if let Ok(d) = den.parse::<u8>() {
                    if d > 0 && d <= 16 {
                        self.time_sig_den = den;
                        if let Some(metronome) = &mut self.metronome {
                            if let Ok(n) = self.time_sig_num.parse::<u8>() {
                                metronome.set_time_signature(n, d);
                            }
                        }
                    }
                }
                Command::none()
            }
            Message::ToggleAutoStartOnTap => {
                self.auto_start_on_tap = !self.auto_start_on_tap;
                Command::none()
            }
        }
    }

  fn view(&self) -> Element<Message> {
        let play_button = button(if self.playing { "Stop" } else { "Play" })
            .on_press(Message::TogglePlay);

        let bpm_input = text_input("BPM", &self.bpm_input)
            .on_input(Message::BpmChanged)
            .size(32);

        let tap_text = if let Ok(num_beats) = self.time_sig_num.parse::<usize>() {
            if self.current_tap_count == 0 || self.current_tap_count >= num_beats {
                "Tap"
            } else {
                match self.current_tap_count {
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    8 => "8",
                    9 => "9",
                    10 => "10",
                    11 => "11",
                    12 => "12",
                    13 => "13",
                    14 => "14",
                    15 => "15",
                    16 => "16",
                    _ => "Tap",
                }
            }
        } else {
            "Tap"
        };

        let tap_button = button(tap_text)
            .on_press(Message::Tap)
            .width(Length::Fixed(60.0))
            .padding(10)
            .style(iced::theme::Button::Secondary);

        let time_sig = Row::new()
            .spacing(5)
            .push(
                text_input("Num", &self.time_sig_num)
                    .on_input(Message::TimeSignatureNumChanged)
                    .size(32)
                    .width(Length::Fixed(40.0))
            )
            .push(text_input("Den", &self.time_sig_den)
                .on_input(Message::TimeSignatureDenChanged)
                .size(32)
                .width(Length::Fixed(40.0))
            );

        let auto_start_checkbox = checkbox(
            "Auto-start on tap",
            self.auto_start_on_tap,
            |_| Message::ToggleAutoStartOnTap,
        );

        let controls = Row::new()
            .spacing(20)
            .push(play_button)
            .push(bpm_input)
            .push(tap_button)
            .push(time_sig)
            .push(auto_start_checkbox);

        Container::new(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::events_with(|event, _status| {
            if let event::Event::Keyboard(keyboard::Event::KeyPressed { 
                key_code,
                modifiers: _,
            }) = event {
                match key_code {
                    keyboard::KeyCode::Space => Some(Message::TogglePlay),
                    _ => None,
                }
            } else {
                None
            }
        })
    }
} 