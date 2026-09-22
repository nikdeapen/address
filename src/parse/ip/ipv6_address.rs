use crate::ParseError::InvalidIPv6Address;
use crate::parse_digits;
use crate::{IPv6Address, ParseError, impl_parse};
use std::net::Ipv6Addr;
use std::str::FromStr;

impl IPv6Address {
    //! Parse

    /// Parses an [IPv6Address] from the `text`.
    ///
    /// # Notes
    /// - The embedded IPv4 form is accepted. (`::ffff:1.2.3.4`)
    /// - Brackets & zones are not accepted; see [`SocketAddressV6`](crate::SocketAddressV6).
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        const MAX_STR_LEN: usize = "ffff:ffff:ffff:ffff:ffff:ffff:255.255.255.255".len();
        if text.len() > MAX_STR_LEN {
            return Err(InvalidIPv6Address);
        }
        let text: &str = std::str::from_utf8(text).map_err(|_| InvalidIPv6Address)?;
        Ok(Ipv6Addr::from_str(text)
            .map_err(|_| InvalidIPv6Address)?
            .into())
    }

    /// Parses the bracketed IPv6 address text, ignoring an optional numeric zone.
    pub(crate) fn parse_bracketed(text: &[u8]) -> Result<Self, ParseError> {
        let text: &[u8] = text
            .strip_prefix(b"[")
            .and_then(|text| text.strip_suffix(b"]"))
            .ok_or(InvalidIPv6Address)?;
        let text: &[u8] = Self::strip_zone(text).ok_or(InvalidIPv6Address)?;
        Self::parse(text)
    }

    /// Strips the ignored zone suffix from the `text`, the inner text of a bracketed IPv6 address.
    ///
    /// # Notes
    /// - The text is returned unchanged if there is no `%`; an invalid zone is `None`.
    /// - The zone must be a decimal `u32`, with no sign.
    /// - Leading zeros are allowed, matching the scope ids accepted by the std-lib socket parser.
    ///
    /// # Examples
    /// `fe80::1%1` -> `Some("fe80::1")`
    /// `fe80::1`   -> `Some("fe80::1")`
    /// `fe80::1%`  -> `None`
    fn strip_zone(text: &[u8]) -> Option<&[u8]> {
        match text.iter().position(|c| *c == b'%') {
            Some(percent) => parse_digits(&text[percent + 1..]).map(|_| &text[..percent]),
            None => Some(text),
        }
    }
}

impl_parse!(IPv6Address);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidIPv6Address;
    use crate::{IPv6Address, ParseError};
    use std::str::FromStr;

    type TestCase<'a> = (&'a [u8], Result<IPv6Address, ParseError>);
    const MAX_STR_LEN: usize = "ffff:ffff:ffff:ffff:ffff:ffff:255.255.255.255".len();

    #[test]
    fn parse() {
        let over_max: Vec<u8> = vec![b'0'; MAX_STR_LEN + 1];
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidIPv6Address)),
            (b"::", Ok(IPv6Address::UNSPECIFIED)),
            (b"::1", Ok(IPv6Address::LOCALHOST)),
            (b"[::1]", Err(InvalidIPv6Address)),
            (b"[fe80::1]", Err(InvalidIPv6Address)),
            (b"[fe80::1%1]", Err(InvalidIPv6Address)),
            (b"fe80::1%1", Err(InvalidIPv6Address)),
            (b"fe80::1%0", Err(InvalidIPv6Address)),
            (b"::\xFF", Err(InvalidIPv6Address)),
            (b"\xFF\xFF", Err(InvalidIPv6Address)),
            (over_max.as_slice(), Err(InvalidIPv6Address)),
        ];

        for (input, expected) in test_cases {
            let result: Result<IPv6Address, ParseError> = IPv6Address::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<IPv6Address, ParseError> = IPv6Address::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<IPv6Address, ParseError> = IPv6Address::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    #[test]
    fn parse_bracketed() {
        let test_cases: &[(&str, Result<IPv6Address, ParseError>)] = &[
            ("[::1]", Ok(IPv6Address::LOCALHOST)),
            ("[::1%1]", Ok(IPv6Address::LOCALHOST)),
            ("[::1%eth0]", Err(InvalidIPv6Address)),
            ("[]", Err(InvalidIPv6Address)),
            ("[127.0.0.1]", Err(InvalidIPv6Address)),
            ("::1", Err(InvalidIPv6Address)),
            ("[::1", Err(InvalidIPv6Address)),
            ("::1]", Err(InvalidIPv6Address)),
        ];

        for (input, expected) in test_cases {
            let result: Result<IPv6Address, ParseError> =
                IPv6Address::parse_bracketed(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
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
