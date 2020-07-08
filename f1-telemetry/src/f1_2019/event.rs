use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::packet::event::*;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, unpack_string};

use super::consts::*;

pub(crate) fn parse_event_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketEventData, UnpackError> {
    assert_packet_size(size, EVENT_PACKET_SIZE)?;

    let event_code = unpack_string(reader, 4)?;

    let event = match event_code.as_str() {
        "SSTA" => Ok(Event::SessionStarted),
        "SEND" => Ok(Event::SessionEnded),
        "FTLP" => {
            let vehicle_idx = reader.read_u8().unwrap();
            let lap_time = reader.read_f32::<LittleEndian>().unwrap();

            let evt_detail = FastestLap::new(vehicle_idx, lap_time);
            Ok(Event::FastestLap(evt_detail))
        }
        "RTMT" => {
            let vehicle_idx = reader.read_u8().unwrap();

            let evt_detail = Retirement::new(vehicle_idx);
            Ok(Event::Retirement(evt_detail))
        }
        "DRSE" => Ok(Event::DRSEnabled),
        "DRSD" => Ok(Event::DRSDisabled),
        "TMPT" => {
            let vehicle_idx = reader.read_u8().unwrap();

            let evt_detail = TeamMateInPits::new(vehicle_idx);
            Ok(Event::TeamMateInPits(evt_detail))
        }
        "CHQF" => Ok(Event::ChequeredFlag),
        "RCWN" => {
            let vehicle_idx = reader.read_u8().unwrap();

            let evt_detail = RaceWinner::new(vehicle_idx);
            Ok(Event::RaceWinner(evt_detail))
        }
        _ => Err(UnpackError(format!("Invalid Event Code: {}", event_code))),
    }?;

    Ok(PacketEventData::new(header, event))
}
