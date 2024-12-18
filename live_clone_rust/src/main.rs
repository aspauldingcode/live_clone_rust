use iced::{Application, Settings, Command, Element, Clipboard};

#[cfg(target_os = "macos")]
mod menu;
mod gui;

use gui::MainWindow;

fn main() -> iced::Result {
    #[cfg(target_os = "macos")]
    {
        if let Err(e) = menu::create_menu_bar() {
            eprintln!("Failed to create menu bar: {}", e);
        }
    }

    let mut settings = Settings::default();
    settings.antialiasing = true;
    settings.exit_on_close_request = true;
    settings.window.size = (1280, 720);
    settings.window.min_size = Some((800, 600));

    LiveClone::run(settings)
}

struct LiveClone {
    main_window: MainWindow,
}

impl Application for LiveClone {
    type Executor = iced::executor::Default;
    type Message = gui::Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                main_window: MainWindow::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Live Clone Rust")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        self.main_window.update(message);
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.main_window.view()
    }
}
