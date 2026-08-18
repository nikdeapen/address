use crate::ParseError::InvalidDomain;
use crate::parse_port;
use crate::{Domain, Endpoint, InvalidAddressError, ParseError, impl_parse, impl_parse_string};

impl Endpoint {
    //! Parse

    /// A domain name & a decimal port: `localhost:80`.
    /// Domain names are normalized to lowercase.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
        let (name, port): (&[u8], u16) = parse_port(text)?;
        let domain: Domain = Domain::parse_text(name)?;
        Ok(domain.to_endpoint(port))
    }
}

impl_parse!(
    Endpoint,
    "A domain name & a decimal port: `localhost:80`.",
    "Domain names are normalized to lowercase."
);

impl_parse_string!(
    Endpoint,
    "A domain name & a decimal port: `localhost:80`.",
    "Domain names are normalized to lowercase."
);

impl TryFrom<Vec<u8>> for Endpoint {
    type Error = InvalidAddressError<Vec<u8>>;

    /// A domain name & a decimal port: `localhost:80`.
    /// Domain names are normalized to lowercase.
    /// The error contains the unmodified `text`, which `TryFrom<String>` soundly recovers as a string.
    fn try_from(text: Vec<u8>) -> Result<Self, Self::Error> {
        match parse_port(text.as_slice()) {
            Ok((name, port)) => {
                let name_len: usize = name.len();
                Domain::parse_vec_prefix(text, name_len)
                    .map(|domain| domain.to_endpoint(port))
                    .map_err(|text| InvalidAddressError::new(text, InvalidDomain))
            }
            Err(error) => Err(InvalidAddressError::new(text, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidDomain, InvalidPort};
    use crate::{DomainRef, Endpoint, InvalidAddressError, ParseError};
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<Endpoint, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("localhost:", Err(InvalidPort)),
            ("localhost:xx", Err(InvalidPort)),
            (":80", Err(InvalidDomain)),
            ("[localhost]:80", Err(InvalidDomain)),
            ("localhost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("LocalHost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("Local_Host:80", Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Endpoint, ParseError> = Endpoint::from_str(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn try_from_str() {
        let result: Result<Endpoint, ParseError> = Endpoint::try_from("localhost:80");
        let expected: Result<Endpoint, ParseError> = Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80));
        assert_eq!(result, expected);

        let result: Result<Endpoint, ParseError> = Endpoint::try_from("LocalHost:80");
        let expected: Result<Endpoint, ParseError> = Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_text() {
        let result: Result<Endpoint, ParseError> = Endpoint::parse_text("LocalHost:80".as_bytes());
        let expected: Result<Endpoint, ParseError> = Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80));
        assert_eq!(result, expected);

        let result: Result<Endpoint, ParseError> = Endpoint::parse_text(b"\xFF:80".as_slice());
        let expected: Result<Endpoint, ParseError> = Err(InvalidDomain);
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_string() {
        let test_cases: &[(&str, Result<Endpoint, ParseError>)] = &[
            ("localhost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("LocalHost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("Local!Host:80", Err(InvalidDomain)),
            ("localhost:", Err(InvalidPort)),
            ("localhost:99999", Err(InvalidPort)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Endpoint, InvalidAddressError<String>> = Endpoint::try_from(input.to_string());
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
        let test_cases: &[(&str, Result<Endpoint, ParseError>)] = &[
            ("localhost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("LocalHost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("Local!Host:80", Err(InvalidDomain)),
            ("localhost:", Err(InvalidPort)),
            ("localhost:99999", Err(InvalidPort)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Endpoint, InvalidAddressError<Vec<u8>>> = Endpoint::try_from(Vec::from(*input));
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

    /// Mixed-case domain names are lowercased on every owned parse path.
    #[test]
    fn normalizes_case() {
        let test_cases: &[(&str, &str)] = &[
            ("LocalHost:80", "localhost:80"),
            ("WWW.Example.COM:443", "www.example.com:443"),
        ];

        for (input, expected) in test_cases {
            let endpoint: Endpoint = input.parse().unwrap();
            assert_eq!(endpoint.to_string(), *expected, "from_str input={}", input);

            let endpoint: Endpoint = Endpoint::parse_text(input.as_bytes()).unwrap();
            assert_eq!(endpoint.to_string(), *expected, "parse_text input={}", input);

            let endpoint: Endpoint = Endpoint::try_from(input.to_string()).unwrap();
            assert_eq!(endpoint.to_string(), *expected, "try_from(String) input={}", input);

            let endpoint: Endpoint = Endpoint::try_from(Vec::from(*input)).unwrap();
            assert_eq!(endpoint.to_string(), *expected, "try_from(Vec<u8>) input={}", input);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["localhost:80", "example.com:443", "a.b.c:65535", "x:0"];

        for input in canonical {
            let value: Endpoint = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
