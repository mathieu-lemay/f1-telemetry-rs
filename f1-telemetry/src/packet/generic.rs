use getset::CopyGetters;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Flag {
    None,
    Green,
    Blue,
    Yellow,
    Red,
    Invalid,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResultStatus {
    Invalid,
    Inactive,
    Active,
    Finished,
    Disqualified,
    NotClassified,
    Retired,
}

impl Default for ResultStatus {
    fn default() -> Self {
        Self::Invalid
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TyreCompound {
    C5,
    C4,
    C3,
    C2,
    C1,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
    Invalid,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TyreCompoundVisual {
    Soft,
    Medium,
    Hard,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
    Invalid,
}

impl Default for TyreCompoundVisual {
    fn default() -> Self {
        TyreCompoundVisual::Invalid
    }
}

impl TyreCompoundVisual {
    pub fn name<'a>(self) -> &'a str {
        match self {
            TyreCompoundVisual::Soft => "Soft",
            TyreCompoundVisual::Medium => "Medium",
            TyreCompoundVisual::Hard => "Hard",
            TyreCompoundVisual::Inter => "Intermediate",
            TyreCompoundVisual::Wet => "Wet",
            TyreCompoundVisual::ClassicDry => "Dry (Classic)",
            TyreCompoundVisual::ClassicWet => "Wet (Classic)",
            TyreCompoundVisual::F2SuperSoft => "Super Soft (F2)",
            TyreCompoundVisual::F2Soft => "Soft (F2)",
            TyreCompoundVisual::F2Medium => "Medium (F2)",
            TyreCompoundVisual::F2Hard => "Hard (F2)",
            TyreCompoundVisual::F2Wet => "Wet (F2)",
            TyreCompoundVisual::Invalid => "Invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, CopyGetters)]
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
