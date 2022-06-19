use std::io::BufRead;

use serde::Deserialize;

use crate::f1_2020::generic::{
    unpack_result_status, unpack_tyre_compound, unpack_tyre_compound_visual,
};
use crate::packet::final_classification::{FinalClassification, PacketFinalClassificationData};
use crate::packet::generic::{TyreCompound, TyreCompoundVisual};
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, seconds_to_millis};

use super::consts::*;

/// This packet details the final classification at the end of the race, and the data will match
/// with the post race results screen. This is especially useful for multiplayer games where it
/// is not always possible to send lap times on the final frame because of network delay.
///
/// Frequency: Once at the end of a race
/// Size: 839 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                Header
/// num_cars:              Number of cars in the final classification
/// final_classifications: List of final classifications.
/// ```
#[derive(Deserialize)]
struct RawFinalClassificationData {
    num_cars: u8,
    final_classifications: [RawFinalClassification; NUMBER_CARS],
}

/// This type is used for the `classification_data` array of the [`PacketFinalClassificationData`] type.
///
/// ## Specification
/// ```text
/// position:           Finishing position
/// num_laps:           Number of laps completed
/// grid_position:      Grid position of the car
/// points:             Number of points scored
/// num_pit_stops:      Number of pit stops made
/// result_status:      Result status - 0 = invalid, 1 = inactive, 2 = active
///                     3 = finished, 4 = disqualified, 5 = not classified
///                     6 = retired
/// best_lap_time:      Best lap time of the session in seconds
/// total_race_time:    Total race time in seconds without penalties
/// penalties_time:     Total penalties accumulated in seconds
/// num_penalties:      Number of penalties applied to this driver
/// num_tyre_stints:    Number of tyres stints up to maximum
/// tyre_stints_actual: Actual tyres used by this driver
/// tyre_stints_visual: Visual tyres used by this driver
/// ```
#[derive(Deserialize)]
struct RawFinalClassification {
    position: u8,
    num_laps: u8,
    grid_position: u8,
    points: u8,
    num_pit_stops: u8,
    result_status: u8,
    best_lap_time: f32,
    total_race_time: f64,
    penalties_time: u8,
    num_penalties: u8,
    num_tyre_stints: u8,
    tyre_stints_actual: [u8; 8],
    tyre_stints_visual: [u8; 8],
}

impl FinalClassification {
    fn from_2020(fc: &RawFinalClassification) -> Result<Self, UnpackError> {
        let result_status = unpack_result_status(fc.result_status)?;
        let best_lap_time = seconds_to_millis(fc.best_lap_time as f64);
        let total_race_time = seconds_to_millis(fc.total_race_time);

        let tyre_stints_actual = fc
            .tyre_stints_actual
            .iter()
            .map(|&t| unpack_tyre_compound(t))
            .collect::<Result<Vec<TyreCompound>, UnpackError>>()?;

        let tyre_stints_visual = fc
            .tyre_stints_visual
            .iter()
            .map(|&t| unpack_tyre_compound_visual(t))
            .collect::<Result<Vec<TyreCompoundVisual>, UnpackError>>()?;

        Ok(FinalClassification::new(
            fc.position,
            fc.num_laps,
            fc.grid_position,
            fc.points,
            fc.num_pit_stops,
            result_status,
            best_lap_time,
            total_race_time,
            fc.penalties_time,
            fc.num_penalties,
            fc.num_tyre_stints,
            tyre_stints_actual,
            tyre_stints_visual,
        ))
    }
}

pub(crate) fn parse_final_classification_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketFinalClassificationData, UnpackError> {
    assert_packet_size(size, FINAL_CLASSIFICATION_PACKET_SIZE)?;

    let final_classification: RawFinalClassificationData = bincode::deserialize_from(reader)?;

    let final_classifications = final_classification
        .final_classifications
        .iter()
        .map(FinalClassification::from_2020)
        .collect::<Result<Vec<FinalClassification>, UnpackError>>()?;

    Ok(PacketFinalClassificationData::new(
        header,
        final_classification.num_cars,
        final_classifications,
    ))
}
