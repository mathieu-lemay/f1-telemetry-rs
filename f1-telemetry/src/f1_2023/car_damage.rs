use std::io::BufRead;

use serde::Deserialize;

use crate::packet::car_damage::*;
use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

/// This packet details car damage parameters for all the cars in the race.
///
/// Frequency: 2 per second
/// Size: 948 bytes
/// Version: 1
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
/// drs_fault:               Indicator for DRS fault, 0 = OK, 1 = fault
/// ers_fault:               Indicator for ERS fault, 0 = OK, 1 = fault
/// gear_box_damage:         Gear box damage (percentage)
/// engine_damage:           Engine damage (percentage)
/// engine_mguh_wear:        Engine wear MGU-H (percentage)
/// engine_es_wear:          Engine wear ES (percentage)
/// engine_ce_wear:          Engine wear CE (percentage)
/// engine_ice_wear:         Engine wear ICE (percentage)
/// engine_mguk_wear:        Engine wear MGU-K (percentage)
/// engine_tc_wear:          Engine wear TC (percentage)
/// engine_blown:            Engine blown, 0 = OK, 1 = fault
/// engine_seized:           Engine seized, 0 = OK, 1 = fault
/// ```
#[derive(Deserialize)]
struct RawCarDamage {
    tyres_wear: WheelData<f32>,
    tyres_damage: WheelData<u8>,
    brakes_damage: WheelData<u8>,
    front_left_wing_damage: u8,
    front_right_wing_damage: u8,
    rear_wing_damage: u8,
    floor_damage: u8,
    diffuser_damage: u8,
    sidepod_damage: u8,
    drs_fault: bool,
    ers_fault: bool,
    gear_box_damage: u8,
    engine_damage: u8,
    engine_mguh_wear: u8,
    engine_es_wear: u8,
    engine_ce_wear: u8,
    engine_ice_wear: u8,
    engine_mguk_wear: u8,
    engine_tc_wear: u8,
    engine_blown: bool,
    engine_seized: bool,
}

impl TryFrom<&RawCarDamage> for CarDamageData {
    type Error = UnpackError;

    fn try_from(packet: &RawCarDamage) -> Result<Self, Self::Error> {
        Ok(CarDamageData {
            tyres_wear: packet.tyres_wear,
            tyres_damage: packet.tyres_damage,
            brakes_damage: packet.brakes_damage,
            front_left_wing_damage: packet.front_left_wing_damage,
            front_right_wing_damage: packet.front_right_wing_damage,
            rear_wing_damage: packet.rear_wing_damage,
            floor_damage: packet.floor_damage,
            diffuser_damage: packet.diffuser_damage,
            sidepod_damage: packet.sidepod_damage,
            drs_fault: packet.drs_fault,
            ers_fault: packet.ers_fault,
            gear_box_damage: packet.gear_box_damage,
            engine_damage: packet.engine_damage,
            engine_mguh_wear: packet.engine_mguh_wear,
            engine_es_wear: packet.engine_es_wear,
            engine_ce_wear: packet.engine_ce_wear,
            engine_ice_wear: packet.engine_ice_wear,
            engine_mguk_wear: packet.engine_mguk_wear,
            engine_tc_wear: packet.engine_tc_wear,
            engine_blown: packet.engine_blown,
            engine_seized: packet.engine_seized,
        })
    }
}

pub fn parse_car_damage_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarDamageData, UnpackError> {
    assert_packet_size(size, CAR_DAMAGE_PACKET_SIZE)?;

    let car_damage: [RawCarDamage; NUMBER_CARS] = bincode::deserialize_from(reader)?;

    let car_damage_data = car_damage
        .iter()
        .map(|cd| cd.try_into())
        .collect::<Result<Vec<CarDamageData>, UnpackError>>()?;

    Ok(PacketCarDamageData {
        header,
        car_damage_data,
    })
}
