use getset::{CopyGetters, Getters};

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
    telemetry_access: Telemetry,
}

impl ParticipantData {
    pub(crate) fn new(
        ai_controlled: bool,
        driver: Driver,
        team: Team,
        race_number: u8,
        nationality: Nationality,
        name: String,
        telemetry_access: Telemetry,
    ) -> ParticipantData {
        ParticipantData {
            ai_controlled,
            driver,
            team,
            race_number,
            nationality,
            name,
            telemetry_access,
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
