use crate::gui::state::State;
use iced::window::{self, icon};
use log::info;

mod chart;
mod gui;

fn main() -> iced::Result {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();
    info!("Starting");

    let icon_bytes = include_bytes!("ce_icon.png");
    let window = window::Settings {
        icon: icon::from_file_data(icon_bytes, None).ok(),
        ..Default::default()
    };

    iced::application(State::default, gui::update, gui::view)
        .title("ChartEd")
        .window(window)
        .theme(gui::theme)
        .window_size(iced::Size::new(1280.0, 720.0))
        .run()
}
