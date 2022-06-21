use crate::packet::generic::{Nationality, Team};

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
    EstebanOcon,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Telemetry {
    Restricted,
    Public,
}

/// This type is used for the `participants` array of the `PacketParticipantsData` type.
///
/// ## Specification
/// ```text
/// ai_controlled:  Set to true if the vehicle is AI controlled.
/// driver:         Driver. See [`Driver`].
/// team:           Team. See [`Team`].
/// race_number:    Race number of the car.
/// nationality:    Nationality of the driver.
/// name:           Name of participant.
/// your_telemetry: The player's UDP setting. See [`Telemetry`].
/// ```
///
/// See also [`Driver`], [`Team`] and [`Telemetry`]
#[derive(Debug, Clone, PartialEq)]
pub struct ParticipantData {
    pub ai_controlled: bool,
    pub driver: Driver,
    pub team: Team,
    pub race_number: u8,
    pub nationality: Nationality,
    pub name: String,
    pub telemetry_access: Telemetry,
}

/// This is a list of participants in the race. If the vehicle is controlled by AI, then the name
/// will be the driver name. If this is a multiplayer game, the names will be the Steam Id on PC, or
/// the LAN name if appropriate.
///
/// N.B. on Xbox One, the names will always be the driver name, on PS4 the name will be the LAN name
/// if playing a LAN game, otherwise it will be the driver name.
///
/// The array should be indexed by vehicle index.
///
/// Frequency: Every 5 seconds
///
/// ## Specification
/// ```text
/// header:          Header
/// num_active_cars: Number of active cars in the data – should match number of
///                  cars on HUD
/// participants:    List of participants
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PacketParticipantsData {
    pub header: PacketHeader,
    pub num_active_cars: u8,
    pub participants: Vec<ParticipantData>,
}
