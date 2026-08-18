use crate::parse_port;
use crate::{DomainRef, EndpointRef, ParseError, impl_parse_ref};

impl<'a> EndpointRef<'a> {
    //! Parse

    /// A domain name & a decimal port: `localhost:80`.
    /// Domain names must already be in lowercase. Use [`Endpoint`](crate::Endpoint) to parse mixed-case input.
    pub fn parse_text(text: &'a [u8]) -> Result<Self, ParseError> {
        let (domain, port): (&[u8], u16) = parse_port(text)?;
        let domain: DomainRef = DomainRef::parse_text(domain)?;
        Ok(Self::new(domain, port))
    }
}

impl_parse_ref!(
    EndpointRef,
    "A domain name & a decimal port: `localhost:80`.",
    "Domain names must already be in lowercase. Use [`Endpoint`](crate::Endpoint) to parse mixed-case input."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidDomain;
    use crate::{DomainRef, EndpointRef, ParseError};

    #[test]
    fn try_from_str() {
        let result: Result<EndpointRef, ParseError> = EndpointRef::try_from("localhost:80");
        let expected: Result<EndpointRef, ParseError> = Ok(EndpointRef::new(DomainRef::LOCALHOST, 80));
        assert_eq!(result, expected);

        let result: Result<EndpointRef, ParseError> = EndpointRef::try_from("LocalHost:80");
        let expected: Result<EndpointRef, ParseError> = Err(InvalidDomain);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_text() {
        let test_cases: &[(&[u8], Result<EndpointRef, ParseError>)] = &[
            (
                "localhost:80".as_bytes(),
                Ok(EndpointRef::new(DomainRef::LOCALHOST, 80)),
            ),
            ("LocalHost:80".as_bytes(), Err(InvalidDomain)),
            (b"\xFF:80".as_slice(), Err(InvalidDomain)),
            ("ü:80".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<EndpointRef, ParseError> = EndpointRef::parse_text(input);
            assert_eq!(result, *expected, "input={:?}", input);
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
