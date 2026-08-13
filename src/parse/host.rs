use crate::{Domain, Host, HostRef, InvalidAddress, ParseError, impl_parse};

impl_parse!(Host, "Domain names are normalized to lowercase.");

impl TryFrom<&[u8]> for Host {
    type Error = ParseError;

    /// Domain names are normalized to lowercase.
    fn try_from(host: &[u8]) -> Result<Self, Self::Error> {
        match HostRef::try_from(host) {
            Ok(host) => Ok(host.to_host()),
            Err(error) => {
                // Lowercasing can only rescue a mixed-case domain; other failures keep the original error.
                if host.iter().any(|b| b.is_ascii_uppercase())
                    && let Ok(domain) = Domain::try_from(host)
                {
                    Ok(domain.to_host())
                } else {
                    Err(error)
                }
            }
        }
    }
}

impl TryFrom<String> for Host {
    type Error = InvalidAddress<String>;

    /// Domain names are normalized to lowercase.
    fn try_from(host: String) -> Result<Self, Self::Error> {
        match HostRef::try_from(host.as_bytes()) {
            Ok(HostRef::Address(ip)) => Ok(Self::Address(ip)),
            Ok(HostRef::Name(_)) => Ok(Self::Name(unsafe { Domain::new_unchecked(host) })),
            Err(error) => {
                // Lowercasing can only rescue a mixed-case domain; other failures keep the original error.
                if Domain::is_valid_name(host.as_bytes(), true) {
                    let mut name: String = host;
                    name.make_ascii_lowercase();
                    Ok(Self::Name(unsafe { Domain::new_unchecked(name) }))
                } else {
                    Err(InvalidAddress::new(host, error))
                }
            }
        }
    }
}

impl TryFrom<Vec<u8>> for Host {
    type Error = InvalidAddress<Vec<u8>>;

    /// Domain names are normalized to lowercase.
    fn try_from(host: Vec<u8>) -> Result<Self, Self::Error> {
        match HostRef::try_from(host.as_slice()) {
            Ok(HostRef::Address(ip)) => Ok(Self::Address(ip)),
            Ok(HostRef::Name(_)) => {
                let name: String = unsafe { String::from_utf8_unchecked(host) };
                Ok(Self::Name(unsafe { Domain::new_unchecked(name) }))
            }
            Err(error) => {
                // Lowercasing can only rescue a mixed-case domain; other failures keep the original error.
                if Domain::is_valid_name(host.as_slice(), true) {
                    let mut name: Vec<u8> = host;
                    name.make_ascii_lowercase();
                    let name: String = unsafe { String::from_utf8_unchecked(name) };
                    Ok(Self::Name(unsafe { Domain::new_unchecked(name) }))
                } else {
                    Err(InvalidAddress::new(host, error))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidHost;
    use crate::{Domain, Host, IPv4Address, IPv6Address, InvalidAddress, ParseError};
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<Host, ParseError>)] = &[
            ("", Err(InvalidHost)),
            ("127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host())),
            ("::1", Ok(IPv6Address::LOCALHOST.to_host())),
            ("::FFFF", Ok(IPv6Address::from([0, 0, 0, 0, 0, 0, 0, 0xFFFF]).to_host())),
            ("[::1]", Err(InvalidHost)),
            ("localhost", Ok(Domain::localhost().to_host())),
            ("LocalHost", Ok(Domain::localhost().to_host())),
            ("Local_Host", Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Host, ParseError> = Host::from_str(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn try_from_str() {
        let result: Result<Host, ParseError> = Host::try_from("localhost");
        let expected: Result<Host, ParseError> = Ok(Domain::localhost().to_host());
        assert_eq!(result, expected);

        let result: Result<Host, ParseError> = Host::try_from("LocalHost");
        let expected: Result<Host, ParseError> = Ok(Domain::localhost().to_host());
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_slice() {
        let result: Result<Host, ParseError> = Host::try_from("LocalHost".as_bytes());
        let expected: Result<Host, ParseError> = Ok(Domain::localhost().to_host());
        assert_eq!(result, expected);

        let result: Result<Host, ParseError> = Host::try_from(b"\xFF".as_slice());
        let expected: Result<Host, ParseError> = Err(InvalidHost);
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_string() {
        let test_cases: &[(&str, Result<Host, &str>)] = &[
            ("localhost", Ok(Domain::localhost().to_host())),
            ("LocalHost", Ok(Domain::localhost().to_host())),
            ("127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host())),
            ("Local!Host", Err("Local!Host")),
        ];

        for (input, expected) in test_cases {
            let result: Result<Host, InvalidAddress<String>> = Host::try_from(input.to_string());
            let result: Result<Host, String> = result.map_err(|e| e.into_value());
            assert_eq!(result, expected.clone().map_err(String::from), "input={}", input);
        }
    }

    #[test]
    fn try_from_vec() {
        let test_cases: &[(&str, Result<Host, &str>)] = &[
            ("localhost", Ok(Domain::localhost().to_host())),
            ("LocalHost", Ok(Domain::localhost().to_host())),
            ("127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host())),
            ("Local!Host", Err("Local!Host")),
        ];

        for (input, expected) in test_cases {
            let result: Result<Host, InvalidAddress<Vec<u8>>> = Host::try_from(Vec::from(*input));
            let result: Result<Host, Vec<u8>> = result.map_err(|e| e.into_value());
            assert_eq!(result, expected.clone().map_err(Vec::from), "input={}", input);
        }
    }
}
