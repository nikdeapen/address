use crate::parse_port;
use crate::{DomainRef, EndpointRef, ParseError, impl_parse_ref};

impl<'a> EndpointRef<'a> {
    //! Parse

    /// A domain name & a decimal port: `localhost:80`.
    /// Domain names must already be in lowercase. Use [`Endpoint`](crate::Endpoint) to parse
    /// mixed-case input.
    pub fn parse_text(text: &'a [u8]) -> Result<Self, ParseError> {
        let (domain, port): (&[u8], u16) = parse_port(text)?;
        let domain: DomainRef = DomainRef::parse_text(domain)?;
        Ok(Self::new(domain, port))
    }
}

impl_parse_ref!(
    EndpointRef,
    "A domain name & a decimal port: `localhost:80`.",
    "Domain names must already be in lowercase.",
    "Use [`Endpoint`](crate::Endpoint) to parse mixed-case input."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidDomain, InvalidPort};
    use crate::{DomainRef, EndpointRef, ParseError};

    type TestCase<'a> = (&'a [u8], Result<EndpointRef<'a>, ParseError>);

    /// Every entry point must agree on every case; mixed case is rejected, not normalized.
    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidPort)),
            (b"localhost", Err(InvalidPort)),
            (b"localhost:", Err(InvalidPort)),
            (b"localhost:xx", Err(InvalidPort)),
            (
                b"localhost:80",
                Ok(EndpointRef::new(DomainRef::LOCALHOST, 80)),
            ),
            (
                b"example.com:443",
                Ok(EndpointRef::new(DomainRef::EXAMPLE, 443)),
            ),
            (b":80", Err(InvalidDomain)),
            (b"LocalHost:80", Err(InvalidDomain)),
            (b"127.0.0.1:80", Err(InvalidDomain)),
            (b"\xFF:80", Err(InvalidDomain)),
            ("ü:80".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<EndpointRef, ParseError> = EndpointRef::parse_text(input);
            assert_eq!(result, *expected, "parse_text input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<EndpointRef, ParseError> = EndpointRef::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["localhost:80", "example.com:443", "a.b.c:65535", "x:0"];

        for input in canonical {
            let value: EndpointRef = EndpointRef::try_from(*input).unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
