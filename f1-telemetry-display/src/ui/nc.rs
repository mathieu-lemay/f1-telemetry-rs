use std::time::Duration;

use async_trait::async_trait;
use ncurses::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::sleep;

use f1_telemetry::packet::generic::{ResultStatus, SessionType, TyreCompoundVisual};
use f1_telemetry::packet::session::SafetyCar;
use f1_telemetry::packet::Packet;

use crate::fmt as cfmt;
use crate::models::*;
use crate::ui::Ui;

mod car;
mod fmt;
mod suspension;
mod weather;

const WIDTH: i32 = 132;
const HEIGHT: i32 = 35;
const SESSION_Y_OFFSET: i32 = 1;
const WINDOW_Y_OFFSET: i32 = 5;
const LEFT_BORDER_X_OFFSET: i32 = 2;
const CURRENT_CAR_DATA_Y_OFFSET: i32 = 24;

#[derive(Debug, Eq, PartialEq)]
pub enum View {
    Dashboard,
    TrackOverview,
    LapDetail,
}

struct DashboardView {
    win: WINDOW,
    tyres_swnd: WINDOW,
    lap_times_swnd: WINDOW,
    car_swnd: WINDOW,
    rel_pos_swnd: WINDOW,
}

struct TrackView {
    win: WINDOW,
}

struct LapDetailView {
    win: WINDOW,
    lap_detail_swnd: WINDOW,
    best_sectors_swnd: WINDOW,
    handling_swnd: WINDOW,
}

pub struct NcursesUi {
    main_window: WINDOW,
    active_view: View,
    dashboard_view: DashboardView,
    track_view: TrackView,
    lap_detail_view: LapDetailView,
    session_rotation: bool,
}

enum Event {
    UpdateGame(Box<Packet>),
    SwitchView(View),
    EnableRotation,
    Quit,
}

#[async_trait]
impl Ui for NcursesUi {
    fn new() -> Self {
        setlocale(ncurses::LcCategory::all, "");

        let mwnd = initscr();

        let w = getmaxx(mwnd);
        let h = getmaxy(mwnd);

        if w < WIDTH || h < HEIGHT {
            endwin();

            panic!(
                "Terminal must be at least {}x{}. Current size: {}x{}",
                WIDTH, HEIGHT, w, h
            );
        }

        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        cbreak();
        noecho();
        keypad(mwnd, true);
        timeout(-1);
        fmt::init_colors();

        wresize(mwnd, h, w);

        refresh();

        let win_w = w - 2;
        let win_h = h - WINDOW_Y_OFFSET - 2;

        let dashboard_wnd = Self::create_win(win_h, win_w, WINDOW_Y_OFFSET, 1, Some("Dashboard"));
        let tyres_swnd = derwin(dashboard_wnd, 23, 2, 1, 2);
        let lap_times_swnd = derwin(dashboard_wnd, 23, 80, 1, 4);
        let car_swnd = derwin(dashboard_wnd, 24, 39, 1, win_w - 40);
        // let motion_swnd = derwin(dashboard_wnd, 15, 30, 3, win_w-100);
        let rel_pos_swnd = derwin(
            dashboard_wnd,
            13,
            getmaxx(dashboard_wnd) / 2,
            getmaxy(dashboard_wnd) - 15,
            2,
        );
        let handling_swnd = derwin(dashboard_wnd, 23, 58, win_h - 24, win_w - 60);

        let dashboard_view = DashboardView {
            win: dashboard_wnd,
            tyres_swnd,
            lap_times_swnd,
            car_swnd,
            rel_pos_swnd,
        };

        let track_wnd = Self::create_win(win_h, win_w, WINDOW_Y_OFFSET, 1, Some("Track Status"));
        let track_view = TrackView { win: track_wnd };

        let laps_wnd = Self::create_win(win_h, win_w, WINDOW_Y_OFFSET, 1, Some("Lap Details"));
        let lap_detail_swnd = derwin(laps_wnd, 23, 123, 1, 4);
        let best_sectors_swnd = derwin(laps_wnd, 2, 80, 24, 3);

        let lap_detail_view = LapDetailView {
            win: laps_wnd,
            lap_detail_swnd,
            best_sectors_swnd,
            handling_swnd,
        };

        wrefresh(dashboard_wnd);

        Self {
            main_window: mwnd,
            active_view: View::Dashboard,
            dashboard_view,
            track_view,
            lap_detail_view,
            session_rotation: false,
        }
    }

    async fn run(&mut self) {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let sender = tx.clone();
        let stream_thread = tokio::spawn(async move {
            while let Some(p) = crate::CHANNEL.rx.write().await.recv().await {
                let _ = sender.send(Event::UpdateGame(Box::new(p)));
            }
        });

        let sender = tx.clone();
        let input_thread = tokio::spawn(async move {
            let _ = process_input(sender.clone()).await;
        });

        let mut game_state = GameState::default();

        while let Some(evt) = rx.recv().await {
            match evt {
                Event::UpdateGame(p) => {
                    debug!("Packet");
                    trace!("Packet: {:?}", p);
                    game_state.update(&p);
                    self.render(&game_state, &p).await;
                }
                Event::SwitchView(v) => {
                    debug!("Switch View: {:?}", v);
                    self.disable_rotation();
                    self.switch_view(v);
                }
                Event::EnableRotation => {
                    debug!("Enable Rotation");
                    self.enable_rotation();
                }
                Event::Quit => {
                    debug!("Quit");
                    break;
                }
            }
        }

        debug!("Aborting input thread...");
        input_thread.abort();
        debug!("Done");

        debug!("Aborting stream thread...");
        stream_thread.abort();
        debug!("Done");

        rx.close();
    }

    fn destroy(&self) {
        debug!("Timeout to 0");
        timeout(0);

        ungetch(KEY_ENTER); // Workaround for app hanging
        while let Some(c) = get_wch() {
            debug!("Purging key {:?}", c);
        }
        ungetch(KEY_ENTER); // Workaround for app hanging

        debug!("Done purging keys");

        endwin();
    }
}

unsafe impl Send for NcursesUi {}

impl NcursesUi {
    fn create_win(h: i32, w: i32, y: i32, x: i32, title: Option<&str>) -> WINDOW {
        let wnd = newwin(h, w, y, x);
        box_(wnd, 0, 0);

        if let Some(title) = title {
            mvwaddstr(wnd, 0, 2, &format!(" {} ", title));
        };

        wnd
    }

    async fn render(&mut self, game_state: &GameState, packet: &Packet) {
        self.render_main_view(game_state, packet);

        if self.session_rotation {
            self.rotate_view(game_state.session_info.session_type);
        }

        match self.active_view {
            View::Dashboard => self.render_dashboard_view(game_state, packet),
            View::TrackOverview => self.render_track_view(game_state, packet),
            View::LapDetail => self.render_lap_view(game_state, packet),
        };
    }

    fn enable_rotation(&mut self) {
        self.session_rotation = true
    }

    fn disable_rotation(&mut self) {
        self.session_rotation = false
    }

    fn rotate_view(&mut self, game_state_session: SessionType) {
        match game_state_session {
            SessionType::Race => self.switch_view(View::Dashboard),
            _ => self.switch_view(View::LapDetail),
        }
    }

    fn switch_view(&mut self, view: View) {
        if view == self.active_view {
            return;
        }

        let neww = match &view {
            View::Dashboard => self.dashboard_view.win,
            View::TrackOverview => self.track_view.win,
            View::LapDetail => self.lap_detail_view.win,
        };

        self.active_view = view;

        redrawwin(neww);
        self.commit(neww);
    }

    fn commit(&self, w: WINDOW) {
        fmt::wreset(w);
        wrefresh(w);
        redrawwin(w);
    }

    fn render_main_view(&mut self, game_state: &GameState, packet: &Packet) {
        match packet {
            Packet::Session(_) => {
                self.print_session_info(game_state);
            }
            Packet::Event(_) => {
                self.print_event_info(game_state);
            }
            Packet::Lap(_) => {
                self.print_session_info(game_state);
            }
            _ => {}
        }
    }

    fn render_dashboard_view(&mut self, game_state: &GameState, packet: &Packet) {
        match packet {
            Packet::Lap(_) => {
                self.print_dashboard_lap_info(game_state);
                self.print_track_status_lap_info(game_state);
            }
            Packet::CarTelemetry(_) => self.print_telemetry_info(game_state),
            Packet::CarStatus(_) => {
                self.print_car_status(game_state);
                self.print_tyres_compounds(game_state);
            }
            Packet::FinalClassification(_) => {
                self.print_final_classification_info(game_state, self.dashboard_view.lap_times_swnd)
            }
            Packet::Motion(_) => {
                // self.print_motion_info(&game_state);
                self.print_handling_info(game_state);
            }
            _ => {}
        }
    }

    fn render_lap_view(&mut self, game_state: &GameState, packet: &Packet) {
        match packet {
            Packet::Lap(_) => {
                self.print_lap_details_lap_info(game_state);
                self.print_best_sectors_lap_info(game_state);
            }
            Packet::CarStatus(_) => {
                self.print_car_status(game_state);
                self.print_tyres_compounds(game_state);
            }
            Packet::FinalClassification(_) => self
                .print_final_classification_info(game_state, self.lap_detail_view.lap_detail_swnd),
            Packet::Motion(_) => self.print_handling_info(game_state),
            _ => {}
        }
    }

    fn render_track_view(&self, game_state: &GameState, packet: &Packet) {
        match packet {
            Packet::Lap(_) => {
                self.print_track_status_lap_info(game_state);
            }
            Packet::Session(_) => self.print_weather_info(game_state),
            _ => {}
        }
    }

    fn print_session_info(&self, game_state: &GameState) {
        let sinfo = &game_state.session_info;

        let session_name = cfmt::get_session_name(sinfo);
        let lap_info = cfmt::get_lap_count(sinfo);
        let session_time = cfmt::get_session_time(sinfo);

        addstr_center(self.main_window, SESSION_Y_OFFSET, &session_name);
        addstr_center(self.main_window, SESSION_Y_OFFSET + 1, &lap_info);
        addstr_center(self.main_window, SESSION_Y_OFFSET + 2, &session_time);

        if sinfo.safety_car == SafetyCar::Virtual || sinfo.safety_car == SafetyCar::Full {
            fmt::blink_colour(COLOR_WHITE, COLOR_YELLOW);
            addstr_center(
                self.main_window,
                SESSION_Y_OFFSET + 3,
                sinfo.safety_car.name(),
            );
            fmt::reset();
        } else {
            wmove(self.main_window, SESSION_Y_OFFSET + 3, 0);
            clrtoeol();
        }
    }

    fn print_lap_details_lap_info(&self, game_state: &GameState) {
        let wnd = self.lap_detail_view.lap_detail_swnd;

        werase(wnd);
        fmt::wset_bold(wnd);

        let header =
            "  P. NAME            | CURRENT   | LAST      | BEST      | SECTOR 1  | SECTOR 2  | SECTOR 3  | STATUS ";

        mvwaddstr(wnd, 0, 0, header);

        for (idx, li) in game_state.lap_infos.iter().enumerate() {
            if let ResultStatus::Invalid = li.status {
                continue;
            }

            let row = li.position as i32;

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
                "                     | {} |           |           |           |           |           | {}{}{} ",
                cfmt::milliseconds_to_msf(li.current_lap_time),
                if li.in_pit { "P" } else { " " },
                if li.lap_invalid { "!" } else { " " },
                penalties,
            );

            mvwaddstr(wnd, li.position as i32, 0, &s);

            let s = format!(
                "{}. {:15}",
                pos,
                cfmt::format_driver_name(participant, game_state.session_info.is_online)
            );
            fmt::set_team_color(wnd, participant.team);
            mvwaddstr(wnd, row, 0, &s);

            let s = cfmt::milliseconds_to_msf(li.last_lap_time);
            fmt::set_lap_time_color(
                Some(wnd),
                li.last_lap_time,
                li.best_lap_time,
                game_state.session_best_times.lap,
            );
            mvwaddstr(wnd, row, 35, &s);

            let s = cfmt::milliseconds_to_msf(li.best_lap_time);
            fmt::set_lap_time_color(
                Some(wnd),
                li.best_lap_time,
                li.best_lap_time,
                game_state.session_best_times.lap,
            );
            mvwaddstr(wnd, row, 47, &s);

            let s = cfmt::milliseconds_to_msf(li.sector_1);
            fmt::set_lap_time_color(
                Some(wnd),
                li.sector_1,
                li.best_sector_1,
                game_state.session_best_times.sector_1,
            );
            mvwaddstr(wnd, row, 59, &s);

            let s = cfmt::milliseconds_to_msf(li.sector_2);
            fmt::set_lap_time_color(
                Some(wnd),
                li.sector_2,
                li.best_sector_2,
                game_state.session_best_times.sector_2,
            );
            mvwaddstr(wnd, row, 71, &s);

            let s = cfmt::milliseconds_to_msf(li.sector_3);
            fmt::set_lap_time_color(
                Some(wnd),
                li.sector_3,
                li.best_sector_3,
                game_state.session_best_times.sector_3,
            );
            mvwaddstr(wnd, row, 83, &s);

            fmt::reset_color(Some(wnd));
        }

        self.commit(wnd);
    }

    fn print_best_sectors_lap_info(&self, game_state: &GameState) {
        let wnd = self.lap_detail_view.best_sectors_swnd;

        werase(wnd);

        fmt::wset_bold(wnd);

        let header = "BEST SECTOR 1 | BEST SECTOR 2 | BEST SECTOR 3 | THEORETICAL BEST LAP ";

        mvwaddstr(wnd, 0, 2, header);

        let session_best_times = &game_state.session_best_times;
        let best_lap = game_state.compute_theoretical_best_lap();

        let s = format!(
            "{}     | {}     | {}     | {}   ",
            cfmt::milliseconds_to_msf(session_best_times.sector_1),
            cfmt::milliseconds_to_msf(session_best_times.sector_2),
            cfmt::milliseconds_to_msf(session_best_times.sector_3),
            cfmt::milliseconds_to_msf(best_lap),
        );
        mvwaddstr(wnd, 1, 2, s.as_str());

        self.commit(wnd);
    }

    fn print_final_classification_info(&self, game_state: &GameState, view_to_overwrite: WINDOW) {
        let wnd = view_to_overwrite;
        werase(wnd);

        fmt::wset_bold(wnd);

        let header =
            "  P.    | NAME          | GRID  | BEST LAP    | TIME DELTA   | PENALTIES | TYRES";

        mvwaddstr(wnd, 0, 0, header);

        for (idx, fi) in game_state.final_classifications.iter().enumerate() {
            if fi.position == 0 {
                continue;
            }
            let participant = &game_state.participants[idx];
            let pos = cfmt::format_position(fi.position, &fi.status);

            let grid = match fi.grid_position {
                0 => String::from("N/A"),
                _ => format!("{:3}", fi.grid_position),
            };

            let change = match fi.delta_pos {
                d if d < 0 => "Λ",
                d if d > 0 => "V",
                _ => "",
            }
            .to_string();

            let penalties = if fi.penalties > 0 {
                format!("+{:2}s", fi.penalties)
            } else {
                format!("{: <4}", "")
            };

            let time_delta = cfmt::format_time_delta(
                fi.position,
                fi.total_race_time,
                fi.delta_time,
                fi.delta_laps,
                fi.penalties,
            );

            let s = format!(
                "{}. {:2} | {:13} | {}   | {}   | {:12} | {}      |",
                pos,
                change,
                cfmt::format_driver_name(participant, game_state.session_info.is_online),
                grid,
                cfmt::milliseconds_to_msf(fi.best_lap_time),
                time_delta,
                penalties,
            );
            fmt::set_team_color(wnd, participant.team);
            mvwaddstr(wnd, fi.position as i32, 0, s.as_str());

            for (idx, t) in fi.tyres_visual.iter().enumerate() {
                fmt::set_tyre_color(wnd, *t);
                mvwaddstr(wnd, fi.position as i32, idx as i32 + 75, "o");
            }
        }
        self.commit(wnd);
    }

    // fn print_motion_info(&self, game_state: &GameState) {
    //     let wnd = self.dashboard_view.motion_swnd;
    //     werase(wnd);
    //
    //     let header = "MOTION";
    //
    //     mvwaddstr(wnd, 0, 0, header);
    //
    //     let roll = format!(
    //         "Roll        : {:03.3}",
    //         game_state.motion_info.roll.to_degrees()
    //     );
    //     let pitch = format!(
    //         "Pitch       : {:03.3}",
    //         game_state.motion_info.pitch.to_degrees()
    //     );
    //     let g_lat = format!(
    //         "G-Lat       : {:03.3}",
    //         game_state.motion_info.g_force_lateral
    //     );
    //     let g_long = format!(
    //         "G-Long      : {:03.3}",
    //         game_state.motion_info.g_force_longitudinal
    //     );
    //     let angle = format!(
    //         "Wheel Angle : {:03.3}",
    //         game_state.motion_info.front_wheels_angle.to_degrees()
    //     );
    //
    //     let front_slip = format!(
    //         "Front Slip  : {:03.2} --- {:03.2}",
    //         game_state.motion_info.wheel_slip.front_left,
    //         game_state.motion_info.wheel_slip.front_right
    //     );
    //     let rear_slip = format!(
    //         "Rear Slip   : {:03.2} --- {:03.2}",
    //         game_state.motion_info.wheel_slip.rear_left,
    //         game_state.motion_info.wheel_slip.rear_right
    //     );
    //
    //     let front_suspension = format!(
    //         "Front Susp  : {:03.2} --- {:03.2}",
    //         game_state.motion_info.suspension_position.front_left,
    //         game_state.motion_info.suspension_position.front_right
    //     );
    //     let rear_suspension = format!(
    //         "Rear Susp   : {:03.2} --- {:03.2}",
    //         game_state.motion_info.suspension_position.rear_left,
    //         game_state.motion_info.suspension_position.rear_right
    //     );
    //
    //     mvwaddstr(wnd, 1, 0, roll.as_str());
    //     mvwaddstr(wnd, 2, 0, pitch.as_str());
    //     mvwaddstr(wnd, 3, 0, g_lat.as_str());
    //     mvwaddstr(wnd, 4, 0, g_long.as_str());
    //     mvwaddstr(wnd, 5, 0, angle.as_str());
    //
    //     mvwaddstr(wnd, 7, 0, front_slip.as_str());
    //     mvwaddstr(wnd, 8, 0, rear_slip.as_str());
    //
    //     mvwaddstr(wnd, 9, 0, front_suspension.as_str());
    //     mvwaddstr(wnd, 10, 0, rear_suspension.as_str());
    //
    //     self.commit(wnd)
    // }

    fn print_dashboard_lap_info(&self, game_state: &GameState) {
        let wnd = self.dashboard_view.lap_times_swnd;

        werase(wnd);

        fmt::wset_bold(wnd);

        let header = "  P. NAME                 | CURRENT LAP | LAST LAP    | BEST LAP    | STATUS";

        mvwaddstr(wnd, 0, 0, header);

        for (idx, li) in game_state.lap_infos.iter().enumerate() {
            if let ResultStatus::Invalid = li.status {
                continue;
            }

            let participant = &game_state.participants[idx];

            let pos = cfmt::format_position(li.position, &li.status);

            let penalties = if li.penalties > 0 {
                format!("+{:2}s", li.penalties)
            } else {
                format!("{: <4}", "")
            };

            let s = format!(
                "{}. {:20} | {}   | {}   | {}   | {}{}{} ",
                pos,
                cfmt::format_driver_name(participant, game_state.session_info.is_online),
                cfmt::milliseconds_to_msf(li.current_lap_time),
                cfmt::milliseconds_to_msf(li.last_lap_time),
                cfmt::milliseconds_to_msf(li.best_lap_time),
                if li.in_pit { "P" } else { " " },
                if li.lap_invalid { "!" } else { " " },
                penalties,
            );

            fmt::set_team_color(wnd, participant.team);
            mvwaddstr(wnd, li.position as i32, 0, s.as_str());
        }

        self.commit(wnd);
    }

    fn print_track_status_lap_info(&self, game_state: &GameState) {
        let wnd = self.dashboard_view.rel_pos_swnd;
        let w = 79;

        let relative_positions = &game_state.relative_positions;

        fmt::wset_bold(wnd);

        let header = format!(
            "Last {} First",
            (0..17).map(|_| "---->").collect::<String>()
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

    fn print_event_info(&self, game_state: &GameState) {
        fmt::set_bold();

        let event_info = &game_state.event_info;

        let msg = cfmt::format_event_info(event_info);

        mvaddstr(getmaxy(self.main_window) - 1, LEFT_BORDER_X_OFFSET, &msg);
        clrtoeol();

        fmt::reset();
    }

    fn print_telemetry_info(&self, game_state: &GameState) {
        let wnd = self.dashboard_view.win;

        let telemetry_info = &game_state.telemetry_info;

        fmt::set_bold();

        let gear_msg = format!(
            "Gear     : {}    Speed : {}",
            cfmt::format_gear(telemetry_info.gear),
            cfmt::format_speed(telemetry_info.speed)
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

    fn print_weather_info(&self, game_state: &GameState) {
        let wnd = self.track_view.win;

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

    fn print_handling_info(&self, game_state: &GameState) {
        let wnd = self.lap_detail_view.handling_swnd;

        suspension::render_suspension(wnd, &game_state.motion_info, &game_state.telemetry_info);
        // wclrtoeol(wnd);

        self.commit(wnd);
    }

    fn print_car_status(&self, game_state: &GameState) {
        let wnd = self.dashboard_view.car_swnd;

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

    fn print_tyres_compounds(&self, game_state: &GameState) {
        let wnd = self.dashboard_view.tyres_swnd;

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

async fn process_input(tx: UnboundedSender<Event>) -> Result<(), SendError<Event>> {
    loop {
        let ch = ncurses::get_wch();

        if let Some(ch) = ch {
            match ch {
                ncurses::WchResult::Char(49) => {
                    // 1
                    tx.send(Event::SwitchView(View::Dashboard))?;
                }
                ncurses::WchResult::Char(50) => {
                    // 2
                    tx.send(Event::SwitchView(View::TrackOverview))?;
                }
                ncurses::WchResult::Char(51) => {
                    // 3
                    tx.send(Event::SwitchView(View::LapDetail))?;
                }
                ncurses::WchResult::Char(52) => {
                    //4
                    tx.send(Event::EnableRotation)?;
                }
                ncurses::WchResult::Char(113) => {
                    // q
                    tx.send(Event::Quit)?;
                }
                _ => {}
            }
        } else {
            debug!("No key");
        };

        sleep(Duration::from_millis(5)).await;
    }
}
