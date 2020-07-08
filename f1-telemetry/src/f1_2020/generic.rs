use crate::packet::generic::{Flag, ResultStatus, TyreCompound, TyreCompoundVisual};
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

pub(crate) fn unpack_tyre_compound(value: u8) -> Result<TyreCompound, UnpackError> {
    match value {
        16 => Ok(TyreCompound::C5),
        17 => Ok(TyreCompound::C4),
        18 => Ok(TyreCompound::C3),
        19 => Ok(TyreCompound::C2),
        20 => Ok(TyreCompound::C1),
        7 => Ok(TyreCompound::Inter),
        8 => Ok(TyreCompound::Wet),
        9 => Ok(TyreCompound::ClassicDry),
        10 => Ok(TyreCompound::ClassicWet),
        11 => Ok(TyreCompound::F2SuperSoft),
        12 => Ok(TyreCompound::F2Soft),
        13 => Ok(TyreCompound::F2Medium),
        14 => Ok(TyreCompound::F2Hard),
        15 => Ok(TyreCompound::F2Wet),
        0 | 255 => Ok(TyreCompound::Invalid),
        _ => Err(UnpackError(format!(
            "Invalid TyreCompound value: {}",
            value
        ))),
    }
}

pub(crate) fn unpack_tyre_compound_visual(value: u8) -> Result<TyreCompoundVisual, UnpackError> {
    match value {
        16 => Ok(TyreCompoundVisual::Soft),
        17 => Ok(TyreCompoundVisual::Medium),
        18 => Ok(TyreCompoundVisual::Hard),
        7 => Ok(TyreCompoundVisual::Inter),
        8 => Ok(TyreCompoundVisual::Wet),
        9 => Ok(TyreCompoundVisual::ClassicDry),
        10 => Ok(TyreCompoundVisual::ClassicWet),
        11 => Ok(TyreCompoundVisual::F2SuperSoft),
        12 => Ok(TyreCompoundVisual::F2Soft),
        13 => Ok(TyreCompoundVisual::F2Medium),
        14 => Ok(TyreCompoundVisual::F2Hard),
        15 => Ok(TyreCompoundVisual::F2Wet),
        0 => Ok(TyreCompoundVisual::Invalid),
        _ => Err(UnpackError(format!(
            "Invalid TyreCompoundVisual value: {}",
            value
        ))),
    }
}
