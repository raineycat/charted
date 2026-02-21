use std::fmt::Display;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Easing {
    #[default]
    Linear,

    OutElastic,
    OutBack,
    OutQuart,

    InExpo,
    OutExpo,
    InOutExpo,

    InQuad,
    OutQuad,
    InOutQuad,

    InCubic,
    OutCubic,
    InOutCubic,

    InSine,
    OutSine,
    InOutSine,

    InCirc,
    OutCirc,
    InOutCirc,
}

impl Easing {
    pub const ALL: [Self; 19] = [
        Self::Linear,
        Self::OutElastic,
        Self::InExpo,
        Self::OutExpo,
        Self::InOutExpo,
        Self::InQuad,
        Self::OutQuad,
        Self::InOutQuad,
        Self::InCubic,
        Self::OutCubic,
        Self::InOutCubic,
        Self::OutBack,
        Self::InSine,
        Self::OutSine,
        Self::InOutSine,
        Self::OutQuart,
        Self::InOutCirc,
        Self::InCirc,
        Self::OutCirc,
    ];

    pub(crate) fn from_byte(b: u8) -> Option<Easing> {
        Self::ALL.get((b as usize) - 1).cloned()
    }

    pub(crate) fn to_byte(&self) -> u8 {
        Self::ALL.iter().position(|e| *e == *self).unwrap_or(0) as u8 + 1
    }
}

impl Display for Easing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
