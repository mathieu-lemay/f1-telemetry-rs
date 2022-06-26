use serde::Serialize;

use crate::packet::generic::WheelData;

use super::header::PacketHeader;

/// This type is used for the 20-element `car_setups` array of the [`PacketCarSetupData`] type.
///
/// ```text
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
/// tyres_pressure          Tyres pressure (PSI)
/// ballast                 Ballast
/// fuel_load               Fuel load
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct CarSetupData {
    pub front_wing: u8,
    pub rear_wing: u8,
    pub on_throttle: u8,
    pub off_throttle: u8,
    pub front_camber: f32,
    pub rear_camber: f32,
    pub front_toe: f32,
    pub rear_toe: f32,
    pub front_suspension: u8,
    pub rear_suspension: u8,
    pub front_anti_roll_bar: u8,
    pub rear_anti_roll_bar: u8,
    pub front_suspension_height: u8,
    pub rear_suspension_height: u8,
    pub brake_pressure: u8,
    pub brake_bias: u8,
    pub tyres_pressure: WheelData<f32>,
    pub ballast: u8,
    pub fuel_load: f32,
}

/// This packet details the car setups for each vehicle in the session.
///
/// Note that in multiplayer games, other player cars will appear as blank, you will only be able to see your car setup and AI cars.
///
/// Frequency: 2 per second
///
/// ## Specification
/// ```text
/// header:     Header
/// car_setups: List of car setups
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketCarSetupData {
    pub header: PacketHeader,
    pub car_setups: Vec<CarSetupData>,
}
