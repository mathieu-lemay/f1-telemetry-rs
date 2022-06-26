use serde::Serialize;

use crate::packet::generic::Flag;

use super::header::PacketHeader;

#[derive(Debug, Clone, Serialize, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Copy, Eq, PartialEq)]
pub enum TemperatureChange {
    Up,
    Down,
    NoChange,
}

impl Default for TemperatureChange {
    fn default() -> Self {
        Self::Up // The default in F1 2021 for default WeatherForecast
    }
}

#[derive(Debug, Clone, Serialize, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Copy, Eq, PartialEq)]
pub enum Formula {
    F1Modern,
    F1Classic,
    F2,
    F1Generic,
}

#[derive(Debug, Clone, Serialize, Copy, Eq, PartialEq)]
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

/// Weather forecast
///
/// ## Specification
/// ```text
/// number_of_samples: Number of available forecasts
/// samples:           List of forecast samples
/// accuracy:          Accuracy of the forecasts
/// ```
#[derive(Debug, Clone, Serialize, Default, Eq, PartialEq)]
pub struct WeatherForecast {
    pub number_of_samples: u8,
    pub samples: Vec<WeatherForecastSample>,
    pub accuracy: ForecastAccuracy,
}

/// Defines the weather forecast for a given time in the future.
///
/// ## Specification
/// ```text
/// session_type:             Type of session the forecast applies to. See [`SessionType`].
/// time_offset:              Time in minutes the forecast is for.
/// weather:                  Expected weather. See [`Weather`].
/// track_temperature:        Track temperature in degrees celsius.
/// track_temperature_change: Track temperature change.
/// air_temperature:          Air temperature in degrees celsius.
/// air_temperature_change:   Air temperature change.
/// rain_percentage:          Rain percentage
/// ```
#[derive(Debug, Copy, Clone, Serialize, Eq, PartialEq, Default)]
pub struct WeatherForecastSample {
    pub session_type: SessionType,
    pub time_offset: u8,
    pub weather: Weather,
    pub track_temperature: i8,
    pub track_temperature_change: TemperatureChange,
    pub air_temperature: i8,
    pub air_temperature_change: TemperatureChange,
    pub rain_percentage: u8,
}

/// This type is used for the `marshal_zones` array of the [`PacketSessionData`] type.
///
/// ## Specification
/// ```text
/// zone_start: Fraction (0..1) of way through the lap the marshal zone starts.
/// zone_flag:  Flag active in the zone. See [`Flag`].
/// ```
/// See also [`Flag`].
#[derive(Debug, PartialEq, Clone, Serialize, Copy)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: Flag,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum ForecastAccuracy {
    Perfect,
    Approximate,
    Unknown,
}

impl Default for ForecastAccuracy {
    fn default() -> Self {
        ForecastAccuracy::Unknown
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum BrakingAssist {
    Off,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum GearboxAssist {
    Manual,
    ManualAndSuggestedGear,
    Automatic,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DynamicRacingLine {
    Off,
    CornersOnly,
    Full,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DynamicRacingLineType {
    TwoDimensions,
    ThreeDimensions,
}

/// Status of various driving assistances.
///
/// ```text
/// steering_assist:                Wether steering assist is on or not.
/// braking_assist:                 Braking assist. See [`BrakingAssist`]
/// gearbox_assist:                 Gearbox assist. See [`GearboxAssist`]
/// pit_assist:                     Wether pit assist is on or not.
/// pit_relase_assist:              Wether pit release assist is on or not.
/// ers_assist:                     Wether ERS assist is on or not.
/// drs_assist:                     Wether DRS assist is on or not.
/// dynamic_racing_line:            Dynamic racing line. See [`DynamicRacingLine`]
/// dynamic_racing_line_type:       Dynamic racing line type. See [`DynamicRacingLineType`]
/// ```
/// See also: [`BrakingAssist`], [`DynamicRacingLine`], [`DynamicRacingLineType`] and
/// [`GearboxAssist`].
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DrivingAssists {
    pub steering_assist: bool,
    pub braking_assist: BrakingAssist,
    pub gearbox_assist: GearboxAssist,
    pub pit_assist: bool,
    pub pit_relase_assist: bool,
    pub ers_assist: bool,
    pub drs_assist: bool,
    pub dynamic_racing_line: DynamicRacingLine,
    pub dynamic_racing_line_type: DynamicRacingLineType,
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
/// weather_forecast:               Weather forecast for the rest of the session and next one
///                                 (if available)
/// ai_difficulty:                  AI Difficulty rating – 0-110
/// season_identifier:              Identifier for season - persists across saves
/// weekend_identifier:             Identifier for weekend - persists across saves
/// session_identifier:             Identifier for session - persists across saves
/// pit_stop_window_ideal_lap:      Ideal lap to pit on for current strategy (player)
/// pit_stop_window_latest_lap:     Latest lap to pit on for current strategy (player)
/// pit_stop_rejoin_position:       Predicted position to rejoin at (player)
/// driving_assists:                Status of various driving assistances.
/// ```
/// See also: [`DrivingAssists`], [`Formula`], [`MarshalZone`], [`SafetyCar`], [`SessionType`],
/// [`Track`], [`WeatherForecastSample`] and [`Weather`]
#[derive(Debug, Clone, Serialize, PartialEq)]
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
    pub weather_forecast: Option<WeatherForecast>,
    pub ai_difficulty: Option<u8>,
    pub season_identifier: Option<u32>,
    pub weekend_identifier: Option<u32>,
    pub session_identifier: Option<u32>,
    pub pit_stop_window_ideal_lap: Option<u8>,
    pub pit_stop_window_latest_lap: Option<u8>,
    pub pit_stop_rejoin_position: Option<u8>,
    pub driving_assists: Option<DrivingAssists>,
}
