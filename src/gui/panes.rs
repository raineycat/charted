use iced::{
    Element,
    Length::Fill,
    widget::{button, canvas, column, pick_list, row, space, text, text_input},
};
use iced_aw::{iced_aw_font, iced_fonts, number_input};

use crate::{
    chart::{
        Chart,
        easing::Easing,
        gimmick::Modifier,
        note::{Note, NoteExtra},
        note_kind::NoteKind,
    },
    gui::{
        note_draw::NoteCanvasState,
        state::{Message, State},
    },
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
            text_input("Gimmick object", &chart.gimmick.gm_object_name)
                .on_input(Message::SetGimmickObject),
            space(),
            text!("{} notes", chart.notes.len()).center(),
            row![
                number_input(&chart.gimmick.proxies, 0..u8::MAX, Message::SetProxyCount),
                text!("proxies")
            ]
            .align_y(iced::Alignment::Center)
            .spacing(5),
            space(),
            row![
                button(iced_fonts::lucide::save()).on_press(Message::SaveChart),
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
        canvas(NoteCanvasState {
            chart: chart,
            music_pos: state
                .audio_track
                .as_ref()
                .map(|t| (t.position() * 1000.0) as f32),
            music_offset: state.audio_offset_ms,
        })
        .into()
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
        .padding(10)
        .into()
    } else {
        text("Left click to select notes,\nright click to remove them.\n\nShift-click to add notes,\nand hold alt to disable snapping.")
            .center()
            .style(text::secondary)
            .into()
    }
}

pub fn mod_edit(state: &State) -> Element<'_, Message> {
    if let Some(chart) = &state.loaded_chart
        && let Some(selected_mod_idx) = &state.selected_mod
    {
        let selected_mod = chart.gimmick.mods[*selected_mod_idx].clone();

        column![
            text!("Selected modifier").size(20),
            space(),
            text!("Start / duration (beats)"),
            row![
                number_input(
                    &selected_mod.start_beat,
                    f32::MIN..f32::MAX,
                    Message::SetModStartBeat
                ),
                number_input(
                    &selected_mod.duration,
                    f32::MIN..f32::MAX,
                    Message::SetModDuration
                ),
            ],
            text!("Type value"),
            number_input(&selected_mod.kind, 0..255, Message::SetModKind),
            text!("Well-known types"),
            pick_list(
                Modifier::KNOWN_KINDS,
                Modifier::KNOWN_KINDS.get(selected_mod.kind as usize),
                move |s| Message::SetModKind(
                    Modifier::KNOWN_KINDS
                        .iter()
                        .position(|&x| x == s)
                        .map(|x| x as u8)
                        .unwrap_or(selected_mod.kind)
                )
            ),
            text!("Value range from/to"),
            row![
                number_input(&selected_mod.start_val, f32::MIN..f32::MAX, move |x| {
                    let new_end = selected_mod.end_val + (x - selected_mod.start_val);
                    Message::SetModRange(x..new_end)
                }),
                number_input(&selected_mod.end_val, f32::MIN..f32::MAX, move |x| {
                    Message::SetModRange(selected_mod.start_val..x)
                }),
            ],
            text!("Ease function"),
            pick_list(Easing::ALL, Some(selected_mod.ease), Message::SetModEase),
            text!("Proxy index"),
            number_input(
                &selected_mod.proxy_index,
                i8::MIN..i8::MAX,
                Message::SetModProxy
            ),
        ]
        .spacing(10)
        .padding(10)
        .into()
    } else {
        text("No mod has been selected.")
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

pub fn audio(state: &State) -> Element<'_, Message> {
    column![
        text("Audio controls").center().width(iced::Length::Fill),
        row![
            button(iced_fonts::lucide::music())
                .on_press(Message::PickMusicFile)
                .width(iced::Length::Fill),
            button(iced_fonts::lucide::play())
                .on_press(Message::MusicPlay)
                .width(iced::Length::Fill),
            button(iced_fonts::lucide::pause())
                .on_press(Message::MusicPause)
                .width(iced::Length::Fill),
            button(iced_fonts::lucide::square())
                .on_press(Message::MusicStop)
                .width(iced::Length::Fill),
        ]
        .spacing(10)
        .width(iced::Length::Fill),
        number_input(
            &state.audio_offset_ms,
            -250.0..250.0,
            Message::SetAudioOffset
        )
        .width(iced::Length::Fill)
    ]
    .spacing(10)
    .padding(10)
    .into()
}
