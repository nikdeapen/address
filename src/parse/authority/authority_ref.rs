use crate::ParseError::InvalidHost;
use crate::parse_port;
use crate::{AuthorityRef, DomainRef, HostForm, ParseError, impl_parse_ref};

impl<'a> AuthorityRef<'a> {
    //! Parse

    /// Parses an [AuthorityRef] from the `text`.
    pub fn parse(text: &'a [u8]) -> Result<Self, ParseError> {
        let (host, port): (&'a [u8], u16) = parse_port(text)?;
        match HostForm::classify(host)? {
            HostForm::IP(ip) => Ok(ip.to_host_ref().to_authority_ref(port)),
            HostForm::Domain => {
                let domain: DomainRef = DomainRef::parse(host).map_err(|_| InvalidHost)?;
                Ok(domain.to_host_ref().to_authority_ref(port))
            }
        }
    }
}

impl_parse_ref!(
    AuthorityRef,
    "An IPv6 host must be bracketed: `[::1]:80`.",
    "A domain must already be lowercase; use [`Authority`](crate::Authority) for mixed-case input.",
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidIPv6Address, InvalidPort};
    use crate::{AuthorityRef, DomainRef, IPv4Address, IPv6Address, ParseError};

    type TestCase<'a> = (&'a [u8], Result<AuthorityRef<'a>, ParseError>);

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidPort)),
            (b"localhost", Err(InvalidPort)),
            (b"localhost:", Err(InvalidPort)),
            (
                b"localhost:80",
                Ok(DomainRef::LOCALHOST.to_host_ref().to_authority_ref(80)),
            ),
            (
                b"127.0.0.1:80",
                Ok(IPv4Address::LOCALHOST.to_host_ref().to_authority_ref(80)),
            ),
            (
                b"[::1]:80",
                Ok(IPv6Address::LOCALHOST.to_host_ref().to_authority_ref(80)),
            ),
            (
                b"[::1%1]:80",
                Ok(IPv6Address::LOCALHOST.to_host_ref().to_authority_ref(80)),
            ),
            (b"[::1%eth0]:80", Err(InvalidIPv6Address)),
            (b"[]:80", Err(InvalidIPv6Address)),
            (b"::1:80", Err(InvalidAuthority)),
            (b"fe80::1:80", Err(InvalidAuthority)),
            (b"::80", Err(InvalidHost)),
            (b":80", Err(InvalidHost)),
            (b"LocalHost:80", Err(InvalidHost)),
            (b"Local!Host:80", Err(InvalidHost)),
            (b"\xFF:80", Err(InvalidHost)),
            ("ü:80".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<AuthorityRef, ParseError> = AuthorityRef::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

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
