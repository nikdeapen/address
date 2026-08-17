use crate::ParseError::{InvalidAuthority, InvalidHost};
use crate::parse_port;
use crate::{
    Authority, Domain, Host, IPAddress, IPv6Address, InvalidAddress, ParseError, doc_ignored_zone, doc_normalized,
    doc_recovers_value, impl_parse, impl_parse_string,
};

impl_parse!(Authority, doc_normalized!(), doc_ignored_zone!());
impl_parse_string!(Authority, doc_normalized!(), doc_ignored_zone!());

impl TryFrom<&[u8]> for Authority {
    type Error = ParseError;

    #[doc = doc_normalized!()]
    #[doc = doc_ignored_zone!()]
    fn try_from(authority: &[u8]) -> Result<Self, Self::Error> {
        let (s, port): (&[u8], u16) = parse_port(authority)?;
        if let Some(ip) = IPv6Address::parse_bracketed(s) {
            Ok(ip?.to_host().to_authority(port))
        } else {
            let host: Host = Host::try_from(s)?;
            if let Host::Address(ip) = &host
                && ip.is_v6()
            {
                return Err(InvalidAuthority);
            }
            Ok(host.to_authority(port))
        }
    }
}

impl TryFrom<Vec<u8>> for Authority {
    type Error = InvalidAddress<Vec<u8>>;

    #[doc = doc_normalized!()]
    #[doc = doc_ignored_zone!()]
    #[doc = doc_recovers_value!("authority")]
    fn try_from(authority: Vec<u8>) -> Result<Self, Self::Error> {
        match parse_port(authority.as_slice()) {
            Ok((s, port)) => {
                if let Some(ip) = IPv6Address::parse_bracketed(s) {
                    ip.map(|ip| ip.to_host().to_authority(port))
                        .map_err(|error| InvalidAddress::new(authority, error))
                } else if let Ok(ip) = IPAddress::parse(s) {
                    if ip.is_v6() {
                        Err(InvalidAddress::new(authority, InvalidAuthority))
                    } else {
                        Ok(ip.to_host().to_authority(port))
                    }
                } else {
                    let name_len: usize = s.len();
                    Domain::from_vec_prefix(authority, name_len)
                        .map(|domain| domain.to_host().to_authority(port))
                        .map_err(|name| InvalidAddress::new(name, InvalidHost))
                }
            }
            Err(error) => Err(InvalidAddress::new(authority, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidIPv6Address, InvalidPort};
    use crate::{Authority, Domain, IPv4Address, IPv6Address, InvalidAddress, ParseError};
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<Authority, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("localhost:", Err(InvalidPort)),
            ("localhost:xx", Err(InvalidPort)),
            (":80", Err(InvalidHost)),
            ("127.0.0.1:80", Ok(IPv4Address::LOCALHOST.to_host().to_authority(80))),
            ("::1:80", Err(InvalidAuthority)),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            (
                "[::FFFF]:80",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0, 0, 0xFFFF])
                    .to_host()
                    .to_authority(80)),
            ),
            ("[::1%1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            ("[::1%eth0]:80", Err(InvalidIPv6Address)),
            ("localhost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("LocalHost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("Local_Host:80", Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Authority, ParseError> = Authority::from_str(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn try_from_str() {
        let result: Result<Authority, ParseError> = Authority::try_from("localhost:80");
        let expected: Result<Authority, ParseError> = Ok(Domain::localhost().to_host().to_authority(80));
        assert_eq!(result, expected);

        let result: Result<Authority, ParseError> = Authority::try_from("LocalHost:80");
        let expected: Result<Authority, ParseError> = Ok(Domain::localhost().to_host().to_authority(80));
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_slice() {
        let result: Result<Authority, ParseError> = Authority::try_from("LocalHost:80".as_bytes());
        let expected: Result<Authority, ParseError> = Ok(Domain::localhost().to_host().to_authority(80));
        assert_eq!(result, expected);

        let result: Result<Authority, ParseError> = Authority::try_from(b"\xFF:80".as_slice());
        let expected: Result<Authority, ParseError> = Err(InvalidHost);
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_string() {
        let test_cases: &[(&str, Result<Authority, ParseError>)] = &[
            ("localhost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("LocalHost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            ("[::1%1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            ("[::1%eth0]:80", Err(InvalidIPv6Address)),
            ("::1:80", Err(InvalidAuthority)),
            ("Local!Host:80", Err(InvalidHost)),
            ("localhost:", Err(InvalidPort)),
            ("127.0.0.1:80", Ok(IPv4Address::LOCALHOST.to_host().to_authority(80))),
        ];

        for (input, expected) in test_cases {
            let result: Result<Authority, InvalidAddress<String>> = Authority::try_from(input.to_string());
            match result {
                Ok(value) => assert_eq!(Ok(value), *expected, "input={}", input),
                Err(error) => {
                    assert_eq!(error.value().as_str(), *input, "recovered input={}", input);
                    assert_eq!(Err(error.error()), *expected, "input={}", input);
                }
            }
        }
    }

    #[test]
    fn try_from_vec() {
        let test_cases: &[(&str, Result<Authority, ParseError>)] = &[
            ("localhost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("LocalHost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            ("[::1%1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            ("[::1%eth0]:80", Err(InvalidIPv6Address)),
            ("::1:80", Err(InvalidAuthority)),
            ("Local!Host:80", Err(InvalidHost)),
            ("localhost:", Err(InvalidPort)),
            ("127.0.0.1:80", Ok(IPv4Address::LOCALHOST.to_host().to_authority(80))),
        ];

        for (input, expected) in test_cases {
            let result: Result<Authority, InvalidAddress<Vec<u8>>> = Authority::try_from(Vec::from(*input));
            match result {
                Ok(value) => assert_eq!(Ok(value), *expected, "input={}", input),
                Err(error) => {
                    assert_eq!(Err(error.error()), *expected, "input={}", input);
                    assert_eq!(
                        error.into_value().as_slice(),
                        input.as_bytes(),
                        "recovered input={}",
                        input
                    );
                }
            }
        }
    }

    /// Mixed-case domain names are lowercased on every owned parse path.
    #[test]
    fn normalizes_case() {
        let test_cases: &[(&str, &str)] = &[
            ("LocalHost:80", "localhost:80"),
            ("WWW.Example.COM:443", "www.example.com:443"),
            ("A-B.C--D.EXAMPLE:0", "a-b.c--d.example:0"),
        ];

        for (input, expected) in test_cases {
            let authority: Authority = input.parse().unwrap();
            assert_eq!(authority.to_string(), *expected, "from_str input={}", input);

            let authority: Authority = Authority::try_from(input.to_string()).unwrap();
            assert_eq!(authority.to_string(), *expected, "try_from(String) input={}", input);

            let authority: Authority = Authority::try_from(Vec::from(*input)).unwrap();
            assert_eq!(authority.to_string(), *expected, "try_from(Vec<u8>) input={}", input);
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
            let value: Authority = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
