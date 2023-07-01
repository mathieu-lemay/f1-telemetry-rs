use serde::Serialize;

use crate::packet::generic::{Nationality, Team};

use super::{generic::Platform, header::PacketHeader};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum Driver {
    CarlosSainz,
    DaniilKvyat,
    DanielRicciardo,
    FernandoAlonso,
    FelipeMassa,
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
    AlainProst,
    AyrtonSenna,
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
    MichaelSchumacher,
    DanTicktum,
    MarcusArmstrong,
    ChristianLundgaard,
    YukiTsunoda,
    JehanDaruvala,
    GulhermeSamaia,
    PedroPiquet,
    FelipeDrugovich,
    RobertSchwartzman,
    RoyNissany,
    MarinoSato,
    AidanJackson,
    CasperAkkerman,
    JensonButton,
    DavidCoulthard,
    NicoRosberg,
    OscarPiastri,
    LiamLawson,
    JuriVips,
    TheoPourchaire,
    RichardVerschoor,
    LirimZendeli,
    DavidBeckmann,
    GianlucaPetecof,
    MatteoNannini,
    AlessioDeledda,
    BentViscaal,
    EnzoFittipaldi,
    DevonButler,
    LukasWeber,
    MarkWebber,
    JacquesVilleneuve,
    CallieMayer,
    NoahBell,
    JakeHughes,
    FrederikVesti,
    OlliCaldwell,
    LoganSargeant,
    CemBolukbasi,
    AyumuIwasa,
    ClementNovalak,
    JackDoohan,
    AmauryCordeel,
    DennisHauger,
    CalanWilliams,
    JamieChadwick,
    KamuiKobayashi,
    PastorMaldonado,
    MikaHakkinen,
    NigelMansell,
    Player,
    #[default]
    Unknown,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum Telemetry {
    #[default]
    Restricted,
    Public,
}

/// This type is used for the `participants` array of the `PacketParticipantsData` type.
///
/// See also [`Driver`], [`Team`] and [`Telemetry`]
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub struct ParticipantData {
    /// Set to true if the vehicle is AI controlled.
    pub ai_controlled: bool,
    /// Driver. See [`Driver`].
    pub driver: Driver,
    /// Network id – unique identifier for network players
    pub network_id: Option<u8>,
    /// Team. See [`Team`].
    pub team: Team,
    /// My team flag – true = My Team, false = otherwise
    pub my_team: bool,
    /// Race number of the car.
    pub race_number: u8,
    /// Nationality of the driver.
    pub nationality: Nationality,
    /// Name of participant.
    pub name: String,
    /// The player's UDP setting. See [`Telemetry`].
    pub telemetry_access: Telemetry,
    /// The player's show online names setting
    pub show_online_names: bool,
    /// Gaming platform used by the player. New in F1 23.
    pub platform: Platform,
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct PacketParticipantsData {
    /// Packet header
    pub header: PacketHeader,
    /// Number of active cars in the data – should match number of
    /// cars on HUD
    pub num_active_cars: u8,
    /// List of participants
    pub participants: Vec<ParticipantData>,
}
