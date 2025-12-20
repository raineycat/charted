use iced::{
    Element,
    Length::Fill,
    widget::{button, canvas, column, pick_list, row, space, text},
};
use iced_aw::number_input;

use crate::{
    chart::{
        Chart,
        note::{Note, NoteExtra},
        note_kind::NoteKind,
    },
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
            space(),
            row![
                button("SAVE").on_press(Message::SaveChart),
                button("CLOSE").on_press(Message::CloseChart),
            ]
            .spacing(5)
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
        if chart.gimmick.mods.is_empty() {
            text("This chart has no modifiers")
                .center()
                .style(text::secondary)
                .into()
        } else {
            text("Modifier handling is not implemented!")
                .center()
                .style(text::warning)
                .into()
        }
    } else {
        text("No chart").into()
    }
}

pub fn per_frames(state: &State) -> Element<'_, Message> {
    if let Some(chart) = &state.loaded_chart {
        if chart.gimmick.per_frames.is_empty() {
            text("This chart has no per-frames")
                .center()
                .style(text::secondary)
                .into()
        } else {
            text("Per-frame handling is not implemented!")
                .center()
                .style(text::warning)
                .into()
        }
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
            NoteKind::Chip,
            NoteKind::Mine,
            NoteKind::Hold,
            NoteKind::Bumper,
            NoteKind::BumperMine,
            NoteKind::AbsoluteBumper,
            NoteKind::TempoChange,
            NoteKind::UnknownType,
        ];

        column![
            text!("Selected note").size(20),
            space(),
            text!(
                "{} beats",
                chart
                    .bpm_handler
                    .beat_from_time(selected_note.time)
                    .unwrap_or(0.0)
            ),
            text!("{} ms", selected_note.time),
            pick_list(note_types, Some(selected_note.kind), Message::SetNoteKind).width(Fill),
            note_extra_edit(chart, selected_note),
        ]
        .spacing(10)
        .into()
    } else {
        text("Left click to select notes,\nright click to remove them.\n\nShift-click to add notes,\nand hold alt to disable snapping.")
            .center()
            .style(text::secondary)
            .into()
    }
}

fn note_extra_edit<'a>(chart: &Chart, note: &Note) -> Element<'a, Message> {
    match &note.extra {
        None => text("This note has no extra data")
            .style(text::secondary)
            .into(),
        Some(extra) => match extra {
            NoteExtra::HoldEndTime(time) => {
                let start_beat = chart.bpm_handler.beat_from_time(note.time).unwrap_or(0.0);
                let end_beat = chart
                    .bpm_handler
                    .beat_from_time(*time as f32)
                    .unwrap_or(0.0);
                let duration_beats = end_beat - start_beat;

                number_input(&duration_beats, 0.0..256.0, move |new_dur| {
                    Message::SetHoldEndBeat(start_beat + new_dur)
                })
                .width(Fill)
                .into()
            }
            NoteExtra::TempoChange(new_bpm) => {
                number_input(new_bpm, 0.0..500.0, Message::SetTempChangeValue)
                    .width(Fill)
                    .into()
            }
        },
    }
}
