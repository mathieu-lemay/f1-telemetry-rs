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
