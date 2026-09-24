use crate::{AuthorityRef, DomainRef, HostForm, ParseError, impl_parse_ref, parse_port};

impl<'a> AuthorityRef<'a> {
    //! Parse

    /// Parses an [AuthorityRef] from the `text`.
    ///
    /// # Notes
    /// - An IPv6 host must be bracketed: `[::1]:80`.
    /// - A domain must already be lowercase; use [`Authority`](crate::Authority) for normalization.
    /// - A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    pub fn parse(text: &'a [u8]) -> Result<Self, ParseError> {
        let (host, port): (&[u8], u16) = parse_port(text)?;
        match HostForm::classify(host)? {
            HostForm::IP(ip) => Ok(ip.to_host_ref().to_authority_ref(port)),
            HostForm::Domain => {
                let domain: DomainRef =
                    DomainRef::parse(host).map_err(|_| HostForm::domain_error(host))?;
                Ok(domain.to_host_ref().to_authority_ref(port))
            }
        }
    }
}

impl_parse_ref!(AuthorityRef);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidIPv6Address, InvalidPort};
    use crate::{AuthorityRef, DomainRef, IPv4Address, IPv6Address, ParseError};

    type TestCase<'a> = (&'a [u8], Result<AuthorityRef<'a>, ParseError>);

    fn authority(name: &'static str, port: u16) -> AuthorityRef<'static> {
        DomainRef::try_from(name)
            .unwrap()
            .to_host_ref()
            .to_authority_ref(port)
    }

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidPort)),
            (b"localhost", Err(InvalidPort)),
            (b"localhost:", Err(InvalidPort)),
            (b"localhost:xx", Err(InvalidPort)),
            (b"localhost:80", Ok(authority("localhost", 80))),
            (b"LocalHost:80", Err(InvalidHost)),
            (
                b"www.example.com:443",
                Ok(authority("www.example.com", 443)),
            ),
            (b"a-b.c--d.example:0", Ok(authority("a-b.c--d.example", 0))),
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
            (
                b"[::FFFF]:80",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0, 0, 0xFFFF])
                    .to_host_ref()
                    .to_authority_ref(80)),
            ),
            (b"[::1%eth0]:80", Err(InvalidIPv6Address)),
            (b"[]:80", Err(InvalidIPv6Address)),
            (b"::1:80", Err(InvalidAuthority)),
            (b"fe80::1:80", Err(InvalidAuthority)),
            (b"2001:db8::1", Err(InvalidAuthority)),
            (b"::80", Err(InvalidAuthority)),
            (b":80", Err(InvalidHost)),
            (b"local_host:80", Err(InvalidHost)),
            (b"local!host:80", Err(InvalidHost)),
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
