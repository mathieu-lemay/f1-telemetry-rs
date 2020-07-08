use crate::packet::generic::Flag;
use crate::packet::UnpackError;

pub(crate) fn unpack_flag(value: i8) -> Result<Flag, UnpackError> {
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
