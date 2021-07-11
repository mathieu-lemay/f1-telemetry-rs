use std::io::BufRead;

use byteorder::ReadBytesExt;

use crate::f1_2020::generic::{unpack_nationality, unpack_team};
use crate::packet::header::PacketHeader;
use crate::packet::participants::{Driver, PacketParticipantsData, ParticipantData, Telemetry};
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, read_string};

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

fn unpack_telemetry(value: u8) -> Result<Telemetry, UnpackError> {
    match value {
        0 => Ok(Telemetry::Restricted),
        1 => Ok(Telemetry::Public),
        _ => Err(UnpackError(format!("Invalid Telemetry value: {}", value))),
    }
}

fn parse_participant<T: BufRead>(reader: &mut T) -> Result<ParticipantData, UnpackError> {
    let ai_controlled = reader.read_u8().unwrap() == 1;
    let driver = unpack_driver(reader.read_u8().unwrap())?;
    let team = unpack_team(reader.read_u8().unwrap())?;
    let race_number = reader.read_u8().unwrap();
    let nationality = unpack_nationality(reader.read_u8().unwrap())?;
    let name = read_string(reader, 48)?;
    let telemetry_access = unpack_telemetry(reader.read_u8().unwrap())?;

    Ok(ParticipantData::new(
        ai_controlled,
        driver,
        team,
        race_number,
        nationality,
        name,
        telemetry_access,
    ))
}

pub(crate) fn parse_participants_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketParticipantsData, UnpackError> {
    assert_packet_size(size, PARTICIPANTS_PACKET_SIZE)?;

    let num_active_cars = reader.read_u8().unwrap();

    let mut participants = Vec::with_capacity(NUMBER_CARS);
    for _ in 0..NUMBER_CARS {
        let p = parse_participant(&mut reader)?;
        participants.push(p);
    }

    Ok(PacketParticipantsData::new(
        header,
        num_active_cars,
        participants,
    ))
}
