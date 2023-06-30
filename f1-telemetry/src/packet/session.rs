use serde::Serialize;

use crate::packet::generic::Flag;

use super::generic::SessionType;
use super::header::PacketHeader;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum Weather {
    #[default]
    Clear,
    LightCloud,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum TemperatureChange {
    #[default]
    Up, // The default in F1 2021 for WeatherForecast
    Down,
    NoChange,
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
    Imola,
    Portimao,
    Jeddah,
    Miami,
    LasVegas,
    Losail,
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
            Track::Imola => "Autodromo Internazionale Enzo e Dino Ferrari",
            Track::Portimao => "Autódromo Internacional do Algarve",
            Track::Jeddah => "Jeddah Corniche Circuit",
            Track::Miami => "Miami International Autodrome",
            Track::LasVegas => "Las Vegas Street Circuit",
            Track::Losail => "Losail International Circuit",
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

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum SafetyCar {
    #[default]
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
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub enum ForecastAccuracy {
    Perfect,
    Approximate,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum BrakingAssist {
    Off,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum GearboxAssist {
    Manual,
    ManualAndSuggestedGear,
    Automatic,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum DynamicRacingLine {
    Off,
    CornersOnly,
    Full,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum DynamicRacingLineType {
    TwoDimensions,
    ThreeDimensions,
}

/// Status of various driving assistances
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum GameMode {
    EventMode,
    GrandPrix,
    GrandPrix23,
    TimeTrial,
    Splitscreen,
    OnlineCustom,
    OnlineLeague,
    CareerInvitational,
    ChampionshipInvitational,
    Championship,
    OnlineChampionship,
    OnlineWeeklyEvent,
    StoryMode,
    Career22,
    Career22Online,
    Career23,
    Career23Online,
    Benchmark,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum RuleSet {
    PracticeAndQualifying,
    Race,
    TimeTrial,
    TimeAttack,
    CheckpointChallenge,
    Autocross,
    Drift,
    AverageSpeedZone,
    RivalDuel,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum SessionLength {
    None,
    VeryShort,
    Short,
    Medium,
    MediumLong,
    Long,
    Full,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum SpeedUnits {
    MPH,
    KPH,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
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
    /// Whether the game is paused (network game only)
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
    /// Game mode
    pub game_mode: Option<GameMode>,
    /// Rule set
    pub rule_set: Option<RuleSet>,
    /// Local time of day (minutes since midnight)
    pub time_of_day: Option<u32>,
    /// Session Length
    pub session_length: Option<SessionLength>,
    /// Speed units of the lead player. New in F1 23.
    pub speed_units_lead_player: Option<SpeedUnits>,
    /// Temperature units of the lead player. New in F1 23.
    pub temperature_units_lead_player: Option<TemperatureUnits>,
    /// Speed units of the secondary player. New in F1 23.
    pub speed_units_secondary_player: Option<SpeedUnits>,
    /// Temperature units of the secondary player. New in F1 23.
    pub temperature_units_secondary_player: Option<TemperatureUnits>,
    /// Number of safety cars called during session. New in F1 23.
    pub num_safety_car_periods: Option<u8>,
    /// Number of virtual safety cars called. New in F1 23.
    pub num_virtual_safety_car_periods: Option<u8>,
    /// Number of red flags called during session. New in F1 23.
    pub num_red_flag_periods: Option<u8>,
}
