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
