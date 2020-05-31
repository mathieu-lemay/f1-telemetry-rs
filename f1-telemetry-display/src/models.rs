use f1_telemetry::packet::lap::ResultStatus;
use f1_telemetry::packet::participants::Team;

pub struct SessionInfo<'a> {
    pub session_name: &'a str,
    pub track_name: &'a str,
    pub elapsed_time: u16,
    pub duration: u16,
    pub current_lap: u8,
    pub number_of_laps: u8,
}

pub struct LapInfo<'a> {
    pub position: u8,
    pub name: &'a str,
    pub team: Team,
    pub current_lap_time: f32,
    pub last_lap_time: f32,
    pub best_lap_time: f32,
    pub status: ResultStatus,
    pub in_pit: bool,
    pub lap_invalid: bool,
}
