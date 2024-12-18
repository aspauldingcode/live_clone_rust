use iced::{Application, Settings};
use crate::gui::MainWindow;

mod gui;

fn main() {
    let _ = MainWindow::run(Settings::default());
}
