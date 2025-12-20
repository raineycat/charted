use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum NoteKind {
    #[default]
    Chip,
    Mine,
    Hold,

    Bumper,
    BumperMine,
    AbsoluteBumper,

    TempoChange,
    UnknownType,
}

impl NoteKind {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(Self::Chip),
            1 => Some(Self::Bumper),
            2 => Some(Self::Hold),
            3 => Some(Self::TempoChange),
            4 => Some(Self::UnknownType),
            6 => Some(Self::Mine),
            7 => Some(Self::BumperMine),
            8 => Some(Self::AbsoluteBumper),
            _ => None,
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Chip => 0,
            Self::Bumper => 1,
            Self::Hold => 2,
            Self::TempoChange => 3,
            Self::UnknownType => 4,
            Self::Mine => 6,
            Self::BumperMine => 7,
            Self::AbsoluteBumper => 8,
        }
    }
}

impl Display for NoteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Chip => "Chip",
            Self::Bumper => "Bumper",
            Self::Hold => "Hold",
            Self::TempoChange => "Tempo Change",
            Self::UnknownType => "Unknown",
            Self::Mine => "Mine",
            Self::BumperMine => "Bumper Mine",
            Self::AbsoluteBumper => "Absolute Bumper",
        })
    }
}
