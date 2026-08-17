use crate::ParseError::InvalidHost;
use crate::{
    Domain, Host, IPAddress, InvalidAddress, ParseError, doc_normalized, doc_recovers_value, impl_parse,
    impl_parse_string,
};

impl_parse!(Host, doc_normalized!());
impl_parse_string!(Host, doc_normalized!());

impl Host {
    //! Owned Parsing

    /// Creates a host from the first `len` bytes of `host`, normalizing domain names to lowercase.
    ///
    /// Returns the unmodified `host` if the prefix is not a valid host.
    pub(crate) fn from_vec_prefix(host: Vec<u8>, len: usize) -> Result<Self, Vec<u8>> {
        if let Ok(ip) = IPAddress::parse(&host[..len]) {
            Ok(ip.to_host())
        } else {
            Domain::from_vec_prefix(host, len).map(Domain::to_host)
        }
    }
}

impl TryFrom<&[u8]> for Host {
    type Error = ParseError;

    #[doc = doc_normalized!()]
    fn try_from(host: &[u8]) -> Result<Self, Self::Error> {
        if let Ok(ip) = IPAddress::parse(host) {
            Ok(ip.to_host())
        } else if let Ok(domain) = Domain::try_from(host) {
            Ok(domain.to_host())
        } else {
            Err(InvalidHost)
        }
    }
}

impl TryFrom<Vec<u8>> for Host {
    type Error = InvalidAddress<Vec<u8>>;

    #[doc = doc_normalized!()]
    #[doc = doc_recovers_value!("host")]
    fn try_from(host: Vec<u8>) -> Result<Self, Self::Error> {
        let len: usize = host.len();
        Self::from_vec_prefix(host, len).map_err(|host| InvalidAddress::new(host, InvalidHost))
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
        let test_cases: &[(&str, Result<Host, ParseError>)] = &[
            ("localhost", Ok(Domain::localhost().to_host())),
            ("LocalHost", Ok(Domain::localhost().to_host())),
            ("127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host())),
            ("Local!Host", Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Host, InvalidAddress<String>> = Host::try_from(input.to_string());
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
        let test_cases: &[(&str, Result<Host, ParseError>)] = &[
            ("localhost", Ok(Domain::localhost().to_host())),
            ("LocalHost", Ok(Domain::localhost().to_host())),
            ("127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host())),
            ("Local!Host", Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Host, InvalidAddress<Vec<u8>>> = Host::try_from(Vec::from(*input));
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

    /// Each canonical string must parse and display back to the exact same string.
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
            let value: Host = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
