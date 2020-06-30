use std::collections::BTreeMap;
use std::f32::INFINITY;

use f1_telemetry::packet::car_status::PacketCarStatusData;
use f1_telemetry::packet::car_telemetry::PacketCarTelemetryData;
use f1_telemetry::packet::event::PacketEventData;
use f1_telemetry::packet::lap::{PacketLapData, PitStatus, ResultStatus};
use f1_telemetry::packet::participants::PacketParticipantsData;
use f1_telemetry::packet::session::PacketSessionData;
use f1_telemetry::packet::Packet;

use crate::models::{CarStatus, EventInfo, LapInfo, RelativePositions, SessionInfo};
use crate::models::{TelemetryInfo, WeatherInfo};
use crate::ui::Ui;

#[derive(Eq, PartialEq)]
pub enum View {
    Dashboard,
    TrackOverview,
}

pub struct Renderer {
    ui: Ui,
    active_view: View,
    current_lap: u8,
    participants: Option<PacketParticipantsData>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let ui = Ui::init();

        Renderer {
            ui,
            active_view: View::Dashboard,
            current_lap: 0,
            participants: None,
        }
    }

    pub fn destroy(&self) {
        self.ui.destroy();
    }

    pub fn switch_view(&mut self, view: View) {
        self.ui.switch_window(&view);
        self.active_view = view;
    }

    pub fn render(&mut self, packet: &Packet) {
        self.render_main_view(packet);
        self.render_dashboard_view(packet);
        self.render_track_view(packet);
    }

    fn render_main_view(&mut self, packet: &Packet) {
        match packet {
            Packet::Session(s) => {
                let sinfo = parse_session_data(&s, self.current_lap);
                self.ui.print_session_info(&sinfo);
            }

            Packet::Event(evt) => {
                if let Some(evt_info) = parse_event_data(&evt, &self.participants) {
                    self.ui.print_event_info(&evt_info);
                }
            }
            Packet::Lap(ld) => {
                self.current_lap = get_current_lap(&ld);
            }
            Packet::Participants(p) => self.participants = Some(p.clone()),
            _ => {}
        }
    }

    fn render_dashboard_view(&mut self, packet: &Packet) {
        if !self.should_render(View::Dashboard) {
            return;
        }

        match packet {
            Packet::Lap(ld) => {
                if let Some(lap_info) = parse_lap_data(&ld, &self.participants) {
                    self.ui.print_dashboard_lap_info(&lap_info)
                }
            }
            Packet::CarTelemetry(td) => {
                let telemetry_info = parse_telemetry_data(&td);
                self.ui.print_telemetry_info(&telemetry_info)
            }
            Packet::CarStatus(cs) => {
                let csd = parse_car_status_data(&cs);
                self.ui.print_car_status(&csd)
            }

            _ => {}
        }
    }

    fn render_track_view(&self, packet: &Packet) {
        if !self.should_render(View::TrackOverview) {
            return;
        }
        match packet {
            Packet::Lap(ld) => {
                if let Some(rel_positions) = parse_relative_positions_data(&ld, &self.participants)
                {
                    self.ui.print_track_status_lap_info(&rel_positions);
                }
            }
            Packet::Session(sd) => {
                if let Some(weather_info) = parse_weather_data(&sd) {
                    self.ui.print_weather_info(&weather_info)
                }
            }
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

fn parse_session_data(session: &PacketSessionData, current_lap: u8) -> SessionInfo {
    SessionInfo {
        session_name: session.session_type().name(),
        track_name: session.track().name(),
        elapsed_time: session.session_duration() - session.session_time_left(),
        duration: session.session_duration(),
        current_lap,
        number_of_laps: session.total_laps(),
        safety_car: session.safety_car_status(),
    }
}

fn parse_lap_data<'a>(
    lap_data: &'a PacketLapData,
    participants: &'a Option<PacketParticipantsData>,
) -> Option<Vec<LapInfo<'a>>> {
    if participants.is_none() {
        return None;
    }

    let participants = participants.as_ref().unwrap().participants();

    let mut lap_info = Vec::with_capacity(participants.len());

    for (i, p) in participants.iter().enumerate() {
        let ld = &lap_data.lap_data()[i];

        let li = LapInfo {
            position: ld.car_position(),
            name: p.name(),
            driver: p.driver(),
            team: p.team(),
            current_lap_time: ld.current_lap_time(),
            last_lap_time: ld.last_lap_time(),
            best_lap_time: ld.best_lap_time(),
            status: ld.result_status(),
            in_pit: ld.pit_status() != PitStatus::None,
            lap_invalid: ld.current_lap_invalid(),
            penalties: ld.penalties(),
            lap_distance: ld.lap_distance(),
            total_distance: ld.total_distance(),
        };

        lap_info.push(li);
    }

    Some(lap_info)
}

fn parse_event_data<'a>(
    event_data: &'a PacketEventData,
    participants: &'a Option<PacketParticipantsData>,
) -> Option<EventInfo<'a>> {
    if event_data.vehicle_idx().is_some() && participants.is_none() {
        return None;
    }

    let description = event_data.event().description();
    let driver_name = match event_data.vehicle_idx() {
        Some(idx) => {
            let name = participants.as_ref().unwrap().participants()[idx as usize].name();
            Some(name.as_str())
        }
        None => None,
    };

    Some(EventInfo {
        timestamp: event_data.header().session_time(),
        description,
        driver_name,
        lap_time: event_data.lap_time(),
    })
}

fn parse_telemetry_data(telemetry_data: &PacketCarTelemetryData) -> TelemetryInfo {
    let player_index = telemetry_data.header().player_car_index();
    let telemetry_data = &telemetry_data.car_telemetry_data()[player_index as usize];

    TelemetryInfo {
        speed: telemetry_data.speed(),
        throttle: telemetry_data.throttle(),
        brake: telemetry_data.brake(),
        gear: telemetry_data.gear(),
        engine_rpm: telemetry_data.engine_rpm(),
        drs: telemetry_data.drs(),
        rev_lights_percent: telemetry_data.rev_lights_percent(),
        engine_temperature: telemetry_data.engine_temperature(),
    }
}

fn parse_car_status_data(car_status_data: &PacketCarStatusData) -> CarStatus {
    let player_index = car_status_data.header().player_car_index();
    let csd = &car_status_data.car_status_data()[player_index as usize];

    CarStatus {
        tyres_damage: csd.tyres_damage(),
        left_front_wing_damage: csd.front_left_wing_damage(),
        right_front_wing_damage: csd.front_right_wing_damage(),
        rear_wing_damage: csd.rear_wing_damage(),
        engine_damage: csd.engine_damage(),
        gearbox_damage: csd.gear_box_damage(),
        fuel_in_tank: csd.fuel_in_tank(),
        fuel_remaining_laps: csd.fuel_remaining_laps(),
        tyre_compound: csd.visual_tyre_compound(),
    }
}

fn parse_relative_positions_data(
    lap_data: &PacketLapData,
    participants: &Option<PacketParticipantsData>,
) -> Option<RelativePositions> {
    if participants.is_none() {
        return None;
    }

    let participants = participants.as_ref().unwrap();

    let mut positions = BTreeMap::new();

    let mut min = INFINITY;
    let mut max = -INFINITY;

    for (i, p) in participants.participants().iter().enumerate() {
        let ld = &lap_data.lap_data()[i];

        if ld.result_status() != ResultStatus::Active {
            continue;
        }

        let distance = ld.total_distance();

        if distance > max {
            max = distance;
        }
        if distance < min {
            min = distance;
        }

        positions
            .entry(p.team())
            .or_insert_with(Vec::new)
            .push(ld.total_distance());
    }

    Some(RelativePositions {
        positions,
        min,
        max,
    })
}

fn parse_weather_data(session_data: &PacketSessionData) -> Option<WeatherInfo> {
    Some(WeatherInfo {
        weather: session_data.weather(),
        track_temperature: session_data.track_temperature(),
        air_temperature: session_data.air_temperature(),
    })
}

fn get_current_lap(lap_data: &PacketLapData) -> u8 {
    lap_data
        .lap_data()
        .iter()
        .map(|l| l.current_lap_num())
        .max()
        .unwrap_or(0)
}
