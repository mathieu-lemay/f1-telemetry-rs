use header::PacketHeader;
use lap::PacketLapData;
use session::PacketSessionData;
use std::convert::TryFrom;
use std::io::Cursor;
use std::mem;

pub mod header;
pub mod lap;
pub mod session;

//struct UnpackError(&'static str);
#[derive(Debug)]
pub struct UnpackError(pub String);

pub enum Packet {
    Session(PacketSessionData),
    LapData(PacketLapData),
}

#[derive(Debug)]
enum PacketID {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
}

impl TryFrom<u8> for PacketID {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketID::Motion),
            1 => Ok(PacketID::Session),
            2 => Ok(PacketID::LapData),
            3 => Ok(PacketID::Event),
            4 => Ok(PacketID::Participants),
            5 => Ok(PacketID::CarSetups),
            6 => Ok(PacketID::CarTelemetry),
            7 => Ok(PacketID::CarStatus),
            _ => Err(UnpackError(format!("Invalid PacketID: {}", value))),
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

    let packet_id: PacketID = PacketID::try_from(header.packet_id)?;

    match packet_id {
        //PacketID::Motion => {}
        PacketID::Session => {
            let packet = PacketSessionData::new(&mut cursor, header)?;

            Ok(Packet::Session(packet))
        }
        PacketID::LapData => {
            let packet = PacketLapData::new(&mut cursor, header)?;

            Ok(Packet::LapData(packet))
        }
        //PacketID::Event => {}
        //PacketID::Participants => {}
        //PacketID::CarSetups => {}
        //PacketID::CarTelemetry => {}
        //PacketID::CarStatus => {}
        _ => Err(UnpackError(format!(
            "Unpacking not implemented for {:?}",
            packet_id
        ))),
    }
}
