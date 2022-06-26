use serde::Serialize;

use crate::packet::generic::WheelData;

use super::header::PacketHeader;

/// This type is used for the `car_damage` array of the [`PacketCarDamageData`] type.
///
/// ```text
/// ## Specification
/// tyres_wear:              Tyre wear (percentage)
/// tyres_damage:            Tyre damage (percentage)
/// brakes_damage:           Brakes damage (percentage)
/// front_left_wing_damage:  Front left wing damage (percentage)
/// front_right_wing_damage: Front right wing damage (percentage)
/// rear_wing_damage:        Rear wing damage (percentage)
/// floor_damage:            Floor damage (percentage)
/// diffuser_damage:         Diffuser damage (percentage)
/// sidepod_damage:          Sidepod damage (percentage)
/// drs_fault:               Indicator for DRS fault
/// gear_box_damage:         Gear box damage (percentage)
/// engine_damage:           Engine damage (percentage)
/// engine_mguh_wear:        Engine wear MGU-H (percentage)
/// engine_es_wear:          Engine wear ES (percentage)
/// engine_ce_wear:          Engine wear CE (percentage)
/// engine_ice_wear:         Engine wear ICE (percentage)
/// engine_mguk_wear:        Engine wear MGU-K (percentage)
/// engine_tc_wear:          Engine wear TC (percentage)
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct CarDamageData {
    pub tyres_wear: WheelData<f32>,
    pub tyres_damage: WheelData<u8>,
    pub brakes_damage: WheelData<u8>,
    pub front_left_wing_damage: u8,
    pub front_right_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub floor_damage: u8,
    pub diffuser_damage: u8,
    pub sidepod_damage: u8,
    pub drs_fault: bool,
    pub gear_box_damage: u8,
    pub engine_damage: u8,
    pub engine_mguh_wear: u8,
    pub engine_es_wear: u8,
    pub engine_ce_wear: u8,
    pub engine_ice_wear: u8,
    pub engine_mguk_wear: u8,
    pub engine_tc_wear: u8,
}

/// This packet details car damage parameters for all the cars in the race.
///
/// Frequency: 2 per second
///
/// ## Specification
/// ```text
/// header:     Header
/// car_setups: List of car damage data
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketCarDamageData {
    pub header: PacketHeader,
    pub car_damage_data: Vec<CarDamageData>,
}
