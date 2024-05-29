use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum Flag {
    None,
    Green,
    Blue,
    Yellow,
    Red,
    #[default]
    Invalid,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum Nationality {
    American,
    Argentinean,
    Australian,
    Austrian,
    Azerbaijani,
    Bahraini,
    Barbadian,
    Belgian,
    Bolivian,
    Brazilian,
    British,
    Bulgarian,
    Cameroonian,
    Canadian,
    Chilean,
    Chinese,
    Colombian,
    CostaRican,
    Croatian,
    Cypriot,
    Czech,
    Danish,
    Dutch,
    Ecuadorian,
    Emirian,
    English,
    Estonian,
    Finnish,
    French,
    German,
    Ghanaian,
    Greek,
    Guatemalan,
    Honduran,
    HongKonger,
    Hungarian,
    Icelander,
    Indian,
    Indonesian,
    Irish,
    Israeli,
    Italian,
    Jamaican,
    Japanese,
    Jordanian,
    Kuwaiti,
    Latvian,
    Lebanese,
    Lithuanian,
    Luxembourger,
    Malaysian,
    Maltese,
    Mexican,
    Monegasque,
    NewZealander,
    Nicaraguan,
    NorthKorean,
    NorthernIrish,
    Norwegian,
    Omani,
    Pakistani,
    Panamanian,
    Paraguayan,
    Peruvian,
    Polish,
    Portuguese,
    Qatari,
    Romanian,
    Russian,
    Salvadoran,
    Saudi,
    Scottish,
    Serbian,
    Singaporean,
    Slovakian,
    Slovenian,
    SouthAfrican,
    SouthKorean,
    Spanish,
    Swedish,
    Swiss,
    Thai,
    Turkish,
    Ukrainian,
    Uruguayan,
    Venezuelan,
    Vietnamese,
    Welsh,
    #[default]
    Invalid,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum ResultStatus {
    #[default]
    Invalid,
    Inactive,
    Active,
    Finished,
    Disqualified,
    NotClassified,
    Retired,
    DidNotFinish,
}

impl ResultStatus {
    pub fn is_valid(&self) -> bool {
        self != &Self::Invalid
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub enum Team {
    Mercedes,
    Ferrari,
    RedBullRacing,
    Williams,
    RacingPoint,
    Renault,
    ToroRosso,
    Haas,
    McLaren,
    AlfaRomeo,
    AlphaTauri,
    Alpine,
    AstonMartin,
    McLaren1988,
    McLaren1991,
    Williams1992,
    Ferrari1995,
    Williams1996,
    McLaren1998,
    Ferrari2002,
    Ferrari2004,
    Renault2006,
    Ferrari2007,
    RedBull2010,
    Ferrari1976,
    ARTGrandPrix,
    CamposVexatecRacing,
    Carlin,
    CharouzRacingSystem,
    DAMS,
    RussianTime,
    MPMotorsport,
    Pertamina,
    McLaren1990,
    Trident,
    BWTArden,
    McLaren1976,
    Lotus1972,
    Ferrari1979,
    McLaren1982,
    Williams2003,
    Brawn2009,
    Lotus1978,
    ArtGP2019,
    Campos2019,
    Carlin2019,
    SauberJuniorCharouz2019,
    Dams2019,
    UniVirtuosi2019,
    MPMotorsport2019,
    Prema2019,
    Trident2019,
    Arden2019,
    Ferrari1990,
    McLaren2010,
    Ferrari2010,
    McLaren2008,
    F1GenericCar,
    Benetton1994,
    Benetton1995,
    Ferrari2000,
    Jordan1991,
    ArtGP2020,
    Campos2020,
    Carlin2020,
    Charouz2020,
    Dams2020,
    UniVirtuosi2020,
    MPMotorsport2020,
    Prema2020,
    Trident2020,
    BWT2020,
    Hitech2020,
    Mercedes2020,
    Ferrari2020,
    RedBull2020,
    Williams2020,
    RacingPoint2020,
    Renault2020,
    AlphaTauri2020,
    Haas2020,
    McLaren2020,
    AlfaRomeo2020,
    Prema2021,
    UniVirtuosi2021,
    Carlin2021,
    Hitech2021,
    ArtGP2021,
    MPMotorsport2021,
    Charouz2021,
    Dams2021,
    Campos2021,
    BWT2021,
    Trident2021,
    AstonMartinDB11V12,
    AstonMartinVantageF1Edition,
    AstonMartinVantageSafetyCar,
    FerrariF8Tributo,
    FerrariRoma,
    McLaren720S,
    McLarenArtura,
    MercedesAMGGTBlackSeriesSafetyCar,
    MercedesAMGGTRPro,
    F1CustomTeam,
    MercedesAMGGTBlackSeries,
    Mercedes2022,
    Ferrari2022,
    RedBullRacing2022,
    Williams2022,
    AstonMartin2022,
    Alpine2022,
    AlphaTauri2022,
    Haas2022,
    McLaren2022,
    AlfaRomeo2022,
    Konnersport2022,
    Konnersport,
    Prema2022,
    Virtuosi2022,
    Carlin2022,
    MPMotorsport2022,
    Charouz2022,
    Dams2022,
    Campos2022,
    VanAmersfoortRacing2022,
    Trident2022,
    Hitech2022,
    ArtGP2022,
    ArtGP2023,
    Campos2023,
    Carlin2023,
    Phm2023,
    Dams2023,
    Hitech2023,
    MPMotorsport2023,
    Prema2023,
    Trident2023,
    VanAmersfoortRacing2023,
    Virtuosi2023,
    MyTeam,
    F1WorldCar,
    #[default]
    Unknown,
}

impl Team {
    pub fn name(self) -> &'static str {
        match self {
            Team::Mercedes => "Mercedes",
            Team::Ferrari => "Ferrari",
            Team::RedBullRacing => "Red Bull Racing",
            Team::Williams => "Williams",
            Team::RacingPoint => "Racing Point",
            Team::Renault => "Renault",
            Team::ToroRosso => "Toro Rosso",
            Team::Haas => "Haas",
            Team::McLaren => "McLaren",
            Team::AlfaRomeo => "Alfa Romeo",
            Team::AlphaTauri => "Alpha Tauri",
            Team::Alpine => "Alpine",
            Team::AstonMartin => "Aston Martin",
            Team::MyTeam => "My Team",
            _ => "[N/A]",
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum Platform {
    Steam,
    PlayStation,
    Xbox,
    Origin,
    #[default]
    Unknown,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum TyreCompound {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
    #[default]
    Invalid,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum TyreCompoundVisual {
    Soft,
    Medium,
    Hard,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
    #[default]
    Invalid,
}

impl TyreCompoundVisual {
    pub fn name<'a>(self) -> &'a str {
        match self {
            TyreCompoundVisual::Soft => "Soft",
            TyreCompoundVisual::Medium => "Medium",
            TyreCompoundVisual::Hard => "Hard",
            TyreCompoundVisual::Inter => "Intermediate",
            TyreCompoundVisual::Wet => "Wet",
            TyreCompoundVisual::ClassicDry => "Dry (Classic)",
            TyreCompoundVisual::ClassicWet => "Wet (Classic)",
            TyreCompoundVisual::F2SuperSoft => "Super Soft (F2)",
            TyreCompoundVisual::F2Soft => "Soft (F2)",
            TyreCompoundVisual::F2Medium => "Medium (F2)",
            TyreCompoundVisual::F2Hard => "Hard (F2)",
            TyreCompoundVisual::F2Wet => "Wet (F2)",
            TyreCompoundVisual::Invalid => "Invalid",
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum SessionType {
    #[default]
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
    Race3,
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
            SessionType::Race3 => "Race 3",
            SessionType::TimeTrial => "Time Trial",
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct WheelData<T>
where
    T: Clone + Copy,
{
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

impl<T> WheelData<T>
where
    T: Clone + Copy,
{
    pub fn new(rear_left: T, rear_right: T, front_left: T, front_right: T) -> WheelData<T> {
        WheelData {
            rear_left,
            rear_right,
            front_left,
            front_right,
        }
    }
}

impl From<WheelData<u8>> for WheelData<u16> {
    fn from(wheel_data: WheelData<u8>) -> Self {
        Self {
            rear_left: wheel_data.rear_left as u16,
            rear_right: wheel_data.rear_right as u16,
            front_left: wheel_data.front_left as u16,
            front_right: wheel_data.front_right as u16,
        }
    }
}
