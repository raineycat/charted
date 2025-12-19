use crate::{
    chart::{
        Chart,
        note::{Note, NoteExtra},
    },
    gui::state::Message,
};
use iced::{
    Rectangle, Renderer, Theme, color,
    keyboard::{Modifiers, key},
    mouse,
    widget::{self, canvas},
};

pub struct NoteDrawState {
    scroll_pos: f32,
    lane_width: f32,
    gutter_width: f32,
    note_height: f32,
    units_per_ms: f32,
    x_padding: f32,

    place_notes: bool,
    disable_snapping: bool,
    current_note_kind: u8,
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
            current_note_kind: Note::CHIP,
        }
    }
}

impl canvas::Program<Message> for Chart {
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

                key::Key::Character(ch) => {
                    state.current_note_kind = match ch.as_str() {
                        "q" => Note::CHIP,
                        "w" => Note::HOLD,
                        "e" => Note::MINE,
                        "r" => Note::BUMPER,
                        "a" => Note::TEMPO_CHANGE,
                        "s" => Note::ABSOLUTE_BUMPER,
                        "d" => Note::BUMPER_MINE,
                        "f" => Note::UNKNOWN,
                        _ => Note::CHIP,
                    }
                }

                _ => {}
            },

            iced::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(modifiers)) => {
                state.place_notes = modifiers.contains(Modifiers::SHIFT);
                state.disable_snapping = state.place_notes && modifiers.contains(Modifiers::ALT);
            }

            iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta }) => {
                state.scroll_pos += match delta {
                    mouse::ScrollDelta::Pixels { x: _, y } => *y,
                    mouse::ScrollDelta::Lines { x: _, y } => y * state.note_height,
                };
            }

            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                if let Some(note_idx) = get_hovered_note(&self, &state, &cursor, bounds) {
                    return Some(widget::Action::publish(Message::SelectNote(note_idx)));
                } else if state.place_notes
                    && let Some(beat) = mouse_y_to_beat(&self, &state, &cursor, bounds)
                    && let Some(lane) = mouse_x_to_lane(&state, &cursor, bounds)
                {
                    let snapped_time = if state.disable_snapping {
                        self.bpm_handler.time_from_beat(beat)
                    } else {
                        self.bpm_handler.time_from_beat(beat.round())
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
                } else if cursor.is_over(bounds) {
                    return Some(widget::Action::publish(Message::DeselectNote));
                }
            }

            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right)) => {
                if let Some(note_idx) = get_hovered_note(&self, &state, &cursor, bounds) {
                    return Some(widget::Action::publish(Message::RemoveNote(note_idx)));
                }
            }

            _ => {}
        }

        state.lane_width = (bounds.width * 0.85) / 4.0;
        state.gutter_width = bounds.width * 0.15;
        Some(widget::Action::request_redraw())
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
            let time = match self.bpm_handler.time_from_beat(marker_beat) {
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

        for note in &self.notes {
            let color_val = match note.kind {
                Note::CHIP | Note::HOLD | Note::BUMPER | Note::ABSOLUTE_BUMPER => {
                    get_lane_color(note.lane)
                }
                Note::MINE | Note::BUMPER_MINE => color!(0x6b0000),
                Note::TEMPO_CHANGE => color!(0x96ff9d),
                _ => color!(0xFF00FF),
            };

            let (pos, size) = calc_note_display(&state, &note);
            frame.fill_rectangle(pos, size, canvas::Fill::from(color_val));
        }

        for modifier in &self.gimmick.mods {
            let start_time = &self
                .bpm_handler
                .time_from_beat(modifier.start_beat)
                .unwrap_or(-1.0);

            let end_time = &self
                .bpm_handler
                .time_from_beat(modifier.start_beat + modifier.duration)
                .unwrap_or(-1.0);

            let gutter_portion: f32 = 1.0 / 4.0;
            let start = iced::Point::new(
                bounds.width - (state.gutter_width * gutter_portion) + state.x_padding,
                start_time * state.units_per_ms + state.scroll_pos,
            );
            let end = start + iced::Vector::new(0.0, (*end_time - start_time) * state.units_per_ms);
            let width = (state.gutter_width * gutter_portion) - (state.x_padding * 2.0);

            let line = canvas::Path::line(start, end);
            let stroke = canvas::Stroke::default()
                .with_color(color!(0x96ff9d))
                .with_width(width)
                .with_line_cap(canvas::LineCap::Round);
            frame.stroke(&line, stroke);
        }

        vec![frame.into_geometry()]
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
        Note::BUMPER | Note::BUMPER_MINE | Note::ABSOLUTE_BUMPER => 2.0,
        Note::TEMPO_CHANGE => 4.0,
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

fn mouse_y_to_beat(
    chart: &Chart,
    state: &NoteDrawState,
    cursor: &mouse::Cursor,
    bounds: Rectangle<f32>,
) -> Option<f32> {
    let mouse = cursor.position_in(bounds)?;
    let time = (mouse.y - state.scroll_pos) / state.units_per_ms;
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
