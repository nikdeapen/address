/// Parses the unsigned decimal `text`.
///
/// # Notes
/// - The text must be decimal digits only, with no sign.
/// - Leading zeros are allowed.
/// - Empty text & values over `u32::MAX` are `None`.
pub(crate) fn parse_digits(text: &[u8]) -> Option<u32> {
    if text.is_empty() {
        return None;
    }
    let mut value: u32 = 0;
    for c in text {
        if !c.is_ascii_digit() {
            return None;
        }
        value = value.checked_mul(10)?.checked_add(u32::from(c - b'0'))?;
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use crate::parse_digits;

    #[test]
    fn digits() {
        let test_cases: &[(&[u8], Option<u32>)] = &[
            (b"", None),
            (b"0", Some(0)),
            (b"00", Some(0)),
            (b"80", Some(80)),
            (b"080", Some(80)),
            (b"4294967295", Some(4294967295)),
            (b"4294967296", None),
            (b"00000000000000000000080", Some(80)),
            (b"+1", None),
            (b"-1", None),
            (b" 1", None),
            (b"1 ", None),
            (b"1x", None),
            (b"\xFF", None),
        ];

        for (input, expected) in test_cases {
            let result: Option<u32> = parse_digits(input);
            assert_eq!(result, *expected, "input={:?}", input);
        }
    }
}
