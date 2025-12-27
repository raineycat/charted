use std::path::PathBuf;

use iced::widget::pane_grid;

use crate::chart::{Chart, note_kind::NoteKind};

pub struct State {
    pub file_path: Option<PathBuf>,
    pub loaded_chart: Option<Chart>,
    pub selected_note: Option<usize>,
    pub file_modified: bool,
    pub error_msg: Option<String>,
    pub panes: pane_grid::State<ChartPane>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            file_path: None,
            loaded_chart: None,
            selected_note: None,
            file_modified: false,
            error_msg: None,
            panes: pane_grid::State::with_configuration(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.2,
                a: Box::new(pane_grid::Configuration::Split {
                    axis: pane_grid::Axis::Horizontal,
                    ratio: 0.45,
                    a: Box::new(pane_grid::Configuration::Pane(ChartPane::InfoPane)),
                    b: Box::new(pane_grid::Configuration::Pane(ChartPane::NoteEditPane)),
                }),
                b: Box::new(pane_grid::Configuration::Pane(ChartPane::NotePane)),
            }),
        }
    }
}

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
}

#[derive(Clone, Debug)]
pub enum ChartPane {
    InfoPane,
    NotePane,
    ModPane,
    PerFramePane,
    NoteEditPane,
}
