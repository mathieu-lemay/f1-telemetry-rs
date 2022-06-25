use serde::Serialize;

use crate::packet::generic::Flag;

use super::header::PacketHeader;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum Formula {
    F1Modern,
    F1Classic,
    F2,
    F1Generic,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
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
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub struct WeatherForecast {
    /// Number of available forecasts
    pub number_of_samples: u8,
    /// List of forecast samples
    pub samples: Vec<WeatherForecastSample>,
    /// Accuracy of the forecasts
    pub accuracy: ForecastAccuracy,
}

/// Defines the weather forecast for a given time in the future
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Default)]
pub struct WeatherForecastSample {
    /// Type of session the forecast applies to
    pub session_type: SessionType,
    /// Time in minutes the forecast is for
    pub time_offset: u8,
    /// Expected weather
    pub weather: Weather,
    /// Track temperature in degrees celsius
    pub track_temperature: i8,
    /// Track temperature change
    pub track_temperature_change: TemperatureChange,
    /// Air temperature in degrees celsius
    pub air_temperature: i8,
    /// Air temperature change
    pub air_temperature_change: TemperatureChange,
    /// Rain probability (percentage)
    pub rain_percentage: u8,
}

/// Description of a marshal zone
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub struct MarshalZone {
    /// Fraction (0..1) of way through the lap the marshal zone starts
    pub zone_start: f32,
    /// Flag active in the zone
    pub zone_flag: Flag,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum BrakingAssist {
    Off,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum GearboxAssist {
    Manual,
    ManualAndSuggestedGear,
    Automatic,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DynamicRacingLine {
    Off,
    CornersOnly,
    Full,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DynamicRacingLineType {
    TwoDimensions,
    ThreeDimensions,
}

/// Status of various driving assistances
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DrivingAssists {
    /// Wether steering assist is on or not
    pub steering_assist: bool,
    /// Braking assist
    pub braking_assist: BrakingAssist,
    /// Gearbox assist
    pub gearbox_assist: GearboxAssist,
    /// Wether pit assist is on or not
    pub pit_assist: bool,
    /// Wether pit release assist is on or not
    pub pit_relase_assist: bool,
    /// Wether ERS assist is on or not
    pub ers_assist: bool,
    /// Wether DRS assist is on or not
    pub drs_assist: bool,
    /// Dynamic racing line
    pub dynamic_racing_line: DynamicRacingLine,
    /// Dynamic racing line type
    pub dynamic_racing_line_type: DynamicRacingLineType,
}

/// The session packet includes details about the current session in progress
///
/// Frequency: 2 per second
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketSessionData {
    /// Packet Header
    pub header: PacketHeader,
    /// Current weather
    pub weather: Weather,
    /// Track temperature in celsius
    pub track_temperature: i8,
    /// Air temperature in celsius
    pub air_temperature: i8,
    /// Total number of laps in this race
    pub total_laps: u8,
    /// Track length in metres
    pub track_length: u16,
    /// Type of session
    pub session_type: SessionType,
    /// Current track
    pub track: Track,
    /// Current formula
    pub formula: Formula,
    /// Time left in session in seconds
    pub session_time_left: u16,
    /// Session duration in seconds
    pub session_duration: u16,
    /// Pit speed limit in kilometres per hour
    pub pit_speed_limit: u8,
    /// Whether the game is paused
    pub game_paused: bool,
    /// Whether the player is spectating
    pub is_spectating: bool,
    /// Index of the car being spectated
    pub spectator_car_index: u8,
    /// Whether SLI pro is active
    pub sli_pro_native_support: bool,
    /// Number of marshal zones
    pub num_marshal_zones: u8,
    /// List of marshal zones
    pub marshal_zones: Vec<MarshalZone>,
    /// Safety car status
    pub safety_car_status: SafetyCar,
    /// Whether the game is online or not
    pub network_game: bool,
    /// Weather forecast for the rest of the session and next ones, if any
    pub weather_forecast: Option<WeatherForecast>,
    /// AI Difficulty rating – 0-110
    pub ai_difficulty: Option<u8>,
    /// Identifier for season - persists across saves
    pub season_identifier: Option<u32>,
    /// Identifier for weekend - persists across saves
    pub weekend_identifier: Option<u32>,
    /// Identifier for session - persists across saves
    pub session_identifier: Option<u32>,
    /// Ideal lap to pit on for current strategy (player)
    pub pit_stop_window_ideal_lap: Option<u8>,
    /// Latest lap to pit on for current strategy (player)
    pub pit_stop_window_latest_lap: Option<u8>,
    /// Predicted position to rejoin at (player)
    pub pit_stop_rejoin_position: Option<u8>,
    /// Status of various driving assistances
    pub driving_assists: Option<DrivingAssists>,
}
