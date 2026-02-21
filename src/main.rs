#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;

use crate::gui::state::{Message, State};
use iced::window::{self, icon};
use iced_fonts::LUCIDE_FONT_BYTES;
use image::ImageReader;
use log::info;

mod chart;
mod gui;

fn main() -> iced::Result {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();
    info!("Starting");

    let window = window::Settings {
        icon: load_embedded_icon(),
        ..Default::default()
    };

    iced::application(State::new, gui::update, gui::view)
        .title("ChartEd")
        .window(window)
        .theme(gui::theme)
        .font(LUCIDE_FONT_BYTES)
        .window_size(iced::Size::new(1280.0, 720.0))
        // .subscription(|_state| {
        //     iced::time::every(iced::time::Duration::from_millis(5))
        //         .map(|t| Message::UpdatePlayback(t))
        // })
        .run()
}

fn load_embedded_icon() -> Option<icon::Icon> {
    let icon_data = io::Cursor::new(include_bytes!("ce_icon.png"));

    let mut icon_reader = ImageReader::new(icon_data);
    icon_reader.set_format(image::ImageFormat::Png);

    let icon = icon_reader.decode().ok()?.into_rgba8();
    icon::from_rgba(icon.to_vec(), icon.width(), icon.height()).ok()
}
