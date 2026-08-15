use crate::ParseError::{InvalidAuthority, InvalidHost};
use crate::{Authority, Domain, Host, IPAddress, IPv6Address, InvalidAddress, ParseError, impl_parse};
use crate::{parse_port, strip_brackets};

impl_parse!(Authority, "Domain names are normalized to lowercase.");

impl TryFrom<&[u8]> for Authority {
    type Error = ParseError;

    /// Domain names are normalized to lowercase.
    fn try_from(authority: &[u8]) -> Result<Self, Self::Error> {
        let (s, port): (&[u8], u16) = parse_port(authority)?;
        if let Some(s) = strip_brackets(s) {
            let host: Host = IPv6Address::parse(s)?.to_host();
            Ok(host.to_authority(port))
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

impl TryFrom<String> for Authority {
    type Error = InvalidAddress<String>;

    /// Domain names are normalized to lowercase.
    fn try_from(authority: String) -> Result<Self, Self::Error> {
        match parse_port(authority.as_bytes()) {
            Ok((s, port)) => {
                if let Some(s) = strip_brackets(s) {
                    match IPv6Address::parse(s) {
                        Ok(ip) => Ok(ip.to_host().to_authority(port)),
                        Err(error) => Err(InvalidAddress::new(authority, error)),
                    }
                } else if let Ok(ip) = IPAddress::parse(s) {
                    if ip.is_v6() {
                        Err(InvalidAddress::new(authority, InvalidAuthority))
                    } else {
                        Ok(ip.to_host().to_authority(port))
                    }
                } else {
                    let name_len: usize = s.len();
                    match Domain::from_string_prefix(authority, name_len) {
                        Ok(domain) => Ok(domain.to_host().to_authority(port)),
                        Err(name) => Err(InvalidAddress::new(name, InvalidHost)),
                    }
                }
            }
            Err(error) => Err(InvalidAddress::new(authority, error)),
        }
    }
}

impl TryFrom<Vec<u8>> for Authority {
    type Error = InvalidAddress<Vec<u8>>;

    /// Domain names are normalized to lowercase.
    fn try_from(authority: Vec<u8>) -> Result<Self, Self::Error> {
        match parse_port(authority.as_slice()) {
            Ok((s, port)) => {
                if let Some(s) = strip_brackets(s) {
                    match IPv6Address::parse(s) {
                        Ok(ip) => Ok(ip.to_host().to_authority(port)),
                        Err(error) => Err(InvalidAddress::new(authority, error)),
                    }
                } else if let Ok(ip) = IPAddress::parse(s) {
                    if ip.is_v6() {
                        Err(InvalidAddress::new(authority, InvalidAuthority))
                    } else {
                        Ok(ip.to_host().to_authority(port))
                    }
                } else {
                    let name_len: usize = s.len();
                    match Domain::from_vec_prefix(authority, name_len) {
                        Ok(domain) => Ok(domain.to_host().to_authority(port)),
                        Err(name) => Err(InvalidAddress::new(name, InvalidHost)),
                    }
                }
            }
            Err(error) => Err(InvalidAddress::new(authority, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidPort};
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
        let test_cases: &[(&str, Result<Authority, &str>)] = &[
            ("localhost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("LocalHost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            ("Local!Host:80", Err("Local!Host:80")),
        ];

        for (input, expected) in test_cases {
            let result: Result<Authority, InvalidAddress<String>> = Authority::try_from(input.to_string());
            let result: Result<Authority, String> = result.map_err(|e| e.into_value());
            assert_eq!(result, expected.clone().map_err(String::from), "input={}", input);
        }
    }

    #[test]
    fn try_from_vec() {
        let test_cases: &[(&str, Result<Authority, &str>)] = &[
            ("localhost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("LocalHost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            ("Local!Host:80", Err("Local!Host:80")),
        ];

        for (input, expected) in test_cases {
            let result: Result<Authority, InvalidAddress<Vec<u8>>> = Authority::try_from(Vec::from(*input));
            let result: Result<Authority, Vec<u8>> = result.map_err(|e| e.into_value());
            assert_eq!(result, expected.clone().map_err(Vec::from), "input={}", input);
        }
    }
}
