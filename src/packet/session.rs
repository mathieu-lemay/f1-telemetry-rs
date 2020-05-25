use super::header::PacketHeader;
use crate::packet::UnpackError;
use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::BufRead;

#[derive(Debug)]
pub enum Flag {
    None,
    Green,
    Blue,
    Yellow,
    Red,
    Invalid,
}

impl TryFrom<i8> for Flag {
    type Error = UnpackError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Flag::None),
            1 => Ok(Flag::Green),
            2 => Ok(Flag::Blue),
            3 => Ok(Flag::Yellow),
            4 => Ok(Flag::Red),
            -1 => Ok(Flag::Invalid),
            _ => Err(UnpackError(format!("Invalid Flag value: {}", value))),
        }
    }
}

/// This type is used for the 21-element `marshal_zones` array of the [`PacketSessionData`] type.
///
/// Size: 5 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// zone_start: Fraction (0..1) of way through the lap the marshal zone starts
/// zone_flag:  -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
/// ```
/// [`PacketSessionData`]: ./struct.PacketSessionData.html
#[derive(Debug)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: Flag,
}

impl MarshalZone {
    pub fn new<T: BufRead>(reader: &mut T) -> Result<MarshalZone, UnpackError> {
        let zone_start = reader.read_f32::<LittleEndian>().unwrap();
        let zone_flag = Flag::try_from(reader.read_i8().unwrap())?;

        Ok(MarshalZone {
            zone_start,
            zone_flag,
        })
    }
}

#[derive(Debug)]
pub enum Weather {
    Clear,
    LightCloud,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
}

impl TryFrom<u8> for Weather {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Weather::Clear),
            1 => Ok(Weather::LightCloud),
            2 => Ok(Weather::Overcast),
            3 => Ok(Weather::LightRain),
            4 => Ok(Weather::HeavyRain),
            5 => Ok(Weather::Storm),
            _ => Err(UnpackError(format!("Invalid Weather value: {}", value))),
        }
    }
}

#[derive(Debug)]
pub enum SessionType {
    Unknown,
    Practice1,
    Practice2,
    Practice3,
    PracticeShort,
    Qualifying1,
    Qualifying2,
    Qualifying3,
    QualifyingShort,
    OneShotQualifying,
    Race,
    Race2,
    TimeTrial,
}

impl TryFrom<u8> for SessionType {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SessionType::Unknown),
            1 => Ok(SessionType::Practice1),
            2 => Ok(SessionType::Practice2),
            3 => Ok(SessionType::Practice3),
            4 => Ok(SessionType::PracticeShort),
            5 => Ok(SessionType::Qualifying1),
            6 => Ok(SessionType::Qualifying2),
            7 => Ok(SessionType::Qualifying3),
            8 => Ok(SessionType::QualifyingShort),
            9 => Ok(SessionType::OneShotQualifying),
            10 => Ok(SessionType::Race),
            11 => Ok(SessionType::Race2),
            12 => Ok(SessionType::TimeTrial),
            _ => Err(UnpackError(format!("Invalid SessionType value: {}", value))),
        }
    }
}

#[derive(Debug)]
pub enum Formula {
    F1Modern,
    F1Classic,
    F2,
    F1Generic,
}

impl TryFrom<u8> for Formula {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Formula::F1Modern),
            1 => Ok(Formula::F1Classic),
            2 => Ok(Formula::F2),
            3 => Ok(Formula::F1Generic),
            _ => Err(UnpackError(format!("Invalid Formula value: {}", value))),
        }
    }
}

#[derive(Debug)]
pub enum SafetyCar {
    None,
    Full,
    Virtual,
}

impl TryFrom<u8> for SafetyCar {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SafetyCar::None),
            1 => Ok(SafetyCar::Full),
            2 => Ok(SafetyCar::Virtual),
            _ => Err(UnpackError(format!("Invalid SafetyCar value: {}", value))),
        }
    }
}

/// The session packet includes details about the current session in progress.
///
/// Frequency: 2 per second
///
/// Size: 149 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                 Header
/// weather:                Weather - 0 = clear, 1 = light cloud, 2 = overcast
///                         3 = light rain, 4 = heavy rain, 5 = storm
/// track_temperature:      Track temp. in degrees celsius
/// air_temperature:        Air temp. in degrees celsius
/// total_laps:             Total number of laps in this race
/// track_length:           Track length in metres
/// session_type:           0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
///                         5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
///                         10 = R, 11 = R2, 12 = Time Trial
/// track_id:               -1 for unknown, 0-21 for tracks, see appendix
/// formula:                Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
///                         3 = F1 Generic
/// session_time_left:      Time left in session in seconds
/// session_duration:       Session duration in seconds
/// pit_speed_limit:        Pit speed limit in kilometres per hour
/// game_paused:            Whether the game is paused
/// is_spectating:          Whether the player is spectating
/// spectator_car_index:    Index of the car being spectated
/// sli_pro_native_support: SLI Pro support, 0 = inactive, 1 = active
/// numMarshal_zones:       Number of marshal zones to follow
/// marshal_zones:          List of marshal zones – max 21
/// safety_car_status:      0 = no safety car, 1 = full safety car
///                         2 = virtual safety car
/// network_game:           0 = offline, 1 = online
/// ```
#[derive(Debug)]
pub struct PacketSessionData {
    pub header: PacketHeader,
    pub weather: Weather,
    pub track_temperature: i8,
    pub air_temperature: i8,
    pub total_laps: u8,
    pub track_length: u16,
    pub session_type: SessionType,
    pub track_id: i8,
    pub formula: Formula,
    pub session_time_left: u16,
    pub session_duration: u16,
    pub pit_speed_limit: u8,
    pub game_paused: u8,
    pub is_spectating: u8,
    pub spectator_car_index: u8,
    pub sli_pro_native_support: bool,
    pub num_marshal_zones: u8,
    pub marshal_zones: [MarshalZone; 21],
    pub safety_car_status: SafetyCar,
    pub network_game: bool,
}

impl PacketSessionData {
    pub fn new<T: BufRead>(
        mut reader: &mut T,
        header: PacketHeader,
    ) -> Result<PacketSessionData, UnpackError> {
        let weather = Weather::try_from(reader.read_u8().unwrap())?;
        let track_temperature = reader.read_i8().unwrap();
        let air_temperature = reader.read_i8().unwrap();
        let total_laps = reader.read_u8().unwrap();
        let track_length = reader.read_u16::<LittleEndian>().unwrap();
        let session_type = SessionType::try_from(reader.read_u8().unwrap())?;
        let track_id = reader.read_i8().unwrap();
        let formula = Formula::try_from(reader.read_u8().unwrap())?;
        let session_time_left = reader.read_u16::<LittleEndian>().unwrap();
        let session_duration = reader.read_u16::<LittleEndian>().unwrap();
        let pit_speed_limit = reader.read_u8().unwrap();
        let game_paused = reader.read_u8().unwrap();
        let is_spectating = reader.read_u8().unwrap();
        let spectator_car_index = reader.read_u8().unwrap();
        let sli_pro_native_support = reader.read_u8().unwrap() == 1;
        let num_marshal_zones = reader.read_u8().unwrap();

        // Ugly but avoids double initialization
        let marshal_zones = [
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
        ];

        let safety_car_status = SafetyCar::try_from(reader.read_u8().unwrap())?;
        let network_game = reader.read_u8().unwrap() == 1;

        Ok(PacketSessionData {
            header,
            weather,
            track_temperature,
            air_temperature,
            total_laps,
            track_length,
            session_type,
            track_id,
            formula,
            session_time_left,
            session_duration,
            pit_speed_limit,
            game_paused,
            is_spectating,
            spectator_car_index,
            sli_pro_native_support,
            num_marshal_zones,
            marshal_zones,
            safety_car_status,
            network_game,
        })
    }
}
