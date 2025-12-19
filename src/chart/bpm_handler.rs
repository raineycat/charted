use crate::chart::{Chart, note::NoteExtra};

#[derive(Debug, Default, Clone)]
struct BpmChange {
    beat_of_change: f32,
    new_bpm: f32,
}

#[derive(Default, Debug, Clone)]
pub struct BpmHandler {
    changes: Vec<BpmChange>,
}

impl BpmHandler {
    pub fn add_changes_from_chart(&mut self, chart: &Chart) {
        let mut bpm: f32 = 0.0;
        let mut last_change_time: f32 = 0.0;
        let mut last_change_beats: f32 = 0.0;

        for note in &chart.notes {
            if let Some(NoteExtra::TempoChange(new_bpm)) = note.extra {
                if bpm == 0.0 {
                    bpm = new_bpm;
                    self.changes.push(BpmChange {
                        beat_of_change: 0.0,
                        new_bpm,
                    });
                    continue;
                }

                let old_beat_duration = 60_000.0 / bpm;
                let beats_since_change = (note.time - last_change_time) / old_beat_duration;
                let total_beats = beats_since_change + last_change_beats;

                self.changes.push(BpmChange {
                    beat_of_change: total_beats,
                    new_bpm,
                });

                last_change_time = note.time;
                last_change_beats = total_beats;
                bpm = new_bpm;
            }
        }
        self.sort_changes();
    }

    fn sort_changes(&mut self) {
        self.changes.sort_by(|a, b| {
            if a.beat_of_change < b.beat_of_change {
                std::cmp::Ordering::Less
            } else if a.beat_of_change > b.beat_of_change {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }

    pub fn beat_from_time(&self, mut time_ms: f32) -> Option<f32> {
        for i in 0..self.changes.len() {
            let current = &self.changes[i];

            let start_beat = current.beat_of_change;
            let bps = current.new_bpm / 60.0;

            let is_last = i == self.changes.len() - 1;

            let next_start_beat = if is_last {
                f32::INFINITY
            } else {
                self.changes[i + 1].beat_of_change
            };

            let beats_in_segment = next_start_beat - start_beat;
            let seconds_in_segment = beats_in_segment / bps;
            let ms_in_segment = seconds_in_segment * 1000.0;

            if is_last || time_ms <= ms_in_segment {
                let beats_elapsed = (time_ms / 1000.0) * bps;
                return Some(start_beat + beats_elapsed);
            }

            time_ms -= ms_in_segment;
        }

        None
    }

    pub fn time_from_beat(&self, mut beat: f32) -> Option<f32> {
        let mut elapsed_ms = 0.0;

        for i in 0..self.changes.len() {
            let current = &self.changes[i];

            let start_beat = current.beat_of_change;
            let bpm = current.new_bpm;
            let bps = bpm / 60.0;

            let is_last = i == self.changes.len() - 1;

            let next_start_beat = if is_last {
                f32::INFINITY
            } else {
                self.changes[i + 1].beat_of_change
            };

            let beats_in_segment = next_start_beat - start_beat;

            if beat <= beats_in_segment {
                let seconds = beat / bps;
                return Some(elapsed_ms + seconds * 1000.0);
            }

            let segment_seconds = beats_in_segment / bps;
            elapsed_ms += segment_seconds * 1000.0;
            beat -= beats_in_segment;
        }

        None
    }
}
