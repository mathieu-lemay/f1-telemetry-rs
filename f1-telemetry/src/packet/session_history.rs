use crate::packet::generic::{TyreCompound, TyreCompoundVisual};
use crate::packet::header::PacketHeader;

/// This type is used for the `lap_history` array of the [`PacketSessionHistoryData`] type.
///
/// ## Specification
/// ```text
/// lap_time:        Lap time in milliseconds
/// sector_1_time:   Sector 1 time in milliseconds
/// sector_2_time:   Sector 2 time in milliseconds
/// sector_3_time:   Sector 3 time in milliseconds
/// lap_valid_flags: Bit flags specifying if the lap / sectors are valid
/// ```
#[derive(Debug, Default, Eq, PartialEq)]
pub struct LapHistoryData {
    pub lap_time: u32,
    pub sector_1_time: u16,
    pub sector_2_time: u16,
    pub sector_3_time: u16,
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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValidSectorFlag {
    Lap = 0x0001,
    Sector1 = 0x0002,
    Sector2 = 0x0004,
    Sector3 = 0x0008,
}

/// This type is used for the `tyre_stints` array of the [`PacketSessionHistoryData`] type.
///
/// ## Specification
/// ```text
/// end_lap:              Lap the tyre usage ends on (255 if current tyre)
/// tyre_compound:        Actual tyres used by this driver
/// tyre_compound_visual: Visual tyres used by this driver
/// ```
/// See also [`TyreCompound`] and [`TyreCompoundVisual`].
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct TyreStintData {
    pub end_lap: u8,
    pub tyre_compound: TyreCompound,
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
/// lap_history:              List of lap history
/// tyre_stints:              List of tyre stints
/// ```
#[derive(Debug, PartialEq)]
pub struct PacketSessionHistoryData {
    pub header: PacketHeader,
    pub car_index: u8,
    pub number_of_laps: u8,
    pub number_of_tyre_stints: u8,
    pub best_lap_time_lap_number: u8,
    pub best_sector_1_lap_number: u8,
    pub best_sector_2_lap_number: u8,
    pub best_sector_3_lap_number: u8,
    pub lap_history: Vec<LapHistoryData>,
    pub tyre_stints: Vec<TyreStintData>,
}
