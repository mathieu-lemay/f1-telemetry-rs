use std::borrow::Cow;
use std::fmt::Write as _;

use f1_telemetry::packet::generic::ResultStatus;
use f1_telemetry::packet::participants::Driver;

use crate::models::{EventInfo, Participant, SessionInfo};

pub trait AsPercentage {
    fn as_percentage(&self) -> String;
}

impl AsPercentage for f32 {
    fn as_percentage(&self) -> String {
        format!("{0:.2}%", self)
    }
}

pub trait AsWeight {
    fn as_weight(&self) -> String;
}

impl AsWeight for f32 {
    fn as_weight(&self) -> String {
        format!("{0:.2}kg", self)
    }
}

pub trait AsMinuteTimeString {
    fn as_minute_time_string(&self) -> String;
}

impl AsMinuteTimeString for u32 {
    fn as_minute_time_string(&self) -> String {
        let minutes = self / 60000;
        let seconds = self % 60000 / 1000;
        let millis = self % 1000;

        format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
    }
}

pub trait AsHourTimeString {
    fn as_hour_time_string(&self) -> String;
}

impl AsHourTimeString for u32 {
    fn as_hour_time_string(&self) -> String {
        let millis = self % 1000;
        let seconds = self / 1000;
        let hours = seconds / 3600;
        let minutes = (seconds - hours * 3600) / 60;
        let seconds = seconds % 60;

        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
    }
}

impl AsHourTimeString for u16 {
    fn as_hour_time_string(&self) -> String {
        let hours = self / 3600;
        let minutes = (self - hours * 3600) / 60;
        let seconds = self % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

pub fn get_session_name(sinfo: &SessionInfo) -> String {
    format!("{} - {}", sinfo.session_type.name(), sinfo.track_name)
}

pub fn get_lap_count(sinfo: &SessionInfo) -> String {
    format!("Lap {} of {}", sinfo.current_lap, sinfo.number_of_laps)
}

pub fn get_session_time(sinfo: &SessionInfo) -> String {
    format!(
        "{} / {}",
        sinfo.elapsed_time.as_hour_time_string(),
        sinfo.duration.as_hour_time_string()
    )
}

pub fn format_driver_name(participant: &Participant, is_online: bool) -> Cow<str> {
    if is_online {
        Cow::Borrowed(&participant.name)
    } else {
        match participant.driver {
            Driver::Player => capitalize_name(&participant.name),
            _ => Cow::Borrowed(&participant.name),
        }
    }
}

pub fn format_position(position: u8, status: &ResultStatus) -> String {
    match status {
        ResultStatus::Retired => String::from("RET"),
        ResultStatus::NotClassified => String::from("N/C"),
        ResultStatus::Disqualified => String::from("DSQ"),
        _ => format!("{:3}", position),
    }
}

pub fn format_time_delta(
    position: u8,
    time: u32,
    delta_time: u32,
    delta_laps: u8,
    penalties: u8,
) -> String {
    let p = penalties as u32 * 1000;

    if position == 1 {
        (time + p).as_hour_time_string()
    } else if delta_laps > 0 {
        format!("+{} laps  ", delta_laps)
    } else if delta_time > 1_000_000 {
        "Invalid Time".to_string()
    } else {
        format!("+{}  ", (delta_time + p).as_minute_time_string())
    }
}

fn capitalize_name(name: &str) -> Cow<str> {
    let n: Vec<&str> = name.split_ascii_whitespace().collect();
    if n.len() == 2 {
        Cow::Owned(n[1].to_ascii_uppercase())
    } else {
        Cow::Borrowed(name)
    }
}

pub fn milliseconds_to_msf(ts: u32) -> String {
    let minutes = ts / 60000;
    let seconds = (ts - minutes * 60000) / 1000;
    let millis = ts % 1000;

    format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
}

pub fn format_gear(gear: i8) -> Cow<'static, str> {
    match gear {
        -1 => "R".into(),
        0 => "N".into(),
        _ => format!("{}", gear).into(),
    }
}

pub fn format_speed(speed: u16) -> String {
    format!("{:3} km/h", speed)
}

pub fn format_event_info(event_info: &EventInfo) -> String {
    let mut msg = format!(
        "{}: {}",
        event_info.timestamp.as_hour_time_string(),
        event_info.description
    );

    if let Some(driver) = &event_info.driver_name {
        let _ = write!(msg, ": {}", driver); // Safe to ignore Err
    }

    if let Some(detail) = &event_info.detail {
        let _ = write!(msg, " ({})", detail); // Safe to ignore Err
    }

    msg
}

#[cfg(test)]
mod test_fmt_delta_time {
    use super::*;

    #[test]
    fn test_first_place_returns_race_time() {
        let position = 1;
        let time = 3_601_001;
        let delta_time = 0;
        let delta_laps = 0;
        let penalties = 0;
        let expected = format!("01:00:01.001");

        let actual = format_time_delta(position, time, delta_time, delta_laps, penalties);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_not_first_place_same_lap_returns_race_time_delta() {
        let position = 2;
        let time = 3_601_001;
        let delta_time = 3_601;
        let delta_laps = 0;
        let penalties = 0;
        let expected = format!("+00:03.601  ");

        let actual = format_time_delta(position, time, delta_time, delta_laps, penalties);

        assert_eq!(expected, actual)
    }
    #[test]
    fn test_not_first_place_2_laps_down_returns_lap_delta() {
        let position = 2;
        let time = 3_601_001;
        let delta_time = 3_601;
        let delta_laps = 2;
        let penalties = 0;
        let expected = format!("+2 laps  ");

        let actual = format_time_delta(position, time, delta_time, delta_laps, penalties);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_2_delta_time_2_penalties_returns_4_time_delta() {
        let position = 2;
        let time = 10_000;
        let delta_time = 2_000;
        let delta_laps = 0;
        let penalties = 2;
        let expected = format!("+00:04.000  ");

        let actual = format_time_delta(position, time, delta_time, delta_laps, penalties);

        assert_eq!(expected, actual)
    }
}
