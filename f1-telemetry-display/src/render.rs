use crate::ui::Ui;
use crate::{
    get_current_lap, parse_event_data, parse_lap_data, parse_session_data, parse_telemetry_data,
};
use f1_telemetry::packet::Packet;

pub trait Renderer {
    fn new() -> Self;
    fn render(&self, ui: &mut Ui, p: &Packet);
}

pub struct LapRenderer {}

pub struct TrackRenderer {}

pub struct CarRenderer {}

pub struct MainRenderer {}

impl Renderer for MainRenderer {
    fn new() -> MainRenderer {
        MainRenderer {}
    }

    fn render(&self, ui: &mut Ui, p: &Packet) {
        match p {
            Packet::Session(s) => {
                let sinfo = parse_session_data(&s, ui.current_lap);
                ui.print_session_info(&sinfo);
            }

            Packet::Event(evt) => {
                if let Some(evt_info) = parse_event_data(&evt, &ui.participants) {
                    ui.print_event_info(&evt_info);
                }
            }
            Packet::Lap(ld) => {
                ui.current_lap = get_current_lap(&ld);
            }
            Packet::Participants(p) => ui.participants = Some(p.clone()),

            _ => {}
        }
    }
}

impl Renderer for CarRenderer {
    fn new() -> CarRenderer {
        CarRenderer {}
    }

    fn render(&self, ui: &mut Ui, p: &Packet) {}
}
impl Renderer for TrackRenderer {
    fn new() -> TrackRenderer {
        TrackRenderer {}
    }

    fn render(&self, ui: &mut Ui, p: &Packet) {
        match p {
            Packet::Lap(ld) => {
                if let Some(lap_info) = parse_lap_data(&ld, &ui.participants) {
                    ui.print_track_status_lap_info(&lap_info);
                }
            }
            _ => {}
        }
    }
}

impl Renderer for LapRenderer {
    fn new() -> LapRenderer {
        LapRenderer {}
    }

    fn render(&self, ui: &mut Ui, p: &Packet) {
        match p {
            Packet::Lap(ld) => {
                if let Some(lap_info) = parse_lap_data(&ld, &ui.participants) {
                    ui.print_dashboard_lap_info(&lap_info)
                }
            }
            Packet::CarTelemetry(td) => {
                if let Some(telemetry_info) = parse_telemetry_data(&td) {
                    ui.print_telemetry_info(&telemetry_info)
                }
            }
            _ => {}
        }
    }
}
