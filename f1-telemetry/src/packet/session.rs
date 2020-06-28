use byteorder::{LittleEndian, ReadBytesExt};
use getset::{CopyGetters, Getters};
use std::convert::TryFrom;
use std::io::BufRead;

use super::header::PacketHeader;
use crate::packet::generic::Flag;
use crate::packet::UnpackError;

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
#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct MarshalZone {
    zone_start: f32,
    zone_flag: Flag,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl SessionType {
    pub fn name<'a>(self) -> &'a str {
        match self {
            SessionType::Unknown => "Unknown",
            SessionType::Practice1 => "Free Practice 1",
            SessionType::Practice2 => "Free Practice 2",
            SessionType::Practice3 => "Free Practice 3",
            SessionType::PracticeShort => "Free Practice (Short)",
            SessionType::Qualifying1 => "Qualifying 1",
            SessionType::Qualifying2 => "Qualifying 2",
            SessionType::Qualifying3 => "Qualifying 3",
            SessionType::QualifyingShort => "Qualifying (Short)",
            SessionType::OneShotQualifying => "One-Shot Qualifying",
            SessionType::Race => "Race",
            SessionType::Race2 => "Race 2",
            SessionType::TimeTrial => "Time Trial",
        }
    }
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Track {
    Melbourne,
    PaulRicard,
    Shanghai,
    Sakhir,
    Catalunya,
    Monaco,
    Montreal,
    Silverstone,
    Hockenheim,
    Hungaroring,
    Spa,
    Monza,
    Singapore,
    Suzuka,
    AbuDhabi,
    Texas,
    Brazil,
    Austria,
    Sochi,
    Mexico,
    Baku,
    SakhirShort,
    SilverstoneShort,
    TexasShort,
    SuzukaShort,
    Unknown,
}

impl Track {
    pub fn name<'a>(self) -> &'a str {
        match self {
            Track::Melbourne => "Melbourne Grand Prix Circuit",
            Track::PaulRicard => "Circuit Paul Ricard",
            Track::Shanghai => "Shanghai International Circuit",
            Track::Sakhir => "Bahrain International Circuit",
            Track::Catalunya => "Circuit de Barcelona-Catalunya",
            Track::Monaco => "Circuit de Monaco",
            Track::Montreal => "Circuit Gilles Villeneuve",
            Track::Silverstone => "Silverstone Circuit",
            Track::Hockenheim => "Hockenheimring",
            Track::Hungaroring => "Hungaroring",
            Track::Spa => "Circuit de Spa-Francorchamps",
            Track::Monza => "Autodromo Nazionale Monza",
            Track::Singapore => "Marina Bay Street Circuit",
            Track::Suzuka => "Suzuka International Racing Course",
            Track::AbuDhabi => "Yas Marina Circuit",
            Track::Texas => "Circuit of the Americas",
            Track::Brazil => "Autódromo José Carlos Pace",
            Track::Austria => "Red Bull Ring",
            Track::Sochi => "Sochi Autodrom",
            Track::Mexico => "Autódromo Hermanos Rodríguez",
            Track::Baku => "Baku City Circuit",
            Track::SakhirShort => "Bahrain International Circuit (Short)",
            Track::SilverstoneShort => "Silverstone Circuit (Short)",
            Track::TexasShort => "Circuit of the Americas (Short)",
            Track::SuzukaShort => "Suzuka International Racing Course (Short)",
            Track::Unknown => "[UNKNOWN]",
        }
    }
}

impl TryFrom<i8> for Track {
    type Error = UnpackError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Track::Melbourne),
            1 => Ok(Track::PaulRicard),
            2 => Ok(Track::Shanghai),
            3 => Ok(Track::Sakhir),
            4 => Ok(Track::Catalunya),
            5 => Ok(Track::Monaco),
            6 => Ok(Track::Montreal),
            7 => Ok(Track::Silverstone),
            8 => Ok(Track::Hockenheim),
            9 => Ok(Track::Hungaroring),
            10 => Ok(Track::Spa),
            11 => Ok(Track::Monza),
            12 => Ok(Track::Singapore),
            13 => Ok(Track::Suzuka),
            14 => Ok(Track::AbuDhabi),
            15 => Ok(Track::Texas),
            16 => Ok(Track::Brazil),
            17 => Ok(Track::Austria),
            18 => Ok(Track::Sochi),
            19 => Ok(Track::Mexico),
            20 => Ok(Track::Baku),
            21 => Ok(Track::SakhirShort),
            22 => Ok(Track::SilverstoneShort),
            23 => Ok(Track::TexasShort),
            24 => Ok(Track::SuzukaShort),
            -1 => Ok(Track::Unknown),
            _ => Err(UnpackError(format!("Invalid Track value: {}", value))),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SafetyCar {
    None,
    Full,
    Virtual,
}

impl SafetyCar {
    pub fn name<'a>(self) -> &'a str {
        match self {
            SafetyCar::None => "No Safety Car",
            SafetyCar::Virtual => "Virtual Safety Car",
            SafetyCar::Full => "Safety Car",
        }
    }
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
/// num_marshal_zones:      Number of marshal zones to follow
/// marshal_zones:          List of marshal zones – max 21
/// safety_car_status:      0 = no safety car, 1 = full safety car
///                         2 = virtual safety car
/// network_game:           0 = offline, 1 = online
/// ```
#[derive(Debug, CopyGetters, Getters)]
pub struct PacketSessionData {
    #[getset(get = "pub")]
    header: PacketHeader,
    #[getset(get_copy = "pub")]
    weather: Weather,
    #[getset(get_copy = "pub")]
    track_temperature: i8,
    #[getset(get_copy = "pub")]
    air_temperature: i8,
    #[getset(get_copy = "pub")]
    total_laps: u8,
    #[getset(get_copy = "pub")]
    track_length: u16,
    #[getset(get_copy = "pub")]
    session_type: SessionType,
    #[getset(get_copy = "pub")]
    track: Track,
    #[getset(get_copy = "pub")]
    formula: Formula,
    #[getset(get_copy = "pub")]
    session_time_left: u16,
    #[getset(get_copy = "pub")]
    session_duration: u16,
    #[getset(get_copy = "pub")]
    pit_speed_limit: u8,
    #[getset(get_copy = "pub")]
    game_paused: u8,
    #[getset(get_copy = "pub")]
    is_spectating: bool,
    #[getset(get_copy = "pub")]
    spectator_car_index: u8,
    #[getset(get_copy = "pub")]
    sli_pro_native_support: bool,
    #[getset(get_copy = "pub")]
    num_marshal_zones: u8,
    #[getset(get = "pub")]
    marshal_zones: Vec<MarshalZone>,
    #[getset(get_copy = "pub")]
    safety_car_status: SafetyCar,
    #[getset(get_copy = "pub")]
    network_game: bool,
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
        let track = Track::try_from(reader.read_i8().unwrap())?;
        let formula = Formula::try_from(reader.read_u8().unwrap())?;
        let session_time_left = reader.read_u16::<LittleEndian>().unwrap();
        let session_duration = reader.read_u16::<LittleEndian>().unwrap();
        let pit_speed_limit = reader.read_u8().unwrap();
        let game_paused = reader.read_u8().unwrap();
        let is_spectating = reader.read_u8().unwrap() == 1;
        let spectator_car_index = reader.read_u8().unwrap();
        let sli_pro_native_support = reader.read_u8().unwrap() == 1;
        let num_marshal_zones = reader.read_u8().unwrap();

        let mut marshal_zones = Vec::with_capacity(21);
        for _ in 0..21 {
            let mz = MarshalZone::new(&mut reader)?;
            marshal_zones.push(mz);
        }

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
            track,
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
