use getset::{CopyGetters, Getters};

use crate::packet::generic::Flag;

use super::header::PacketHeader;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Weather {
    Clear,
    LightCloud,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
}

impl Default for Weather {
    fn default() -> Self {
        Self::Clear
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

impl Default for SessionType {
    fn default() -> Self {
        Self::Unknown
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
    Hanoi,
    Zandvoort,
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
            Track::Hanoi => "Hanoi Street Circuit",
            Track::Zandvoort => "Circuit Zandvoort",
            Track::Unknown => "[UNKNOWN]",
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

impl Default for SafetyCar {
    fn default() -> Self {
        Self::None
    }
}

/// This type is used for the `weather_forecast_sample` array of the [`PacketSessionData`] type.
///
/// Size: 5 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// session_type:      Type of session the forecast applies to. See [`SessionType`].
/// time_offset:       Time in minutes the forecast is for.
/// weather:           Expected weather. See [`Weather`].
/// track_temperature: Track temp. in degrees celsius.
/// air_temperature:   Air temp. in degrees celsius.
/// ```
/// [`PacketSessionData`]: ./struct.PacketSessionData.html
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct WeatherForecastSample {
    session_type: SessionType,
    time_offset: u8,
    weather: Weather,
    track_temperature: i8,
    air_temperature: i8,
}

impl WeatherForecastSample {
    pub(crate) fn new(
        session_type: SessionType,
        time_offset: u8,
        weather: Weather,
        track_temperature: i8,
        air_temperature: i8,
    ) -> WeatherForecastSample {
        WeatherForecastSample {
            session_type,
            time_offset,
            weather,
            track_temperature,
            air_temperature,
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
#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct MarshalZone {
    zone_start: f32,
    zone_flag: Flag,
}

impl MarshalZone {
    pub(crate) fn new(zone_start: f32, zone_flag: Flag) -> MarshalZone {
        MarshalZone {
            zone_start,
            zone_flag,
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
    #[getset(get_copy = "pub")]
    num_weather_forecast_samples: u8,
    #[getset(get = "pub")]
    weather_forecast_samples: Vec<WeatherForecastSample>,
}

#[allow(clippy::too_many_arguments)]
impl PacketSessionData {
    pub(crate) fn from_2019(
        header: PacketHeader,
        weather: Weather,
        track_temperature: i8,
        air_temperature: i8,
        total_laps: u8,
        track_length: u16,
        session_type: SessionType,
        track: Track,
        formula: Formula,
        session_time_left: u16,
        session_duration: u16,
        pit_speed_limit: u8,
        game_paused: u8,
        is_spectating: bool,
        spectator_car_index: u8,
        sli_pro_native_support: bool,
        num_marshal_zones: u8,
        marshal_zones: Vec<MarshalZone>,
        safety_car_status: SafetyCar,
        network_game: bool,
    ) -> PacketSessionData {
        PacketSessionData {
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
            num_weather_forecast_samples: 0,
            weather_forecast_samples: Vec::new(),
        }
    }

    pub(crate) fn from_2020(
        header: PacketHeader,
        weather: Weather,
        track_temperature: i8,
        air_temperature: i8,
        total_laps: u8,
        track_length: u16,
        session_type: SessionType,
        track: Track,
        formula: Formula,
        session_time_left: u16,
        session_duration: u16,
        pit_speed_limit: u8,
        game_paused: u8,
        is_spectating: bool,
        spectator_car_index: u8,
        sli_pro_native_support: bool,
        num_marshal_zones: u8,
        marshal_zones: Vec<MarshalZone>,
        safety_car_status: SafetyCar,
        network_game: bool,
        num_weather_forecast_samples: u8,
        weather_forecast_samples: Vec<WeatherForecastSample>,
    ) -> PacketSessionData {
        PacketSessionData {
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
            num_weather_forecast_samples,
            weather_forecast_samples,
        }
    }
}
