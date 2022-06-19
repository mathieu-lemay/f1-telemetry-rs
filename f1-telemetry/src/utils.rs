use crate::packet::UnpackError;

pub(crate) fn unpack_string(chars: &[u8]) -> Result<String, UnpackError> {
    match std::str::from_utf8(chars) {
        Ok(v) => Ok(v.trim_end_matches(char::from(0)).to_string()),
        Err(e) => Err(UnpackError(format!("Error decoding name: {}", e))),
    }
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
