use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::f1_2020::generic::{
    unpack_result_status, unpack_tyre_compound, unpack_tyre_compound_visual,
};
use crate::packet::final_classification::{FinalClassification, PacketFinalClassificationData};
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

fn parse_final_classification<T: BufRead>(
    reader: &mut T,
) -> Result<FinalClassification, UnpackError> {
    let position = reader.read_u8().unwrap();
    let num_laps = reader.read_u8().unwrap();
    let grid_position = reader.read_u8().unwrap();
    let points = reader.read_u8().unwrap();
    let num_pit_stops = reader.read_u8().unwrap();
    let result_status = unpack_result_status(reader.read_u8().unwrap())?;
    let best_lap_time = reader.read_f32::<LittleEndian>().unwrap();
    let total_race_time = reader.read_f64::<LittleEndian>().unwrap();
    let penalties_time = reader.read_u8().unwrap();
    let num_penalties = reader.read_u8().unwrap();
    let num_tyre_stints = reader.read_u8().unwrap();

    let mut tyre_stints_actual = Vec::with_capacity(8);
    for _ in 0..8 {
        let tc = unpack_tyre_compound(reader.read_u8().unwrap())?;
        tyre_stints_actual.push(tc);
    }

    let mut tyre_stints_visual = Vec::with_capacity(8);
    for _ in 0..8 {
        let tc = unpack_tyre_compound_visual(reader.read_u8().unwrap())?;
        tyre_stints_visual.push(tc);
    }

    Ok(FinalClassification::new(
        position,
        num_laps,
        grid_position,
        points,
        num_pit_stops,
        result_status,
        best_lap_time,
        total_race_time,
        penalties_time,
        num_penalties,
        num_tyre_stints,
        tyre_stints_actual,
        tyre_stints_visual,
    ))
}

pub(crate) fn parse_final_classification_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketFinalClassificationData, UnpackError> {
    assert_packet_size(size, FINAL_CLASSIFICATION_PACKET_SIZE)?;

    let num_cars = reader.read_u8().unwrap();

    let mut final_classifications = Vec::with_capacity(NUMBER_CARS);

    for _ in 0..NUMBER_CARS {
        let fc = parse_final_classification(&mut reader)?;
        final_classifications.push(fc);
    }

    Ok(PacketFinalClassificationData::new(
        header,
        num_cars,
        final_classifications,
    ))
}
