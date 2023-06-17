use serde::Serialize;

use crate::packet::generic::{TyreCompound, TyreCompoundVisual};
use crate::packet::header::PacketHeader;

use super::generic::SessionType;

/// This type is used for the `tyre_set` array of the [`PacketTyreSetsData`] type.
///
/// See also [`TyreCompound`] and [`TyreCompoundVisual`].
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub struct TyreSetData {
    /// Actual tyre compound used
    pub tyre_compound: TyreCompound,
    /// Visual tyre compound used
    pub tyre_compound_visual: TyreCompoundVisual,
    /// Tyre wear (percentage)
    pub wear_pct: u8,
    /// Whether this set is currently available
    pub is_available: bool,
    /// Recommended session for tyre set
    pub recommended_session: SessionType,
    /// Laps left in this tyre set
    pub laps_left: u8,
    /// Max number of laps recommended for this compound
    pub usable_life: u8,
    /// Lap delta time in milliseconds compared to fitted set
    pub lap_delta_time: u16,
    /// Whether the set is fitted or not
    pub is_fitted: bool,
}

/// This packet gives more in-depth details about tyre sets assigned to a
/// vehicle during the session.
///
/// Frequency: 20 per second but cycling through cars
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct PacketTyreSetsData {
    /// Packet header
    pub header: PacketHeader,
    /// Index of the car this lap data relates to
    pub car_index: u8,
    /// List of tyre sets
    pub lap_history: Vec<TyreSetData>,
    /// Index into array of fitted tyre
    pub fitted_idx: u8,
}
