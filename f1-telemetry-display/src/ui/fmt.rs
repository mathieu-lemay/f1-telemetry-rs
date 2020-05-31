const TEAM_COLOUR_OFFSET: i16 = 100;

use f1_telemetry::packet::participants::Team;
use ncurses::*;

pub fn init_colors() {
    start_color();

    init_team_colors()
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

pub fn set_bold() {
    attron(A_BOLD());
}

pub fn set_team_color(team: Team) {
    color_set(TEAM_COLOUR_OFFSET + team.id() as i16);
}

pub fn reset() {
    attrset(0);
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

pub fn center(hwnd: WINDOW, s: &str) -> i32 {
    let w = getmaxx(hwnd);
    (w - s.len() as i32) / 2
}
