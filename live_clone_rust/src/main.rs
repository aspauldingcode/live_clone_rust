use iced::{Application, Settings};
use crate::gui::MainWindow;

mod gui;
mod audio;

fn main() {
    let settings = Settings::default();
    MainWindow::run(settings).expect("Failed to run application");
}
