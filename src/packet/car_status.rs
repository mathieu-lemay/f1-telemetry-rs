use byteorder::{LittleEndian, ReadBytesExt};
use getset::Getters;
use std::convert::TryFrom;
use std::io::BufRead;

use super::header::PacketHeader;
use crate::packet::generic::{Flag, WheelData};
use crate::packet::UnpackError;

#[derive(Debug)]
pub enum TractionControl {
    Off,
    Low,
    High,
}

impl TryFrom<u8> for TractionControl {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TractionControl::Off),
            1 => Ok(TractionControl::Low),
            2 => Ok(TractionControl::High),
            _ => Err(UnpackError(format!(
                "Invalid TractionControl value: {}",
                value
            ))),
        }
    }
}

#[derive(Debug)]
pub enum FuelMix {
    Lean,
    Standard,
    Rich,
    Max,
}

impl TryFrom<u8> for FuelMix {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FuelMix::Lean),
            1 => Ok(FuelMix::Standard),
            2 => Ok(FuelMix::Rich),
            3 => Ok(FuelMix::Max),
            _ => Err(UnpackError(format!("Invalid FuelMix value: {}", value))),
        }
    }
}

#[derive(Debug)]
pub enum DRS {
    NotAllowed,
    Allowed,
    Unknown,
}

impl TryFrom<i8> for DRS {
    type Error = UnpackError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DRS::NotAllowed),
            1 => Ok(DRS::Allowed),
            -1 => Ok(DRS::Unknown),
            _ => Err(UnpackError(format!("Invalid DRS value: {}", value))),
        }
    }
}

#[derive(Debug)]
pub enum TyreCompound {
    C5,
    C4,
    C3,
    C2,
    C1,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
}

impl TryFrom<u8> for TyreCompound {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            16 => Ok(TyreCompound::C5),
            17 => Ok(TyreCompound::C4),
            18 => Ok(TyreCompound::C3),
            19 => Ok(TyreCompound::C2),
            20 => Ok(TyreCompound::C1),
            7 => Ok(TyreCompound::Inter),
            8 => Ok(TyreCompound::Wet),
            9 => Ok(TyreCompound::ClassicDry),
            10 => Ok(TyreCompound::ClassicWet),
            11 => Ok(TyreCompound::F2SuperSoft),
            12 => Ok(TyreCompound::F2Soft),
            13 => Ok(TyreCompound::F2Medium),
            14 => Ok(TyreCompound::F2Hard),
            15 => Ok(TyreCompound::F2Wet),
            _ => Err(UnpackError(format!(
                "Invalid TyreCompound value: {}",
                value
            ))),
        }
    }
}

#[derive(Debug)]
pub enum TyreCompoundVisual {
    Soft,
    Medium,
    Hard,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
}

impl TryFrom<u8> for TyreCompoundVisual {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            16 => Ok(TyreCompoundVisual::Soft),
            17 => Ok(TyreCompoundVisual::Medium),
            18 => Ok(TyreCompoundVisual::Hard),
            7 => Ok(TyreCompoundVisual::Inter),
            8 => Ok(TyreCompoundVisual::Wet),
            9 => Ok(TyreCompoundVisual::ClassicDry),
            10 => Ok(TyreCompoundVisual::ClassicWet),
            11 => Ok(TyreCompoundVisual::F2SuperSoft),
            12 => Ok(TyreCompoundVisual::F2Soft),
            13 => Ok(TyreCompoundVisual::F2Medium),
            14 => Ok(TyreCompoundVisual::F2Hard),
            15 => Ok(TyreCompoundVisual::F2Wet),
            _ => Err(UnpackError(format!(
                "Invalid TyreCompoundVisual value: {}",
                value
            ))),
        }
    }
}

#[derive(Debug)]
pub enum ERSDeployMode {
    None,
    Low,
    Medium,
    High,
    Overtake,
    Hotlap,
}

impl TryFrom<u8> for ERSDeployMode {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ERSDeployMode::None),
            1 => Ok(ERSDeployMode::Low),
            2 => Ok(ERSDeployMode::Medium),
            3 => Ok(ERSDeployMode::High),
            4 => Ok(ERSDeployMode::Overtake),
            5 => Ok(ERSDeployMode::Hotlap),
            _ => Err(UnpackError(format!(
                "Invalid ERSDeployMode value: {}",
                value
            ))),
        }
    }
}

/// This type is used for the 20-element `car_status_data` array of the [`PacketCarStatusData`] type.
///
/// There is some data in the Car Status packets that you may not want other players seeing if you are in a multiplayer game.
/// This is controlled by the "Your Telemetry" setting in the Telemetry options. The options are:
///
///     Restricted (Default) – other players viewing the UDP data will not see values for your car;
///     Public – all other players can see all the data for your car.
///
/// Note: You can always see the data for the car you are driving regardless of the setting.
///
/// The following data items are set to zero if the player driving the car in question has their "Your Telemetry" set to "Restricted":
///
///     fuelInTank
///     fuelCapacity
///     fuelMix
///     fuelRemainingLaps
///     frontBrakeBias
///     frontLeftWingDamage
///     frontRightWingDamage
///     rearWingDamage
///     engineDamage
///     gearBoxDamage
///     tyresWear (All four wheels)
///     tyresDamage (All four wheels)
///     ersDeployMode
///     ersStoreEnergy
///     ersDeployedThisLap
///     ersHarvestedThisLapMGUK
///     ersHarvestedThisLapMGUH
///
/// ## Specification
/// ```text
/// traction_control:            0 (off) - 2 (high)
/// anti_lock_brakes:            0 (off) - 1 (on)
/// fuel_mix:                    fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
/// front_brake_bias:            front brake bias (percentage)
/// pit_limiter_status:          pit limiter status - 0 = off, 1 = on
/// fuel_in_tank:                current fuel mass
/// fuel_capacity:               fuel capacity
/// fuel_remaining_laps:         fuel remaining in terms of laps (value on MFD)
/// max_rpm:                     cars max RPM, point of rev limiter
/// idle_rpm:                    cars idle RPM
/// max_gears:                   maximum number of gears
/// drs_allowed:                 0 = not allowed, 1 = allowed, -1 = unknown
/// tyres_wear:                  tyre wear percentage
/// actual_tyre_compound:        f1 modern - 16 = c5, 17 = c4, 18 = c3, 19 = c2, 20 = c1
///                              7 = inter, 8 = wet
///                              f1 classic - 9 = dry, 10 = wet
///                              f2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
///                              15 = wet
/// tyre_visual_compound:        f1 visual (can be different from actual compound)
///                              16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
///                              f1 classic – same as above
///                              f2 – same as above
/// tyres_damage:                tyre damage (percentage)
/// front_left_wing_damage:      front left wing damage (percentage)
/// front_right_wing_damage:     front right wing damage (percentage)
/// rear_wing_damage:            rear wing damage (percentage)
/// engine_damage:               engine damage (percentage)
/// gear_box_damage:             gear box damage (percentage)
/// vehicle_fia_flags:           -1 = invalid/unknown, 0 = none, 1 = green
///                              2 = blue, 3 = yellow, 4 = red
/// ers_store_energy:            ERS energy store in joules
/// ers_deploy_mode:             ERS deployment mode, 0 = none, 1 = low, 2 = medium
///                              3 = high, 4 = overtake, 5 = hotlap
/// ers_harvested_this_lap_mguk: ERS energy harvested this lap by MGU-k
/// ers_harvested_this_lap_mguh: ERS energy harvested this lap by MGU-h
/// ers_deployed_this_lap:       ERS energy deployed this lap
/// ```
///
/// [`PacketCarStatusData`]: ./struct.CarStatusData.html
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CarStatusData {
    traction_control: TractionControl,
    anti_lock_brakes: bool,
    fuel_mix: FuelMix,
    front_brake_bias: u8,
    pit_limiter: bool,
    fuel_in_tank: f32,
    fuel_capacity: f32,
    fuel_remaining_laps: f32,
    max_rpm: u16,
    idle_rpm: u16,
    max_gears: u8,
    drs_allowed: DRS,
    tyres_wear: WheelData<u8>,
    actual_tyre_compound: TyreCompound,
    visual_tyre_compound: TyreCompoundVisual,
    tyres_damage: WheelData<u8>,
    front_left_wing_damage: u8,
    front_right_wing_damage: u8,
    rear_wing_damage: u8,
    engine_damage: u8,
    gear_box_damage: u8,
    vehicle_fia_flags: Flag,
    ers_store_energy: f32,
    ers_deploy_mode: ERSDeployMode,
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployed_this_lap: f32,
}

impl CarStatusData {
    pub fn new<T: BufRead>(reader: &mut T) -> Result<CarStatusData, UnpackError> {
        let traction_control = TractionControl::try_from(reader.read_u8().unwrap())?;
        let anti_lock_brakes = reader.read_u8().unwrap() == 1;
        let fuel_mix = FuelMix::try_from(reader.read_u8().unwrap())?;
        let front_brake_bias = reader.read_u8().unwrap();
        let pit_limiter = reader.read_u8().unwrap() == 1;
        let fuel_in_tank = reader.read_f32::<LittleEndian>().unwrap();
        let fuel_capacity = reader.read_f32::<LittleEndian>().unwrap();
        let fuel_remaining_laps = reader.read_f32::<LittleEndian>().unwrap();
        let max_rpm = reader.read_u16::<LittleEndian>().unwrap();
        let idle_rpm = reader.read_u16::<LittleEndian>().unwrap();
        let max_gears = reader.read_u8().unwrap();
        let drs_allowed = DRS::try_from(reader.read_i8().unwrap())?;
        let tyres_wear = WheelData::new(
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
        );
        let actual_tyre_compound = TyreCompound::try_from(reader.read_u8().unwrap())?;
        let visual_tyre_compound = TyreCompoundVisual::try_from(reader.read_u8().unwrap())?;
        let tyres_damage = WheelData::new(
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
        );
        let front_left_wing_damage = reader.read_u8().unwrap();
        let front_right_wing_damage = reader.read_u8().unwrap();
        let rear_wing_damage = reader.read_u8().unwrap();
        let engine_damage = reader.read_u8().unwrap();
        let gear_box_damage = reader.read_u8().unwrap();
        let vehicle_fia_flags = Flag::try_from(reader.read_i8().unwrap())?;
        let ers_store_energy = reader.read_f32::<LittleEndian>().unwrap();
        let ers_deploy_mode = ERSDeployMode::try_from(reader.read_u8().unwrap())?;
        let ers_harvested_this_lap_mguk = reader.read_f32::<LittleEndian>().unwrap();
        let ers_harvested_this_lap_mguh = reader.read_f32::<LittleEndian>().unwrap();
        let ers_deployed_this_lap = reader.read_f32::<LittleEndian>().unwrap();

        Ok(CarStatusData {
            traction_control,
            anti_lock_brakes,
            fuel_mix,
            front_brake_bias,
            pit_limiter,
            fuel_in_tank,
            fuel_capacity,
            fuel_remaining_laps,
            max_rpm,
            idle_rpm,
            max_gears,
            drs_allowed,
            tyres_wear,
            actual_tyre_compound,
            visual_tyre_compound,
            tyres_damage,
            front_left_wing_damage,
            front_right_wing_damage,
            rear_wing_damage,
            engine_damage,
            gear_box_damage,
            vehicle_fia_flags,
            ers_store_energy,
            ers_deploy_mode,
            ers_harvested_this_lap_mguk,
            ers_harvested_this_lap_mguh,
            ers_deployed_this_lap,
        })
    }
}

/// This packet details car statuses for all the cars in the race. It includes values such as the damage readings on the car.
///
/// Frequency: Rate as specified in menus
///
/// Size: 1143 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:          Header
/// car_status_data: List of cars (20)
/// ```
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct PacketCarStatusData {
    header: PacketHeader,
    car_status_data: Vec<CarStatusData>,
}

impl PacketCarStatusData {
    pub fn new<T: BufRead>(
        mut reader: &mut T,
        header: PacketHeader,
    ) -> Result<PacketCarStatusData, UnpackError> {
        let mut car_status_data = Vec::with_capacity(20);
        for _ in 0..20 {
            let csd = CarStatusData::new(&mut reader)?;
            car_status_data.push(csd);
        }

        Ok(PacketCarStatusData {
            header,
            car_status_data,
        })
    }
}
