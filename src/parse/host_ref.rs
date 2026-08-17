use crate::ParseError::InvalidHost;
use crate::{DomainRef, HostRef, IPAddress, ParseError, doc_lowercase_required, impl_parse_ref};

impl_parse_ref!(HostRef, doc_lowercase_required!(Host));

impl<'a> TryFrom<&'a [u8]> for HostRef<'a> {
    type Error = ParseError;

    #[doc = doc_lowercase_required!(Host)]
    fn try_from(host: &'a [u8]) -> Result<Self, Self::Error> {
        if let Ok(ip) = IPAddress::parse(host) {
            Ok(ip.to_host_ref())
        } else if let Ok(domain) = DomainRef::try_from(host) {
            Ok(domain.to_host_ref())
        } else {
            Err(InvalidHost)
        }
    }
}

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
    fn try_from_slice() {
        let test_cases: &[(&[u8], Result<HostRef, ParseError>)] = &[
            ("localhost".as_bytes(), Ok(HostRef::Name(DomainRef::LOCALHOST))),
            ("127.0.0.1".as_bytes(), Ok(IPv4Address::LOCALHOST.to_host_ref())),
            ("LocalHost".as_bytes(), Err(InvalidHost)),
            (b"\xFF".as_slice(), Err(InvalidHost)),
            ("ü".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<HostRef, ParseError> = HostRef::try_from(*input);
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
