use f1_telemetry::packet::participants::{Driver, Team};
use ncurses::*;
use std::borrow::Cow;

const TEAM_COLOUR_OFFSET: i16 = 100;
const STATUS_COLOUR_OFFSET: i16 = 200;
const PERCENTAGE_BAR_SLICES: i8 = 20;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Status {
    OK = (STATUS_COLOUR_OFFSET + 1) as isize,
    CAUTION = (STATUS_COLOUR_OFFSET + 2) as isize,
    WARNING = (STATUS_COLOUR_OFFSET + 3) as isize,
    DANGER = (STATUS_COLOUR_OFFSET + 4) as isize,
}

pub fn init_colors() {
    start_color();

    init_team_colors();
    init_status_colors()
}

fn init_team_colors() {
    for (t, c) in &[
        (Team::Mercedes, (0, 210, 190)),
        (Team::Ferrari, (220, 0, 0)),
        (Team::RedBullRacing, (30, 65, 255)),
        (Team::Williams, (255, 255, 255)),
        (Team::RacingPoint, (245, 150, 200)),
        (Team::Renault, (255, 245, 0)),
        (Team::ToroRosso, (70, 155, 255)),
        (Team::Haas, (240, 215, 135)),
        (Team::McLaren, (255, 135, 0)),
        (Team::AlfaRomeo, (155, 0, 0)),
    ] {
        let idx = TEAM_COLOUR_OFFSET + t.id() as i16;
        init_color(idx, c.0, c.1, c.2);
        init_pair(idx, COLOR_WHITE, idx);
    }
}

fn init_status_colors() {
    let color_orange = TEAM_COLOUR_OFFSET + Team::McLaren.id() as i16;

    for (status, c) in &[
        (Status::OK, COLOR_GREEN),
        (Status::CAUTION, COLOR_YELLOW),
        (Status::WARNING, color_orange),
        (Status::DANGER, COLOR_RED),
    ] {
        init_pair(*status as i16, *c, COLOR_BLACK);
    }
}

pub fn set_bold() {
    attron(A_BOLD());
}

pub fn set_team_color(team: Team) {
    color_set(TEAM_COLOUR_OFFSET + team.id() as i16);
}

pub fn set_green() {
    color_set((STATUS_COLOUR_OFFSET + 1) as i16);
}

pub fn set_red() {
    color_set((STATUS_COLOUR_OFFSET + 4) as i16);
}

pub fn reset() {
    attrset(0);
}

pub fn format_driver_name(name: &str, driver: Driver) -> Cow<str> {
    match driver {
        Driver::Player => Cow::Owned(capitalize_name(name)),
        _ => Cow::Borrowed(name),
    }
}

fn capitalize_name(name: &str) -> String {
    let n: Vec<&str> = name.split_ascii_whitespace().collect();
    let first = n[0].chars().next().unwrap().to_ascii_uppercase();
    let last = n[1].to_ascii_uppercase();

    format!("{}. {}", first, last)
}

pub fn format_time(ts: u16) -> String {
    let hours = ts / 3600;
    let minutes = (ts - hours * 3600) / 60;
    let seconds = ts % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn format_time_ms(ts: f32) -> String {
    let seconds = ts as i64;
    let millis = ((ts - ts.floor()) * 1000.0).floor();

    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

pub fn format_gear(gear: i8) -> String {
    match gear {
        -1 => format!("R"),
        0 => format!("N"),
        _ => format!("{}", gear),
    }
}

pub fn format_perc_bar(perc_value: f32) -> String {
    let bars = 100 / PERCENTAGE_BAR_SLICES;
    let used_bars = (perc_value * 100.0) as i8 / bars;
    (0..used_bars).map(|_| "|").collect::<String>()
}

pub fn center(hwnd: WINDOW, s: &str) -> i32 {
    let w = getmaxx(hwnd);
    (w - s.len() as i32) / 2
}
