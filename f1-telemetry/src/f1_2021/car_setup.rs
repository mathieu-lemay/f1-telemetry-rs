use std::io::BufRead;

use serde::Deserialize;

use crate::packet::car_setup::{CarSetupData, PacketCarSetupData};
use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

/// This packet details the car setups for each vehicle in the session.
///
/// Note that in multiplayer games, other player cars will appear as blank, you will only be able to
/// see your car setup and AI cars.
///
/// Frequency: 2 per second
/// Size: 1102 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// front_wing:                 Front wing aero
/// rear_wing:                  Rear wing aero
/// on_throttle:                Differential adjustment on throttle (percentage)
/// off_throttle:               Differential adjustment off throttle (percentage)
/// front_camber:               Front camber angle (suspension geometry)
/// rear_camber:                Rear camber angle (suspension geometry)
/// front_toe:                  Front toe angle (suspension geometry)
/// rear_toe:                   Rear toe angle (suspension geometry)
/// front_suspension:           Front suspension
/// rear_suspension:            Rear suspension
/// front_anti_roll_bar:        Front anti-roll bar
/// rear_anti_roll_bar:         Rear anti-roll bar
/// front_suspension_height:    Front ride height
/// rear_suspension_height:     Rear ride height
/// brake_pressure:             Brake pressure (percentage)
/// brake_bias:                 Brake bias (percentage)
/// rear_left_tyre_pressure:    Rear left tyre pressure (PSI)
/// rear_right_tyre_pressure:   Rear right tyre pressure (PSI)
/// front_left_tyre_pressure:   Front left tyre pressure (PSI)
/// front_right_tyre_pressure:  Front right tyre pressure (PSI)
/// ballast:                    Ballast
/// fuel_load:                  Fuel load
/// ```
#[derive(Deserialize)]
struct RawCarSetup {
    front_wing: u8,
    rear_wing: u8,
    on_throttle: u8,
    off_throttle: u8,
    front_camber: f32,
    rear_camber: f32,
    front_toe: f32,
    rear_toe: f32,
    front_suspension: u8,
    rear_suspension: u8,
    front_anti_roll_bar: u8,
    rear_anti_roll_bar: u8,
    front_suspension_height: u8,
    rear_suspension_height: u8,
    brake_pressure: u8,
    brake_bias: u8,
    rear_left_tyre_pressure: f32,
    rear_right_tyre_pressure: f32,
    front_left_tyre_pressure: f32,
    front_right_tyre_pressure: f32,
    ballast: u8,
    fuel_load: f32,
}

impl From<&RawCarSetup> for CarSetupData {
    fn from(car_setup: &RawCarSetup) -> Self {
        let tyres_pressure = WheelData {
            rear_left: car_setup.rear_left_tyre_pressure,
            rear_right: car_setup.rear_right_tyre_pressure,
            front_left: car_setup.front_left_tyre_pressure,
            front_right: car_setup.front_right_tyre_pressure,
        };

        Self {
            front_wing: car_setup.front_wing,
            rear_wing: car_setup.rear_wing,
            on_throttle: car_setup.on_throttle,
            off_throttle: car_setup.off_throttle,
            front_camber: car_setup.front_camber,
            rear_camber: car_setup.rear_camber,
            front_toe: car_setup.front_toe,
            rear_toe: car_setup.rear_toe,
            front_suspension: car_setup.front_suspension,
            rear_suspension: car_setup.rear_suspension,
            front_anti_roll_bar: car_setup.front_anti_roll_bar,
            rear_anti_roll_bar: car_setup.rear_anti_roll_bar,
            front_suspension_height: car_setup.front_suspension_height,
            rear_suspension_height: car_setup.rear_suspension_height,
            brake_pressure: car_setup.brake_pressure,
            brake_bias: car_setup.brake_bias,
            tyres_pressure,
            ballast: car_setup.ballast,
            fuel_load: car_setup.fuel_load,
        }
    }
}

pub(crate) fn parse_car_setup_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarSetupData, UnpackError> {
    assert_packet_size(size, CAR_SETUPS_PACKET_SIZE)?;

    let car_setups: [RawCarSetup; NUMBER_CARS] = bincode::deserialize_from(reader)?;

    let car_setups: Vec<CarSetupData> = car_setups
        .iter()
        .map(|cs| cs.into())
        .collect::<Vec<CarSetupData>>();

    Ok(PacketCarSetupData { header, car_setups })
}
