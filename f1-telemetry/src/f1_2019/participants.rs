use std::io::BufRead;

use serde::Deserialize;

use crate::packet::generic::{Nationality, Team};
use crate::packet::header::PacketHeader;
use crate::packet::participants::*;
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, unpack_string};

use super::consts::*;

fn unpack_driver(value: u8) -> Result<Driver, UnpackError> {
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

fn unpack_team(value: u8) -> Result<Team, UnpackError> {
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

fn unpack_nationality(value: u8) -> Result<Nationality, UnpackError> {
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
        0 => Ok(Nationality::Invalid),
        _ => Err(UnpackError(format!("Invalid Nationality value: {}", value))),
    }
}

fn unpack_telemetry(value: u8) -> Result<Telemetry, UnpackError> {
    match value {
        0 => Ok(Telemetry::Restricted),
        1 => Ok(Telemetry::Public),
        _ => Err(UnpackError(format!("Invalid Telemetry value: {}", value))),
    }
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
/// Size: 1104 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:          Header
/// num_active_cars: Number of active cars in the data – should match number of
///                  cars on HUD
/// participants:    List of participants (20)
/// ```
#[derive(Deserialize)]
struct RawParticipantData {
    num_active_cars: u8,
    participants: [RawParticipant; NUMBER_CARS],
}

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
#[derive(Deserialize)]
struct RawParticipant {
    ai_controlled: bool,
    driver: u8,
    team: u8,
    race_number: u8,
    nationality: u8,
    name1: [u8; 32], // FIXME: Ugly hack
    name2: [u8; 16],
    telemetry: u8,
}

impl ParticipantData {
    fn from(participant: &RawParticipant) -> Result<Self, UnpackError> {
        let name: [u8; 48] = {
            let mut whole: [u8; 48] = [0; 48];
            let (part1, part2) = whole.split_at_mut(participant.name1.len());
            part1.copy_from_slice(&participant.name1);
            part2.copy_from_slice(&participant.name2);
            whole
        };

        Ok(ParticipantData::new(
            participant.ai_controlled,
            unpack_driver(participant.driver)?,
            unpack_team(participant.team)?,
            participant.race_number,
            unpack_nationality(participant.nationality)?,
            unpack_string(&name)?,
            unpack_telemetry(participant.telemetry)?,
        ))
    }
}

pub(crate) fn parse_participants_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketParticipantsData, UnpackError> {
    assert_packet_size(size, PARTICIPANTS_PACKET_SIZE)?;

    let participant_data: RawParticipantData = bincode::deserialize_from(reader)?;
    let participants: Vec<ParticipantData> = participant_data
        .participants
        .iter()
        .map(ParticipantData::from)
        .collect::<Result<Vec<ParticipantData>, UnpackError>>()?;

    Ok(PacketParticipantsData::new(
        header,
        participant_data.num_active_cars,
        participants,
    ))
}
