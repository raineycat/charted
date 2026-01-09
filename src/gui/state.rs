use std::path::PathBuf;

use iced::widget::pane_grid;
use kira::sound::static_sound;

use crate::chart::{Chart, note_kind::NoteKind};

#[derive(Clone, Debug)]
pub enum Message {
    QuitApp,

    OpenChart,
    OpenChartCancelled,
    OpenChartFailed,
    NewChart,
    LoadedChart(PathBuf, Chart),

    SaveChart,
    SaveChartCancelled,
    SaveChartAs(PathBuf),
    CloseChart,
    ChartHasBeenModified,

    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),

    SelectNote(usize),
    DeselectNote,
    RemoveNote(usize),

    SetNoteKind(NoteKind),
    NudgeLane(i8),
    NudgeBeat(f32),
    CreateNote(f32, NoteKind, u8),
    SetHoldEndBeat(f32),
    SetTempChangeValue(f32),

    OpenAudioDevice,
    PickMusicFile,
    OpenMusicFrom(PathBuf),

    MusicPlay,
    MusicPause,
    MusicStop,
}

pub struct State {
    pub file_path: Option<PathBuf>,
    pub loaded_chart: Option<Chart>,
    pub selected_note: Option<usize>,
    pub file_modified: bool,
    pub error_msg: Option<String>,
    pub audio_mgr: Option<kira::AudioManager>,
    pub audio_track: Option<static_sound::StaticSoundHandle>,
    pub panes: pane_grid::State<ChartPane>,
}

impl State {
    pub fn new() -> (Self, iced::Task<Message>) {
        let state = Self {
            file_path: None,
            loaded_chart: None,
            selected_note: None,
            file_modified: false,
            error_msg: None,
            audio_mgr: None,
            audio_track: None,
            panes: State::initial_pane_layout(),
        };

        let startup_messages = [Message::OpenAudioDevice];

        (
            state,
            iced::Task::batch(startup_messages.map(|m| iced::Task::done(m))),
        )
    }

    fn initial_pane_layout() -> pane_grid::State<ChartPane> {
        pane_grid::State::with_configuration(pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.2,
            a: Box::new(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.35,
                a: Box::new(pane_grid::Configuration::Pane(ChartPane::InfoPane)),
                b: Box::new(pane_grid::Configuration::Split {
                    axis: pane_grid::Axis::Horizontal,
                    ratio: 0.6,
                    a: Box::new(pane_grid::Configuration::Pane(ChartPane::NoteEditPane)),
                    b: Box::new(pane_grid::Configuration::Pane(ChartPane::AudioPane)),
                }),
            }),
            b: Box::new(pane_grid::Configuration::Pane(ChartPane::NotePane)),
        })
    }
}

#[derive(Clone, Debug)]
pub enum ChartPane {
    InfoPane,
    NotePane,
    ModPane,
    PerFramePane,
    NoteEditPane,
    AudioPane,
}
