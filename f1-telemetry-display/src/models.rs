use f1_telemetry::packet::lap::ResultStatus;
use f1_telemetry::packet::participants::{Driver, Team};
use f1_telemetry::packet::session::SafetyCar;

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
