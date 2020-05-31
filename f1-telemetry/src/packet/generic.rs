use getset::CopyGetters;
use std::convert::TryFrom;

use crate::packet::UnpackError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct WheelData<T>
where
    T: Clone + Copy,
{
    rear_left: T,
    rear_right: T,
    front_left: T,
    front_right: T,
}

impl<T> WheelData<T>
where
    T: Clone + Copy,
{
    pub fn new(rear_left: T, rear_right: T, front_left: T, front_right: T) -> WheelData<T> {
        WheelData {
            rear_left,
            rear_right,
            front_left,
            front_right,
        }
    }
}
