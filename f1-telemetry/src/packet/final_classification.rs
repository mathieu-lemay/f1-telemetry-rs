use serde::Serialize;

use crate::packet::generic::{ResultStatus, TyreCompound, TyreCompoundVisual};

use super::header::PacketHeader;

/// This type is used for the `classification_data` array of the [`PacketFinalClassificationData`] type.
///
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub struct FinalClassification {
    /// Finishing position
    pub position: u8,
    /// Number of laps completed
    pub num_laps: u8,
    /// Grid position of the car
    pub grid_position: u8,
    /// Number of points scored
    pub points: u8,
    /// Number of pit stops made
    pub num_pit_stops: u8,
    /// Result status
    pub result_status: ResultStatus,
    /// Best lap time of the session in milliseconds
    pub best_lap_time: u32,
    /// Total race time in milliseconds without penalties
    pub total_race_time: u32,
    /// Total penalties accumulated in seconds
    pub penalties_time: u8,
    /// Number of penalties applied to this driver
    pub num_penalties: u8,
    /// Number of tyres stints up to maximum
    pub num_tyre_stints: u8,
    /// Actual tyres used by this driver
    pub tyre_stints_actual: Vec<TyreCompound>,
    /// Visual tyres used by this driver
    pub tyre_stints_visual: Vec<TyreCompoundVisual>,
    /// The lap number stints end on
    pub tyre_stints_end_lap: Vec<u8>,
}

/// This packet details the final classification at the end of the race, and the data will match
/// with the post race results screen. This is especially useful for multiplayer games where it
/// is not always possible to send lap times on the final frame because of network delay.
///
/// Frequency: Once at the end of a race
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct PacketFinalClassificationData {
    /// Packet header
    pub header: PacketHeader,
    /// Number of cars in the final classification
    pub num_cars: u8,
    /// List of final classifications.
    pub final_classifications: Vec<FinalClassification>,
}
