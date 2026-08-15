use std::str::FromStr;

/// Strips the ignored zone suffix from the bracketed socket `address`.
///
/// Returns the address unchanged if there is no `%`. The zone must be a decimal `u32`, with no sign & leading zeros
/// allowed, matching the scope ids accepted by the standard library socket parser. Returns `None` if the zone is
/// invalid.
///
/// # Examples
/// `fe80::1%1` -> `Some("fe80::1")`
/// `fe80::1`   -> `Some("fe80::1")`
/// `fe80::1%`  -> `None`
pub(crate) fn strip_zone(address: &[u8]) -> Option<&[u8]> {
    if let Some(percent) = address.iter().position(|c| *c == b'%') {
        let zone: &[u8] = &address[percent + 1..];
        let valid: bool = !zone.is_empty() && zone.iter().all(|c| c.is_ascii_digit());
        if !valid {
            return None;
        }
        let zone: &str = std::str::from_utf8(zone).ok()?;
        let _: u32 = u32::from_str(zone).ok()?;
        Some(&address[..percent])
    } else {
        Some(address)
    }
}

#[cfg(test)]
mod tests {
    use crate::strip_zone;

    #[test]
    fn zones() {
        let test_cases: &[(&str, Option<&str>)] = &[
            ("::1", Some("::1")),
            ("::1%1", Some("::1")),
            ("::1%0", Some("::1")),
            ("::1%01", Some("::1")),
            ("::1%4294967295", Some("::1")),
            ("::1%", None),
            ("::1%eth0", None),
            ("::1%+1", None),
            ("::1%4294967296", None),
            ("::1%1%2", None),
        ];

        for (input, expected) in test_cases {
            let result: Option<&[u8]> = strip_zone(input.as_bytes());
            let expected: Option<&[u8]> = expected.map(str::as_bytes);
            assert_eq!(result, expected, "input={}", input);
        }
    }
}
