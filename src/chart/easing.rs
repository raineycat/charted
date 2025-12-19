#[derive(Debug, Clone, Default)]
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
    pub(crate) fn from_byte(b: u8) -> Option<Easing> {
        match b {
            1 => Some(Self::Linear),
            2 => Some(Self::OutElastic),
            3 => Some(Self::InExpo),
            4 => Some(Self::OutExpo),
            5 => Some(Self::InOutExpo),
            6 => Some(Self::InQuad),
            7 => Some(Self::OutQuad),
            8 => Some(Self::InOutQuad),
            9 => Some(Self::InCubic),
            10 => Some(Self::OutCubic),
            11 => Some(Self::InOutCubic),
            12 => Some(Self::OutBack),
            13 => Some(Self::InSine),
            14 => Some(Self::OutSine),
            15 => Some(Self::InOutSine),
            16 => Some(Self::OutQuart),
            17 => Some(Self::InOutCirc),
            18 => Some(Self::InCirc),
            19 => Some(Self::OutCirc),
            _ => None,
        }
    }

    pub(crate) fn to_byte(&self) -> u8 {
        match self {
            Self::Linear => 1,
            Self::OutElastic => 2,
            Self::InExpo => 3,
            Self::OutExpo => 4,
            Self::InOutExpo => 5,
            Self::InQuad => 6,
            Self::OutQuad => 7,
            Self::InOutQuad => 8,
            Self::InCubic => 9,
            Self::OutCubic => 10,
            Self::InOutCubic => 11,
            Self::OutBack => 12,
            Self::InSine => 13,
            Self::OutSine => 14,
            Self::InOutSine => 15,
            Self::OutQuart => 16,
            Self::InOutCirc => 17,
            Self::InCirc => 18,
            Self::OutCirc => 19,
        }
    }
}
