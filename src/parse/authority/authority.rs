use crate::ParseError::InvalidHost;
use crate::parse_port;
use crate::{
    Authority, Domain, HostForm, InvalidAddressError, ParseError, impl_parse, impl_parse_string,
};

impl Authority {
    //! Parse

    /// Parses an [Authority] from the `text`.
    ///
    /// # Notes
    /// - An IPv6 host must be bracketed: `[::1]:80`.
    /// - Domain names are normalized to lowercase.
    /// - A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        let (host, port): (&[u8], u16) = parse_port(text)?;
        match HostForm::classify(host)? {
            HostForm::IP(ip) => Ok(ip.to_host().to_authority(port)),
            HostForm::Domain => {
                let domain: Domain = Domain::parse(host).map_err(|_| InvalidHost)?;
                Ok(domain.to_host().to_authority(port))
            }
        }
    }

    /// Parses an [Authority] from the `text`.
    pub(crate) fn parse_vec(text: Vec<u8>) -> Result<Self, InvalidAddressError<Vec<u8>>> {
        let (host_len, port): (usize, u16) = match parse_port(text.as_slice()) {
            Ok((host, port)) => (host.len(), port),
            Err(error) => return Err(InvalidAddressError::new(text, error)),
        };
        match HostForm::classify(&text[..host_len]) {
            Err(error) => Err(InvalidAddressError::new(text, error)),
            Ok(HostForm::IP(ip)) => Ok(ip.to_host().to_authority(port)),
            Ok(HostForm::Domain) => Domain::parse_vec_prefix(text, host_len)
                .map(|domain| domain.to_host().to_authority(port))
                .map_err(|text| InvalidAddressError::new(text, InvalidHost)),
        }
    }
}

impl_parse!(Authority);

impl_parse_string!(Authority);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidIPv6Address, InvalidPort};
    use crate::{Authority, Domain, IPv4Address, IPv6Address, InvalidAddressError, ParseError};
    use std::str::FromStr;

    type TestCase<'a> = (&'a [u8], Result<Authority, ParseError>);

    fn authority(name: &str, port: u16) -> Authority {
        Domain::try_from(name).unwrap().to_host().to_authority(port)
    }

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidPort)),
            (b"localhost", Err(InvalidPort)),
            (b"localhost:", Err(InvalidPort)),
            (b"localhost:xx", Err(InvalidPort)),
            (b"localhost:80", Ok(authority("localhost", 80))),
            (b"LocalHost:80", Ok(authority("localhost", 80))),
            (
                b"WWW.Example.COM:443",
                Ok(authority("www.example.com", 443)),
            ),
            (b"A-B.C--D.EXAMPLE:0", Ok(authority("a-b.c--d.example", 0))),
            (
                b"127.0.0.1:80",
                Ok(IPv4Address::LOCALHOST.to_host().to_authority(80)),
            ),
            (
                b"[::1]:80",
                Ok(IPv6Address::LOCALHOST.to_host().to_authority(80)),
            ),
            (
                b"[::1%1]:80",
                Ok(IPv6Address::LOCALHOST.to_host().to_authority(80)),
            ),
            (
                b"[::FFFF]:80",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0, 0, 0xFFFF])
                    .to_host()
                    .to_authority(80)),
            ),
            (b"[::1%eth0]:80", Err(InvalidIPv6Address)),
            (b"[]:80", Err(InvalidIPv6Address)),
            (b"::1:80", Err(InvalidAuthority)),
            (b"fe80::1:80", Err(InvalidAuthority)),
            (b"::80", Err(InvalidHost)),
            (b":80", Err(InvalidHost)),
            (b"Local_Host:80", Err(InvalidHost)),
            (b"Local!Host:80", Err(InvalidHost)),
            (b"\xFF:80", Err(InvalidHost)),
            ("ü:80".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Authority, ParseError> = Authority::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<Authority, ParseError> = Authority::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<Authority, ParseError> = Authority::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);

            let result: Result<Authority, InvalidAddressError<String>> =
                Authority::try_from(text.to_string());
            let result: Result<Authority, ParseError> = result.map_err(|error| {
                assert_eq!(error.value().as_str(), text, "recovered input={}", text);
                error.error()
            });
            assert_eq!(result, *expected, "try_from(String) input={}", text);
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
            let value: Authority = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
