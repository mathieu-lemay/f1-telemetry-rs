use ncurses::*;

use f1_telemetry::packet::generic::{ResultStatus, TyreCompoundVisual};
use f1_telemetry::packet::session::SafetyCar;

use crate::models::*;
use crate::render::View;

mod car;
pub mod fmt;
mod weather;

const WIDTH: i32 = 132;
const HEIGHT: i32 = 35;
const SESSION_Y_OFFSET: i32 = 1;
const WINDOW_Y_OFFSET: i32 = 5;
const LEFT_BORDER_X_OFFSET: i32 = 2;
const CURRENT_CAR_DATA_Y_OFFSET: i32 = 24;

pub struct Ui {
    mwnd: WINDOW,
    active_wnd: WINDOW,
    dashboard_wnd: WINDOW,
    track_wnd: WINDOW,
    tyres_swnd: WINDOW,
    lap_times_swnd: WINDOW,
    car_swnd: WINDOW,
    rel_pos_swnd: WINDOW,
    laps_wnd: WINDOW,
    lap_details_swnd: WINDOW,
    best_sectors_swnd: WINDOW,
}

impl Ui {
    pub fn init() -> Ui {
        setlocale(ncurses::LcCategory::all, "");

        let mwnd = initscr();

        let w = getmaxx(mwnd);
        let h = getmaxy(mwnd);

        if w < WIDTH || h < HEIGHT {
            endwin();

            panic!(format!(
                "Terminal must be at least {}x{}. Current size: {}x{}",
                WIDTH, HEIGHT, w, h
            ));
        }

        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        cbreak();
        noecho();
        keypad(mwnd, true);
        timeout(0);
        fmt::init_colors();

        wresize(mwnd, HEIGHT, WIDTH);

        refresh();

        let win_w = WIDTH - 2;
        let win_h = HEIGHT - WINDOW_Y_OFFSET - 2;

        let dashboard_wnd = Ui::create_win(win_h, win_w, WINDOW_Y_OFFSET, 1, Some("Dashboard"));
        let tyres_swnd = derwin(dashboard_wnd, 23, 2, 1, 2);
        let lap_times_swnd = derwin(dashboard_wnd, 23, 80, 1, 4);
        let car_swnd = derwin(dashboard_wnd, 24, 39, 2, 90);

        let track_wnd = Ui::create_win(win_h, win_w, WINDOW_Y_OFFSET, 1, Some("Track Status"));
        let rel_pos_swnd = derwin(track_wnd, 12, getmaxx(track_wnd) - 4, 15, 2);

        let laps_wnd = Ui::create_win(win_h, win_w, WINDOW_Y_OFFSET, 1, Some("Lap Details"));
        let lap_details_swnd = derwin(dashboard_wnd, 23, 120, 1, 4);
        let best_sectors_swnd = derwin(dashboard_wnd, 2, 80, 24, 4);
        let active_wnd = dashboard_wnd;
        wrefresh(active_wnd);

        Ui {
            mwnd,
            active_wnd,
            dashboard_wnd,
            track_wnd,
            tyres_swnd,
            lap_times_swnd,
            car_swnd,
            rel_pos_swnd,
            laps_wnd,
            lap_details_swnd,
            best_sectors_swnd,
        }
    }

    pub fn destroy(&self) {
        endwin();
    }

    pub fn switch_window(&mut self, view: &View) {
        let neww = match view {
            View::Dashboard => self.dashboard_wnd,
            View::TrackOverview => self.track_wnd,
            View::LapDetail => self.laps_wnd,
        };

        if neww == self.active_wnd {
            return;
        }

        redrawwin(neww);

        self.active_wnd = neww;
        self.commit(neww);
    }

    fn commit(&self, w: WINDOW) {
        fmt::wreset(w);
        wrefresh(w);
    }

    fn create_win(h: i32, w: i32, y: i32, x: i32, title: Option<&str>) -> WINDOW {
        let wnd = newwin(h, w, y, x);
        box_(wnd, 0, 0);

        if let Some(title) = title {
            mvwaddstr(wnd, 0, 2, &format!(" {} ", title));
        };

        wnd
    }

    pub fn print_session_info(&self, game_state: &GameState) {
        let sinfo = &game_state.session_info;

        let session_name = &format!("{} - {}", sinfo.session_name, sinfo.track_name);
        let lap_info = &format!("Lap {} of {}", sinfo.current_lap, sinfo.number_of_laps);
        let session_time = &format!(
            "{} / {}",
            fmt::format_time(sinfo.elapsed_time),
            fmt::format_time(sinfo.duration)
        );

        addstr_center(self.mwnd, SESSION_Y_OFFSET, session_name);
        addstr_center(self.mwnd, SESSION_Y_OFFSET + 1, lap_info);
        addstr_center(self.mwnd, SESSION_Y_OFFSET + 2, session_time);

        if sinfo.safety_car == SafetyCar::Virtual || sinfo.safety_car == SafetyCar::Full {
            fmt::blink_colour(COLOR_WHITE, COLOR_YELLOW);
            addstr_center(self.mwnd, SESSION_Y_OFFSET + 3, sinfo.safety_car.name());
            fmt::reset();
        }
    }

    pub fn print_lap_details_lap_info(&self, game_state: &GameState) {
        let wnd = self.lap_details_swnd;

        werase(wnd);
        fmt::wset_bold(wnd);

        let header =
            "  P. NAME            | CURRENT LAP  | LAST LAP     | BEST LAP     | LAST SECTOR 1| LAST SECTOR 2| LAST SECTOR 3| STATUS ";

        mvwaddstr(wnd, 0, 0, header);

        for (idx, li) in game_state.lap_infos.iter().enumerate() {
            if let ResultStatus::Invalid = li.status {
                continue;
            }

            let participant = &game_state.participants[idx];

            let pos = match li.status {
                ResultStatus::Retired => String::from("RET"),
                ResultStatus::NotClassified => String::from("N/C"),
                ResultStatus::Disqualified => String::from("DSQ"),
                _ => format!("{:3}", li.position),
            };

            let penalties = if li.penalties > 0 {
                format!("+{:2}s", li.penalties)
            } else {
                format!("{: <4}", "")
            };

            let s = format!(
                "{}. {:15} | {} | {} | {} | {} | {} | {} | {}{}{} ",
                pos,
                fmt::format_driver_name(&participant.name, participant.driver),
                fmt::format_time_ms(li.current_lap_time),
                fmt::format_time_ms(li.last_lap_time),
                fmt::format_time_ms(li.best_lap_time),
                fmt::format_time_ms(li.sector_1 as f32 / 1000.0),
                fmt::format_time_ms(li.sector_2 as f32 / 1000.0),
                fmt::format_time_ms(li.last_lap_time - li.sector_2 as f32 / 1000.0),
                if li.in_pit { "P" } else { " " },
                if li.lap_invalid { "!" } else { " " },
                penalties,
            );

            fmt::set_team_color(wnd, participant.team);
            mvwaddstr(wnd, li.position as i32, 0, s.as_str());
        }

        self.commit(wnd);
    }

    pub fn print_best_sectors_lap_info(&self, game_state: &GameState) {
        let wnd = self.best_sectors_swnd;

        werase(wnd);

        fmt::wset_bold(wnd);

        let header = "  BEST SECTOR 1| BEST SECTOR 2| BEST SECTOR 3| THEORETICAL BEST LAP ";

        mvwaddstr(wnd, 0, 0, header);

        let mut best_s1 = u16::MAX;
        let mut best_s2 = u16::MAX;
        let mut best_s3 = u16::MAX;

        for li in game_state.lap_infos.iter() {
            if 0 < li.best_sector_1 && li.best_sector_1 < best_s1 {
                best_s1 = li.best_sector_1
            }
            if 0 < li.best_sector_2 && li.best_sector_2 < best_s2 {
                best_s2 = li.best_sector_2
            }
            if 0 < li.best_sector_3 && li.best_sector_3 < best_s3 {
                best_s3 = li.best_sector_3
            }
        }

        let mut best_lap = 0;
        if best_s3 != u16::MAX {
            best_lap = best_s1 + best_s2 + best_s3
        }

        let s = format!(
            "  {} | {} | {} | {}   ",
            fmt::format_time_ms(best_s1 as f32 / 1000.0),
            fmt::format_time_ms(best_s2 as f32 / 1000.0),
            fmt::format_time_ms(best_s3 as f32 / 1000.0),
            fmt::format_time_ms(best_lap as f32 / 1000.0),
        );
        mvwaddstr(wnd, 1, 0, s.as_str());

        self.commit(wnd);
    }

    pub fn print_dashboard_lap_info(&self, game_state: &GameState) {
        let wnd = self.lap_times_swnd;

        werase(wnd);

        fmt::wset_bold(wnd);

        let header =
            "  P. NAME                 | CURRENT LAP  | LAST LAP     | BEST LAP     | STATUS";

        mvwaddstr(wnd, 0, 0, header);

        for (idx, li) in game_state.lap_infos.iter().enumerate() {
            if let ResultStatus::Invalid = li.status {
                continue;
            }

            let participant = &game_state.participants[idx];

            let pos = match li.status {
                ResultStatus::Retired => String::from("RET"),
                ResultStatus::NotClassified => String::from("N/C"),
                ResultStatus::Disqualified => String::from("DSQ"),
                _ => format!("{:3}", li.position),
            };

            let penalties = if li.penalties > 0 {
                format!("+{:2}s", li.penalties)
            } else {
                format!("{: <4}", "")
            };

            let s = format!(
                "{}. {:20} | {} | {} | {} | {}{}{} ",
                pos,
                fmt::format_driver_name(&participant.name, participant.driver),
                fmt::format_time_ms(li.current_lap_time),
                fmt::format_time_ms(li.last_lap_time),
                fmt::format_time_ms(li.best_lap_time),
                if li.in_pit { "P" } else { " " },
                if li.lap_invalid { "!" } else { " " },
                penalties,
            );

            fmt::set_team_color(wnd, participant.team);
            mvwaddstr(wnd, li.position as i32, 0, s.as_str());
        }

        self.commit(wnd);
    }

    pub fn print_track_status_lap_info(&self, game_state: &GameState) {
        let wnd = self.rel_pos_swnd;
        let w = getmaxx(wnd) - 17;

        let relative_positions = &game_state.relative_positions;

        fmt::wset_bold(wnd);

        let header = format!(
            "Last {} First",
            (0..23).map(|_| "---->").collect::<String>()
        );
        let filler = (0..w).map(|_| " ").collect::<String>();

        mvwaddstr(wnd, 0, 0, "Relative Positions");
        mvwaddstr(wnd, 1, 0, &header);

        let scale = relative_positions.max - relative_positions.min;
        let slice = scale / (w - 1) as f32;

        for (idx, (team, positions)) in relative_positions.positions.iter().enumerate() {
            let y = idx as i32 + 2;

            fmt::set_team_color(wnd, *team);
            mvwaddstr(wnd, y, 0, &format!("{:<15.15} |{}", team.name(), &filler));

            for p in positions {
                let place = ((*p - relative_positions.min) / slice).round() as i32 + 17;
                mvwaddstr(wnd, y, place, "o");
            }
        }

        self.commit(wnd);
    }

    pub fn print_event_info(&self, game_state: &GameState) {
        fmt::set_bold();

        let event_info = &game_state.event_info;

        let mut msg = format!(
            "{}: {}",
            fmt::format_time_ms(event_info.timestamp),
            event_info.description
        );

        if let Some(driver) = &event_info.driver_name {
            msg += &format!(": {}", driver);
        }

        if let Some(detail) = &event_info.detail {
            msg += &format!(" ({})", detail);
        }

        mvaddstr(getmaxy(self.mwnd) - 1, LEFT_BORDER_X_OFFSET, &msg);
        clrtoeol();

        fmt::reset();
    }

    pub fn print_telemetry_info(&self, game_state: &GameState) {
        let wnd = self.dashboard_wnd;

        let telemetry_info = &game_state.telemetry_info;

        fmt::set_bold();

        let gear_msg = format!(
            "Gear     : {}    Speed : {}",
            fmt::format_gear(telemetry_info.gear),
            fmt::format_speed(telemetry_info.speed)
        );

        mvwaddstr(
            wnd,
            CURRENT_CAR_DATA_Y_OFFSET,
            LEFT_BORDER_X_OFFSET,
            &gear_msg,
        );

        mvwaddstr(
            wnd,
            CURRENT_CAR_DATA_Y_OFFSET + 1,
            LEFT_BORDER_X_OFFSET,
            "Throttle : ",
        );
        mvwaddstr(
            wnd,
            CURRENT_CAR_DATA_Y_OFFSET + 2,
            LEFT_BORDER_X_OFFSET,
            "Brake    : ",
        );

        let offset = getcurx(wnd);

        let throttle_bar = fmt::format_perc_bar(telemetry_info.throttle);
        fmt::set_color(Some(wnd), COLOR_GREEN);
        mvwaddstr(wnd, CURRENT_CAR_DATA_Y_OFFSET + 1, offset, &throttle_bar);

        let brake_bar = fmt::format_perc_bar(telemetry_info.brake);
        fmt::set_color(Some(wnd), COLOR_RED);
        mvwaddstr(wnd, CURRENT_CAR_DATA_Y_OFFSET + 2, offset, &brake_bar);

        self.commit(wnd)
    }

    pub fn print_weather_info(&self, game_state: &GameState) {
        let wnd = self.track_wnd;

        let session = &game_state.session_info;

        weather::render_weather(wnd, session, 2, 90);
        mvwaddstr(
            wnd,
            2 + 10,
            90,
            &format!("Air Temp   : {}C", session.air_temperature),
        );
        mvwaddstr(
            wnd,
            2 + 11,
            90,
            &format!("Track Temp : {}C", session.track_temperature),
        );

        self.commit(wnd);
    }

    pub fn print_car_status(&self, game_state: &GameState) {
        let wnd = self.car_swnd;

        let car_status = &game_state.car_status;

        car::render_car(wnd, car_status);

        mvwaddstr(
            wnd,
            22,
            0,
            &format!("Tyre Compound: {} ", car_status.tyre_compound.name()),
        );

        fmt::wset_bold(wnd);
        fmt::set_tyre_color(wnd, car_status.tyre_compound);
        waddstr(wnd, "o");
        fmt::wreset(wnd);
        waddstr(wnd, &format!(" ({} Laps)", car_status.tyre_age_laps));

        wclrtoeol(wnd);

        mvwaddstr(
            wnd,
            23,
            0,
            &format!(
                "Fuel Remaining: {:3.2}kg ({:+1.2} laps)",
                car_status.fuel_in_tank,
                car_status.fuel_remaining_laps.abs()
            ),
        );
        wclrtoeol(wnd);

        self.commit(wnd);
    }

    pub fn print_tyres_compounds(&self, game_state: &GameState) {
        let wnd = self.tyres_swnd;

        werase(wnd);

        fmt::wset_bold(wnd);

        mvwaddstr(wnd, 0, 0, "T.");

        for li in &game_state.lap_infos {
            if let TyreCompoundVisual::Invalid = li.tyre_compound {
                continue;
            }

            fmt::set_tyre_color(wnd, li.tyre_compound);
            mvwaddstr(wnd, li.position as i32, 0, "o");
        }

        self.commit(wnd);
    }
}

fn addstr_center(w: WINDOW, y: i32, str_: &str) {
    mv(y, 0);
    clrtoeol();
    mvwaddstr(w, y, fmt::center(w, str_), str_);
}
