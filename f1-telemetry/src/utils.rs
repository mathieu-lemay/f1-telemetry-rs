use crate::packet::UnpackError;
use byteorder::ReadBytesExt;
use std::io::BufRead;

pub(crate) fn unpack_string<T: BufRead>(reader: &mut T, n: usize) -> Result<String, UnpackError> {
    let mut nb_read: usize = 0;

    let mut chars = Vec::with_capacity(n);

    for _ in 0..n {
        nb_read += 1;

        let c = reader.read_u8().unwrap();
        if c == 0 {
            break;
        }

        chars.push(c);
    }

    // Consume the full `n` bytes.
    while nb_read < n {
        nb_read += 1;
        let _ = reader.read_u8();
    }

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
        Err(UnpackError(String::from("Invalid packet size")))
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
            "Invalid packet: too small ({} bytes)",
            actual_size
        )))
    }
}
