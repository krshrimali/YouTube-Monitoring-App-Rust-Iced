use iced::{Sandbox, Settings};

mod yt_monitor;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (1600, 800);
    yt_monitor::Styling::run(settings)
}
