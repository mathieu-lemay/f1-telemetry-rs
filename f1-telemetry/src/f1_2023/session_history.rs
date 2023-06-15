use std::io::BufRead;

use serde::Deserialize;

use crate::f1_2022::generic::{unpack_tyre_compound, unpack_tyre_compound_visual};
use crate::packet::header::PacketHeader;
use crate::packet::session_history::{LapHistoryData, PacketSessionHistoryData, TyreStintData};
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

/// This packet contains lap times and tyre usage for the session.
///
/// This packet works slightly differently to other packets. To reduce CPU and bandwidth, each
/// packet relates to a specific vehicle and is sent every 1/20 s, and the vehicle being sent is
/// cycled through. Therefore in a 20 car race you should receive an update for each vehicle at
/// least once per second.
///
/// Note that at the end of the race, after the final classification packet has been sent, a final
/// bulk update of all the session histories for the vehicles in that session will be sent.
///
/// Frequency: 20 per second but cycling through cars
/// Size: 1155 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                   Header
/// car_index:                Index of the car this lap data relates to
/// number_of_laps:           Number laps in the data (including current partial lap)
/// number_of_tyre_stints:    Number of tyre stints in the data
/// best_lap_time_lap_number: Lap the best lap time was achieved on
/// best_sector_1_lap_number: Lap the best Sector 1 time was achieved on
/// best_sector_2_lap_number: Lap the best Sector 2 time was achieved on
/// best_sector_3_lap_number: Lap the best Sector 3 time was achieved on
/// lap_history:              List of lap history (100)
/// tyre_stints:              List of tyre stints (8)
/// ```
#[derive(Deserialize)]
struct RawSessionHistoryData {
    car_index: u8,
    number_of_laps: u8,
    number_of_tyre_stints: u8,
    best_lap_time_lap_number: u8,
    best_sector_1_lap_number: u8,
    best_sector_2_lap_number: u8,
    best_sector_3_lap_number: u8,
    lap_history_1: [RawLapHistoryData; 32], // FIXME: https://stackoverflow.com/a/62665880
    lap_history_2: [RawLapHistoryData; 32],
    lap_history_3: [RawLapHistoryData; 32],
    lap_history_4: [RawLapHistoryData; 4],
    tyre_stints: [RawTyreStintData; 8],
}

/// Description of a lap history entry
///
/// ## Specification
/// ```text
/// lap_time:        Lap time in milliseconds
/// sector_1_time:   Sector 1 time in milliseconds
/// sector_2_time:   Sector 2 time in milliseconds
/// sector_3_time:   Sector 3 time in milliseconds
/// lap_valid_flags: Bit flags specifying if the lap / sectors are valid
/// ```
#[derive(Deserialize)]
struct RawLapHistoryData {
    lap_time: u32,
    sector_1_time: u16,
    sector_2_time: u16,
    sector_3_time: u16,
    valid_sectors: u8,
}

impl TryFrom<&RawLapHistoryData> for LapHistoryData {
    type Error = UnpackError;

    fn try_from(lh: &RawLapHistoryData) -> Result<Self, Self::Error> {
        Ok(Self {
            lap_time: lh.lap_time,
            sector_1_time: lh.sector_1_time,
            sector_2_time: lh.sector_2_time,
            sector_3_time: lh.sector_3_time,
            valid_sectors: lh.valid_sectors,
        })
    }
}

/// Description of a lap history entry
///
/// ## Specification
/// ```text
/// end_lap:              Lap the tyre usage ends on (255 if current tyre)
/// tyre_compound:        Actual tyres used by this driver
/// tyre_compound_visual: Visual tyres used by this driver
/// ```
#[derive(Deserialize)]
struct RawTyreStintData {
    end_lap: u8,
    tyre_compound: u8,
    tyre_compound_visual: u8,
}

impl TryFrom<&RawTyreStintData> for TyreStintData {
    type Error = UnpackError;

    fn try_from(ts: &RawTyreStintData) -> Result<Self, Self::Error> {
        let tyre_compound = unpack_tyre_compound(ts.tyre_compound)?;
        let tyre_compound_visual = unpack_tyre_compound_visual(ts.tyre_compound_visual)?;

        Ok(Self {
            end_lap: ts.end_lap,
            tyre_compound,
            tyre_compound_visual,
        })
    }
}

pub(crate) fn parse_session_history_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketSessionHistoryData, UnpackError> {
    assert_packet_size(size, SESSION_HISTORY_PACKET_SIZE)?;

    let session_history_data: RawSessionHistoryData = bincode::deserialize_from(reader)?;

    let lap_history = session_history_data
        .lap_history_1
        .iter()
        .chain(session_history_data.lap_history_2.iter())
        .chain(session_history_data.lap_history_3.iter())
        .chain(session_history_data.lap_history_4.iter())
        .map(|lh| lh.try_into())
        .collect::<Result<Vec<LapHistoryData>, UnpackError>>()?;

    let tyre_stints: Vec<TyreStintData> = session_history_data
        .tyre_stints
        .iter()
        .map(|ts| ts.try_into())
        .collect::<Result<Vec<TyreStintData>, UnpackError>>()?;

    Ok(PacketSessionHistoryData {
        header,
        car_index: session_history_data.car_index,
        number_of_laps: session_history_data.number_of_laps,
        number_of_tyre_stints: session_history_data.number_of_tyre_stints,
        best_lap_time_lap_number: session_history_data.best_lap_time_lap_number,
        best_sector_1_lap_number: session_history_data.best_sector_1_lap_number,
        best_sector_2_lap_number: session_history_data.best_sector_2_lap_number,
        best_sector_3_lap_number: session_history_data.best_sector_3_lap_number,
        lap_history,
        tyre_stints,
    })
}
