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

    type TestCase<'a> = (&'a [u8], Result<Host, ParseError>);

    /// Builds the expected host for the canonical domain `name`.
    fn host(name: &str) -> Host {
        Domain::try_from(name).unwrap().to_host()
    }

    /// Every entry point must agree on every case; the owned ones normalize case.
    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidHost)),
            (b"localhost", Ok(host("localhost"))),
            (b"LocalHost", Ok(host("localhost"))),
            (b"WWW.Example.COM", Ok(host("www.example.com"))),
            (b"A-B.C--D.EXAMPLE", Ok(host("a-b.c--d.example"))),
            (b"127.0.0.1", Ok(IPv4Address::LOCALHOST.to_host())),
            (b"::1", Ok(IPv6Address::LOCALHOST.to_host())),
            (
                b"::FFFF",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0, 0, 0xFFFF]).to_host()),
            ),
            (b"[::1]", Err(InvalidHost)),
            (b"Local_Host", Err(InvalidHost)),
            (b"Local!Host", Err(InvalidHost)),
            (b"\xFF", Err(InvalidHost)),
            ("ü".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Host, ParseError> = Host::parse_text(input);
            assert_eq!(result, *expected, "parse_text input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<Host, ParseError> = Host::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<Host, ParseError> = Host::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);

            let result: Result<Host, InvalidAddressError<String>> =
                Host::try_from(text.to_string());
            let result: Result<Host, ParseError> = result.map_err(|error| {
                assert_eq!(error.value().as_str(), text, "recovered input={}", text);
                error.error()
            });
            assert_eq!(result, *expected, "try_from(String) input={}", text);
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
