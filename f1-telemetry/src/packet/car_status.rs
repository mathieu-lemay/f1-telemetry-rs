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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DRS {
    NotAllowed,
    Allowed,
    Unknown,
}

#[allow(clippy::upper_case_acronyms)]
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
///  - Restricted (Default): other players viewing the UDP data will not see values for your car;
///  - Public: all other players can see all the data for your car.
///
/// Note: You can always see the data for the car you are driving regardless of the setting.
///
/// The following data items are set to zero if the player driving the car in question has their "Your Telemetry" set to "Restricted":
///
///  - fuelInTank
///  - fuelCapacity
///  - fuelMix
///  - fuelRemainingLaps
///  - frontBrakeBias
///  - frontLeftWingDamage
///  - frontRightWingDamage
///  - rearWingDamage
///  - engineDamage
///  - gearBoxDamage
///  - tyresWear (All four wheels)
///  - tyresDamage (All four wheels)
///  - ersDeployMode
///  - ersStoreEnergy
///  - ersDeployedThisLap
///  - ersHarvestedThisLapMGUK
///  - ersHarvestedThisLapMGUH
///
/// Frequency: Rate as specified in menus
///
/// ## Specification
/// ```text
/// traction_control:            Traction control mode. See [`TractionControl`].
/// anti_lock_brakes:            ABS mode.
/// fuel_mix:                    Fuel mix. See [`FuelMix`].
/// front_brake_bias:            Front brake bias (percentage)
/// pit_limiter_status:          Pit limiter mode.
/// fuel_in_tank:                Current fuel mass.
/// fuel_capacity:               Fuel capacity.
/// fuel_remaining_laps:         Fuel remaining in terms of laps (value on MFD).
/// max_rpm:                     Cars max RPM, point of rev limiter.
/// idle_rpm:                    Cars idle RPM.
/// max_gears:                   Maximum number of gears.
/// drs_status:                  DRS status. See [`DRS`].
/// tyres_wear:                  Tyre wear percentage.
/// actual_tyre_compound:        Tyre compound used. See [`TyreCompound`].
/// tyre_visual_compound:        Visual representation of the tyre compound. See
///                              [`TyreCompoundVisual`].
/// tyres_damage:                Tyre damage (percentage).
/// front_left_wing_damage:      Front left wing damage (percentage).
/// front_right_wing_damage:     Front right wing damage (percentage).
/// rear_wing_damage:            Rear wing damage (percentage).
/// engine_damage:               Engine damage (percentage).
/// gear_box_damage:             Gear box damage (percentage).
/// vehicle_fia_flag:            Flag being shown to the car. See [`Flag`].
/// ers_store_energy:            ERS energy store in joules
/// ers_deploy_mode:             ERS deployment mode. See [`ERSDeployMode`]
/// ers_harvested_this_lap_mguk: ERS energy harvested this lap by MGU-k
/// ers_harvested_this_lap_mguh: ERS energy harvested this lap by MGU-h
/// ers_deployed_this_lap:       ERS energy deployed this lap
/// ```
///
/// See also: [`DRS`], [`ERSDeployMode`], [`Flag`], [`FuelMix`], [`TractionControl`], [`TyreCompoundVisual`], [`TyreCompound`]
#[derive(Debug, PartialEq, CopyGetters)]
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
    drs_status: DRS,
    drs_activation_distance: Option<u16>,
    tyres_wear: WheelData<u8>,
    actual_tyre_compound: TyreCompound,
    visual_tyre_compound: TyreCompoundVisual,
    tyre_age_laps: Option<u8>,
    tyres_damage: WheelData<u8>,
    front_left_wing_damage: u8,
    front_right_wing_damage: u8,
    rear_wing_damage: u8,
    drs_fault: bool,
    engine_damage: u8,
    gear_box_damage: u8,
    vehicle_fia_flag: Flag,
    ers_store_energy: f32,
    ers_deploy_mode: ERSDeployMode,
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployed_this_lap: f32,
}

#[allow(clippy::too_many_arguments)]
impl CarStatusData {
    pub fn new(
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
        drs_status: DRS,
        drs_activation_distance: Option<u16>,
        tyres_wear: WheelData<u8>,
        actual_tyre_compound: TyreCompound,
        visual_tyre_compound: TyreCompoundVisual,
        tyre_age_laps: Option<u8>,
        tyres_damage: WheelData<u8>,
        front_left_wing_damage: u8,
        front_right_wing_damage: u8,
        rear_wing_damage: u8,
        drs_fault: bool,
        engine_damage: u8,
        gear_box_damage: u8,
        vehicle_fia_flag: Flag,
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
            drs_status,
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
            vehicle_fia_flag,
            ers_store_energy,
            ers_deploy_mode,
            ers_harvested_this_lap_mguk,
            ers_harvested_this_lap_mguh,
            ers_deployed_this_lap,
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
        vehicle_fia_flag: Flag,
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
            drs_status: drs_allowed,
            drs_activation_distance: Some(drs_activation_distance),
            tyres_wear,
            actual_tyre_compound,
            visual_tyre_compound,
            tyre_age_laps: Some(tyre_age_laps),
            tyres_damage,
            front_left_wing_damage,
            front_right_wing_damage,
            rear_wing_damage,
            drs_fault,
            engine_damage,
            gear_box_damage,
            vehicle_fia_flag,
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
#[derive(Debug, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct PacketCarStatusData {
    header: PacketHeader,
    car_status_data: Vec<CarStatusData>,
}

impl PacketCarStatusData {
    pub fn new(header: PacketHeader, car_status_data: Vec<CarStatusData>) -> PacketCarStatusData {
        PacketCarStatusData {
            header,
            car_status_data,
        }
    }
}
