use crate::packet::UnpackError;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Flag {
    None,
    Green,
    Blue,
    Yellow,
    Red,
    Invalid,
}

impl TryFrom<i8> for Flag {
    type Error = UnpackError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Flag::None),
            1 => Ok(Flag::Green),
            2 => Ok(Flag::Blue),
            3 => Ok(Flag::Yellow),
            4 => Ok(Flag::Red),
            -1 => Ok(Flag::Invalid),
            _ => Err(UnpackError(format!("Invalid Flag value: {}", value))),
        }
    }
}
