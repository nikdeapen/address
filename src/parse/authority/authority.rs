use crate::ParseError::{InvalidAuthority, InvalidHost};
use crate::parse_port;
use crate::{
    Authority, Domain, Host, IPAddress, IPv6Address, InvalidAddressError, ParseError, impl_parse, impl_parse_string,
};

impl Authority {
    //! Parse

    /// A host & a decimal port; an IPv6 host must be bracketed: `localhost:80` or `[::1]:80`.
    /// Domain names are normalized to lowercase.
    /// A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
        let (host, port): (&[u8], u16) = parse_port(text)?;
        if let Some(ip) = IPv6Address::parse_bracketed(host) {
            Ok(ip?.to_host().to_authority(port))
        } else {
            let host: Host = Host::parse_text(host)?;
            if let Host::Address(ip) = &host
                && ip.is_v6()
            {
                return Err(InvalidAuthority);
            }
            Ok(host.to_authority(port))
        }
    }
}

impl_parse!(
    Authority,
    "A host & a decimal port; an IPv6 host must be bracketed: `localhost:80` or `[::1]:80`.",
    "Domain names are normalized to lowercase.",
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

impl_parse_string!(
    Authority,
    "A host & a decimal port; an IPv6 host must be bracketed: `localhost:80` or `[::1]:80`.",
    "Domain names are normalized to lowercase.",
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

impl TryFrom<Vec<u8>> for Authority {
    type Error = InvalidAddressError<Vec<u8>>;

    /// A host & a decimal port; an IPv6 host must be bracketed: `localhost:80` or `[::1]:80`.
    /// Domain names are normalized to lowercase.
    /// A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    /// The error contains the unmodified `text`, which `TryFrom<String>` soundly recovers as a string.
    fn try_from(text: Vec<u8>) -> Result<Self, Self::Error> {
        match parse_port(text.as_slice()) {
            Ok((host, port)) => {
                if let Some(ip) = IPv6Address::parse_bracketed(host) {
                    ip.map(|ip| ip.to_host().to_authority(port))
                        .map_err(|error| InvalidAddressError::new(text, error))
                } else if let Ok(ip) = IPAddress::parse_text(host) {
                    if ip.is_v6() {
                        Err(InvalidAddressError::new(text, InvalidAuthority))
                    } else {
                        Ok(ip.to_host().to_authority(port))
                    }
                } else {
                    let name_len: usize = host.len();
                    Domain::parse_vec_prefix(text, name_len)
                        .map(|domain| domain.to_host().to_authority(port))
                        .map_err(|text| InvalidAddressError::new(text, InvalidHost))
                }
            }
            Err(error) => Err(InvalidAddressError::new(text, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidIPv6Address, InvalidPort};
    use crate::{Authority, Domain, IPv4Address, IPv6Address, InvalidAddressError, ParseError};
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
    fn parse_text() {
        let result: Result<Authority, ParseError> = Authority::parse_text("LocalHost:80".as_bytes());
        let expected: Result<Authority, ParseError> = Ok(Domain::localhost().to_host().to_authority(80));
        assert_eq!(result, expected);

        let result: Result<Authority, ParseError> = Authority::parse_text(b"\xFF:80".as_slice());
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
            let result: Result<Authority, InvalidAddressError<String>> = Authority::try_from(input.to_string());
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
            let result: Result<Authority, InvalidAddressError<Vec<u8>>> = Authority::try_from(Vec::from(*input));
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

            let authority: Authority = Authority::parse_text(input.as_bytes()).unwrap();
            assert_eq!(authority.to_string(), *expected, "parse_text input={}", input);

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
