use crate::ParseError::InvalidHost;
use crate::{DomainRef, HostRef, IPAddress, ParseError, impl_parse_ref};

impl<'a> HostRef<'a> {
    //! Parse

    /// A domain name or an unbracketed IP address: `localhost`, `127.0.0.1`, or `::1`.
    /// Domain names must already be in lowercase. Use [`Host`](crate::Host) to parse mixed-case
    /// input.
    pub fn parse_text(text: &'a [u8]) -> Result<Self, ParseError> {
        if let Ok(ip) = IPAddress::parse_text(text) {
            Ok(ip.to_host_ref())
        } else if let Ok(domain) = DomainRef::parse_text(text) {
            Ok(domain.to_host_ref())
        } else {
            Err(InvalidHost)
        }
    }
}

impl_parse_ref!(
    HostRef,
    "A domain name or an unbracketed IP address: `localhost`, `127.0.0.1`, or `::1`.",
    "Domain names must already be in lowercase.",
    "Use [`Host`](crate::Host) to parse mixed-case input."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidHost;
    use crate::{DomainRef, HostRef, IPv4Address, IPv6Address, ParseError};

    type TestCase<'a> = (&'a [u8], Result<HostRef<'a>, ParseError>);

    /// Every entry point must agree on every case; mixed case is rejected, not normalized.
    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidHost)),
            (b"localhost", Ok(DomainRef::LOCALHOST.to_host_ref())),
            (b"example.com", Ok(DomainRef::EXAMPLE.to_host_ref())),
            (b"127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host_ref())),
            (b"::1", Ok(IPv6Address::LOCALHOST.to_host_ref())),
            (b"[::1]", Err(InvalidHost)),
            (b"LocalHost", Err(InvalidHost)),
            (b"Local!Host", Err(InvalidHost)),
            (b"\xFF", Err(InvalidHost)),
            ("ü".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<HostRef, ParseError> = HostRef::parse_text(input);
            assert_eq!(result, *expected, "parse_text input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<HostRef, ParseError> = HostRef::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["localhost", "example.com", "127.0.0.1", "::1", "fe80::1"];

        for input in canonical {
            let value: HostRef = HostRef::try_from(*input).unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
