use crate::parse_port;
use crate::{DomainRef, EndpointRef, ParseError, impl_parse_ref};

impl_parse_ref!(
    EndpointRef,
    "Domain names must already be in lowercase. Use [`Endpoint`](crate::Endpoint) to parse mixed-case input."
);

impl<'a> TryFrom<&'a [u8]> for EndpointRef<'a> {
    type Error = ParseError;

    /// Domain names must already be in lowercase. Use [`Endpoint`](crate::Endpoint) to parse mixed-case input.
    fn try_from(endpoint: &'a [u8]) -> Result<Self, Self::Error> {
        let (domain, port): (&[u8], u16) = parse_port(endpoint)?;
        let domain: DomainRef = DomainRef::try_from(domain)?;
        Ok(Self::new(domain, port))
    }
}

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
    fn try_from_slice() {
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
            let result: Result<EndpointRef, ParseError> = EndpointRef::try_from(*input);
            assert_eq!(result, *expected, "input={:?}", input);
        }
    }
}
