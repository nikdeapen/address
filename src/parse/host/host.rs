use crate::ParseError::InvalidHost;
use crate::{
    Domain, Host, IPAddress, InvalidAddressError, ParseError, impl_parse, impl_parse_string,
};

impl Host {
    //! Parse

    /// A domain name or an unbracketed IP address: `localhost`, `127.0.0.1`, or `::1`.
    /// Domain names are normalized to lowercase.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
        if let Ok(ip) = IPAddress::parse_text(text) {
            Ok(ip.to_host())
        } else if let Ok(domain) = Domain::parse_text(text) {
            Ok(domain.to_host())
        } else {
            Err(InvalidHost)
        }
    }

    /// Creates a host from the `text`, normalizing domain names to lowercase.
    ///
    /// The error holds the unmodified `text`, which `TryFrom<String>` soundly recovers as a string.
    pub(crate) fn parse_vec(text: Vec<u8>) -> Result<Self, InvalidAddressError<Vec<u8>>> {
        if let Ok(ip) = IPAddress::parse_text(text.as_slice()) {
            Ok(ip.to_host())
        } else {
            let len: usize = text.len();
            Domain::parse_vec_prefix(text, len)
                .map(Domain::to_host)
                .map_err(|text| InvalidAddressError::new(text, InvalidHost))
        }
    }
}

impl_parse!(
    Host,
    "A domain name or an unbracketed IP address: `localhost`, `127.0.0.1`, or `::1`.",
    "Domain names are normalized to lowercase."
);

impl_parse_string!(
    Host,
    "A domain name or an unbracketed IP address: `localhost`, `127.0.0.1`, or `::1`.",
    "Domain names are normalized to lowercase."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidHost;
    use crate::{Domain, Host, IPv4Address, IPv6Address, InvalidAddressError, ParseError};
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<Host, ParseError>)] = &[
            ("", Err(InvalidHost)),
            ("127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host())),
            ("::1", Ok(IPv6Address::LOCALHOST.to_host())),
            (
                "::FFFF",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0, 0, 0xFFFF]).to_host()),
            ),
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
    fn parse_text() {
        let result: Result<Host, ParseError> = Host::parse_text("LocalHost".as_bytes());
        let expected: Result<Host, ParseError> = Ok(Domain::localhost().to_host());
        assert_eq!(result, expected);

        let result: Result<Host, ParseError> = Host::parse_text(b"\xFF".as_slice());
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
            let result: Result<Host, InvalidAddressError<String>> =
                Host::try_from(input.to_string());
            match result {
                Ok(value) => assert_eq!(Ok(value), *expected, "input={}", input),
                Err(error) => {
                    assert_eq!(error.value().as_str(), *input, "recovered input={}", input);
                    assert_eq!(Err(error.error()), *expected, "input={}", input);
                }
            }
        }
    }

    /// Mixed-case domain names are lowercased on every owned parse path.
    #[test]
    fn normalizes_case() {
        let test_cases: &[(&str, &str)] = &[
            ("LocalHost", "localhost"),
            ("WWW.Example.COM", "www.example.com"),
            ("A-B.C--D.EXAMPLE", "a-b.c--d.example"),
        ];

        for (input, expected) in test_cases {
            let host: Host = input.parse().unwrap();
            assert_eq!(host.to_string(), *expected, "from_str input={}", input);

            let host: Host = Host::try_from(*input).unwrap();
            assert_eq!(
                host.to_string(),
                *expected,
                "try_from(&str) input={}",
                input
            );

            let host: Host = Host::parse_text(input.as_bytes()).unwrap();
            assert_eq!(host.to_string(), *expected, "parse_text input={}", input);

            let host: Host = Host::try_from(input.to_string()).unwrap();
            assert_eq!(
                host.to_string(),
                *expected,
                "try_from(String) input={}",
                input
            );
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
