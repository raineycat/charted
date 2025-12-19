use binary_rw::{BinaryError, BinaryReader};

use crate::chart::{easing::Easing, terminated_string::ReadTerminatedString};

#[derive(Debug, Clone)]
pub struct Gimmick {
    pub gm_object_name: String,
    pub proxies: u8,
    pub mods: Vec<Modifier>,
    pub per_frames: Vec<PerFrame>,
}

#[derive(Debug, Clone, Default)]
pub struct Modifier {
    pub start_beat: f32,
    pub duration: f32,
    pub ease: Easing,
    pub start_val: f32,
    pub end_val: f32,
    pub kind: u8,
    pub proxy_index: i8,
}

#[derive(Debug, Clone, Default)]
pub struct PerFrame {
    pub start_beat: f32,
    pub end_beat: f32,
    pub gm_function: String,
}

impl Gimmick {
    pub(crate) fn read_binary(r: &mut BinaryReader) -> Result<Self, BinaryError> {
        let mut gimmick = Gimmick::default();
        loop {
            let flag = r.read_u8()?;
            match flag {
                0xE2 => loop {
                    let flag = r.read_u8()?;
                    match flag {
                        0xE9 => gimmick.mods.push(Modifier::read_binary(r)?),
                        0xEC => gimmick.per_frames.push(PerFrame::read_binary(r)?),

                        0xE3 => break,
                        _ => {}
                    }
                },

                0xE4 => gimmick.proxies = r.read_u8()?,
                0xE5 => gimmick.gm_object_name = r.read_null_terminated_string()?,

                0xE1 => break,
                _ => {}
            }
        }
        Ok(gimmick)
    }
}

impl Default for Gimmick {
    fn default() -> Self {
        Self {
            gm_object_name: "obj_base_gimmick".to_owned(),
            proxies: 0,
            mods: vec![],
            per_frames: vec![],
        }
    }
}

impl Modifier {
    pub(crate) fn read_binary(r: &mut BinaryReader) -> Result<Self, BinaryError> {
        Ok(Self {
            start_beat: r.read_f32()?,
            duration: r.read_f32()?,
            ease: Easing::from_byte(r.read_u8()?).unwrap_or(Easing::Linear),
            start_val: r.read_f32()?,
            end_val: r.read_f32()?,
            kind: r.read_u8()?,
            proxy_index: r.read_i8()?,
        })
    }
}

impl PerFrame {
    pub(crate) fn read_binary(r: &mut BinaryReader) -> Result<Self, BinaryError> {
        Ok(Self {
            start_beat: r.read_f32()?,
            end_beat: r.read_f32()?,
            gm_function: r.read_null_terminated_string()?,
        })
    }
}
