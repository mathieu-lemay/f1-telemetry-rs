use serde::Serialize;

use crate::packet::generic::{Flag, TyreCompound, TyreCompoundVisual, WheelData};

use super::header::PacketHeader;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum TractionControl {
    #[default]
    Off,
    Low,
    High,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum FuelMix {
    Lean,
    #[default]
    Standard,
    Rich,
    Max,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum DRS {
    NotAllowed,
    Allowed,
    #[default]
    Unknown,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum ERSDeployMode {
    #[default]
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
/// actual_tyre_compound:        Tyre compound used. See [`TyreCompound`].
/// tyre_visual_compound:        Visual representation of the tyre compound. See
///                              [`TyreCompoundVisual`].
/// vehicle_fia_flag:            Flag being shown to the car. See [`Flag`].
/// engine_power_ice:            Engine power output of ICE (W). New in F1 23.
/// engine_power_mguk:           Engine power output of MGU-K (W). New in F1 23.
/// ers_store_energy:            ERS energy store in joules
/// ers_deploy_mode:             ERS deployment mode. See [`ERSDeployMode`]
/// ers_harvested_this_lap_mguk: ERS energy harvested this lap by MGU-k
/// ers_harvested_this_lap_mguh: ERS energy harvested this lap by MGU-h
/// ers_deployed_this_lap:       ERS energy deployed this lap
/// network_paused:              Wether the car is paused in a network game
///
/// F1 2019 and F1 2020 only
/// tyres_wear:                  Tyre wear percentage.
/// tyres_damage:                Tyre damage (percentage).
/// front_left_wing_damage:      Front left wing damage (percentage).
/// front_right_wing_damage:     Front right wing damage (percentage).
/// rear_wing_damage:            Rear wing damage (percentage).
/// engine_damage:               Engine damage (percentage).
/// gear_box_damage:             Gear box damage (percentage).
/// ```
///
/// See also: [`DRS`], [`ERSDeployMode`], [`Flag`], [`FuelMix`], [`TractionControl`], [`TyreCompoundVisual`], [`TyreCompound`]
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct CarStatusData {
    pub traction_control: TractionControl,
    pub anti_lock_brakes: bool,
    pub fuel_mix: FuelMix,
    pub front_brake_bias: u8,
    pub pit_limiter: bool,
    pub fuel_in_tank: f32,
    pub fuel_capacity: f32,
    pub fuel_remaining_laps: f32,
    pub max_rpm: u16,
    pub idle_rpm: u16,
    pub max_gears: u8,
    pub drs_status: DRS,
    pub drs_activation_distance: Option<u16>,
    pub actual_tyre_compound: TyreCompound,
    pub visual_tyre_compound: TyreCompoundVisual,
    pub tyre_age_laps: Option<u8>,
    pub vehicle_fia_flag: Flag,
    pub engine_power_ice: Option<f32>,
    pub engine_power_mguk: Option<f32>,
    pub ers_store_energy: f32,
    pub ers_deploy_mode: ERSDeployMode,
    pub ers_harvested_this_lap_mguk: f32,
    pub ers_harvested_this_lap_mguh: f32,
    pub ers_deployed_this_lap: f32,
    pub network_paused: bool,
    pub tyres_wear: Option<WheelData<u8>>,
    pub tyres_damage: Option<WheelData<u8>>,
    pub front_left_wing_damage: Option<u8>,
    pub front_right_wing_damage: Option<u8>,
    pub rear_wing_damage: Option<u8>,
    pub drs_fault: Option<bool>,
    pub engine_damage: Option<u8>,
    pub gear_box_damage: Option<u8>,
}

/// This packet details car statuses for all the cars in the race. It includes values such as the damage readings on the car.
///
/// Frequency: Rate as specified in menus
///
/// ## Specification
/// ```text
/// header:          Header
/// car_status_data: List of cars
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketCarStatusData {
    pub header: PacketHeader,
    pub car_status_data: Vec<CarStatusData>,
}
