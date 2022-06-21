use std::io::Cursor;

use header::parse_header;
use session::parse_session_data;

use crate::packet::{Packet, PacketType, UnpackError};

mod consts;
mod generic;
mod header;
mod session;

pub(crate) fn parse_packet(size: usize, packet: &[u8]) -> Result<Packet, UnpackError> {
    let mut cursor = Cursor::new(packet);
    let header = parse_header(&mut cursor, size)?;

    match header.packet_type {
        PacketType::Session => {
            let packet = parse_session_data(&mut cursor, header, size)?;

            Ok(Packet::Session(packet))
        }
        _ => Err(UnpackError(format!(
            "Not implemented: {:?}",
            header.packet_type
        ))),
    }
}
