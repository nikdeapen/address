use crate::{DomainRef, EndpointRef, ParseError, impl_parse_ref, parse_port};

impl<'a> EndpointRef<'a> {
    //! Parse

    /// Parses an [EndpointRef] from the `text`.
    ///
    /// # Notes
    /// - The domain must already be lowercase; use [`Endpoint`](crate::Endpoint) for normalization.
    pub fn parse(text: &'a [u8]) -> Result<Self, ParseError> {
        let (domain, port): (&[u8], u16) = parse_port(text)?;
        let domain: DomainRef = DomainRef::parse(domain)?;
        Ok(domain.to_endpoint_ref(port))
    }
}

impl_parse_ref!(EndpointRef);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidDomain, InvalidPort};
    use crate::{DomainRef, EndpointRef, ParseError};

    type TestCase<'a> = (&'a [u8], Result<EndpointRef<'a>, ParseError>);

    fn endpoint(name: &'static str, port: u16) -> EndpointRef<'static> {
        DomainRef::try_from(name).unwrap().to_endpoint_ref(port)
    }

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidPort)),
            (b"localhost", Err(InvalidPort)),
            (b"localhost:", Err(InvalidPort)),
            (b"localhost:xx", Err(InvalidPort)),
            (b"localhost:99999", Err(InvalidPort)),
            (b"localhost:80", Ok(endpoint("localhost", 80))),
            (b"example.com:443", Ok(endpoint("example.com", 443))),
            (b"LocalHost:80", Err(InvalidDomain)),
            (b"www.example.com:443", Ok(endpoint("www.example.com", 443))),
            (b":80", Err(InvalidDomain)),
            (b"[localhost]:80", Err(InvalidDomain)),
            (b"local_host:80", Err(InvalidDomain)),
            (b"local!host:80", Err(InvalidDomain)),
            (b"127.0.0.1:80", Err(InvalidDomain)),
            (b"\xFF:80", Err(InvalidDomain)),
            ("ü:80".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<EndpointRef, ParseError> = EndpointRef::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<EndpointRef, ParseError> = EndpointRef::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["localhost:80", "example.com:443", "a.b.c:65535", "x:0"];

        for input in canonical {
            let value: EndpointRef = EndpointRef::try_from(*input).unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
