use std::io::BufRead;

use byteorder::ReadBytesExt;

use crate::packet::UnpackError;

pub(crate) fn unpack_string<T: BufRead>(reader: &mut T, n: usize) -> Result<String, UnpackError> {
    let mut chars: Vec<u8> = (0..n).map(|_| reader.read_u8().unwrap()).collect();
    let nb_chars = chars
        .iter()
        .position(|&c| c == 0)
        .unwrap_or_else(|| chars.len());
    chars.truncate(nb_chars);

    match String::from_utf8(chars) {
        Ok(v) => Ok(v),
        Err(e) => Err(UnpackError(format!("Error decoding name: {}", e))),
    }
}

pub(crate) fn assert_packet_size(
    actual_size: usize,
    expected_size: usize,
) -> Result<(), UnpackError> {
    if actual_size == expected_size {
        Ok(())
    } else {
        Err(UnpackError(format!(
            "Invalid packet size: {} bytes (expected {} bytes)",
            actual_size, expected_size
        )))
    }
}

pub(crate) fn assert_packet_at_least_size(
    actual_size: usize,
    minimum_size: usize,
) -> Result<(), UnpackError> {
    if actual_size >= minimum_size {
        Ok(())
    } else {
        Err(UnpackError(format!(
            "Packet too small: {} bytes (minimum: {} bytes)",
            actual_size, minimum_size
        )))
    }
}
