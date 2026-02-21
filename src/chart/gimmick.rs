use binary_rw::{BinaryError, BinaryReader, BinaryWriter};

use crate::chart::{easing::Easing, terminated_string::*};

#[derive(Debug, Clone)]
pub struct Gimmick {
    pub gm_object_name: String,
    pub proxies: u8,
    pub mods: Vec<Modifier>,
    pub per_frames: Vec<PerFrame>,
}

#[derive(Debug, Clone)]
pub struct Modifier {
    pub start_beat: f32,
    pub duration: f32,
    pub ease: Easing,
    pub start_val: f32,
    pub end_val: f32,
    pub kind: u8,
    pub proxy_index: i8,
}

impl Default for Modifier {
    fn default() -> Self {
        Self {
            start_beat: 0.0,
            duration: 0.0,
            ease: Easing::Linear,
            start_val: 1.0,
            end_val: 1.0,
            kind: 0,
            proxy_index: -1,
        }
    }
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

    pub(crate) fn write_binary(&self, w: &mut BinaryWriter) -> Result<(), BinaryError> {
        w.write_u8(0xE0)?; // gimmick start

        w.write_u8(0xE4)?; // proxy count
        w.write_u8(self.proxies)?;

        w.write_u8(0xE5)?; // GM:S object name
        w.write_null_terminated_string(&self.gm_object_name)?;

        w.write_u8(0xE2)?; // start of mods/per-frames
        for modifier in &self.mods {
            modifier.write_binary(w)?;
        }
        for pf in &self.per_frames {
            pf.write_binary(w)?;
        }
        w.write_u8(0xE3)?; // end of mods/per-frames

        w.write_u8(0xE1)?; // gimmick end
        Ok(())
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

    pub(crate) fn write_binary(&self, w: &mut BinaryWriter) -> Result<(), BinaryError> {
        w.write_u8(0xE9)?; // modifier start
        w.write_f32(self.start_beat)?;
        w.write_f32(self.duration)?;
        w.write_u8(self.ease.to_byte())?;
        w.write_f32(self.start_val)?;
        w.write_f32(self.end_val)?;
        w.write_u8(self.kind)?;
        w.write_i8(self.proxy_index)?;
        Ok(())
    }

    pub const KNOWN_KINDS: [&'static str; 73] = [
        "unknown",
        "prx",
        "prxb",
        "prxc",
        "pry",
        "pryb",
        "pryc",
        "prsx",
        "pra",
        "przm",
        "przmb",
        "przx",
        "przy",
        "prrx",
        "prry",
        "prrz",
        "prrzb",
        "shxs",
        "shxp",
        "shxa",
        "shys",
        "shyp",
        "shya",
        "scrollspeed",
        "noterot",
        "velocity",
        "spinradius",
        "spiny",
        "spinx",
        "driven",
        "beat",
        "wave",
        "hom",
        "boost_distance",
        "boost_time",
        "yoffset",
        "notealp",
        "przmc",
        "prxd",
        "pryd",
        "prct",
        "prcb",
        "prcl",
        "prcr",
        "prvib",
        "shct",
        "shft",
        "shcb",
        "shfb",
        "shcl",
        "shfl",
        "shcr",
        "shfr",
        "scrollind0",
        "scrollind1",
        "scrollind2",
        "scrollind3",
        "scrollind4",
        "scrollind5",
        "scrollind6",
        "drawdist",
        "pburstleft",
        "pburstright",
        "particlexpower",
        "particleypower",
        "uialpha",
        "fx_contrast",
        "fx_chroma_distort",
        "fx_film",
        "fx_glow",
        "fx_particleglow",
        "pburstspeed",
        "freeze",
    ];
}

impl PerFrame {
    pub(crate) fn read_binary(r: &mut BinaryReader) -> Result<Self, BinaryError> {
        Ok(Self {
            start_beat: r.read_f32()?,
            end_beat: r.read_f32()?,
            gm_function: r.read_null_terminated_string()?,
        })
    }

    pub(crate) fn write_binary(&self, w: &mut BinaryWriter) -> Result<(), BinaryError> {
        w.write_u8(0xEC)?; // per-frame start
        w.write_f32(self.start_beat)?;
        w.write_f32(self.end_beat)?;
        w.write_null_terminated_string(&self.gm_function)?;
        Ok(())
    }
}
