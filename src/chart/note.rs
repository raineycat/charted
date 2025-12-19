use binary_rw::{BinaryError, BinaryReader, BinaryWriter};

#[derive(Debug, Clone, Default)]
pub struct Note {
    pub kind: u8,
    pub lane: u8,
    pub time: f32,
    pub extra: Option<NoteExtra>,
}

#[derive(Debug, Clone)]
pub enum NoteExtra {
    HoldEndTime(i32),
    TempoChange(f32),
}

impl Note {
    pub const CHIP: u8 = 0;
    pub const BUMPER: u8 = 1;
    pub const HOLD: u8 = 2;
    pub const TEMPO_CHANGE: u8 = 3;
    pub const UNKNOWN: u8 = 4;
    pub const MINE: u8 = 6;
    pub const BUMPER_MINE: u8 = 7;
    pub const ABSOLUTE_BUMPER: u8 = 8;

    pub(crate) fn read_binary(r: &mut BinaryReader) -> Result<Self, BinaryError> {
        let mut note = Self::default();

        loop {
            let flag = r.read_u8()?;
            match flag {
                0xA2 => note.kind = r.read_u8()?,
                0xA3 => note.lane = r.read_u8()?,
                0xA4 => note.time = r.read_f32()?,
                0xA6 => note.extra = NoteExtra::read_binary(note.kind, r)?,

                0xA1 => break,
                _ => {}
            }
        }

        Ok(note)
    }

    pub(crate) fn write_binary(&self, w: &mut BinaryWriter) -> Result<(), BinaryError> {
        w.write_u8(0xA0)?; // note start

        w.write_u8(0xA2)?; // note kind
        w.write_u8(self.kind)?;

        w.write_u8(0xA3)?; // note lane
        w.write_u8(self.lane)?;

        w.write_u8(0xA4)?; // note time
        w.write_f32(self.time)?;

        match &self.extra {
            Some(e) => e.write_binary(w)?,
            None => {}
        };

        w.write_u8(0xA1)?; // note end
        Ok(())
    }
}

impl NoteExtra {
    fn write_binary(&self, w: &mut BinaryWriter) -> Result<(), BinaryError> {
        w.write_u8(0xA6)?; // extra start

        match self {
            Self::HoldEndTime(t) => {
                w.write_u8(0xB3)?; // data type (i32)
                w.write_u8(1)?; // field index (1)
                w.write_i32(t)?;
            }
            Self::TempoChange(t) => {
                w.write_u8(0xB6)?; // data type (f32)
                w.write_u8(1)?; // field index (1)
                w.write_f32(t)?;
            }
        }

        w.write_u8(0xA7)?; // extra end
        Ok(())
    }

    fn read_binary(note_kind: u8, r: &mut BinaryReader) -> Result<Option<Self>, BinaryError> {
        let data_type = r.read_u8()?;
        if data_type == 0xA7 {
            return Ok(None);
        }

        match note_kind {
            Note::HOLD => {
                let field_id = r.read_u8()?;
                assert_eq!(field_id, 1);

                let value = match data_type {
                    0xB3 => Some(r.read_i32()?),
                    0xB6 => Some(r.read_f32()? as i32),
                    _ => None,
                };

                Ok(value.map(|val| Self::HoldEndTime(val)))
            }

            Note::TEMPO_CHANGE => {
                let field_id = r.read_u8()?;
                assert_eq!(field_id, 1);

                let value = match data_type {
                    0xB3 => Some(r.read_i32()? as f32),
                    0xB6 => Some(r.read_f32()?),
                    _ => None,
                };

                Ok(value.map(|val| Self::TempoChange(val)))
            }

            _ => Ok(None),
        }
    }
}
