use f1_telemetry::packet::participants::{Driver, Team};
use ncurses::*;
use std::borrow::Cow;

const TEAM_COLOUR_OFFSET: i16 = 100;
const STATUS_COLOUR_OFFSET: i16 = 200;
const PERCENTAGE_BAR_SLICES: i8 = 20;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Status {
    Ok = (STATUS_COLOUR_OFFSET + 1) as isize,
    Caution = (STATUS_COLOUR_OFFSET + 2) as isize,
    Warning = (STATUS_COLOUR_OFFSET + 3) as isize,
    Danger = (STATUS_COLOUR_OFFSET + 4) as isize,
}

pub fn init_colors() {
    start_color();

    init_base_color_pairs();
    init_team_colors();
    init_status_colors()
}

fn init_base_color_pairs() {
    init_pair(COLOR_BLACK, COLOR_BLACK, COLOR_BLACK);
    init_pair(COLOR_RED, COLOR_RED, COLOR_BLACK);
    init_pair(COLOR_GREEN, COLOR_GREEN, COLOR_BLACK);
    init_pair(COLOR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
    init_pair(COLOR_BLUE, COLOR_BLUE, COLOR_BLACK);
    init_pair(COLOR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(COLOR_CYAN, COLOR_CYAN, COLOR_BLACK);
    init_pair(COLOR_WHITE, COLOR_WHITE, COLOR_BLACK);
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
    init_color(Status::Warning as i16, 1000, 812, 686);

    for (status, c) in &[
        (Status::Ok, COLOR_GREEN),
        (Status::Caution, COLOR_YELLOW),
        (Status::Warning, Status::Warning as i16),
        (Status::Danger, COLOR_RED),
    ] {
        init_pair(*status as i16, *c, COLOR_BLACK);
    }
}

pub fn set_bold() {
    attron(A_BOLD());
}

pub fn wset_bold(w: WINDOW) {
    wattron(w, A_BOLD());
}

pub fn set_team_color(w: WINDOW, team: Team) {
    wcolor_set(w, TEAM_COLOUR_OFFSET + team.id() as i16);
}

pub fn set_color(w: Option<WINDOW>, c: i16) {
    match w {
        Some(w) => wcolor_set(w, c),
        None => color_set(c),
    };
}

pub fn set_damage_color(w: Option<WINDOW>, damage_pct: u8) {
    let c = match damage_pct {
        d if d <= 15 => Status::Ok,
        d if d <= 40 => Status::Caution,
        d if d <= 60 => Status::Warning,
        _ => Status::Danger,
    };

    set_color(w, c as i16);
}

pub fn reset() {
    attrset(0);
}

pub fn wreset(w: WINDOW) {
    wattrset(w, 0);
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

pub fn format_gear(gear: i8) -> Cow<'static, str> {
    match gear {
        -1 => "R".into(),
        0 => "N".into(),
        _ => format!("{}", gear).into(),
    }
}

pub fn format_perc_bar(perc_value: f32) -> String {
    let bars = 100 / PERCENTAGE_BAR_SLICES;
    let used_bars = (perc_value * 100.0) as i8 / bars;
    format!("{: <20}", (0..used_bars).map(|_| "|").collect::<String>())
}

pub fn format_speed(speed: u16) -> String {
    format!("{:3} km/h", speed)
}

pub fn center(hwnd: WINDOW, s: &str) -> i32 {
    let w = getmaxx(hwnd);
    (w - s.len() as i32) / 2
}
