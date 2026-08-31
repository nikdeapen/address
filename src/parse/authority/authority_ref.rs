use crate::ParseError::InvalidHost;
use crate::parse_port;
use crate::{AuthorityRef, DomainRef, HostForm, ParseError, impl_parse_ref};

impl<'a> AuthorityRef<'a> {
    //! Parse

    /// A host & a decimal port; an IPv6 host must be bracketed: `localhost:80` or `[::1]:80`.
    /// Domain names must already be in lowercase. Use [`Authority`](crate::Authority) to parse
    /// mixed-case input.
    /// A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    pub fn parse_text(text: &'a [u8]) -> Result<Self, ParseError> {
        let (host, port): (&'a [u8], u16) = parse_port(text)?;
        match HostForm::classify(host)? {
            HostForm::IP(ip) => Ok(ip.to_host_ref().to_authority_ref(port)),
            HostForm::Domain => {
                let domain: DomainRef = DomainRef::parse_text(host).map_err(|_| InvalidHost)?;
                Ok(domain.to_host_ref().to_authority_ref(port))
            }
        }
    }
}

impl_parse_ref!(
    AuthorityRef,
    "A host & a decimal port; an IPv6 host must be bracketed: `localhost:80` or `[::1]:80`.",
    "Domain names must already be in lowercase.",
    "Use [`Authority`](crate::Authority) to parse mixed-case input.",
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidIPv6Address};
    use crate::{AuthorityRef, DomainRef, HostRef, IPv4Address, IPv6Address, ParseError};

    #[test]
    fn try_from_str() {
        let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from("localhost:80");
        let expected: Result<AuthorityRef, ParseError> =
            Ok(AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80));
        assert_eq!(result, expected);

        let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from("LocalHost:80");
        let expected: Result<AuthorityRef, ParseError> = Err(InvalidHost);
        assert_eq!(result, expected);

        let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from("::1:80");
        let expected: Result<AuthorityRef, ParseError> = Err(InvalidAuthority);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_text() {
        let test_cases: &[(&[u8], Result<AuthorityRef, ParseError>)] = &[
            (
                "localhost:80".as_bytes(),
                Ok(AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80)),
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
            let result: Result<AuthorityRef, ParseError> = AuthorityRef::parse_text(input);
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
