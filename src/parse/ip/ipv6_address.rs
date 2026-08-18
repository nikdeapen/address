use crate::ParseError::InvalidIPv6Address;
use crate::{IPv6Address, ParseError, impl_parse};
use std::net::Ipv6Addr;
use std::str::FromStr;

impl IPv6Address {
    //! Parse

    /// The maximum length of an IPv6 address string. (ffff:ffff:ffff:ffff:ffff:ffff:255.255.255.255)
    const MAX_STR_LEN: usize = 45;

    /// Parses the IPv6 address text.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
        if text.len() > Self::MAX_STR_LEN {
            return Err(InvalidIPv6Address);
        }
        let text: &str = std::str::from_utf8(text).map_err(|_| InvalidIPv6Address)?;
        Ok(Ipv6Addr::from_str(text).map_err(|_| InvalidIPv6Address)?.into())
    }

    /// Parses the bracketed IPv6 address text, ignoring an optional numeric zone.
    ///
    /// Returns `None` if the address is not bracketed. A bracketed address with an invalid interior, the zone
    /// included, is `Some(Err(InvalidIPv6Address))`: the brackets declare the version, so the error blames the
    /// IPv6 address rather than the caller's own variant.
    pub(crate) fn parse_bracketed(text: &[u8]) -> Option<Result<Self, ParseError>> {
        let text: &[u8] = Self::strip_brackets(text)?;
        if let Some(text) = Self::strip_zone(text) {
            Some(Self::parse_text(text))
        } else {
            Some(Err(InvalidIPv6Address))
        }
    }

    /// Strips the surrounding brackets from the `text`.
    ///
    /// Returns `None` if the address is not bracketed.
    ///
    /// # Examples
    /// `[::1]`   -> `Some("::1")`
    /// `[]`      -> `Some("")`
    /// `::1`     -> `None`
    /// `[::1`    -> `None`
    fn strip_brackets(text: &[u8]) -> Option<&[u8]> {
        if !text.is_empty() && text[0] == b'[' && text[text.len() - 1] == b']' {
            Some(&text[1..text.len() - 1])
        } else {
            None
        }
    }

    /// Strips the ignored zone suffix from the `text`, the inner text of a bracketed IPv6 address.
    ///
    /// Returns the text unchanged if there is no `%`. The zone must be a decimal `u32`, with no sign; leading
    /// zeros are allowed, matching the scope ids accepted by the standard library socket parser. Returns `None` if
    /// the zone is invalid.
    ///
    /// The digit check runs first, so the zone is known to be ASCII before it is read as a string.
    ///
    /// # Examples
    /// `fe80::1%1` -> `Some("fe80::1")`
    /// `fe80::1`   -> `Some("fe80::1")`
    /// `fe80::1%`  -> `None`
    fn strip_zone(text: &[u8]) -> Option<&[u8]> {
        if let Some(percent) = text.iter().position(|c| *c == b'%') {
            let zone: &[u8] = &text[percent + 1..];
            let valid: bool = !zone.is_empty() && zone.iter().all(|c| c.is_ascii_digit());
            if !valid {
                return None;
            }
            let zone: &str = unsafe { std::str::from_utf8_unchecked(zone) };
            let _: u32 = u32::from_str(zone).ok()?;
            Some(&text[..percent])
        } else {
            Some(text)
        }
    }
}

impl_parse!(
    IPv6Address,
    "Matches the standard library, including the embedded IPv4 form. (`::ffff:1.2.3.4`)",
    "Brackets & zones are not accepted; see [`SocketAddressV6`](crate::SocketAddressV6)."
);

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

            let result: Result<IPv6Address, ParseError> = IPv6Address::parse_text(input.as_bytes());
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

    /// The bare address takes neither brackets nor a zone; only the bracketed socket parsers accept those.
    #[test]
    fn rejects_brackets_and_zones() {
        let test_cases: &[&str] = &["[::1]", "[fe80::1]", "fe80::1%1", "fe80::1%0", "[fe80::1%1]"];

        for input in test_cases {
            let result: Result<IPv6Address, ParseError> = IPv6Address::from_str(input);
            assert_eq!(result, Err(InvalidIPv6Address), "input={}", input);
        }
    }

    /// The length guard & the UTF-8 check run before the text is read as a string.
    #[test]
    fn parse_text_guards() {
        let over_max: Vec<u8> = vec![b'0'; IPv6Address::MAX_STR_LEN + 1];
        let test_cases: &[&[u8]] = &[over_max.as_slice(), b"::\xFF", b"\xFF\xFF"];

        for input in test_cases {
            let result: Result<IPv6Address, ParseError> = IPv6Address::parse_text(input);
            assert_eq!(result, Err(InvalidIPv6Address), "input={:?}", input);
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
