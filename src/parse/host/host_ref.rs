use crate::ParseError::InvalidHost;
use crate::{DomainRef, HostRef, IPAddress, ParseError, impl_parse_ref};

impl<'a> HostRef<'a> {
    //! Parse

    /// A domain name or an unbracketed IP address: `localhost`, `127.0.0.1`, or `::1`.
    /// Domain names must already be in lowercase. Use [`Host`](crate::Host) to parse mixed-case input.
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
    "Domain names must already be in lowercase. Use [`Host`](crate::Host) to parse mixed-case input."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidHost;
    use crate::{DomainRef, HostRef, IPv4Address, ParseError};

    #[test]
    fn try_from_str() {
        let result: Result<HostRef, ParseError> = HostRef::try_from("localhost");
        let expected: Result<HostRef, ParseError> = Ok(HostRef::Name(DomainRef::LOCALHOST));
        assert_eq!(result, expected);

        let result: Result<HostRef, ParseError> = HostRef::try_from("LocalHost");
        let expected: Result<HostRef, ParseError> = Err(InvalidHost);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_text() {
        let test_cases: &[(&[u8], Result<HostRef, ParseError>)] = &[
            ("localhost".as_bytes(), Ok(HostRef::Name(DomainRef::LOCALHOST))),
            ("127.0.0.1".as_bytes(), Ok(IPv4Address::LOCALHOST.to_host_ref())),
            ("LocalHost".as_bytes(), Err(InvalidHost)),
            (b"\xFF".as_slice(), Err(InvalidHost)),
            ("ü".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<HostRef, ParseError> = HostRef::parse_text(input);
            assert_eq!(result, *expected, "input={:?}", input);
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
