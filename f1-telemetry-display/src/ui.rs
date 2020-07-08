use ncurses::*;

use f1_telemetry::packet::generic::{ResultStatus, TyreCompoundVisual};
use f1_telemetry::packet::session::SafetyCar;

use crate::models::{
    CarStatus, EventInfo, LapInfo, RelativePositions, SessionInfo, TelemetryInfo, WeatherInfo,
};
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
}

impl Ui {
    pub fn init() -> Ui {
        setlocale(ncurses::LcCategory::all, "");

        let mwnd = initscr();

        let w = getmaxx(mwnd);
        let h = getmaxy(mwnd);

        if w < WIDTH || h < HEIGHT {
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
        }
    }

    pub fn destroy(&self) {
        endwin();
    }

    pub fn switch_window(&mut self, view: &View) {
        let neww = match view {
            View::Dashboard => self.dashboard_wnd,
            View::TrackOverview => self.track_wnd,
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

    pub fn print_session_info(&self, sinfo: &SessionInfo) {
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
            fmt::set_color(None, COLOR_YELLOW);
            addstr_center(self.mwnd, SESSION_Y_OFFSET + 3, sinfo.safety_car.name());
            fmt::reset();
        }
    }

    pub fn print_dashboard_lap_info(&self, lap_info: &[LapInfo]) {
        let wnd = self.lap_times_swnd;

        werase(wnd);

        fmt::wset_bold(wnd);

        let header =
            "  P. NAME                 | CURRENT LAP  | LAST LAP     | BEST LAP     | STATUS";

        mvwaddstr(wnd, 0, 0, header);

        for li in lap_info {
            if let ResultStatus::Invalid = li.status {
                continue;
            }

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
                fmt::format_driver_name(li.name, li.driver),
                fmt::format_time_ms(li.current_lap_time),
                fmt::format_time_ms(li.last_lap_time),
                fmt::format_time_ms(li.best_lap_time),
                if li.in_pit { "P" } else { " " },
                if li.lap_invalid { "!" } else { " " },
                penalties,
            );

            fmt::set_team_color(wnd, li.team);
            mvwaddstr(wnd, li.position as i32, 0, s.as_str());
        }

        self.commit(wnd);
    }

    pub fn print_track_status_lap_info(&self, relative_positions: &RelativePositions) {
        let wnd = self.rel_pos_swnd;
        let w = getmaxx(wnd) - 17;

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

    pub fn print_event_info(&self, event_info: &EventInfo) {
        fmt::set_bold();

        let mut msg = format!(
            "{}: {}",
            fmt::format_time_ms(event_info.timestamp),
            event_info.description
        );

        if let Some(driver) = event_info.driver_name {
            msg += &format!(": {}", driver);
        }

        if let Some(detail) = &event_info.detail {
            msg += &format!(" ({})", detail);
        }

        mvaddstr(getmaxy(self.mwnd) - 1, LEFT_BORDER_X_OFFSET, &msg);
        clrtoeol();

        fmt::reset();
    }

    pub fn print_telemetry_info(&self, telemetry_info: &TelemetryInfo) {
        let wnd = self.dashboard_wnd;

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

    pub fn print_weather_info(&self, weather_info: &WeatherInfo) {
        let wnd = self.track_wnd;

        weather::render_weather(wnd, weather_info, 2, 90);
        mvwaddstr(
            wnd,
            2 + 10,
            90,
            &format!("Air Temp   : {}C", weather_info.air_temperature),
        );
        mvwaddstr(
            wnd,
            2 + 11,
            90,
            &format!("Track Temp : {}C", weather_info.track_temperature),
        );

        self.commit(wnd);
    }

    pub fn print_car_status(&self, car_status: &CarStatus) {
        let wnd = self.car_swnd;

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

    pub fn print_tyres_compounds(&self, tyre_compounds: &[TyreCompoundVisual]) {
        let wnd = self.tyres_swnd;

        fmt::wset_bold(wnd);

        mvwaddstr(wnd, 0, 0, "T.");

        for (i, tc) in tyre_compounds.iter().enumerate() {
            fmt::set_tyre_color(wnd, *tc);
            mvwaddstr(wnd, i as i32 + 1, 0, "o");
        }

        self.commit(wnd);
    }
}

fn addstr_center(w: WINDOW, y: i32, str_: &str) {
    mv(y, 0);
    clrtoeol();
    mvwaddstr(w, y, fmt::center(w, str_), str_);
}
