use std::io::Cursor;

use header::parse_header;

use crate::packet::{Packet, UnpackError};

mod consts;
mod header;

pub(crate) fn parse_packet(size: usize, packet: &[u8]) -> Result<Packet, UnpackError> {
    let mut cursor = Cursor::new(packet);
    let header = parse_header(&mut cursor, size)?;

    Err(UnpackError(format!(
        "Not implemented: {:?}",
        header.packet_type
    )))
}
