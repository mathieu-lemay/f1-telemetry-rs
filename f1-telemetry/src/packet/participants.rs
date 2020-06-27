use byteorder::ReadBytesExt;
use getset::{CopyGetters, Getters};
use std::convert::TryFrom;
use std::io::BufRead;

use super::header::PacketHeader;
use crate::packet::UnpackError;

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
}

impl TryFrom<u8> for Driver {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Driver::CarlosSainz),
            1 => Ok(Driver::DaniilKvyat),
            2 => Ok(Driver::DanielRicciardo),
            6 => Ok(Driver::KimiRaikkonen),
            7 => Ok(Driver::LewisHamilton),
            9 => Ok(Driver::MaxVerstappen),
            10 => Ok(Driver::NicoHulkenburg),
            11 => Ok(Driver::KevinMagnussen),
            12 => Ok(Driver::RomainGrosjean),
            13 => Ok(Driver::SebastianVettel),
            14 => Ok(Driver::SergioPerez),
            15 => Ok(Driver::ValtteriBottas),
            19 => Ok(Driver::LanceStroll),
            20 => Ok(Driver::ArronBarnes),
            21 => Ok(Driver::MartinGiles),
            22 => Ok(Driver::AlexMurray),
            23 => Ok(Driver::LucasRoth),
            24 => Ok(Driver::IgorCorreia),
            25 => Ok(Driver::SophieLevasseur),
            26 => Ok(Driver::JonasSchiffer),
            27 => Ok(Driver::AlainForest),
            28 => Ok(Driver::JayLetourneau),
            29 => Ok(Driver::EstoSaari),
            30 => Ok(Driver::YasarAtiyeh),
            31 => Ok(Driver::CallistoCalabresi),
            32 => Ok(Driver::NaotaIzum),
            33 => Ok(Driver::HowardClarke),
            34 => Ok(Driver::WilhelmKaufmann),
            35 => Ok(Driver::MarieLaursen),
            36 => Ok(Driver::FlavioNieves),
            37 => Ok(Driver::PeterBelousov),
            38 => Ok(Driver::KlimekMichalski),
            39 => Ok(Driver::SantiagoMoreno),
            40 => Ok(Driver::BenjaminCoppens),
            41 => Ok(Driver::NoahVisser),
            42 => Ok(Driver::GertWaldmuller),
            43 => Ok(Driver::JulianQuesada),
            44 => Ok(Driver::DanielJones),
            45 => Ok(Driver::ArtemMarkelov),
            46 => Ok(Driver::TadasukeMakino),
            47 => Ok(Driver::SeanGelael),
            48 => Ok(Driver::NyckDeVries),
            49 => Ok(Driver::JackAitken),
            50 => Ok(Driver::GeorgeRussell),
            51 => Ok(Driver::MaximilianGunther),
            52 => Ok(Driver::NireiFukuzumi),
            53 => Ok(Driver::LucaGhiotto),
            54 => Ok(Driver::LandoNorris),
            55 => Ok(Driver::SergioSetteCamara),
            56 => Ok(Driver::LouisDeletraz),
            57 => Ok(Driver::AntonioFuoco),
            58 => Ok(Driver::CharlesLeclerc),
            59 => Ok(Driver::PierreGasly),
            62 => Ok(Driver::AlexanderAlbon),
            63 => Ok(Driver::NicholasLatifi),
            64 => Ok(Driver::DorianBoccolacci),
            65 => Ok(Driver::NikoKari),
            66 => Ok(Driver::RobertoMerhi),
            67 => Ok(Driver::ArjunMaini),
            68 => Ok(Driver::AlessioLorandi),
            69 => Ok(Driver::RubenMeijer),
            70 => Ok(Driver::RashidNair),
            71 => Ok(Driver::JackTremblay),
            72 => Ok(Driver::DevonButler),
            73 => Ok(Driver::LukasWebber),
            74 => Ok(Driver::AntonioGiovinazzi),
            75 => Ok(Driver::RobertKubica),
            78 => Ok(Driver::NobuharuMatsushita),
            79 => Ok(Driver::NikitaMazepin),
            80 => Ok(Driver::GuanyaZhou),
            81 => Ok(Driver::MickSchumacher),
            82 => Ok(Driver::CallumIlott),
            83 => Ok(Driver::JuanManuelCorrea),
            84 => Ok(Driver::JordanKing),
            85 => Ok(Driver::MahaveerRaghunathan),
            86 => Ok(Driver::TatianaCalderon),
            87 => Ok(Driver::AnthoineHubert),
            88 => Ok(Driver::GuilianoAlesi),
            89 => Ok(Driver::RalphBoschung),
            d if d >= 100 => Ok(Driver::Player),
            _ => Err(UnpackError(format!("Invalid Driver value: {}", value))),
        }
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
        }
    }
}

impl TryFrom<u8> for Team {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Team::Mercedes),
            1 => Ok(Team::Ferrari),
            2 => Ok(Team::RedBullRacing),
            3 => Ok(Team::Williams),
            4 => Ok(Team::RacingPoint),
            5 => Ok(Team::Renault),
            6 => Ok(Team::ToroRosso),
            7 => Ok(Team::Haas),
            8 => Ok(Team::McLaren),
            9 => Ok(Team::AlfaRomeo),
            10 => Ok(Team::McLaren1988),
            11 => Ok(Team::McLaren1991),
            12 => Ok(Team::Williams1992),
            13 => Ok(Team::Ferrari1995),
            14 => Ok(Team::Williams1996),
            15 => Ok(Team::McLaren1998),
            16 => Ok(Team::Ferrari2002),
            17 => Ok(Team::Ferrari2004),
            18 => Ok(Team::Renault2006),
            19 => Ok(Team::Ferrari2007),
            21 => Ok(Team::RedBull2010),
            22 => Ok(Team::Ferrari1976),
            23 => Ok(Team::ARTGrandPrix),
            24 => Ok(Team::CamposVexatecRacing),
            25 => Ok(Team::Carlin),
            26 => Ok(Team::CharouzRacingSystem),
            27 => Ok(Team::DAMS),
            28 => Ok(Team::RussianTime),
            29 => Ok(Team::MPMotorsport),
            30 => Ok(Team::Pertamina),
            31 => Ok(Team::McLaren1990),
            32 => Ok(Team::Trident),
            33 => Ok(Team::BWTArden),
            34 => Ok(Team::McLaren1976),
            35 => Ok(Team::Lotus1972),
            36 => Ok(Team::Ferrari1979),
            37 => Ok(Team::McLaren1982),
            38 => Ok(Team::Williams2003),
            39 => Ok(Team::Brawn2009),
            40 => Ok(Team::Lotus1978),
            42 => Ok(Team::ArtGP2019),
            43 => Ok(Team::Campos2019),
            44 => Ok(Team::Carlin2019),
            45 => Ok(Team::SauberJuniorCharouz2019),
            46 => Ok(Team::Dams2019),
            47 => Ok(Team::UniVirtuosi2019),
            48 => Ok(Team::MPMotorsport2019),
            49 => Ok(Team::Prema2019),
            50 => Ok(Team::Trident2019),
            51 => Ok(Team::Arden2019),
            63 => Ok(Team::Ferrari1990),
            64 => Ok(Team::McLaren2010),
            65 => Ok(Team::Ferrari2010),
            _ => Err(UnpackError(format!("Invalid Team value: {}", value))),
        }
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
}

impl TryFrom<u8> for Nationality {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Nationality::American),
            2 => Ok(Nationality::Argentinean),
            3 => Ok(Nationality::Australian),
            4 => Ok(Nationality::Austrian),
            5 => Ok(Nationality::Azerbaijani),
            6 => Ok(Nationality::Bahraini),
            7 => Ok(Nationality::Belgian),
            8 => Ok(Nationality::Bolivian),
            9 => Ok(Nationality::Brazilian),
            10 => Ok(Nationality::British),
            11 => Ok(Nationality::Bulgarian),
            12 => Ok(Nationality::Cameroonian),
            13 => Ok(Nationality::Canadian),
            14 => Ok(Nationality::Chilean),
            15 => Ok(Nationality::Chinese),
            16 => Ok(Nationality::Colombian),
            17 => Ok(Nationality::CostaRican),
            18 => Ok(Nationality::Croatian),
            19 => Ok(Nationality::Cypriot),
            20 => Ok(Nationality::Czech),
            21 => Ok(Nationality::Danish),
            22 => Ok(Nationality::Dutch),
            23 => Ok(Nationality::Ecuadorian),
            24 => Ok(Nationality::English),
            25 => Ok(Nationality::Emirian),
            26 => Ok(Nationality::Estonian),
            27 => Ok(Nationality::Finnish),
            28 => Ok(Nationality::French),
            29 => Ok(Nationality::German),
            30 => Ok(Nationality::Ghanaian),
            31 => Ok(Nationality::Greek),
            32 => Ok(Nationality::Guatemalan),
            33 => Ok(Nationality::Honduran),
            34 => Ok(Nationality::HongKonger),
            35 => Ok(Nationality::Hungarian),
            36 => Ok(Nationality::Icelander),
            37 => Ok(Nationality::Indian),
            38 => Ok(Nationality::Indonesian),
            39 => Ok(Nationality::Irish),
            40 => Ok(Nationality::Israeli),
            41 => Ok(Nationality::Italian),
            42 => Ok(Nationality::Jamaican),
            43 => Ok(Nationality::Japanese),
            44 => Ok(Nationality::Jordanian),
            45 => Ok(Nationality::Kuwaiti),
            46 => Ok(Nationality::Latvian),
            47 => Ok(Nationality::Lebanese),
            48 => Ok(Nationality::Lithuanian),
            49 => Ok(Nationality::Luxembourger),
            50 => Ok(Nationality::Malaysian),
            51 => Ok(Nationality::Maltese),
            52 => Ok(Nationality::Mexican),
            53 => Ok(Nationality::Monegasque),
            54 => Ok(Nationality::NewZealander),
            55 => Ok(Nationality::Nicaraguan),
            56 => Ok(Nationality::NorthKorean),
            57 => Ok(Nationality::NorthernIrish),
            58 => Ok(Nationality::Norwegian),
            59 => Ok(Nationality::Omani),
            60 => Ok(Nationality::Pakistani),
            61 => Ok(Nationality::Panamanian),
            62 => Ok(Nationality::Paraguayan),
            63 => Ok(Nationality::Peruvian),
            64 => Ok(Nationality::Polish),
            65 => Ok(Nationality::Portuguese),
            66 => Ok(Nationality::Qatari),
            67 => Ok(Nationality::Romanian),
            68 => Ok(Nationality::Russian),
            69 => Ok(Nationality::Salvadoran),
            70 => Ok(Nationality::Saudi),
            71 => Ok(Nationality::Scottish),
            72 => Ok(Nationality::Serbian),
            73 => Ok(Nationality::Singaporean),
            74 => Ok(Nationality::Slovakian),
            75 => Ok(Nationality::Slovenian),
            76 => Ok(Nationality::SouthKorean),
            77 => Ok(Nationality::SouthAfrican),
            78 => Ok(Nationality::Spanish),
            79 => Ok(Nationality::Swedish),
            80 => Ok(Nationality::Swiss),
            81 => Ok(Nationality::Thai),
            82 => Ok(Nationality::Turkish),
            83 => Ok(Nationality::Uruguayan),
            84 => Ok(Nationality::Ukrainian),
            85 => Ok(Nationality::Venezuelan),
            86 => Ok(Nationality::Welsh),
            _ => Err(UnpackError(format!("Invalid Nationality value: {}", value))),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Telemetry {
    Restricted,
    Public,
}

impl TryFrom<u8> for Telemetry {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Telemetry::Restricted),
            1 => Ok(Telemetry::Public),
            _ => Err(UnpackError(format!("Invalid Telemetry value: {}", value))),
        }
    }
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
#[derive(Debug, CopyGetters, Getters)]
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
    pub fn new<T: BufRead>(reader: &mut T) -> Result<ParticipantData, UnpackError> {
        let ai_controlled = reader.read_u8().unwrap() == 1;
        let driver = Driver::try_from(reader.read_u8().unwrap())?;
        let team = Team::try_from(reader.read_u8().unwrap())?;
        let race_number = reader.read_u8().unwrap();
        let nationality = Nationality::try_from(reader.read_u8().unwrap())?;
        let name = read_name(reader)?;
        let telemetry = Telemetry::try_from(reader.read_u8().unwrap())?;

        Ok(ParticipantData {
            ai_controlled,
            driver,
            team,
            race_number,
            nationality,
            name,
            telemetry,
        })
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
#[derive(Debug, Getters, CopyGetters)]
pub struct PacketParticipantsData {
    #[getset(get = "pub")]
    header: PacketHeader,
    #[getset(get_copy = "pub")]
    num_active_cars: u8,
    #[getset(get = "pub")]
    participants: Vec<ParticipantData>,
}

impl PacketParticipantsData {
    pub fn new<T: BufRead>(
        mut reader: &mut T,
        header: PacketHeader,
    ) -> Result<PacketParticipantsData, UnpackError> {
        let num_active_cars = reader.read_u8().unwrap();

        let mut participants = Vec::with_capacity(20);
        for _ in 0..20 {
            let p = ParticipantData::new(&mut reader)?;
            participants.push(p);
        }

        Ok(PacketParticipantsData {
            header,
            num_active_cars,
            participants,
        })
    }
}

fn read_name<T: BufRead>(reader: &mut T) -> Result<String, UnpackError> {
    let mut nb_read: u8 = 0;

    let mut chars = Vec::with_capacity(48);

    for _ in 0..48 {
        nb_read += 1;

        let c = reader.read_u8().unwrap();
        if c == 0 {
            break;
        }

        chars.push(c);
    }

    // Consume the full 48 bytes.
    while nb_read < 48 {
        nb_read += 1;
        let _ = reader.read_u8();
    }

    match String::from_utf8(chars) {
        Ok(v) => Ok(v),
        Err(e) => Err(UnpackError(format!("Error decoding name: {}", e))),
    }
}
