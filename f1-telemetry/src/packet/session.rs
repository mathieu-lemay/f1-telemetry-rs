use serde::Deserialize;

use crate::packet::generic::Flag;

use super::header::PacketHeader;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
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
#[derive(Debug, Eq, PartialEq)]
pub struct WeatherForecastSample {
    pub session_type: SessionType,
    pub time_offset: u8,
    pub weather: Weather,
    pub track_temperature: i8,
    pub air_temperature: i8,
}

/// This type is used for the `marshal_zones` array of the [`PacketSessionData`] type.
///
/// ## Specification
/// ```text
/// zone_start: Fraction (0..1) of way through the lap the marshal zone starts.
/// zone_flag:  Flag active in the zone. See [`Flag`].
/// ```
/// See also [`Flag`].
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: Flag,
}

/// The session packet includes details about the current session in progress.
///
/// Frequency: 2 per second
///
/// ## Specification
/// ```text
/// header:                         Header.
/// weather:                        Current weather. See [`Weather`].
/// track_temperature:              Track temperature in celsius.
/// air_temperature:                Air temperature in celsius.
/// total_laps:                     Total number of laps in this race.
/// track_length:                   Track length in metres.
/// session_type:                   Type of session. See [`SessionType`].
/// track_id:                       Current track. See [`Track`].
/// formula:                        Current formula. See [`Formula`].
/// session_time_left:              Time left in session in seconds.
/// session_duration:               Session duration in seconds.
/// pit_speed_limit:                Pit speed limit in kilometres per hour.
/// game_paused:                    Whether the game is paused.
/// is_spectating:                  Whether the player is spectating.
/// spectator_car_index:            Index of the car being spectated.
/// sli_pro_native_support:         Whether SLI pro is active.
/// num_marshal_zones:              Number of marshal zones to follow.
/// marshal_zones:                  List of marshal zones. See [`MarshalZone`].
/// safety_car_status               Safety car status. See [`SafetyCar`].
/// network_game:                   Whether the game is online or not.
/// num_weather_forecast_samples    Number of weather samples to follow.
/// weather_forecast_samples        List of weather forecast samples. See [`WeatherForecastSample`].
/// ```
/// See also: [`Formula`], [`MarshalZone`], [`SafetyCar`], [`SessionType`], [`Track`],
/// [`WeatherForecastSample`] and [`Weather`]
#[derive(Debug, PartialEq)]
pub struct PacketSessionData {
    pub header: PacketHeader,
    pub weather: Weather,
    pub track_temperature: i8,
    pub air_temperature: i8,
    pub total_laps: u8,
    pub track_length: u16,
    pub session_type: SessionType,
    pub track: Track,
    pub formula: Formula,
    pub session_time_left: u16,
    pub session_duration: u16,
    pub pit_speed_limit: u8,
    pub game_paused: bool,
    pub is_spectating: bool,
    pub spectator_car_index: u8,
    pub sli_pro_native_support: bool,
    pub num_marshal_zones: u8,
    pub marshal_zones: Vec<MarshalZone>,
    pub safety_car_status: SafetyCar,
    pub network_game: bool,
    pub num_weather_forecast_samples: u8,
    pub weather_forecast_samples: Vec<WeatherForecastSample>,
}
