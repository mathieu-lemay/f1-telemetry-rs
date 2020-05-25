use header::PacketHeader;
use lap::PacketLapData;
use participants::PacketParticipantsData;
use session::PacketSessionData;
use std::convert::TryFrom;
use std::io::Cursor;
use std::mem;

pub mod header;
pub mod lap;
pub mod participants;
pub mod session;

//struct UnpackError(&'static str);
#[derive(Debug)]
pub struct UnpackError(pub String);

pub enum Packet {
    Session(PacketSessionData),
    LapData(PacketLapData),
    ParticipantsData(PacketParticipantsData),
}

#[derive(Debug)]
enum PacketType {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
}

impl TryFrom<u8> for PacketType {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketType::Motion),
            1 => Ok(PacketType::Session),
            2 => Ok(PacketType::LapData),
            3 => Ok(PacketType::Event),
            4 => Ok(PacketType::Participants),
            5 => Ok(PacketType::CarSetups),
            6 => Ok(PacketType::CarTelemetry),
            7 => Ok(PacketType::CarStatus),
            _ => Err(UnpackError(format!("Invalid PacketType: {}", value))),
        }
    }
}

pub fn parse_packet(size: usize, packet: &[u8]) -> Result<Packet, UnpackError> {
    let header_size = mem::size_of::<PacketHeader>();

    if size < header_size {
        return Err(UnpackError(format!(
            "Invalid packet: too small ({} bytes)",
            size
        )));
    }

    let mut cursor = Cursor::new(packet);
    let header = PacketHeader::new(&mut cursor);

    let packet_id: PacketType = PacketType::try_from(*header.packet_id())?;

    match packet_id {
        //PacketType::Motion => {}
        PacketType::Session => {
            let packet = PacketSessionData::new(&mut cursor, header)?;

            Ok(Packet::Session(packet))
        }
        PacketType::LapData => {
            let packet = PacketLapData::new(&mut cursor, header)?;

            Ok(Packet::LapData(packet))
        }
        //PacketType::Event => {}
        PacketType::Participants => {
            let packet = PacketParticipantsData::new(&mut cursor, header)?;

            Ok(Packet::ParticipantsData(packet))
        }
        //PacketType::CarSetups => {}
        //PacketType::CarTelemetry => {}
        //PacketType::CarStatus => {}
        _ => Err(UnpackError(format!(
            "Unpacking not implemented for {:?}",
            packet_id
        ))),
    }
}
