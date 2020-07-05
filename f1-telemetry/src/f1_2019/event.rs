use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::BufRead;

use crate::packet::event::{Event, PacketEventData};
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::unpack_string;

impl TryFrom<&str> for Event {
    type Error = UnpackError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SSTA" => Ok(Event::SessionStarted),
            "SEND" => Ok(Event::SessionEnded),
            "FTLP" => Ok(Event::FastestLap),
            "RTMT" => Ok(Event::Retirement),
            "DRSE" => Ok(Event::DRSEnabled),
            "DRSD" => Ok(Event::DRSDisabled),
            "TMPT" => Ok(Event::TeamMateInPits),
            "CHQF" => Ok(Event::ChequeredFlag),
            "RCWN" => Ok(Event::RaceWinner),
            _ => Err(UnpackError(format!("Invalid Event value: {}", value))),
        }
    }
}

pub(crate) fn parse_event_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
) -> Result<PacketEventData, UnpackError> {
    let event_code = unpack_string(reader, 4)?;

    let event = Event::try_from(event_code.as_str())?;

    let vehicle_idx = reader.read_u8().unwrap();
    let vehicle_idx = match event {
        Event::FastestLap | Event::Retirement | Event::TeamMateInPits | Event::RaceWinner => {
            Some(vehicle_idx)
        }
        _ => None,
    };

    let lap_time = reader.read_f32::<LittleEndian>().unwrap();
    let lap_time = match event {
        Event::FastestLap => Some(lap_time),
        _ => None,
    };

    Ok(PacketEventData::new(header, event, vehicle_idx, lap_time))
}
