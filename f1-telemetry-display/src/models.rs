use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::f32::INFINITY;

use f1_telemetry::packet::car_status::PacketCarStatusData;
use f1_telemetry::packet::car_telemetry::PacketCarTelemetryData;
use f1_telemetry::packet::event::{Event, PacketEventData};
use f1_telemetry::packet::final_classification::PacketFinalClassificationData;
use f1_telemetry::packet::generic::{ResultStatus, Team, TyreCompoundVisual, WheelData};
use f1_telemetry::packet::lap::{PacketLapData, PitStatus};
use f1_telemetry::packet::participants::{Driver, PacketParticipantsData};
use f1_telemetry::packet::session::{PacketSessionData, SafetyCar, SessionType, Weather};
use f1_telemetry::packet::Packet;

use crate::ui::fmt;

#[derive(Default)]
pub struct LapAndSectorTimes {
    pub sector_1: u32,
    pub sector_2: u32,
    pub sector_3: u32,
    pub lap: u32,
}

#[derive(Default)]
pub struct GameState {
    pub session_uid: Option<u64>,
    pub session_info: SessionInfo,
    pub lap_infos: Vec<LapInfo>,
    pub session_best_times: LapAndSectorTimes,
    pub event_info: EventInfo,
    pub participants: Vec<Participant>,
    pub car_status: CarStatus,
    pub telemetry_info: TelemetryInfo,
    pub relative_positions: RelativePositions,
    pub final_classifications: Vec<FinalClassificationInfo>,
}

impl GameState {
    pub fn update(&mut self, packet: &Packet) {
        self.validate_session(packet);

        match packet {
            // Packet::Motion(p) => p.header(),
            Packet::Session(p) => self.parse_session_data(&p),
            Packet::Lap(p) => self.parse_lap_data(&p),
            Packet::Event(p) => self.parse_event_data(&p),
            Packet::Participants(p) => self.parse_participants(&p),
            // Packet::CarSetups(p) => p.header(),
            Packet::CarTelemetry(p) => self.parse_telemetry_data(&p),
            Packet::CarStatus(p) => self.parse_car_status(&p),
            Packet::FinalClassification(p) => self.parse_final_classification(&p),
            _ => {}
        }
    }

    fn validate_session(&mut self, packet: &Packet) {
        let suid = packet.header().session_uid();

        if self.session_uid.is_some() && self.session_uid.unwrap() == suid {
            return;
        }

        self.session_uid = Some(suid);

        self.participants = Vec::new();
        self.lap_infos = Vec::new();
    }

    fn parse_session_data(&mut self, session: &PacketSessionData) {
        self.session_info.session_type = session.session_type();
        self.session_info.track_name = session.track().name().into();
        self.session_info.elapsed_time = session.session_duration() - session.session_time_left();
        self.session_info.duration = session.session_duration();
        self.session_info.number_of_laps = session.total_laps();
        self.session_info.safety_car = session.safety_car_status();
        self.session_info.weather = session.weather();
        self.session_info.track_temperature = session.track_temperature();
        self.session_info.air_temperature = session.air_temperature();
        self.session_info.is_online = session.network_game();
    }

    fn parse_lap_data(&mut self, lap_data: &PacketLapData) {
        self.parse_lap_data_times(lap_data);
        self.parse_lap_data_current_lap(lap_data);
        self.parse_lap_data_relative_positions(lap_data);
    }

    fn parse_lap_data_times(&mut self, lap_data: &PacketLapData) {
        let mut best_s1 = u32::MAX;
        let mut best_s2 = u32::MAX;
        let mut best_s3 = u32::MAX;
        let mut best_lap = u32::MAX;

        for idx in 0..self.lap_infos.len() {
            let ld = &lap_data.lap_data()[idx];
            let li = &mut self.lap_infos[idx];

            li.position = ld.car_position();
            li.current_lap_time = ld.current_lap_time();
            li.best_lap_time = ld.best_lap_time();
            li.current_lap_num = ld.current_lap_num();
            li.status = ld.result_status();
            li.in_pit = ld.pit_status() != PitStatus::None;
            li.lap_invalid = ld.current_lap_invalid();
            li.penalties = ld.penalties();
            li.lap_distance = ld.lap_distance();
            li.total_distance = ld.total_distance();
            li.best_sector_1 = ld.best_overall_sector_1_time() as u32;
            li.best_sector_2 = ld.best_overall_sector_2_time() as u32;
            li.best_sector_3 = ld.best_overall_sector_3_time() as u32;

            let new_s1 = ld.sector_1_time() as u32;
            let new_s2 = ld.sector_2_time() as u32;
            let new_ll = ld.last_lap_time();

            if new_s1 != li.sector_1 && new_s1 > 0 {
                li.sector_1 = new_s1;
                li.sector_2 = 0;
                li.sector_3 = 0;
            }

            if new_s2 != li.sector_2 && new_s2 > 0 {
                li.sector_2 = new_s2;
            }

            if new_ll != li.last_lap_time {
                li.last_lap_time = new_ll;

                if li.sector_1 != 0 && li.sector_2 != 0 {
                    // Hack to prevent inaccuracies with last_lap_time being a float, if possible.
                    if ld.best_overall_sector_3_lap_num() == li.current_lap_num - 1 {
                        li.sector_3 = li.best_sector_3;
                    } else {
                        li.sector_3 = li.last_lap_time - li.sector_2 - li.sector_1;
                    }
                }
            }

            if li.best_sector_1 > 0 && li.best_sector_1 < best_s1 {
                best_s1 = li.best_sector_1;
            }

            if li.best_sector_2 > 0 && li.best_sector_2 < best_s2 {
                best_s2 = li.best_sector_2;
            }

            if li.best_sector_3 > 0 && li.best_sector_3 < best_s3 {
                best_s3 = li.best_sector_3;
            }

            if li.best_lap_time > 0 && li.best_lap_time < best_lap {
                best_lap = li.best_lap_time;
            }
        }

        self.session_best_times = LapAndSectorTimes {
            sector_1: if best_s1 != u32::MAX { best_s1 } else { 0 },
            sector_2: if best_s2 != u32::MAX { best_s2 } else { 0 },
            sector_3: if best_s3 != u32::MAX { best_s3 } else { 0 },
            lap: if best_lap != u32::MAX { best_lap } else { 0 },
        }
    }

    fn parse_lap_data_current_lap(&mut self, lap_data: &PacketLapData) {
        self.session_info.current_lap = lap_data
            .lap_data()
            .iter()
            .map(|l| l.current_lap_num())
            .max()
            .unwrap_or(0)
    }

    fn parse_lap_data_relative_positions(&mut self, lap_data: &PacketLapData) {
        let mut positions = BTreeMap::new();

        let mut min = INFINITY;
        let mut max = -INFINITY;

        for (i, p) in self.participants.iter().enumerate() {
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
                .entry(p.team)
                .or_insert_with(Vec::new)
                .push(ld.total_distance());
        }

        self.relative_positions = RelativePositions {
            positions,
            min,
            max,
        }
    }

    fn parse_event_data(&mut self, event_data: &PacketEventData) {
        let evt = event_data.event();

        let driver_name = match evt.vehicle_idx() {
            Some(idx) => {
                if self.participants.len() > idx as usize {
                    Some(self.participants[idx as usize].name.clone())
                } else {
                    None
                }
            }
            None => None,
        };

        let detail = match evt {
            Event::FastestLap(f) => Some(fmt::format_time_ms_millis(f.lap_time())),
            Event::Penalty(p) => Some(format!("{:?}", p.penalty_type())),
            Event::SpeedTrap(s) => Some(format!("{:.1} km/h", s.speed())),
            _ => None,
        };

        self.event_info.timestamp = event_data.header().session_time();
        self.event_info.description = evt.description().to_string();
        self.event_info.driver_name = driver_name;
        self.event_info.detail = detail;
    }

    fn parse_participants(&mut self, ppd: &PacketParticipantsData) {
        self.participants = ppd
            .participants()
            .iter()
            .map(|p| Participant {
                name: p.name().into(),
                driver: p.driver(),
                team: p.team(),
            })
            .collect();

        match self.participants.len().cmp(&self.lap_infos.len()) {
            Ordering::Equal => (),
            Ordering::Greater => {
                for _ in self.lap_infos.len()..self.participants.len() {
                    self.lap_infos.push(LapInfo::default());
                }
            }
            Ordering::Less => self.lap_infos.truncate(self.participants.len()),
        }
        match self
            .participants
            .len()
            .cmp(&self.final_classifications.len())
        {
            Ordering::Equal => (),
            Ordering::Greater => {
                for _ in self.final_classifications.len()..self.participants.len() {
                    self.final_classifications
                        .push(FinalClassificationInfo::default());
                }
            }
            Ordering::Less => self.final_classifications.truncate(self.participants.len()),
        }
    }

    fn parse_telemetry_data(&mut self, telemetry_data: &PacketCarTelemetryData) {
        let player_index = telemetry_data.header().player_car_index();
        let td = &telemetry_data.car_telemetry_data()[player_index as usize];

        self.telemetry_info.speed = td.speed();
        self.telemetry_info.throttle = td.throttle();
        self.telemetry_info.brake = td.brake();
        self.telemetry_info.gear = td.gear();
        self.telemetry_info.engine_rpm = td.engine_rpm();
        self.telemetry_info.drs = td.drs();
        self.telemetry_info.rev_lights_percent = td.rev_lights_percent();
        self.telemetry_info.engine_temperature = td.engine_temperature();

        self.car_status.drs = td.drs();
    }

    fn parse_final_classification(&mut self, classification_data: &PacketFinalClassificationData) {
        let mut race_time = 0;
        let mut best_lap = 0;
        let mut laps: i8 = 0;

        for i in 0..self.final_classifications.len() {
            let fc = &classification_data.final_classifications()[i];
            let fi = &mut self.final_classifications[i];

            fi.position = fc.position();

            fi.best_lap_time = fc.best_lap_time();
            fi.grid_position = fc.grid_position();
            fi.total_race_time = fc.total_race_time();
            fi.num_laps = fc.num_laps();
            fi.num_pit_stops = fc.num_pit_stops();
            fi.penalties = fc.penalties_time();
            fi.tyres_visual = fc
                .tyre_stints_visual()
                .iter()
                .filter(|&&t| t != TyreCompoundVisual::Invalid)
                .map(|t| t.clone())
                .collect();
            fi.points = fc.points();
            fi.status = fc.result_status();

            if fi.position == 1 {
                race_time = fi.total_race_time;
                best_lap = fi.best_lap_time;
                laps = fi.num_laps as i8;
            }
        }

        match self.session_info.session_type {
            SessionType::Race | SessionType::Race2 => {
                for fi in &mut self.final_classifications {
                    fi.delta_pos = fi.position as i8 - fi.grid_position as i8;
                    fi.delta_time = fi.total_race_time - race_time;
                    fi.delta_laps = (laps - fi.num_laps as i8).max(0) as u8;
                }
            }
            _ => {
                for fi in &mut self.final_classifications {
                    fi.delta_pos = 0;
                    fi.delta_time = fi.best_lap_time - best_lap;
                    fi.delta_laps = 0;
                }
            }
        }
    }

    fn parse_car_status(&mut self, car_status_data: &PacketCarStatusData) {
        for idx in 0..self.lap_infos.len() {
            let cs = &car_status_data.car_status_data()[idx];
            let li = &mut self.lap_infos[idx];

            li.tyre_compound = cs.visual_tyre_compound();
        }

        let player_index = car_status_data.header().player_car_index();
        let csd = &car_status_data.car_status_data()[player_index as usize];

        self.car_status.tyres_damage = csd.tyres_damage();
        self.car_status.left_front_wing_damage = csd.front_left_wing_damage();
        self.car_status.right_front_wing_damage = csd.front_right_wing_damage();
        self.car_status.rear_wing_damage = csd.rear_wing_damage();
        self.car_status.engine_damage = csd.engine_damage();
        self.car_status.gearbox_damage = csd.gear_box_damage();
        self.car_status.fuel_in_tank = csd.fuel_in_tank();
        self.car_status.fuel_remaining_laps = csd.fuel_remaining_laps();
        self.car_status.tyre_compound = csd.visual_tyre_compound();
        self.car_status.tyre_age_laps = csd.tyre_age_laps();
    }

    pub(crate) fn compute_theoretical_best_lap(&self) -> u32 {
        if self.session_best_times.sector_3 > 0 {
            self.session_best_times.sector_1
                + self.session_best_times.sector_2
                + self.session_best_times.sector_3
        } else {
            0
        }
    }
}

pub struct Participant {
    pub name: String,
    pub driver: Driver,
    pub team: Team,
}

#[derive(Default)]
pub struct EventInfo {
    pub timestamp: u32,
    pub description: String,
    pub driver_name: Option<String>,
    pub detail: Option<String>,
}

#[derive(Default)]
pub struct LapInfo {
    pub position: u8,
    pub current_lap_time: u32,
    pub last_lap_time: u32,
    pub best_lap_time: u32,
    pub current_lap_num: u8,
    pub status: ResultStatus,
    pub in_pit: bool,
    pub lap_invalid: bool,
    pub penalties: u8,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub tyre_compound: TyreCompoundVisual,
    pub best_sector_1: u32,
    pub best_sector_2: u32,
    pub best_sector_3: u32,
    pub sector_1: u32,
    pub sector_2: u32,
    pub sector_3: u32,
}

#[derive(Default)]
pub struct SessionInfo {
    pub session_type: SessionType,
    pub track_name: String,
    pub elapsed_time: u16,
    pub duration: u16,
    pub current_lap: u8,
    pub number_of_laps: u8,
    pub safety_car: SafetyCar,
    pub weather: Weather,
    pub track_temperature: i8,
    pub air_temperature: i8,
    pub is_online: bool,
}

#[derive(Default)]
pub struct TelemetryInfo {
    pub speed: u16,
    pub throttle: f32,
    pub brake: f32,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: bool,
    pub rev_lights_percent: u8,
    pub engine_temperature: u16,
}

#[derive(Default)]
pub struct CarStatus {
    pub tyres_damage: WheelData<u8>,
    pub left_front_wing_damage: u8,
    pub right_front_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub engine_damage: u8,
    pub gearbox_damage: u8,
    pub fuel_in_tank: f32,
    pub fuel_remaining_laps: f32,
    pub tyre_compound: TyreCompoundVisual,
    pub tyre_age_laps: u8,
    pub drs: bool,
}

#[derive(Default)]
pub struct RelativePositions {
    pub positions: BTreeMap<Team, Vec<f32>>,
    pub min: f32,
    pub max: f32,
}

#[derive(Default)]
pub struct FinalClassificationInfo {
    pub position: u8,
    pub grid_position: u8,
    pub points: u8,
    pub num_laps: u8,
    pub num_pit_stops: u8,
    pub status: ResultStatus,
    pub best_lap_time: u32,
    pub total_race_time: u32,
    pub penalties: u8,
    pub tyres_visual: Vec<TyreCompoundVisual>,
    pub delta_pos: i8,
    pub delta_time: u32,
    pub delta_laps: u8,
}
