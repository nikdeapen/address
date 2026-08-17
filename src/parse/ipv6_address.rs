use crate::ParseError::InvalidIPv6Address;
use crate::{IPv6Address, ParseError, impl_parse};
use std::net::Ipv6Addr;
use std::str::FromStr;

impl IPv6Address {
    //! Parse

    /// The maximum length of an IPv6 address string. (ffff:ffff:ffff:ffff:ffff:ffff:255.255.255.255)
    const MAX_STR_LEN: usize = 45;

    /// Parses the IPv6 address text. (a public `&[u8]` conversion would read as raw octets, not text)
    pub(crate) fn parse(ip: &[u8]) -> Result<Self, ParseError> {
        if ip.len() > Self::MAX_STR_LEN {
            return Err(InvalidIPv6Address);
        }
        let ip: &str = std::str::from_utf8(ip).map_err(|_| InvalidIPv6Address)?;
        Ok(Ipv6Addr::from_str(ip).map_err(|_| InvalidIPv6Address)?.into())
    }

    /// Parses the bracketed IPv6 address text, ignoring an optional numeric zone.
    ///
    /// Returns `None` if the address is not bracketed. A bracketed address with an invalid interior, the zone
    /// included, is `Some(Err(InvalidIPv6Address))`: the brackets declare the version, so the error blames the
    /// IPv6 address rather than the caller's own variant.
    pub(crate) fn parse_bracketed(ip: &[u8]) -> Option<Result<Self, ParseError>> {
        let ip: &[u8] = Self::strip_brackets(ip)?;
        if let Some(ip) = Self::strip_zone(ip) {
            Some(Self::parse(ip))
        } else {
            Some(Err(InvalidIPv6Address))
        }
    }

    /// Strips the surrounding brackets from the `address`.
    ///
    /// Returns `None` if the address is not bracketed.
    ///
    /// # Examples
    /// `[::1]`   -> `Some("::1")`
    /// `[]`      -> `Some("")`
    /// `::1`     -> `None`
    /// `[::1`    -> `None`
    fn strip_brackets(address: &[u8]) -> Option<&[u8]> {
        if !address.is_empty() && address[0] == b'[' && address[address.len() - 1] == b']' {
            Some(&address[1..address.len() - 1])
        } else {
            None
        }
    }

    /// Strips the ignored zone suffix from the `address`, the inner text of a bracketed IPv6 address.
    ///
    /// Returns the address unchanged if there is no `%`. The zone must be a decimal `u32`, with no sign; leading
    /// zeros are allowed, matching the scope ids accepted by the standard library socket parser. Returns `None` if
    /// the zone is invalid.
    ///
    /// The digit check runs first, so the zone is known to be ASCII before it is read as a string.
    ///
    /// # Examples
    /// `fe80::1%1` -> `Some("fe80::1")`
    /// `fe80::1`   -> `Some("fe80::1")`
    /// `fe80::1%`  -> `None`
    fn strip_zone(address: &[u8]) -> Option<&[u8]> {
        if let Some(percent) = address.iter().position(|c| *c == b'%') {
            let zone: &[u8] = &address[percent + 1..];
            let valid: bool = !zone.is_empty() && zone.iter().all(|c| c.is_ascii_digit());
            if !valid {
                return None;
            }
            let zone: &str = unsafe { std::str::from_utf8_unchecked(zone) };
            let _: u32 = u32::from_str(zone).ok()?;
            Some(&address[..percent])
        } else {
            Some(address)
        }
    }
}

impl_parse!(IPv6Address, parse);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidIPv6Address;
    use crate::{IPv6Address, ParseError};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<IPv6Address, ParseError>)] = &[
            ("", Err(InvalidIPv6Address)),
            ("::", Ok(IPv6Address::UNSPECIFIED)),
            ("::1", Ok(IPv6Address::LOCALHOST)),
        ];

        for (input, expected) in test_cases {
            let result: Result<IPv6Address, ParseError> = IPv6Address::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<IPv6Address, ParseError> = IPv6Address::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn strip_brackets() {
        let test_cases: &[(&str, Option<&str>)] = &[
            ("[::1]", Some("::1")),
            ("[]", Some("")),
            ("::1", None),
            ("[::1", None),
            ("::1]", None),
            ("", None),
            ("[", None),
        ];

        for (input, expected) in test_cases {
            let result: Option<&[u8]> = IPv6Address::strip_brackets(input.as_bytes());
            let expected: Option<&[u8]> = expected.map(str::as_bytes);
            assert_eq!(result, expected, "input={}", input);
        }
    }

    #[test]
    fn strip_zone() {
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
            let result: Option<&[u8]> = IPv6Address::strip_zone(input.as_bytes());
            let expected: Option<&[u8]> = expected.map(str::as_bytes);
            assert_eq!(result, expected, "input={}", input);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &[
            "::",
            "::1",
            "1::",
            "1::1",
            "1:0:0:1::",
            "1:2:3:4:5:6:7:8",
            "fe80::1",
            "::ffff:1.2.3.4",
            "2001:db8::8a2e:370:7334",
            "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
        ];

        for input in canonical {
            let value: IPv6Address = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
