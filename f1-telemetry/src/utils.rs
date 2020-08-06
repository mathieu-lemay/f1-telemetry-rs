use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

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

pub(crate) fn read_millis_f32<T: BufRead>(reader: &mut T) -> u32 {
    let seconds = reader.read_f32::<LittleEndian>().unwrap();

    seconds_to_millis(seconds as f64)
}

pub(crate) fn read_millis_f64<T: BufRead>(reader: &mut T) -> u32 {
    let seconds = reader.read_f64::<LittleEndian>().unwrap();

    seconds_to_millis(seconds)
}

#[inline]
pub(crate) fn seconds_to_millis(seconds: f64) -> u32 {
    (seconds * 1000.0).floor() as u32
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

#[cfg(test)]
mod test_seconds_to_millis {
    use super::seconds_to_millis;

    #[test]
    fn test_seconds_to_millis() {
        assert_eq!(seconds_to_millis(2.50), 2500);
        assert_eq!(seconds_to_millis(1.5001), 1500);
        assert_eq!(seconds_to_millis(1.4999), 1499);
    }
}
