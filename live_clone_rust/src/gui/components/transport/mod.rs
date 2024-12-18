use iced::widget::{Button, Text, Container, Row};
use iced::{Element, Length};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TransportMessage {
    ToggleLink,
    ToggleTapTempo,
    ToggleTempo,
    NudgeDown,
    NudgeUp,
    ToggleMetronome,
    ToggleMetronomeSettings,
    ToggleQuantization,
    ToggleFollow,
    TogglePosition,
    TogglePlay,
    ToggleStop,
    ToggleRecord,
    ToggleOverdub,
    ToggleAutomationArm,
    ToggleReEnableAutomation,
    ToggleCaptureMidi,
    ToggleSessionRecord,
    ToggleLoopStart,
    TogglePunchIn,
    ToggleLoop,
    TogglePunchOut,
    ToggleLoopLength,
    ToggleDrawMode,
    ToggleMidiKeyboard,
    ToggleKeyMapMode,
    ToggleMidiIn,
    ToggleMidiOut,
    ToggleMidiMapMode,
    ToggleCpuLoad,
    ToggleOverload,
    ToggleTrackIn,
    ToggleTrackOut,
}

pub struct TransportBar {
    link_active: bool,
    tap_tempo_active: bool,
    tempo_active: bool,
    metronome_active: bool,
    quantization_active: bool,
    follow_active: bool,
    play_active: bool,
    record_active: bool,
    loop_active: bool,
    automation_active: bool,
    draw_mode_active: bool,
}

impl TransportBar {
    pub fn new() -> Self {
        TransportBar {
            link_active: false,
            tap_tempo_active: false,
            tempo_active: false,
            metronome_active: false,
            quantization_active: false,
            follow_active: false,
            play_active: false,
            record_active: false,
            loop_active: false,
            automation_active: false,
            draw_mode_active: false,
        }
    }

    pub fn update(&mut self, message: TransportMessage) {
        match message {
            TransportMessage::ToggleLink => self.link_active = !self.link_active,
            TransportMessage::ToggleTapTempo => self.tap_tempo_active = !self.tap_tempo_active,
            TransportMessage::ToggleTempo => self.tempo_active = !self.tempo_active,
            TransportMessage::ToggleMetronome => self.metronome_active = !self.metronome_active,
            TransportMessage::ToggleQuantization => self.quantization_active = !self.quantization_active,
            TransportMessage::ToggleFollow => self.follow_active = !self.follow_active,
            TransportMessage::TogglePlay => self.play_active = !self.play_active,
            TransportMessage::ToggleRecord => self.record_active = !self.record_active,
            TransportMessage::ToggleLoop => self.loop_active = !self.loop_active,
            TransportMessage::ToggleAutomationArm => self.automation_active = !self.automation_active,
            TransportMessage::ToggleDrawMode => self.draw_mode_active = !self.draw_mode_active,
            _ => {},
        }
    }

    pub fn view(&self) -> Element<crate::gui::Message> {
        let row = Row::new()
            .spacing(10)
            .push(self.toggle_button("Link", "🔗", self.link_active, TransportMessage::ToggleLink))
            .push(self.toggle_button("Tap Tempo", "⏱", self.tap_tempo_active, TransportMessage::ToggleTapTempo))
            .push(self.toggle_button("Tempo", "🎵", self.tempo_active, TransportMessage::ToggleTempo))
            .push(self.toggle_button("Metronome", "🎵", self.metronome_active, TransportMessage::ToggleMetronome))
            .push(self.toggle_button("Quantization", "🔢", self.quantization_active, TransportMessage::ToggleQuantization))
            .push(self.toggle_button("Follow", "👀", self.follow_active, TransportMessage::ToggleFollow))
            .push(self.toggle_button("Play", "▶", self.play_active, TransportMessage::TogglePlay))
            .push(self.toggle_button("Record", "🎞", self.record_active, TransportMessage::ToggleRecord))
            .push(self.toggle_button("Loop", "🔄", self.loop_active, TransportMessage::ToggleLoop))
            .push(self.toggle_button("Automation", "🤖", self.automation_active, TransportMessage::ToggleAutomationArm))
            .push(self.toggle_button("Draw Mode", "🖌", self.draw_mode_active, TransportMessage::ToggleDrawMode));

        Container::new(row)
            .width(Length::Fill)
            .height(Length::Units(50))
            .into()
    }

    fn toggle_button(&self, label: &str, icon: &str, active: bool, msg: TransportMessage) -> Button<crate::gui::Message> {
        let style = if active {
            Button::new(Text::new(format!("{} {} (On)", icon, label)).size(16))
        } else {
            Button::new(Text::new(format!("{} {}", icon, label)).size(16))
        };
        
        style.on_press(crate::gui::Message::Transport(msg))
    }
} 