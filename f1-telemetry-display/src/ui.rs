use f1_telemetry::packet::lap::PacketLapData;
use f1_telemetry::packet::participants::PacketParticipantsData;
use f1_telemetry::packet::session::PacketSessionData;
use ncurses::*;

const SESSION_Y_OFFSET: i32 = 0;
const LAP_DATA_HEADER_Y_OFFSET: i32 = 3;
const LAP_DATA_Y_OFFSET: i32 = 5;
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

        refresh();

        Ui { hwnd }
    }

    pub fn destroy(&self) {
        ncurses::endwin();
    }

    pub fn print_session_info(&self, session: &PacketSessionData) {
        let session_name = session.session_type().name();
        let session_elapsed = format_time(session.session_duration() - session.session_time_left());
        let session_duration = format_time(session.session_duration());
        let session_time = &format!("{} / {}", session_elapsed, session_duration);

        mv(SESSION_Y_OFFSET, 0);
        clrtoeol();
        mvaddstr(
            SESSION_Y_OFFSET,
            center(self.hwnd, session_name),
            session_name,
        );

        mvaddstr(
            SESSION_Y_OFFSET + 1,
            center(self.hwnd, session_time),
            session_time,
        );
    }

    pub fn print_lap_data(&self, lap_data: &PacketLapData, participants: &PacketParticipantsData) {
        mvaddstr(
            LAP_DATA_HEADER_Y_OFFSET,
            2,
            " P. NAME                 | CURRENT LAP  | LAST LAP     | BEST LAP",
        );

        for (i, ld) in lap_data.lap_data().iter().enumerate() {
            let pos = ld.car_position();
            let name = participants.participants()[i].name();
            let s = format!(
                "{:2}. {:20} | {} | {} | {}",
                pos,
                name,
                format_time_ms(ld.current_lap_time()),
                format_time_ms(ld.last_lap_time()),
                format_time_ms(ld.best_lap_time())
            );

            mvaddstr(LAP_DATA_Y_OFFSET + pos as i32 - 1, 0, s.as_str());
            clrtoeol();
        }
    }
}

fn format_time(ts: u16) -> String {
    let hours = ts / 3600;
    let minutes = (ts - hours * 3600) / 60;
    let seconds = ts % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn format_time_ms(ts: f32) -> String {
    let seconds = ts as i64;
    let millis = ((ts - ts.floor()) * 1000.0).round();

    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

fn center(hwnd: WINDOW, s: &str) -> i32 {
    let w = getmaxx(hwnd);
    (w - s.len() as i32) / 2
}
