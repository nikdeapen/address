use crate::ParseError::InvalidAuthority;
use crate::parse_port;
use crate::{AuthorityRef, HostRef, IPv6Address, ParseError, doc_ignored_zone, doc_lowercase_required, impl_parse_ref};

impl_parse_ref!(AuthorityRef, doc_lowercase_required!(Authority), doc_ignored_zone!());

impl<'a> TryFrom<&'a [u8]> for AuthorityRef<'a> {
    type Error = ParseError;

    #[doc = doc_lowercase_required!(Authority)]
    #[doc = doc_ignored_zone!()]
    fn try_from(authority: &'a [u8]) -> Result<Self, Self::Error> {
        let (s, port): (&[u8], u16) = parse_port(authority)?;
        if let Some(ip) = IPv6Address::parse_bracketed(s) {
            Ok(ip?.to_host_ref().to_authority_ref(port))
        } else {
            let host: HostRef = HostRef::try_from(s)?;
            if let HostRef::Address(ip) = host
                && ip.is_v6()
            {
                return Err(InvalidAuthority);
            }
            Ok(host.to_authority_ref(port))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidIPv6Address};
    use crate::{AuthorityRef, DomainRef, HostRef, IPv4Address, IPv6Address, ParseError};

    #[test]
    fn try_from_str() {
        let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from("localhost:80");
        let expected: Result<AuthorityRef, ParseError> = Ok(AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80));
        assert_eq!(result, expected);

        let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from("LocalHost:80");
        let expected: Result<AuthorityRef, ParseError> = Err(InvalidHost);
        assert_eq!(result, expected);

        let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from("::1:80");
        let expected: Result<AuthorityRef, ParseError> = Err(InvalidAuthority);
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_slice() {
        let test_cases: &[(&[u8], Result<AuthorityRef, ParseError>)] = &[
            (
                "localhost:80".as_bytes(),
                Ok(AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80)),
            ),
            (
                "[::1%1]:80".as_bytes(),
                Ok(AuthorityRef::new(IPv6Address::LOCALHOST.to_host_ref(), 80)),
            ),
            ("[::1%eth0]:80".as_bytes(), Err(InvalidIPv6Address)),
            ("::1:80".as_bytes(), Err(InvalidAuthority)),
            ("::80".as_bytes(), Err(InvalidHost)),
            ("fe80::1:80".as_bytes(), Err(InvalidAuthority)),
            (
                "127.0.0.1:80".as_bytes(),
                Ok(AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80)),
            ),
            ("LocalHost:80".as_bytes(), Err(InvalidHost)),
            (b"\xFF:80".as_slice(), Err(InvalidHost)),
            ("ü:80".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from(*input);
            assert_eq!(result, *expected, "input={:?}", input);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &[
            "localhost:80",
            "example.com:443",
            "127.0.0.1:80",
            "[::1]:443",
            "[fe80::1]:0",
        ];

        for input in canonical {
            let value: AuthorityRef = AuthorityRef::try_from(*input).unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
