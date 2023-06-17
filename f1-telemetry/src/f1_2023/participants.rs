use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::participants::{Driver, PacketParticipantsData, ParticipantData, Telemetry};
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, unpack_string};

use super::consts::*;
use super::generic::{unpack_nationality, unpack_platform, unpack_team};

fn unpack_driver(value: u8) -> Result<Driver, UnpackError> {
    match value {
        0 => Ok(Driver::CarlosSainz),
        1 => Ok(Driver::DaniilKvyat),
        2 => Ok(Driver::DanielRicciardo),
        3 => Ok(Driver::FernandoAlonso),
        4 => Ok(Driver::FelipeMassa),
        6 => Ok(Driver::KimiRaikkonen),
        7 => Ok(Driver::LewisHamilton),
        9 => Ok(Driver::MaxVerstappen),
        10 => Ok(Driver::NicoHulkenburg),
        11 => Ok(Driver::KevinMagnussen),
        12 => Ok(Driver::RomainGrosjean),
        13 => Ok(Driver::SebastianVettel),
        14 => Ok(Driver::SergioPerez),
        15 => Ok(Driver::ValtteriBottas),
        17 => Ok(Driver::EstebanOcon),
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
        73 => Ok(Driver::LukasWeber),
        74 => Ok(Driver::AntonioGiovinazzi),
        75 => Ok(Driver::RobertKubica),
        76 => Ok(Driver::AlainProst),
        77 => Ok(Driver::AyrtonSenna),
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
        90 => Ok(Driver::MichaelSchumacher),
        91 => Ok(Driver::DanTicktum),
        92 => Ok(Driver::MarcusArmstrong),
        93 => Ok(Driver::ChristianLundgaard),
        94 => Ok(Driver::YukiTsunoda),
        95 => Ok(Driver::JehanDaruvala),
        96 => Ok(Driver::GulhermeSamaia),
        97 => Ok(Driver::PedroPiquet),
        98 => Ok(Driver::FelipeDrugovich),
        99 => Ok(Driver::RobertSchwartzman),
        100 => Ok(Driver::RoyNissany),
        101 => Ok(Driver::MarinoSato),
        102 => Ok(Driver::AidanJackson),
        103 => Ok(Driver::CasperAkkerman),
        109 => Ok(Driver::JensonButton),
        110 => Ok(Driver::DavidCoulthard),
        111 => Ok(Driver::NicoRosberg),
        112 => Ok(Driver::OscarPiastri),
        113 => Ok(Driver::LiamLawson),
        114 => Ok(Driver::JuriVips),
        115 => Ok(Driver::TheoPourchaire),
        116 => Ok(Driver::RichardVerschoor),
        117 => Ok(Driver::LirimZendeli),
        118 => Ok(Driver::DavidBeckmann),
        121 => Ok(Driver::AlessioDeledda),
        122 => Ok(Driver::BentViscaal),
        123 => Ok(Driver::EnzoFittipaldi),
        125 => Ok(Driver::MarkWebber),
        126 => Ok(Driver::JacquesVilleneuve),
        255 => Ok(Driver::Player),
        _ => Err(UnpackError(format!("Invalid Driver value: {}", value))),
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
/// Size: 1306 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:          Header
/// num_active_cars: Number of active cars in the data – should match number of
///                  cars on HUD
/// participants:    List of participants (22)
/// ```
#[derive(Deserialize)]
struct RawParticipantData {
    num_active_cars: u8,
    participants: [RawParticipant; NUMBER_CARS],
}

/// ## Specification
/// ```text
/// ai_controlled:     Whether the vehicle is AI (1) or Human (0) controlled
/// driver_id:         Driver id - see appendix
/// network_id:        Network id – unique identifier for network players
/// team_id:           Team id - see appendix
/// my_team:           My team flag – 1 = My Team, 0 = otherwise
/// race_number:       Race number of the car
/// nationality:       Nationality of the driver
/// name:              Name of participant in UTF-8 format – null terminated
///                    Will be truncated with … (U+2026) if too long
/// your_telemetry:    The player's UDP setting, 0 = restricted, 1 = public
/// show_online_names: The player's show online names setting, 0 = off, 1 = on
/// platform:          1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
/// ```
#[derive(Deserialize)]
struct RawParticipant {
    ai_controlled: bool,
    driver_id: u8,
    network_id: u8,
    team_id: u8,
    my_team: bool,
    race_number: u8,
    nationality: u8,
    name1: [u8; 32], // FIXME: Ugly hack
    name2: [u8; 16],
    telemetry: u8,
    show_online_names: bool,
    platform: u8,
}

impl TryFrom<&RawParticipant> for ParticipantData {
    type Error = UnpackError;

    fn try_from(participant: &RawParticipant) -> Result<Self, Self::Error> {
        let name: [u8; 48] = {
            let mut whole: [u8; 48] = [0; 48];
            let (part1, part2) = whole.split_at_mut(participant.name1.len());
            part1.copy_from_slice(&participant.name1);
            part2.copy_from_slice(&participant.name2);
            whole
        };

        let driver = unpack_driver(participant.driver_id)?;
        let team = unpack_team(participant.team_id)?;
        let nationality = unpack_nationality(participant.nationality)?;
        let name = unpack_string(&name)?;
        let telemetry_access = unpack_telemetry(participant.telemetry)?;
        let platform = unpack_platform(participant.platform)?;

        Ok(Self {
            ai_controlled: participant.ai_controlled,
            driver,
            network_id: Some(participant.network_id),
            team,
            my_team: participant.my_team,
            race_number: participant.race_number,
            nationality,
            name,
            telemetry_access,
            show_online_names: participant.show_online_names,
            platform,
        })
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
        .map(|p| p.try_into())
        .collect::<Result<Vec<ParticipantData>, UnpackError>>()?;

    Ok(PacketParticipantsData {
        header,
        num_active_cars: participant_data.num_active_cars,
        participants,
    })
}
