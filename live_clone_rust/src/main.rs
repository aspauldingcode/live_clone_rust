use iced::{Application, Settings};
use crate::gui::MainWindow;

mod gui;

fn main() {
    let settings = Settings::default();
    MainWindow::run(settings).expect("Failed to run application");
}
