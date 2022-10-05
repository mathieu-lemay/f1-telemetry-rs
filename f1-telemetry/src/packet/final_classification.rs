use serde::Serialize;

use crate::packet::generic::{ResultStatus, TyreCompound, TyreCompoundVisual};

use super::header::PacketHeader;

/// This type is used for the `classification_data` array of the [`PacketFinalClassificationData`] type.
///
/// ## Specification
/// ```text
/// position:           Finishing position
/// num_laps:           Number of laps completed
/// grid_position:      Grid position of the car
/// points:             Number of points scored
/// num_pit_stops:      Number of pit stops made
/// result_status:      Result status
/// best_lap_time:      Best lap time of the session in milliseconds
/// total_race_time:    Total race time in milliseconds without penalties
/// penalties_time:     Total penalties accumulated in seconds
/// num_penalties:      Number of penalties applied to this driver
/// num_tyre_stints:    Number of tyres stints up to maximum
/// tyre_stints_actual: Actual tyres used by this driver
/// tyre_stints_visual: Visual tyres used by this driver
/// tyre_stints_end_lap: The lap number stints end on
/// ```
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub struct FinalClassification {
    pub position: u8,
    pub num_laps: u8,
    pub grid_position: u8,
    pub points: u8,
    pub num_pit_stops: u8,
    pub result_status: ResultStatus,
    pub best_lap_time: u32,
    pub total_race_time: u32,
    pub penalties_time: u8,
    pub num_penalties: u8,
    pub num_tyre_stints: u8,
    pub tyre_stints_actual: Vec<TyreCompound>,
    pub tyre_stints_visual: Vec<TyreCompoundVisual>,
    pub tyre_stints_end_lap: Vec<u8>,
}

/// This packet details the final classification at the end of the race, and the data will match
/// with the post race results screen. This is especially useful for multiplayer games where it
/// is not always possible to send lap times on the final frame because of network delay.
///
/// Frequency: Once at the end of a race
///
/// ## Specification
/// ```text
/// header:                Header
/// num_cars:              Number of cars in the final classification
/// final_classifications: List of final classifications.
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct PacketFinalClassificationData {
    pub header: PacketHeader,
    pub num_cars: u8,
    pub final_classifications: Vec<FinalClassification>,
}
