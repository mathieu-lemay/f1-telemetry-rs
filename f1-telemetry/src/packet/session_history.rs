use serde::Serialize;

use crate::packet::generic::{TyreCompound, TyreCompoundVisual};
use crate::packet::header::PacketHeader;

/// This type is used for the `lap_history` array of the [`PacketSessionHistoryData`] type.
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub struct LapHistoryData {
    /// Lap time in milliseconds
    pub lap_time: u32,
    /// Sector 1 time in milliseconds
    pub sector_1_time: u16,
    /// Sector 1 whole minute part. New in F1 23.
    pub sector_1_time_minutes: u8,
    /// Sector 2 time in milliseconds
    pub sector_2_time: u16,
    /// Sector 2 whole minute part. New in F1 23.
    pub sector_2_time_minutes: u8,
    /// Sector 3 time in milliseconds
    pub sector_3_time: u16,
    /// Sector 3 whole minute part. New in F1 23.
    pub sector_3_time_minutes: u8,
    /// Bit flags specifying if the lap / sectors are valid
    pub valid_sectors: u8,
}

impl LapHistoryData {
    pub fn get_valid_sectors(&self) -> Vec<ValidSectorFlag> {
        let mask = self.valid_sectors;
        let mut sectors = Vec::new();

        if mask == 0 {
            return sectors;
        }

        if mask & (ValidSectorFlag::Lap as u8) > 0 {
            sectors.push(ValidSectorFlag::Lap);
        }
        if mask & (ValidSectorFlag::Sector1 as u8) > 0 {
            sectors.push(ValidSectorFlag::Sector1);
        }
        if mask & (ValidSectorFlag::Sector2 as u8) > 0 {
            sectors.push(ValidSectorFlag::Sector2);
        }
        if mask & (ValidSectorFlag::Sector3 as u8) > 0 {
            sectors.push(ValidSectorFlag::Sector3);
        }

        sectors
    }
}

/// Bit-mask values for the `lap_valid_flags` field in [`LapHistoryData`]
///
/// ## Specification
/// ```text
/// Bit Flag    Description
/// 0x01        Lap is valid
/// 0x02        Sector 1 is valid
/// 0x04        Sector 2 is valid
/// 0x08        Sector 3 is valid
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum ValidSectorFlag {
    Lap = 0x0001,
    Sector1 = 0x0002,
    Sector2 = 0x0004,
    Sector3 = 0x0008,
}

/// This type is used for the `tyre_stints` array of the [`PacketSessionHistoryData`] type.
///
/// See also [`TyreCompound`] and [`TyreCompoundVisual`].
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub struct TyreStintData {
    /// Lap the tyre usage ends on (255 if current tyre)
    pub end_lap: u8,
    /// Actual tyres used by this driver
    pub tyre_compound: TyreCompound,
    /// Visual tyres used by this driver
    pub tyre_compound_visual: TyreCompoundVisual,
}

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
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct PacketSessionHistoryData {
    /// Packet header
    pub header: PacketHeader,
    /// Index of the car this lap data relates to
    pub car_index: u8,
    /// Number laps in the data (including current partial lap)
    pub number_of_laps: u8,
    /// Number of tyre stints in the data
    pub number_of_tyre_stints: u8,
    /// Lap the best lap time was achieved on
    pub best_lap_time_lap_number: u8,
    /// Lap the best Sector 1 time was achieved on
    pub best_sector_1_lap_number: u8,
    /// Lap the best Sector 2 time was achieved on
    pub best_sector_2_lap_number: u8,
    /// Lap the best Sector 3 time was achieved on
    pub best_sector_3_lap_number: u8,
    /// List of lap history
    pub lap_history: Vec<LapHistoryData>,
    /// List of tyre stints
    pub tyre_stints: Vec<TyreStintData>,
}
