use getset::{CopyGetters, Getters};

use super::header::PacketHeader;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Driver {
    CarlosSainz,
    DaniilKvyat,
    DanielRicciardo,
    KimiRaikkonen,
    LewisHamilton,
    MaxVerstappen,
    NicoHulkenburg,
    KevinMagnussen,
    RomainGrosjean,
    SebastianVettel,
    SergioPerez,
    ValtteriBottas,
    LanceStroll,
    ArronBarnes,
    MartinGiles,
    AlexMurray,
    LucasRoth,
    IgorCorreia,
    SophieLevasseur,
    JonasSchiffer,
    AlainForest,
    JayLetourneau,
    EstoSaari,
    YasarAtiyeh,
    CallistoCalabresi,
    NaotaIzum,
    HowardClarke,
    WilhelmKaufmann,
    MarieLaursen,
    FlavioNieves,
    PeterBelousov,
    KlimekMichalski,
    SantiagoMoreno,
    BenjaminCoppens,
    NoahVisser,
    GertWaldmuller,
    JulianQuesada,
    DanielJones,
    ArtemMarkelov,
    TadasukeMakino,
    SeanGelael,
    NyckDeVries,
    JackAitken,
    GeorgeRussell,
    MaximilianGunther,
    NireiFukuzumi,
    LucaGhiotto,
    LandoNorris,
    SergioSetteCamara,
    LouisDeletraz,
    AntonioFuoco,
    CharlesLeclerc,
    PierreGasly,
    AlexanderAlbon,
    NicholasLatifi,
    DorianBoccolacci,
    NikoKari,
    RobertoMerhi,
    ArjunMaini,
    AlessioLorandi,
    RubenMeijer,
    RashidNair,
    JackTremblay,
    AntonioGiovinazzi,
    RobertKubica,
    NobuharuMatsushita,
    NikitaMazepin,
    GuanyaZhou,
    MickSchumacher,
    CallumIlott,
    JuanManuelCorrea,
    JordanKing,
    MahaveerRaghunathan,
    TatianaCalderon,
    AnthoineHubert,
    GuilianoAlesi,
    RalphBoschung,
    DevonButler,
    LukasWebber,
    Player,
    Unknown,
}

impl Default for Driver {
    fn default() -> Self {
        Driver::Unknown
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
    Unknown,
}

impl Team {
    pub fn id(self) -> u8 {
        match self {
            Team::Mercedes => 0,
            Team::Ferrari => 1,
            Team::RedBullRacing => 2,
            Team::Williams => 3,
            Team::RacingPoint => 4,
            Team::Renault => 5,
            Team::ToroRosso => 6,
            Team::Haas => 7,
            Team::McLaren => 8,
            Team::AlfaRomeo => 9,
            Team::McLaren1988 => 10,
            Team::McLaren1991 => 11,
            Team::Williams1992 => 12,
            Team::Ferrari1995 => 13,
            Team::Williams1996 => 14,
            Team::McLaren1998 => 15,
            Team::Ferrari2002 => 16,
            Team::Ferrari2004 => 17,
            Team::Renault2006 => 18,
            Team::Ferrari2007 => 19,
            Team::RedBull2010 => 21,
            Team::Ferrari1976 => 22,
            Team::ARTGrandPrix => 23,
            Team::CamposVexatecRacing => 24,
            Team::Carlin => 25,
            Team::CharouzRacingSystem => 26,
            Team::DAMS => 27,
            Team::RussianTime => 28,
            Team::MPMotorsport => 29,
            Team::Pertamina => 30,
            Team::McLaren1990 => 31,
            Team::Trident => 32,
            Team::BWTArden => 33,
            Team::McLaren1976 => 34,
            Team::Lotus1972 => 35,
            Team::Ferrari1979 => 36,
            Team::McLaren1982 => 37,
            Team::Williams2003 => 38,
            Team::Brawn2009 => 39,
            Team::Lotus1978 => 40,
            Team::ArtGP2019 => 42,
            Team::Campos2019 => 43,
            Team::Carlin2019 => 44,
            Team::SauberJuniorCharouz2019 => 45,
            Team::Dams2019 => 46,
            Team::UniVirtuosi2019 => 47,
            Team::MPMotorsport2019 => 48,
            Team::Prema2019 => 49,
            Team::Trident2019 => 50,
            Team::Arden2019 => 51,
            Team::Ferrari1990 => 63,
            Team::McLaren2010 => 64,
            Team::Ferrari2010 => 65,
            Team::Unknown => 255,
        }
    }

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
pub enum Nationality {
    American,
    Argentinean,
    Australian,
    Austrian,
    Azerbaijani,
    Bahraini,
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
    English,
    Emirian,
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
    SouthKorean,
    SouthAfrican,
    Spanish,
    Swedish,
    Swiss,
    Thai,
    Turkish,
    Uruguayan,
    Ukrainian,
    Venezuelan,
    Welsh,
    Invalid,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Telemetry {
    Restricted,
    Public,
}

/// This type is used for the 20-element `participants` array of the `PacketParticipantsData` type.
///
/// Size: 54 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// ai_controlled:  Whether the vehicle is AI (1) or Human (0) controlled
/// driver_id:      Driver id - see appendix
/// team_id:        Team id - see appendix
/// race_number:    Race number of the car
/// nationality:    Nationality of the driver
/// name:           Name of participant in UTF-8 format – null terminated
///                 Will be truncated with … (U+2026) if too long
/// your_telemetry: The player's UDP setting, 0 = restricted, 1 = public
/// ```
///
/// [`PacketParticipantsData`]: ./struct.PacketParticipantsData.html
#[derive(Debug, CopyGetters, Getters, Clone)]
pub struct ParticipantData {
    ai_controlled: bool,
    #[getset(get_copy = "pub")]
    driver: Driver,
    #[getset(get_copy = "pub")]
    team: Team,
    #[getset(get_copy = "pub")]
    race_number: u8,
    #[getset(get_copy = "pub")]
    nationality: Nationality,
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    telemetry: Telemetry,
}

impl ParticipantData {
    pub(crate) fn new(
        ai_controlled: bool,
        driver: Driver,
        team: Team,
        race_number: u8,
        nationality: Nationality,
        name: String,
        telemetry: Telemetry,
    ) -> ParticipantData {
        ParticipantData {
            ai_controlled,
            driver,
            team,
            race_number,
            nationality,
            name,
            telemetry,
        }
    }
}

/// This is a list of participants in the race.
///
/// If the vehicle is controlled by AI, then the name will be the driver name.
/// If this is a multiplayer game, the names will be the Steam Id on PC, or the LAN name if appropriate.
/// On Xbox One, the names will always be the driver name, on PS4 the name will be the LAN name if playing a LAN game,
/// otherwise it will be the driver name.
///
/// Frequency: Every 5 seconds
///
/// Size: 1104 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:          Header
/// num_active_cars: Number of active cars in the data – should match number of
///                  cars on HUD
/// participants:    List of participants, max 20.
/// ```
#[derive(Debug, Getters, CopyGetters, Clone)]
pub struct PacketParticipantsData {
    #[getset(get = "pub")]
    header: PacketHeader,
    #[getset(get_copy = "pub")]
    num_active_cars: u8,
    #[getset(get = "pub")]
    participants: Vec<ParticipantData>,
}

impl PacketParticipantsData {
    pub(crate) fn new(
        header: PacketHeader,
        num_active_cars: u8,
        participants: Vec<ParticipantData>,
    ) -> PacketParticipantsData {
        PacketParticipantsData {
            header,
            num_active_cars,
            participants,
        }
    }
}
