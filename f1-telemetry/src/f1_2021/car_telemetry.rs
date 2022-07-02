use std::io::BufRead;

use serde::Deserialize;

use crate::packet::car_telemetry::{
    CarTelemetryData, MFDPanel, PacketCarTelemetryData, SurfaceType,
};
use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

fn unpack_surface_type(value: u8) -> Result<SurfaceType, UnpackError> {
    match value {
        0 => Ok(SurfaceType::Tarmac),
        1 => Ok(SurfaceType::RumbleStrip),
        2 => Ok(SurfaceType::Concrete),
        3 => Ok(SurfaceType::Rock),
        4 => Ok(SurfaceType::Gravel),
        5 => Ok(SurfaceType::Mud),
        6 => Ok(SurfaceType::Sand),
        7 => Ok(SurfaceType::Grass),
        8 => Ok(SurfaceType::Water),
        9 => Ok(SurfaceType::Cobblestone),
        10 => Ok(SurfaceType::Metal),
        11 => Ok(SurfaceType::Ridged),
        _ => Err(UnpackError(format!("Invalid SurfaceType value: {}", value))),
    }
}

fn unpack_mfd_panel(value: u8) -> Result<MFDPanel, UnpackError> {
    match value {
        0 => Ok(MFDPanel::CarSetup),
        1 => Ok(MFDPanel::Pits),
        2 => Ok(MFDPanel::Damage),
        3 => Ok(MFDPanel::Engine),
        4 => Ok(MFDPanel::Temperatures),
        255 => Ok(MFDPanel::Closed),
        _ => Err(UnpackError(format!("Invalid MFDPanel value: {}", value))),
    }
}

/// This packet details telemetry for all the cars in the race.
///
/// It details various values that would be recorded on the car such as speed, throttle application,
/// DRS etc.
///
/// Frequency: Rate as specified in menus
/// Size: 1347 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                             Header
/// car_telemetry_data:                 List of car telemetry (22)
/// mfd_panel_index:                    Index of MFD panel open - 255 = MFD closed
///                                     Single player, race – 0 = Car setup, 1 = Pits
///                                     2 = Damage, 3 =  Engine, 4 = Temperatures
///                                     May vary depending on game mode
/// mfd_panel_index_secondary_player:   See above
/// suggested_gear:                     Suggested gear for the player (1-8)
///                                     0 if no gear suggested
/// ```
#[derive(Deserialize)]
struct RawCarTelemetryData {
    car_telemetry: [RawCarTelemetry; NUMBER_CARS],
    mfd_panel_index: u8,
    mfd_panel_index_secondary_player: u8,
    suggested_gear: i8,
}
/// This type is used for the 22-element `car_telemetry` array of the [`RawCarTelemetryData`] type.
///
/// ## Specification
/// ```text
/// speed                     Speed of car in kilometres per hour
/// throttle                  Amount of throttle applied (0.0 to 1.0)
/// steer                     Steering (-1.0 (full lock left) to 1.0 (full lock right))
/// brake                     Amount of brake applied (0 to 1.0)
/// clutch                    Amount of clutch applied (0 to 100)
/// gear                      Gear selected (1-8, N=0, R=-1)
/// engine_rpm                Engine RPM
/// drs                       0 = off, 1 = on
/// rev_lights_percent        Rev lights indicator (percentage)
/// rev_lights_bit_value      Rev lights (bit 0 = leftmost LED, bit 14 = rightmost LED)
/// brakes_temperature        Brakes temperature (celsius)
/// tyres_surface_temperature Tyres surface temperature (celsius)
/// tyres_inner_temperature   Tyres inner temperature (celsius)
/// engine_temperature        Engine temperature (celsius)
/// tyre_pressures            Tyres pressure (PSI)
/// surface_type              Driving surface, see appendices
/// ```
///
/// ### Surface Types
/// ```text
/// ID  Surface
/// 0   Tarmac
/// 1   Rumble strip
/// 2   Concrete
/// 3   Rock
/// 4   Gravel
/// 5   Mud
/// 6   Sand
/// 7   Grass
/// 8   Water
/// 9   Cobblestone
/// 10  Metal
/// 11  Ridged
/// ```
#[derive(Deserialize)]
struct RawCarTelemetry {
    speed: u16,
    throttle: f32,
    steer: f32,
    brake: f32,
    clutch: u8,
    gear: i8,
    engine_rpm: u16,
    drs: bool,
    rev_lights_percent: u8,
    rev_lights_bit_value: u16,
    brakes_temperature: WheelData<u16>,
    tyres_surface_temperature: WheelData<u8>,
    tyres_inner_temperature: WheelData<u8>,
    engine_temperature: u16,
    tyre_pressures: WheelData<f32>,
    surface_types: WheelData<u8>,
}

impl TryFrom<&RawCarTelemetry> for CarTelemetryData {
    type Error = UnpackError;

    fn try_from(packet: &RawCarTelemetry) -> Result<Self, Self::Error> {
        let surface_types = WheelData {
            rear_left: unpack_surface_type(packet.surface_types.rear_left)?,
            rear_right: unpack_surface_type(packet.surface_types.rear_right)?,
            front_left: unpack_surface_type(packet.surface_types.front_left)?,
            front_right: unpack_surface_type(packet.surface_types.front_right)?,
        };

        Ok(Self {
            speed: packet.speed,
            throttle: packet.throttle,
            steer: packet.steer,
            brake: packet.brake,
            clutch: packet.clutch,
            gear: packet.gear,
            engine_rpm: packet.engine_rpm,
            drs: packet.drs,
            rev_lights_percent: packet.rev_lights_percent,
            rev_lights_bit_value: Some(packet.rev_lights_bit_value),
            brakes_temperature: packet.brakes_temperature,
            tyres_surface_temperature: packet.tyres_surface_temperature.into(),
            tyres_inner_temperature: packet.tyres_inner_temperature.into(),
            engine_temperature: packet.engine_temperature,
            tyre_pressures: packet.tyre_pressures,
            surface_types,
        })
    }
}

pub(crate) fn parse_car_telemetry_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarTelemetryData, UnpackError> {
    assert_packet_size(size, CAR_TELEMETRY_PACKET_SIZE)?;

    let packet: RawCarTelemetryData = bincode::deserialize_from(reader)?;

    let car_telemetry_data = packet
        .car_telemetry
        .iter()
        .map(|ct| ct.try_into())
        .collect::<Result<Vec<CarTelemetryData>, UnpackError>>()?;

    let mfd_panel = unpack_mfd_panel(packet.mfd_panel_index)?;
    let secondary_player_mfd_panel = unpack_mfd_panel(packet.mfd_panel_index_secondary_player)?;

    Ok(PacketCarTelemetryData {
        header,
        car_telemetry_data,
        mfd_panel,
        secondary_player_mfd_panel,
        suggested_gear: Some(packet.suggested_gear),
        button_status: None,
    })
}
