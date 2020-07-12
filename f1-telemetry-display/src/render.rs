use f1_telemetry::packet::Packet;

use crate::models::*;
use crate::ui::Ui;

#[derive(Eq, PartialEq)]
pub enum View {
    Dashboard,
    TrackOverview,
    LapDetail,
}

pub struct Renderer {
    ui: Ui,
    active_view: View,
}

impl Renderer {
    pub fn new() -> Renderer {
        let ui = Ui::init();

        Renderer {
            ui,
            active_view: View::Dashboard,
        }
    }

    pub fn destroy(&self) {
        self.ui.destroy();
    }

    pub fn switch_view(&mut self, view: View) {
        self.ui.switch_window(&view);
        self.active_view = view;
    }

    pub fn render(&mut self, game_state: &GameState, packet: &Packet) {
        self.render_main_view(game_state, packet);
        self.render_dashboard_view(game_state, packet);
        self.render_track_view(game_state, packet);
        self.render_lap_view(game_state, packet);
    }

    fn render_main_view(&mut self, game_state: &GameState, packet: &Packet) {
        match packet {
            Packet::Session(_) => {
                self.ui.print_session_info(&game_state);
            }
            Packet::Event(_) => {
                self.ui.print_event_info(&game_state);
            }
            Packet::Lap(_) => {
                self.ui.print_session_info(&game_state);
            }
            _ => {}
        }
    }

    fn render_dashboard_view(&mut self, game_state: &GameState, packet: &Packet) {
        if !self.should_render(View::Dashboard) {
            return;
        }

        match packet {
            Packet::Lap(_) => {
                self.ui.print_dashboard_lap_info(&game_state);
            }
            Packet::CarTelemetry(_) => self.ui.print_telemetry_info(&game_state),
            Packet::CarStatus(_) => {
                self.ui.print_car_status(&game_state);
                self.ui.print_tyres_compounds(&game_state);
            }
            _ => {}
        }
    }
    fn render_lap_view(&mut self, game_state: &GameState, packet: &Packet) {
        if !self.should_render(View::LapDetail) {
            return;
        }

        match packet {
            Packet::Lap(_) => {
                self.ui.print_lap_details_lap_info(&game_state);
                self.ui.print_best_sectors_lap_info(&game_state);
            }
            Packet::CarStatus(_) => {
                self.ui.print_tyres_compounds(&game_state);
            }
            _ => {}
        }
    }

    fn render_track_view(&self, game_state: &GameState, packet: &Packet) {
        if !self.should_render(View::TrackOverview) {
            return;
        }

        match packet {
            Packet::Lap(_) => {
                self.ui.print_track_status_lap_info(game_state);
            }
            Packet::Session(_) => self.ui.print_weather_info(game_state),
            _ => {}
        }
    }

    fn should_render(&self, view: View) -> bool {
        view == self.active_view
    }
}

trait Render {
    fn render(&self, packet: &Packet);
}
