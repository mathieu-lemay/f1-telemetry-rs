use std::collections::BTreeMap;

use f1_telemetry::packet::car_status::TyreCompoundVisual;
use f1_telemetry::packet::generic::{WheelData, Flag};
use f1_telemetry::packet::lap::ResultStatus;
use f1_telemetry::packet::participants::{Driver, Team};
use f1_telemetry::packet::session::{SafetyCar, Weather};

pub struct EventInfo<'a> {
    pub timestamp: f32,
    pub description: &'a str,
    pub driver_name: Option<&'a str>,
    pub lap_time: Option<f32>,
}

pub struct LapInfo<'a> {
    pub position: u8,
    pub name: &'a str,
    pub driver: Driver,
    pub team: Team,
    pub current_lap_time: f32,
    pub last_lap_time: f32,
    pub best_lap_time: f32,
    pub status: ResultStatus,
    pub in_pit: bool,
    pub lap_invalid: bool,
    pub penalties: u8,
    pub lap_distance: f32,
    pub total_distance: f32,
}

pub struct SessionInfo<'a> {
    pub session_name: &'a str,
    pub track_name: &'a str,
    pub elapsed_time: u16,
    pub duration: u16,
    pub current_lap: u8,
    pub number_of_laps: u8,
    pub safety_car: SafetyCar,
}

pub struct TelemetryInfo {
    pub speed: u16,
    pub throttle: f32,
    pub brake: f32,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: bool,
    pub rev_lights_percent: u8,
    pub engine_temperature: u16,
}

pub struct CarStatus {
    pub tyres_damage: WheelData<u8>,
    pub left_front_wing_damage: u8,
    pub right_front_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub engine_damage: u8,
    pub gearbox_damage: u8,
    pub fuel_in_tank: f32,
    pub fuel_remaining_laps: f32,
    pub tyre_compound: TyreCompoundVisual,
}

pub struct RelativePositions {
    pub positions: BTreeMap<Team, Vec<f32>>,
    pub min: f32,
    pub max: f32,
}

pub struct WeatherInfo {
    pub weather: Weather,
    pub track_temperature: i8,
    pub air_temperature: i8,
}

pub struct ZoneFlag {
    pub zone_start: f32,
    pub zone_end: f32,
    pub flag: Flag
}