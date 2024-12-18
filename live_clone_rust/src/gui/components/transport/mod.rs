use iced::widget::{Button, Container, Row, Text};
use iced::{Element, Length};
use crate::gui::Message;

pub struct TransportBar {
    // Add state fields if needed
}

impl TransportBar {
    pub fn new() -> Self {
        TransportBar {
            // Initialize state
        }
    }

    pub fn view(&self) -> Element<Message> {
        let row = Row::new()
            .spacing(10)
            .push(self.text_button("Link", "🔗"))
            .push(self.text_button("Tap Tempo", "⏱"))
            .push(self.text_button("Tempo", "🎵"))
            .push(self.text_button("Nudge Down", "🔽"))
            .push(self.text_button("Nudge Up", "🔼"))
            .push(self.text_button("Time Sig Num", "#"))
            .push(self.text_button("Time Sig Den", "#"))
            .push(self.text_button("Metronome", "🎵"))
            .push(self.text_button("Metronome Settings", "⚙"))
            .push(self.text_button("Quantization", "🔢"))
            .push(self.text_button("Follow", "👀"))
            .push(self.text_button("Position", "📍"))
            .push(self.text_button("Play", "▶"))
            .push(self.text_button("Stop", "⏹"))
            .push(self.text_button("Record", "🎞"))
            .push(self.text_button("Overdub", "🔄"))
            .push(self.text_button("Automation Arm", "🤖"))
            .push(self.text_button("Re-Enable Automation", "↩"))
            .push(self.text_button("Capture MIDI", "🎹"))
            .push(self.text_button("Session Record", "🎞"))
            .push(self.text_button("Loop Start", "🔄"))
            .push(self.text_button("Punch-In", "▶"))
            .push(self.text_button("Loop", "🔄"))
            .push(self.text_button("Punch-Out", "◀"))
            .push(self.text_button("Loop Length", "📏"))
            .push(self.text_button("Draw Mode", "🖌"))
            .push(self.text_button("MIDI Keyboard", "🎹"))
            .push(self.text_button("Key Map Mode", "📋"))
            .push(self.text_button("MIDI In", "🔽"))
            .push(self.text_button("MIDI Out", "🔼"))
            .push(self.text_button("MIDI Map Mode", "📋"))
            .push(self.text_button("CPU Load", "🧩"))
            .push(self.text_button("Overload", "❗"))
            .push(self.text_button("Track In", "🔽"))
            .push(self.text_button("Track Out", "🔼"));

        Container::new(row)
            .width(Length::Fill)
            .height(Length::Units(50))
            .into()
    }

    fn text_button(&self, label: &str, icon: &str) -> Button<Message> {
        Button::new(
            Text::new(format!("{} {}", icon, label)).size(16),
        )
    }
} 