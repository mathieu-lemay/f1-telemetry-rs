use crate::packet::generic::{Flag, ResultStatus};
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

pub(crate) fn unpack_result_status(value: u8) -> Result<ResultStatus, UnpackError> {
    match value {
        0 => Ok(ResultStatus::Invalid),
        1 => Ok(ResultStatus::Inactive),
        2 => Ok(ResultStatus::Active),
        3 => Ok(ResultStatus::Finished),
        4 => Ok(ResultStatus::Disqualified),
        5 => Ok(ResultStatus::NotClassified),
        6 => Ok(ResultStatus::Retired),
        _ => Err(UnpackError(format!(
            "Invalid ResultStatus value: {}",
            value
        ))),
    }
}
