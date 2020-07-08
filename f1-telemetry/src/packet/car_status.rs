use getset::{CopyGetters, Getters};

use crate::packet::generic::{Flag, TyreCompound, TyreCompoundVisual, WheelData};

use super::header::PacketHeader;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TractionControl {
    Off,
    Low,
    High,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FuelMix {
    Lean,
    Standard,
    Rich,
    Max,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DRS {
    NotAllowed,
    Allowed,
    Unknown,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ERSDeployMode {
    None,
    Low,
    Medium,
    High,
    Overtake,
    Hotlap,
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
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
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
    drs_activation_distance: u16,
    tyres_wear: WheelData<u8>,
    actual_tyre_compound: TyreCompound,
    visual_tyre_compound: TyreCompoundVisual,
    tyre_age_laps: u8,
    tyres_damage: WheelData<u8>,
    front_left_wing_damage: u8,
    front_right_wing_damage: u8,
    rear_wing_damage: u8,
    drs_fault: bool,
    engine_damage: u8,
    gear_box_damage: u8,
    vehicle_fia_flags: Flag,
    ers_store_energy: f32,
    ers_deploy_mode: ERSDeployMode,
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployed_this_lap: f32,
}

#[allow(clippy::too_many_arguments)]
impl CarStatusData {
    pub(crate) fn from_2019(
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
    ) -> CarStatusData {
        CarStatusData {
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
            drs_activation_distance: 0,
            tyre_age_laps: 0,
            drs_fault: false,
        }
    }

    pub(crate) fn from_2020(
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
        drs_activation_distance: u16,
        tyres_wear: WheelData<u8>,
        actual_tyre_compound: TyreCompound,
        visual_tyre_compound: TyreCompoundVisual,
        tyre_age_laps: u8,
        tyres_damage: WheelData<u8>,
        front_left_wing_damage: u8,
        front_right_wing_damage: u8,
        rear_wing_damage: u8,
        drs_fault: bool,
        engine_damage: u8,
        gear_box_damage: u8,
        vehicle_fia_flags: Flag,
        ers_store_energy: f32,
        ers_deploy_mode: ERSDeployMode,
        ers_harvested_this_lap_mguk: f32,
        ers_harvested_this_lap_mguh: f32,
        ers_deployed_this_lap: f32,
    ) -> CarStatusData {
        CarStatusData {
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
            drs_activation_distance,
            tyres_wear,
            actual_tyre_compound,
            visual_tyre_compound,
            tyre_age_laps,
            tyres_damage,
            front_left_wing_damage,
            front_right_wing_damage,
            rear_wing_damage,
            drs_fault,
            engine_damage,
            gear_box_damage,
            vehicle_fia_flags,
            ers_store_energy,
            ers_deploy_mode,
            ers_harvested_this_lap_mguk,
            ers_harvested_this_lap_mguh,
            ers_deployed_this_lap,
        }
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
    pub(crate) fn new(
        header: PacketHeader,
        car_status_data: Vec<CarStatusData>,
    ) -> PacketCarStatusData {
        PacketCarStatusData {
            header,
            car_status_data,
        }
    }
}
