use crate::ParseError::InvalidHost;
use crate::{DomainRef, HostRef, IPAddress, ParseError, impl_parse_ref};

impl<'a> HostRef<'a> {
    //! Parse

    /// Parses a [HostRef] from the `text`.
    ///
    /// # Notes
    /// - IP addresses must be unbracketed: `::1`, not `[::1]`.
    /// - A domain must already be lowercase; use [`Host`](crate::Host) for normalization.
    pub fn parse(text: &'a [u8]) -> Result<Self, ParseError> {
        if let Ok(ip) = IPAddress::parse(text) {
            Ok(ip.to_host_ref())
        } else if let Ok(domain) = DomainRef::parse(text) {
            Ok(domain.to_host_ref())
        } else {
            Err(InvalidHost)
        }
    }
}

impl_parse_ref!(HostRef);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidHost;
    use crate::{DomainRef, HostRef, IPv4Address, IPv6Address, ParseError};

    type TestCase<'a> = (&'a [u8], Result<HostRef<'a>, ParseError>);

    fn host(name: &'static str) -> HostRef<'static> {
        DomainRef::try_from(name).unwrap().to_host_ref()
    }

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidHost)),
            (b"localhost", Ok(host("localhost"))),
            (b"example.com", Ok(host("example.com"))),
            (b"LocalHost", Err(InvalidHost)),
            (b"www.example.com", Ok(host("www.example.com"))),
            (b"a-b.c--d.example", Ok(host("a-b.c--d.example"))),
            (b"127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host_ref())),
            (b"::1", Ok(IPv6Address::LOCALHOST.to_host_ref())),
            (
                b"::FFFF",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0, 0, 0xFFFF]).to_host_ref()),
            ),
            (b"[::1]", Err(InvalidHost)),
            (b"local_host", Err(InvalidHost)),
            (b"local!host", Err(InvalidHost)),
            (b"\xFF", Err(InvalidHost)),
            ("ü".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<HostRef, ParseError> = HostRef::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<HostRef, ParseError> = HostRef::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &[
            "localhost",
            "example.com",
            "a-b.c--d.example",
            "127.0.0.1",
            "::1",
            "fe80::1",
        ];

        for input in canonical {
            let value: HostRef = HostRef::try_from(*input).unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
