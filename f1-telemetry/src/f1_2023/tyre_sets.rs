use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::tyre_sets::{PacketTyreSetsData, TyreSetData};
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;
use super::generic::{unpack_session_type, unpack_tyre_compound, unpack_tyre_compound_visual};

/// This packet gives more in-depth details about tyre sets assigned to a vehicle during the session.
///
/// Frequency: 20 per second but cycling through cars
/// Size: 231 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// car_index:   Index of the car this lap data relates to
/// lap_history: List of tyre sets
/// fittex_idx:  Index into array of fitted tyre
/// ```
#[derive(Deserialize)]
struct RawTyreSetsData {
    car_index: u8,
    lap_history: [RawTyreSet; NUMBER_TYRE_SETS],
    fittex_idx: u8,
}

/// ## Specification
/// ```text
/// actual_tyre_compound: Actual tyre compound used
/// visual_tyre_compound: Visual tyre compound used
/// wear:                 Tyre wear (percentage)
/// available:            Whether this set is currently available
/// recommended_session:  Recommended session for tyre set
/// life_span:            Laps left in this tyre set
/// usable_life:          Max number of laps recommended for this compound
/// lap_delta_time:       Lap delta time in milliseconds compared to fitted set
/// fitted:               Whether the set is fitted or not
/// ```
#[derive(Deserialize)]
struct RawTyreSet {
    actual_tyre_compound: u8,
    visual_tyre_compound: u8,
    wear: u8,
    available: bool,
    recommended_session: u8,
    life_span: u8,
    usable_life: u8,
    lap_delta_time: u16,
    fitted: bool,
}

impl TryFrom<&RawTyreSet> for TyreSetData {
    type Error = UnpackError;

    fn try_from(tyre_set: &RawTyreSet) -> Result<Self, Self::Error> {
        let tyre_compound = unpack_tyre_compound(tyre_set.actual_tyre_compound)?;
        let tyre_compound_visual = unpack_tyre_compound_visual(tyre_set.visual_tyre_compound)?;
        let recommended_session = unpack_session_type(tyre_set.recommended_session)?;

        Ok(Self {
            tyre_compound,
            tyre_compound_visual,
            wear_pct: tyre_set.wear,
            is_available: tyre_set.available,
            recommended_session,
            laps_left: tyre_set.life_span,
            usable_life: tyre_set.usable_life,
            lap_delta_time: tyre_set.lap_delta_time,
            is_fitted: tyre_set.fitted,
        })
    }
}

pub(crate) fn parse_tyre_sets_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketTyreSetsData, UnpackError> {
    assert_packet_size(size, TYRE_SETS_PACKET_SIZE)?;

    let tyre_set_data: RawTyreSetsData = bincode::deserialize_from(reader)?;
    let lap_history: Vec<TyreSetData> = tyre_set_data
        .lap_history
        .iter()
        .map(|l| l.try_into())
        .collect::<Result<Vec<TyreSetData>, UnpackError>>()?;

    Ok(PacketTyreSetsData {
        header,
        car_index: tyre_set_data.car_index,
        lap_history,
        fitted_idx: tyre_set_data.fittex_idx,
    })
}
