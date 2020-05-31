use f1_telemetry::packet::lap::PacketLapData;
use f1_telemetry::packet::participants::PacketParticipantsData;
use f1_telemetry::packet::session::PacketSessionData;
use ncurses::*;

mod fmt;

const SESSION_Y_OFFSET: i32 = 0;
const LAP_DATA_HEADER_Y_OFFSET: i32 = 4;
const LAP_DATA_Y_OFFSET: i32 = 6;
// const CURRENT_CAR_DATA_Y_OFFSET: i32 = 26;
// const CAR_X_OFFSET: i32 = 80;

pub struct Ui {
    hwnd: WINDOW,
}

impl Ui {
    pub fn init() -> Ui {
        setlocale(ncurses::LcCategory::all, "");

        let hwnd = initscr();

        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        cbreak();
        noecho();
        keypad(hwnd, true);
        timeout(0);

        fmt::init_colors();

        refresh();

        Ui { hwnd }
    }

    pub fn destroy(&self) {
        ncurses::endwin();
    }

    pub fn print_session_info(&self, session: &PacketSessionData) {
        let session_name = session.session_type().name();

        let track_name = session.track().name();

        let session_elapsed =
            fmt::format_time(session.session_duration() - session.session_time_left());
        let session_duration = fmt::format_time(session.session_duration());
        let session_time = &format!("{} / {}", session_elapsed, session_duration);

        mv(SESSION_Y_OFFSET, 0);
        clrtoeol();
        mvaddstr(
            SESSION_Y_OFFSET,
            fmt::center(self.hwnd, session_name),
            session_name,
        );

        mv(SESSION_Y_OFFSET + 1, 0);
        clrtoeol();
        mvaddstr(
            SESSION_Y_OFFSET + 1,
            fmt::center(self.hwnd, track_name),
            track_name,
        );

        mv(SESSION_Y_OFFSET + 2, 0);
        clrtoeol();
        mvaddstr(
            SESSION_Y_OFFSET + 2,
            fmt::center(self.hwnd, session_time),
            session_time,
        );
    }

    pub fn print_lap_data(&self, lap_data: &PacketLapData, participants: &PacketParticipantsData) {
        mvaddstr(
            LAP_DATA_HEADER_Y_OFFSET,
            2,
            " P. NAME                 | CURRENT LAP  | LAST LAP     | BEST LAP",
        );

        fmt::set_bold();

        for (i, ld) in lap_data.lap_data().iter().enumerate() {
            let pos = ld.car_position();
            let name = participants.participants()[i].name();
            let team = participants.participants()[i].team();

            let s = format!(
                "{:2}. {:20} | {} | {} | {}",
                pos,
                name,
                fmt::format_time_ms(ld.current_lap_time()),
                fmt::format_time_ms(ld.last_lap_time()),
                fmt::format_time_ms(ld.best_lap_time()),
            );

            fmt::set_team_color(team);
            mvaddstr(LAP_DATA_Y_OFFSET + pos as i32 - 1, 2, s.as_str());
            clrtoeol();
        }

        fmt::reset();
    }
}
