use crate::models::{LapInfo, SessionInfo};
use f1_telemetry::packet::lap::ResultStatus;
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

    pub fn print_session_info(&self, sinfo: &SessionInfo) {
        let session_name = &format!("{} - {}", sinfo.session_name, sinfo.track_name);
        let lap_info = &format!("Lap {} of {}", sinfo.current_lap, sinfo.number_of_laps);
        let session_time = &format!(
            "{} / {}",
            fmt::format_time(sinfo.elapsed_time),
            fmt::format_time(sinfo.duration)
        );

        addstr_center(self.hwnd, SESSION_Y_OFFSET, session_name);
        addstr_center(self.hwnd, SESSION_Y_OFFSET + 1, lap_info);
        addstr_center(self.hwnd, SESSION_Y_OFFSET + 2, session_time);
    }

    pub fn print_lap_info(&self, lap_info: &[LapInfo]) {
        mvaddstr(
            LAP_DATA_HEADER_Y_OFFSET,
            2,
            "  P. NAME                 | CURRENT LAP  | LAST LAP     | BEST LAP     | STATUS",
        );

        fmt::set_bold();

        for li in lap_info {
            let pos = match li.status {
                ResultStatus::Retired => String::from("RET"),
                ResultStatus::NotClassified => String::from("N/C"),
                ResultStatus::Disqualified => String::from("DSQ"),
                _ => format!("{:3}", li.position),
            };
            let name = li.name;
            let team = li.team;

            let s = format!(
                "{}. {:20} | {} | {} | {} | {}{}     ",
                pos,
                name,
                fmt::format_time_ms(li.current_lap_time),
                fmt::format_time_ms(li.last_lap_time),
                fmt::format_time_ms(li.best_lap_time),
                if li.in_pit { "P" } else { " " },
                if li.lap_invalid { "!" } else { " " },
            );

            fmt::set_team_color(team);
            mvaddstr(LAP_DATA_Y_OFFSET + li.position as i32 - 1, 2, s.as_str());
            clrtoeol();
        }

        fmt::reset();
    }
}

fn addstr_center(w: WINDOW, y: i32, str_: &str) {
    mv(y, 0);
    clrtoeol();
    mvaddstr(y, fmt::center(w, str_), str_);
}
