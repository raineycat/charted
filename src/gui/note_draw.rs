use crate::{
    chart::{
        Chart,
        gimmick::Modifier,
        note::{Note, NoteExtra},
        note_kind::NoteKind,
    },
    gui::{mod_tracker::ModTracker, state::Message},
};
use iced::{
    Rectangle, Renderer, Theme, color,
    keyboard::{Modifiers, key},
    mouse,
    widget::{self, canvas},
};

const MOD_RADIUS: f32 = 5.0;

pub struct NoteCanvasState<'a> {
    pub chart: &'a Chart,
    pub music_pos: Option<f32>,
    pub music_offset: f32,
}

pub struct NoteDrawState {
    scroll_pos: f32,
    lane_width: f32,
    gutter_width: f32,
    note_height: f32,
    units_per_ms: f32,
    x_padding: f32,

    place_notes: bool,
    disable_snapping: bool,
    current_note_kind: NoteKind,

    show_mod_layer: bool,
    seeking: bool,
}

impl Default for NoteDrawState {
    fn default() -> Self {
        Self {
            scroll_pos: 0.0,
            lane_width: 0.0,
            gutter_width: 0.0,
            note_height: 20.0,
            units_per_ms: 0.25,
            x_padding: 2.5,

            place_notes: false,
            disable_snapping: false,
            current_note_kind: NoteKind::Chip,

            show_mod_layer: false,
            seeking: false,
        }
    }
}

impl canvas::Program<Message> for NoteCanvasState<'_> {
    type State = NoteDrawState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match event {
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                modified_key: _,
                physical_key: _,
                location: _,
                modifiers: _,
                text: _,
                repeat: _,
            }) => match key {
                key::Key::Named(key::Named::Home) => state.scroll_pos = 0.,
                key::Key::Named(key::Named::PageUp) => state.scroll_pos += state.note_height * 8.,
                key::Key::Named(key::Named::PageDown) => state.scroll_pos -= state.note_height * 8.,

                key::Key::Named(key::Named::ArrowUp) => {
                    return Some(widget::Action::publish(Message::NudgeBeat(-0.25)));
                }
                key::Key::Named(key::Named::ArrowDown) => {
                    return Some(widget::Action::publish(Message::NudgeBeat(0.25)));
                }
                key::Key::Named(key::Named::ArrowLeft) => {
                    return Some(widget::Action::publish(Message::NudgeLane(-1)));
                }
                key::Key::Named(key::Named::ArrowRight) => {
                    return Some(widget::Action::publish(Message::NudgeLane(1)));
                }

                key::Key::Named(key::Named::Space) => {
                    state.show_mod_layer = !state.show_mod_layer;
                }

                key::Key::Character(ch) => {
                    state.current_note_kind = match ch.as_str() {
                        "q" => NoteKind::Chip,
                        "w" => NoteKind::Hold,
                        "e" => NoteKind::Mine,
                        "r" => NoteKind::Bumper,
                        "a" => NoteKind::TempoChange,
                        "s" => NoteKind::AbsoluteBumper,
                        "d" => NoteKind::BumperMine,
                        "f" => NoteKind::UnknownType,
                        _ => NoteKind::Chip,
                    }
                }

                _ => {}
            },

            iced::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(modifiers)) => {
                state.place_notes = modifiers.contains(Modifiers::SHIFT);
                state.disable_snapping = state.place_notes && modifiers.contains(Modifiers::ALT);
                state.seeking = modifiers.contains(Modifiers::CTRL);
            }

            iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta }) => {
                state.scroll_pos += match delta {
                    mouse::ScrollDelta::Pixels { x: _, y } => *y,
                    mouse::ScrollDelta::Lines { x: _, y } => y * state.note_height,
                };
            }

            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                if !state.show_mod_layer
                    && let Some(note_idx) = get_hovered_note(&self.chart, &state, &cursor, bounds)
                {
                    return Some(widget::Action::publish(Message::SelectNote(note_idx)));
                } else if state.show_mod_layer
                    && let Some(mod_idx) = get_hovered_mod(&self.chart, &state, &cursor, bounds)
                {
                    log::debug!("Select mod: {mod_idx}");
                    return Some(widget::Action::publish(Message::SelectModifier(mod_idx)));
                } else if !state.show_mod_layer
                    && state.place_notes
                    && let Some(beat) = mouse_y_to_beat(&self.chart, &state, &cursor, bounds)
                    && let Some(lane) = mouse_x_to_lane(&state, &cursor, bounds)
                {
                    let snapped_time = if state.disable_snapping {
                        self.chart.bpm_handler.time_from_beat(beat)
                    } else {
                        self.chart.bpm_handler.time_from_beat(beat.round())
                    };
                    match snapped_time {
                        Some(t) => {
                            return Some(widget::Action::publish(Message::CreateNote(
                                t,
                                state.current_note_kind,
                                lane,
                            )));
                        }
                        None => {}
                    }
                } else if state.show_mod_layer
                    && state.place_notes
                    && let Some(beat) = mouse_y_to_beat(&self.chart, state, &cursor, bounds)
                {
                    return Some(widget::Action::publish(Message::CreateModifier(Modifier {
                        start_beat: beat,
                        ..Default::default()
                    })));
                } else if cursor.is_over(bounds) && state.seeking {
                    if let Some(seek_pos) = mouse_y_to_time(state, &cursor, bounds) {
                        log::info!("Seeking to: {seek_pos}");
                        return Some(widget::Action::publish(Message::MusicSeek(seek_pos)));
                    }
                } else if cursor.is_over(bounds) {
                    return Some(widget::Action::publish(Message::DeselectNote));
                }
            }

            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right)) => {
                if !state.show_mod_layer
                    && let Some(note_idx) = get_hovered_note(&self.chart, &state, &cursor, bounds)
                {
                    return Some(widget::Action::publish(Message::RemoveNote(note_idx)));
                } else if state.show_mod_layer
                    && let Some(mod_idx) = get_hovered_mod(&self.chart, &state, &cursor, bounds)
                {
                    return Some(widget::Action::publish(Message::RemoveMod(mod_idx)));
                }
            }

            _ => {}
        }

        let lane_percent: f32 = 0.5;
        state.lane_width = (bounds.width * lane_percent) / 4.0;
        state.gutter_width = bounds.width * (1.0 - lane_percent);
        Some(widget::Action::publish(Message::UpdatePlayback))
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let marker_stroke = canvas::Stroke::default()
            .with_color(color!(0xc9c9c9))
            .with_width(2.5);

        for i in 1..5 {
            let x_pos = i as f32 * state.lane_width;
            let lane_marker = canvas::Path::line(
                iced::Point::new(x_pos, 0.0),
                iced::Point::new(x_pos, bounds.height),
            );
            frame.stroke(&lane_marker, marker_stroke);
        }

        let mut marker_beat: f32 = 0.0;
        loop {
            let time = match self.chart.bpm_handler.time_from_beat(marker_beat) {
                Some(t) => t,
                None => break,
            };
            marker_beat += 1.0;

            let y_pos = time * state.units_per_ms + state.scroll_pos;
            if y_pos > bounds.height {
                break;
            }

            let marker = canvas::Path::line(
                iced::Point::new(0.0, y_pos),
                iced::Point::new(bounds.width, y_pos),
            );
            frame.stroke(&marker, marker_stroke.with_width(0.5));

            let mut beat_label = canvas::Text::from(format!("{marker_beat}"));
            beat_label.color = theme.palette().text;
            beat_label.position =
                iced::Point::new(bounds.width - state.gutter_width + 5.0, y_pos + 1.5);
            frame.fill_text(beat_label);
        }

        let earliest_visible_time = (0.0 - state.scroll_pos) / state.units_per_ms;
        let latest_visible_time = (bounds.height - state.scroll_pos) / state.units_per_ms;

        for note in &self.chart.notes {
            if note.time > latest_visible_time {
                continue;
            }
            // hack: i cba to calculate this for hold notes
            if note.kind != NoteKind::Hold && note.time < earliest_visible_time {
                continue;
            }

            let (pos, size) = calc_note_display(&state, &note);
            let fill = canvas::Fill::from(get_note_color(&note));
            frame.fill_rectangle(pos, size, fill);
        }

        if state.show_mod_layer {
            let translucent_fill = canvas::Fill::from(iced::Color {
                a: 0.8,
                ..theme.palette().background
            });
            frame.fill_rectangle(iced::Point::default(), bounds.size(), translucent_fill);

            let mut tracker = ModTracker::new();

            for modifier in &self.chart.gimmick.mods {
                let mod_time = self
                    .chart
                    .bpm_handler
                    .time_from_beat(modifier.start_beat)
                    .unwrap_or(-1.0);

                let end_time = self
                    .chart
                    .bpm_handler
                    .time_from_beat(modifier.start_beat + modifier.duration)
                    .unwrap_or(-1.0);

                if end_time < earliest_visible_time || mod_time > latest_visible_time {
                    tracker.add(modifier.start_beat, modifier.start_beat + modifier.duration);
                    continue;
                }

                let offset_x = MOD_RADIUS
                    + tracker.count_at_beat(modifier.start_beat) as f32 * MOD_RADIUS * 3.0;
                let start_point =
                    iced::Point::new(offset_x, mod_time * state.units_per_ms + state.scroll_pos);
                let end_point =
                    iced::Point::new(offset_x, end_time * state.units_per_ms + state.scroll_pos);

                let circle = canvas::Path::circle(start_point, MOD_RADIUS);
                let line = canvas::Path::line(start_point, end_point);

                const MOD_COLOUR: iced::Color = color!(0x96ff9d);
                let fill = canvas::Fill::from(MOD_COLOUR);
                let stroke = canvas::Stroke::default()
                    .with_color(MOD_COLOUR)
                    .with_width(MOD_RADIUS / 2.0);

                frame.fill(&circle, fill);
                frame.stroke(&line, stroke);

                tracker.add(modifier.start_beat, modifier.start_beat + modifier.duration);
            }
        }

        if let Some(music_pos) = self.music_pos {
            let line_y = (music_pos + self.music_offset) * state.units_per_ms + state.scroll_pos;
            let line = canvas::Path::line(
                iced::Point::new(0.0, line_y),
                iced::Point::new(bounds.width, line_y),
            );
            let stroke = canvas::Stroke::default()
                .with_color(color!(0xFF00FF))
                .with_width(3.0);
            frame.stroke(&line, stroke);
        }

        vec![frame.into_geometry()]
    }
}

fn get_note_color(note: &Note) -> iced::Color {
    match note.kind {
        NoteKind::Chip | NoteKind::Hold | NoteKind::Bumper | NoteKind::AbsoluteBumper => {
            get_lane_color(note.lane)
        }
        NoteKind::Mine | NoteKind::BumperMine => color!(0x6b0000),
        NoteKind::TempoChange => color!(0x96ff9d),
        _ => color!(0xFF00FF),
    }
}

fn get_lane_color(lane: u8) -> iced::Color {
    match lane {
        0 | 1 => color!(0xcaf4ff),
        2 | 3 => color!(0xffcbf9),
        _ => color!(0xff00ff),
    }
}

fn calc_note_display(state: &NoteDrawState, note: &Note) -> (iced::Point, iced::Size) {
    let pos = iced::Point::new(
        note.lane as f32 * state.lane_width + state.x_padding,
        note.time * state.units_per_ms + state.scroll_pos,
    );

    let width_mult = match note.kind {
        NoteKind::Bumper | NoteKind::BumperMine | NoteKind::AbsoluteBumper => 2.0,
        NoteKind::TempoChange => 4.0,
        _ => 1.0,
    };

    let mut size = iced::Size::new(
        (state.lane_width * width_mult) - (state.x_padding * 2.0),
        state.note_height,
    );

    if let Some(NoteExtra::HoldEndTime(end_time)) = note.extra {
        size.height = (end_time as f32 - note.time) * state.units_per_ms;
    }

    (pos, size)
}

fn get_hovered_note(
    chart: &Chart,
    state: &NoteDrawState,
    cursor: &mouse::Cursor,
    bounds: Rectangle<f32>,
) -> Option<usize> {
    let mouse = cursor.position_in(bounds)?;

    for i in 0..chart.notes.len() {
        let note = &chart.notes[i];
        let (pos, size) = calc_note_display(&state, &note);
        let max_pos = iced::Point {
            x: pos.x + size.width,
            y: pos.y + size.height,
        };

        if mouse.x > pos.x && mouse.y > pos.y && mouse.x < max_pos.x && mouse.y < max_pos.y {
            return Some(i);
        }
    }
    None
}

fn get_hovered_mod(
    chart: &Chart,
    state: &NoteDrawState,
    cursor: &mouse::Cursor,
    bounds: Rectangle<f32>,
) -> Option<usize> {
    let mut tracker = ModTracker::new();
    let mouse = cursor.position_in(bounds)?;

    for i in 0..chart.gimmick.mods.len() {
        let modifier = &chart.gimmick.mods[i];
        let mod_time = chart
            .bpm_handler
            .time_from_beat(modifier.start_beat)
            .unwrap_or(-1.0);

        let offset_x =
            MOD_RADIUS + tracker.count_at_beat(modifier.start_beat) as f32 * MOD_RADIUS * 3.0;
        let start_point =
            iced::Point::new(offset_x, mod_time * state.units_per_ms + state.scroll_pos);

        if mouse.distance(start_point) <= MOD_RADIUS {
            return Some(i);
        }

        tracker.add(modifier.start_beat, modifier.start_beat + modifier.duration);
    }

    None
}

fn mouse_y_to_time(
    state: &NoteDrawState,
    cursor: &mouse::Cursor,
    bounds: Rectangle<f32>,
) -> Option<f32> {
    let mouse = cursor.position_in(bounds)?;
    Some((mouse.y - state.scroll_pos) / state.units_per_ms)
}

fn mouse_y_to_beat(
    chart: &Chart,
    state: &NoteDrawState,
    cursor: &mouse::Cursor,
    bounds: Rectangle<f32>,
) -> Option<f32> {
    let time = mouse_y_to_time(state, cursor, bounds)?;
    chart.bpm_handler.beat_from_time(time)
}

fn mouse_x_to_lane(
    state: &NoteDrawState,
    cursor: &mouse::Cursor,
    bounds: Rectangle<f32>,
) -> Option<u8> {
    let mouse = cursor.position_in(bounds)?;
    let lane = (mouse.x / state.lane_width) as u8;
    if lane < 4 { Some(lane) } else { None }
}
