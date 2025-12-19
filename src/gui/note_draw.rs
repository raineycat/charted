use iced::{
    Renderer, Theme, color,
    keyboard::key,
    mouse,
    widget::{self, canvas},
};

use crate::chart::{
    Chart,
    note::{Note, NoteExtra},
};

pub struct NoteDrawState {
    scroll_pos: f32,
    lane_width: f32,
    gutter_width: f32,
    note_height: f32,
    units_per_ms: f32,
    x_padding: f32,
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
        }
    }
}

impl<Message> canvas::Program<Message> for Chart {
    type State = NoteDrawState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
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
                key::Key::Named(key::Named::Home) => state.scroll_pos = 0.0,
                key::Key::Named(key::Named::ArrowUp) => state.scroll_pos += state.note_height,
                key::Key::Named(key::Named::ArrowDown) => state.scroll_pos -= state.note_height,
                _ => {}
            },

            iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta }) => {
                state.scroll_pos += match delta {
                    mouse::ScrollDelta::Pixels { x: _, y } => *y,
                    mouse::ScrollDelta::Lines { x: _, y } => y * state.note_height,
                };
            }

            _ => {}
        }

        state.lane_width = (bounds.width * 0.8) / 4.0;
        state.gutter_width = bounds.width * 0.2;
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
            let time = &self.bpm_handler.time_from_beat(marker_beat).unwrap_or(-1.0);
            let y_pos = time * state.units_per_ms + state.scroll_pos;
            if y_pos > bounds.height {
                break;
            }

            let marker = canvas::Path::line(
                iced::Point::new(0.0, y_pos),
                iced::Point::new(bounds.width, y_pos),
            );
            frame.stroke(&marker, marker_stroke.with_width(1.0));

            let mut beat_label = canvas::Text::from(format!("{marker_beat}"));
            beat_label.color = theme.palette().text;
            beat_label.position =
                iced::Point::new(bounds.width - state.gutter_width + 3.0, y_pos + 1.5);
            frame.fill_text(beat_label);

            marker_beat += 1.0;
        }

        for note in &self.notes {
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

            let color_val = match note.kind {
                Note::CHIP | Note::HOLD | Note::BUMPER | Note::ABSOLUTE_BUMPER => {
                    get_lane_color(note.lane)
                }
                Note::MINE | Note::BUMPER_MINE => color!(0x6b0000),
                Note::TEMPO_CHANGE => color!(0x96ff9d),
                _ => color!(0xFF00FF),
            };

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

            let gutter_portion: f32 = 1.0 / 3.0;
            let pos = iced::Point::new(
                bounds.width - (state.gutter_width * gutter_portion) + state.x_padding,
                start_time * state.units_per_ms + state.scroll_pos,
            );
            let size = iced::Size::new(
                (state.gutter_width * gutter_portion) - (state.x_padding * 2.0),
                *end_time * state.units_per_ms,
            );

            frame.fill_rectangle(pos, size, canvas::Fill::from(color!(0x96ff9d)));
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
