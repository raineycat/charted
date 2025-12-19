use binary_rw::{BinaryError, BinaryReader, BinaryWriter};

use crate::chart::{bpm_handler::BpmHandler, gimmick::Gimmick, note::Note};

pub mod bpm_handler;
pub mod easing;
pub mod gimmick;
pub mod note;
mod terminated_string;

#[derive(Debug, Clone, Default)]
pub struct Chart {
    pub notes: Vec<Note>,
    pub gimmick: Gimmick,
    pub bpm_handler: BpmHandler,
}

impl Chart {
    pub fn recalc_bpm(&mut self) {
        let mut bpm_handler = BpmHandler::default();
        bpm_handler.add_changes_from_chart(&self);
        self.bpm_handler = bpm_handler;
    }

    pub fn read_binary(r: &mut BinaryReader) -> Result<Self, BinaryError> {
        let mut chart = Self::default();

        for c in "VSC".chars() {
            assert_eq!(r.read_u8()?, c as u8);
        }

        assert_eq!(r.read_u8()?, 1);
        assert_eq!(r.read_u8()?, 0);

        loop {
            let flag = r.read_u8()?;
            match flag {
                0xC0 => loop {
                    let flag = r.read_u8()?;
                    match flag {
                        0xA0 => chart.notes.push(Note::read_binary(r)?),
                        0xC1 => break,
                        _ => {}
                    }
                },

                0xE0 => chart.gimmick = Gimmick::read_binary(r)?,

                0xFF => break,
                _ => {}
            }
        }

        chart.recalc_bpm();
        Ok(chart)
    }

    pub fn write_binary(&self, w: &mut BinaryWriter) -> Result<(), BinaryError> {
        for c in "VSC".chars() {
            w.write_u8(c as u8)?;
        }

        w.write_u8(1)?;
        w.write_u8(0)?;

        w.write_u8(0xC0)?; // note list start
        for note in &self.notes {
            note.write_binary(w)?;
        }
        w.write_u8(0xC1)?; // note list end

        self.gimmick.write_binary(w)?;
        w.write_u8(0xFF)?;

        // append bytes to fill the signature
        w.write_bytes_with_value(0x180, 0)?;
        Ok(())
    }
}
