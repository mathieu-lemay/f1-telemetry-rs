use std::borrow::Cow;

use f1_telemetry::packet::participants::Driver;

pub fn format_driver_name(name: &str, driver: Driver, is_online: bool) -> Cow<str> {
    if is_online {
        Cow::Borrowed(name)
    } else {
        match driver {
            Driver::Player => capitalize_name(name),
            _ => Cow::Borrowed(name),
        }
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
        format_time_hms_millis(time + p)
    } else if delta_laps > 0 {
        format!("+{} laps  ", delta_laps)
    } else if delta_time > 1_000_000 {
        "Invalid Time".to_string()
    } else {
        format!("+{}  ", format_time_ms_millis(delta_time + p))
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

pub fn format_time_hms(ts: u16) -> String {
    let hours = ts / 3600;
    let minutes = (ts - hours * 3600) / 60;
    let seconds = ts % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn format_time_hms_millis(ts: u32) -> String {
    let millis = ts % 1000;
    let seconds = ts / 1000;
    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

pub fn format_time_ms_millis(ts: u32) -> String {
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
