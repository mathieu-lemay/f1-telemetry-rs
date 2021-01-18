use getset::CopyGetters;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Flag {
    None,
    Green,
    Blue,
    Yellow,
    Red,
    Invalid,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    Invalid,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResultStatus {
    Invalid,
    Inactive,
    Active,
    Finished,
    Disqualified,
    NotClassified,
    Retired,
}

impl Default for ResultStatus {
    fn default() -> Self {
        Self::Invalid
    }
}

impl ResultStatus {
    pub fn is_valid(&self) -> bool {
        self != &Self::Invalid
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
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
    AlphaTauri,
    McLaren2008,
    F1GenericCar,
    Benetton1994,
    Benetton1995,
    Ferrari2000,
    Jordan1991,
    MyTeam,
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
            Team::MyTeam => "My Team",
            _ => "[N/A]",
        }
    }
}

impl Default for Team {
    fn default() -> Self {
        Team::Unknown
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TyreCompound {
    C5,
    C4,
    C3,
    C2,
    C1,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
    Invalid,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    Invalid,
}

impl Default for TyreCompoundVisual {
    fn default() -> Self {
        TyreCompoundVisual::Invalid
    }
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

#[derive(Debug, Clone, Copy, Default, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct WheelData<T>
where
    T: Clone + Copy,
{
    rear_left: T,
    rear_right: T,
    front_left: T,
    front_right: T,
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
