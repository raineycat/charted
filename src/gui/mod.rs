use std::path::PathBuf;

use binary_rw::{BinaryError, BinaryReader, BinaryWriter, FileStream};
use iced::{
    Alignment, Element, Length, Task, Theme, border,
    widget::{button, column, container, pane_grid, row, text},
};
use rfd::FileHandle;

use crate::{
    chart::{
        Chart,
        note::{Note, NoteExtra},
    },
    gui::state::{ChartPane, Message, State},
};

mod note_draw;
mod panes;
pub mod state;

pub fn theme(_state: &State) -> Theme {
    Theme::CatppuccinMocha
}

pub fn update(state: &mut State, msg: Message) -> Task<Message> {
    match msg {
        Message::QuitApp => iced::exit(),

        Message::OpenChart => open_chart_file(),
        Message::OpenChartCancelled => Task::none(),
        Message::OpenChartFailed => {
            state.error_msg = Some("Failed to load chart!".to_owned());
            Task::none()
        }
        Message::NewChart => {
            let mut chart = Chart::default();
            chart.notes.push(Note {
                kind: Note::TEMPO_CHANGE,
                lane: 0,
                time: 0.0,
                extra: Some(NoteExtra::TempoChange(120.0)),
            });
            chart.recalc_bpm();

            state.file_path = None;
            state.loaded_chart = Some(chart);
            state.selected_note = None;
            Task::none()
        }
        Message::LoadedChart(path, c) => {
            state.file_path = Some(path);
            state.loaded_chart = Some(c);
            state.selected_note = None;
            Task::none()
        }

        Message::SaveChart => {
            if let Some(path) = &state.file_path {
                Task::done(Message::SaveChartAs(path.clone()))
            } else {
                save_chart_file()
            }
        }
        Message::SaveChartCancelled => Task::none(),
        Message::SaveChartAs(path) => {
            if let Some(chart) = &state.loaded_chart {
                match write_chart(&path, chart) {
                    Ok(()) => {
                        state.file_path = Some(path);
                        state.file_modified = false;
                    }
                    Err(e) => {
                        log::error!("Failed to save chart {:?}: {:#?}", path, e);
                    }
                };
            }
            Task::none()
        }
        Message::CloseChart => {
            state.loaded_chart = None;
            state.file_path = None;
            state.selected_note = None;
            state.file_modified = false;
            Task::none()
        }
        Message::ChartHasBeenModified => {
            state.file_modified = true;
            Task::none()
        }

        Message::PaneDragged(e) => {
            if let pane_grid::DragEvent::Dropped { pane, target } = e {
                state.panes.drop(pane, target);
            }
            Task::none()
        }
        Message::PaneResized(e) => {
            state.panes.resize(e.split, e.ratio);
            Task::none()
        }

        Message::SelectNote(note_idx) => {
            state.selected_note = Some(note_idx);
            Task::none()
        }

        Message::DeselectNote => {
            state.selected_note = None;
            Task::none()
        }

        Message::RemoveNote(note_idx) => {
            if let Some(chart) = state.loaded_chart.as_mut() {
                chart.notes.remove(note_idx);
                chart.recalc_bpm();
                state.selected_note = None;
                Task::done(Message::ChartHasBeenModified)
            } else {
                Task::none()
            }
        }

        Message::SetNoteKind(kind) => {
            if let Some(chart) = state.loaded_chart.as_mut()
                && let Some(selected_note_idx) = &state.selected_note
            {
                chart.notes[*selected_note_idx].kind = kind;
                Task::done(Message::ChartHasBeenModified)
            } else {
                Task::none()
            }
        }

        Message::NudgeLane(amount) => {
            if let Some(chart) = state.loaded_chart.as_mut()
                && let Some(selected_note_idx) = &state.selected_note
            {
                let mut prev_lane = chart.notes[*selected_note_idx].lane as i8;
                prev_lane = (prev_lane + amount).clamp(0, 4);
                chart.notes[*selected_note_idx].lane = prev_lane as u8;

                Task::done(Message::ChartHasBeenModified)
            } else {
                Task::none()
            }
        }

        Message::NudgeBeat(amount) => {
            if let Some(chart) = state.loaded_chart.as_mut()
                && let Some(selected_note_idx) = &state.selected_note
            {
                let prev_time = chart.notes[*selected_note_idx].time;
                match chart
                    .bpm_handler
                    .beat_from_time(prev_time)
                    .map(|b| b + amount)
                    .and_then(|b| chart.bpm_handler.time_from_beat(b))
                {
                    Some(new_time) => chart.notes[*selected_note_idx].time = new_time,
                    None => {}
                }
                chart.recalc_bpm();
                Task::done(Message::ChartHasBeenModified)
            } else {
                Task::none()
            }
        }

        Message::CreateNote(time, kind, lane) => {
            if let Some(chart) = state.loaded_chart.as_mut() {
                chart.notes.push(Note {
                    kind: kind,
                    lane: lane,
                    time: time,
                    extra: match kind {
                        Note::HOLD => Some(NoteExtra::HoldEndTime((time + 1000.0) as i32)),
                        Note::TEMPO_CHANGE => Some(NoteExtra::TempoChange(120.0)),
                        _ => None,
                    },
                });
                chart.recalc_bpm();
                Task::done(Message::ChartHasBeenModified)
            } else {
                Task::none()
            }
        }

        Message::SetHoldEndBeat(end_beat) => {
            if let Some(chart) = state.loaded_chart.as_mut()
                && let Some(selected_note_idx) = &state.selected_note
            {
                let time = chart.bpm_handler.time_from_beat(end_beat).unwrap_or(0.0);
                chart.notes[*selected_note_idx].extra = Some(NoteExtra::HoldEndTime(time as i32));
                Task::done(Message::ChartHasBeenModified)
            } else {
                Task::none()
            }
        }

        Message::SetTempChangeValue(new_bpm) => {
            if let Some(chart) = state.loaded_chart.as_mut()
                && let Some(selected_note_idx) = &state.selected_note
            {
                chart.notes[*selected_note_idx].extra = Some(NoteExtra::TempoChange(new_bpm));
                chart.recalc_bpm();
                Task::done(Message::ChartHasBeenModified)
            } else {
                Task::none()
            }
        }
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let content: Element<'_, Message> = match &state.loaded_chart {
        Some(_) => pane_grid(&state.panes, |_pane, pane_type, _is_maximized| {
            pane_grid::Content::new(match pane_type {
                ChartPane::InfoPane => panes::info(state),
                ChartPane::NotePane => panes::notes(state),
                ChartPane::ModPane => panes::mods(state),
                ChartPane::PerFramePane => panes::per_frames(state),
                ChartPane::NoteEditPane => panes::note_edit(state),
            })
            .style(|theme| container::Style {
                border: iced::Border {
                    color: theme.palette().primary,
                    width: 0.5,
                    radius: border::Radius::new(5.0),
                },
                ..Default::default()
            })
        })
        .spacing(15)
        .on_drag(Message::PaneDragged)
        .on_resize(10, Message::PaneResized)
        .into(),
        None => column![
            text("No chart loaded!").style(text::danger),
            row![
                button("New").on_press(Message::NewChart),
                button("Open").on_press(Message::OpenChart),
                button("Quit").on_press(Message::QuitApp)
            ]
            .spacing(10)
        ]
        .spacing(10)
        .align_x(Alignment::Center)
        .into(),
    };

    container(content)
        .padding(10)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn open_chart_file() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .add_filter("Binary chart files", &["vsb"])
            .pick_file(),
    )
    .then(|handle| match handle {
        Some(file_handle) => Task::done(match load_chart(&file_handle) {
            Ok(c) => Message::LoadedChart(file_handle.path().to_path_buf(), c),
            Err(e) => {
                log::error!("Failed to load chart {}: {:#?}", file_handle.file_name(), e);
                Message::OpenChartFailed
            }
        }),
        None => Task::done(Message::OpenChartCancelled),
    })
}

fn save_chart_file() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .add_filter("Binary chart files", &["vsb"])
            .save_file(),
    )
    .then(|handle| match handle {
        Some(file_handle) => Task::done(Message::SaveChartAs(file_handle.path().to_path_buf())),
        None => Task::done(Message::SaveChartCancelled),
    })
}

fn load_chart(handle: &FileHandle) -> Result<Chart, BinaryError> {
    let mut stream = FileStream::open(handle.path())?;
    let mut reader = BinaryReader::new(&mut stream, binary_rw::Endian::Little);
    Chart::read_binary(&mut reader)
}

fn write_chart(path: &PathBuf, chart: &Chart) -> Result<(), BinaryError> {
    let mut stream = FileStream::create(path)?;
    let mut writer = BinaryWriter::new(&mut stream, binary_rw::Endian::Little);
    chart.write_binary(&mut writer)
}
