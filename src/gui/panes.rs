use iced::{
    Element,
    widget::{canvas, column, space, text},
};

use crate::gui::state::{Message, State};

pub fn info(state: &State) -> Element<'_, Message> {
    let open_file_name = state
        .file_path
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("<none>");
    let modified_mark = if state.file_modified { " (*)" } else { "" };

    if let Some(chart) = &state.loaded_chart {
        column![
            text!("{open_file_name}{modified_mark}").size(20),
            space(),
            text!("- {} notes", chart.notes.len()),
            text!("- {} mods", chart.gimmick.mods.len()),
            text!("- {} per-frames", chart.gimmick.per_frames.len()),
            text!("- {} proxies", chart.gimmick.proxies),
        ]
        .spacing(10)
        .padding(10)
        .into()
    } else {
        text("No chart").into()
    }
}

pub fn notes(state: &State) -> Element<'_, Message> {
    if let Some(chart) = &state.loaded_chart {
        canvas(chart).into()
    } else {
        text("No chart").into()
    }
}
