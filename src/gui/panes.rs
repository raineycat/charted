use iced::{
    Element,
    widget::{canvas, column, grid, pick_list, scrollable, space, text},
};

use crate::{
    chart::note::Note,
    gui::state::{Message, State},
};

pub fn info(state: &State) -> Element<'_, Message> {
    let open_file_name = state
        .file_path
        .as_ref()
        .map(|p| {
            let mut paths: Vec<&str> = p
                .ancestors()
                .take(2)
                .filter_map(|it| it.file_name().and_then(|f| f.to_str()))
                .collect();
            paths.reverse();
            paths
        })
        .map(|p| p.join(" / "))
        .unwrap_or_else(|| "(none)".to_owned());

    let modified_mark = if state.file_modified { " (*)" } else { "" };

    if let Some(chart) = &state.loaded_chart {
        column![
            text!("{open_file_name}{modified_mark}").size(20),
            space(),
            text!("refs: {}", chart.gimmick.gm_object_name),
            space(),
            text!("- {} notes", chart.notes.len()),
            text!("- {} tempo changes", chart.bpm_handler.count_changes()),
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

pub fn mods(state: &State) -> Element<'_, Message> {
    if let Some(chart) = &state.loaded_chart {
        scrollable(grid!(
            text("meow1"),
            text("meow2"),
            text("meow3"),
            text("meow4"),
            text("meow5"),
            text("meow6"),
            text("meow7"),
            text("meow8"),
        ))
        .into()
    } else {
        text("No chart").into()
    }
}

pub fn per_frames(state: &State) -> Element<'_, Message> {
    if let Some(chart) = &state.loaded_chart {
        scrollable(grid!(
            text("arf1"),
            text("arf2"),
            text("arf3"),
            text("arf4"),
            text("arf5"),
            text("arf6"),
            text("arf7"),
            text("arf8"),
        ))
        .into()
    } else {
        text("No chart").into()
    }
}

pub fn note_edit(state: &State) -> Element<'_, Message> {
    if let Some(chart) = &state.loaded_chart
        && let Some(selected_note_idx) = &state.selected_note
    {
        let selected_note = &chart.notes[*selected_note_idx];
        let note_types = [
            Note::CHIP,
            Note::MINE,
            Note::HOLD,
            Note::BUMPER,
            Note::BUMPER_MINE,
            Note::ABSOLUTE_BUMPER,
            Note::TEMPO_CHANGE,
        ];

        column![
            text!("Selected note").size(20),
            space(),
            text("Type:"),
            pick_list(note_types, Some(selected_note.kind), Message::SetNoteKind)
        ]
        .spacing(10)
        .into()
    } else {
        text("Left click to add or select notes, right click to remove them.")
            .center()
            .style(text::secondary)
            .into()
    }
}
