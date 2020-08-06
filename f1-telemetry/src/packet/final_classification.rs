use getset::{CopyGetters, Getters};

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
/// best_lap_time:      Best lap time of the session in seconds
/// total_race_time:    Total race time in seconds without penalties
/// penalties_time:     Total penalties accumulated in seconds
/// num_penalties:      Number of penalties applied to this driver
/// num_tyre_stints:    Number of tyres stints up to maximum
/// tyre_stints_actual: Actual tyres used by this driver
/// tyre_stints_visual: Visual tyres used by this driver
/// ```
/// [`PacketFinalClassificationData`]: ./struct.PacketFinalClassificationData.html
#[derive(Debug, Getters, CopyGetters)]
pub struct FinalClassification {
    #[getset(get_copy = "pub")]
    position: u8,
    #[getset(get_copy = "pub")]
    num_laps: u8,
    #[getset(get_copy = "pub")]
    grid_position: u8,
    #[getset(get_copy = "pub")]
    points: u8,
    #[getset(get_copy = "pub")]
    num_pit_stops: u8,
    #[getset(get_copy = "pub")]
    result_status: ResultStatus,
    #[getset(get_copy = "pub")]
    best_lap_time: u32,
    #[getset(get_copy = "pub")]
    total_race_time: u32,
    #[getset(get_copy = "pub")]
    penalties_time: u8,
    #[getset(get_copy = "pub")]
    num_penalties: u8,
    #[getset(get_copy = "pub")]
    num_tyre_stints: u8,
    #[getset(get = "pub")]
    tyre_stints_actual: Vec<TyreCompound>,
    #[getset(get = "pub")]
    tyre_stints_visual: Vec<TyreCompoundVisual>,
}

impl FinalClassification {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        position: u8,
        num_laps: u8,
        grid_position: u8,
        points: u8,
        num_pit_stops: u8,
        result_status: ResultStatus,
        best_lap_time: u32,
        total_race_time: u32,
        penalties_time: u8,
        num_penalties: u8,
        num_tyre_stints: u8,
        tyre_stints_actual: Vec<TyreCompound>,
        tyre_stints_visual: Vec<TyreCompoundVisual>,
    ) -> Self {
        Self {
            position,
            num_laps,
            grid_position,
            points,
            num_pit_stops,
            result_status,
            best_lap_time,
            total_race_time,
            penalties_time,
            num_penalties,
            num_tyre_stints,
            tyre_stints_actual,
            tyre_stints_visual,
        }
    }
}

/// This packet details the final classification at the end of the race, and the data will match
/// with the post race results screen. This is especially useful for multiplayer games where it
/// is not always possible to send lap times on the final frame because of network delay.
///
/// ## Specification
/// ```text
/// header:                Header
/// num_cars:              Number of cars in the final classification
/// final_classifications: List of final classifications.
/// ```
#[derive(Debug, CopyGetters, Getters)]
pub struct PacketFinalClassificationData {
    #[getset(get = "pub")]
    header: PacketHeader,
    #[getset(get_copy = "pub")]
    num_cars: u8,
    #[getset(get = "pub")]
    final_classifications: Vec<FinalClassification>,
}

impl PacketFinalClassificationData {
    pub(crate) fn new(
        header: PacketHeader,
        num_cars: u8,
        final_classifications: Vec<FinalClassification>,
    ) -> Self {
        Self {
            header,
            num_cars,
            final_classifications,
        }
    }
}
