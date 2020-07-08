use getset::{CopyGetters, Getters};

use crate::packet::generic::WheelData;

use super::header::PacketHeader;

/// This type is used for the 20-element `car_setups` array of the [`PacketCarSetupData`] type.
///
/// ## Specification
/// front_wing              Front wing aero
/// rear_wing               Rear wing aero
/// on_throttle             Differential adjustment on throttle (percentage)
/// off_throttle            Differential adjustment off throttle (percentage)
/// front_camber            Front camber angle (suspension geometry)
/// rear_camber             Rear camber angle (suspension geometry)
/// front_toe               Front toe angle (suspension geometry)
/// rear_toe                Rear toe angle (suspension geometry)
/// front_suspension        Front suspension
/// rear_suspension         Rear suspension
/// front_anti_roll_bar     Front anti-roll bar
/// rear_anti_roll_bar      Rear anti-roll bar
/// front_suspension_height Front ride height
/// rear_suspension_height  Rear ride height
/// brake_pressure          Brake pressure (percentage)
/// brake_bias              Brake bias (percentage)
/// tyres_pressure          Tyres pressure
/// ballast                 Ballast
/// fuel_load               Fuel load
///
/// [`PacketCarSetupData`]: ./struct.CarSetupData.html
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CarSetupData {
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
    tyres_pressure: WheelData<f32>,
    ballast: u8,
    fuel_load: f32,
}

#[allow(clippy::too_many_arguments)]
impl CarSetupData {
    pub(crate) fn from_2019(
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
        front_tyre_pressure: f32,
        rear_tyre_pressure: f32,
        ballast: u8,
        fuel_load: f32,
    ) -> CarSetupData {
        let tyres_pressure = WheelData::new(
            rear_tyre_pressure,
            rear_tyre_pressure,
            front_tyre_pressure,
            front_tyre_pressure,
        );
        CarSetupData {
            front_wing,
            rear_wing,
            on_throttle,
            off_throttle,
            front_camber,
            rear_camber,
            front_toe,
            rear_toe,
            front_suspension,
            rear_suspension,
            front_anti_roll_bar,
            rear_anti_roll_bar,
            front_suspension_height,
            rear_suspension_height,
            brake_pressure,
            brake_bias,
            tyres_pressure,
            ballast,
            fuel_load,
        }
    }

    pub(crate) fn from_2020(
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
        tyres_pressure: WheelData<f32>,
        ballast: u8,
        fuel_load: f32,
    ) -> CarSetupData {
        CarSetupData {
            front_wing,
            rear_wing,
            on_throttle,
            off_throttle,
            front_camber,
            rear_camber,
            front_toe,
            rear_toe,
            front_suspension,
            rear_suspension,
            front_anti_roll_bar,
            rear_anti_roll_bar,
            front_suspension_height,
            rear_suspension_height,
            brake_pressure,
            brake_bias,
            tyres_pressure,
            ballast,
            fuel_load,
        }
    }
}

/// This packet details the car setups for each vehicle in the session.
///
/// Note that in multiplayer games, other player cars will appear as blank, you will only be able to see your car setup and AI cars.
///
/// Frequency: 2 per second
///
/// Size: 843 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:     Header
/// car_setups: List of car setups (20)
/// ```
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct PacketCarSetupData {
    header: PacketHeader,
    car_setups: Vec<CarSetupData>,
}

impl PacketCarSetupData {
    pub(crate) fn new(header: PacketHeader, car_setups: Vec<CarSetupData>) -> PacketCarSetupData {
        PacketCarSetupData { header, car_setups }
    }
}
