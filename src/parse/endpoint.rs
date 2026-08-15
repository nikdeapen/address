use crate::ParseError::InvalidDomain;
use crate::parse_port;
use crate::{Domain, Endpoint, InvalidAddress, ParseError, impl_parse};

impl_parse!(Endpoint, "Domain names are normalized to lowercase.");

impl TryFrom<&[u8]> for Endpoint {
    type Error = ParseError;

    /// Domain names are normalized to lowercase.
    fn try_from(endpoint: &[u8]) -> Result<Self, Self::Error> {
        let (name, port): (&[u8], u16) = parse_port(endpoint)?;
        let domain: Domain = Domain::try_from(name)?;
        Ok(domain.to_endpoint(port))
    }
}

impl TryFrom<String> for Endpoint {
    type Error = InvalidAddress<String>;

    /// Domain names are normalized to lowercase.
    fn try_from(endpoint: String) -> Result<Self, Self::Error> {
        match parse_port(endpoint.as_bytes()) {
            Ok((name, port)) => {
                let name_len: usize = name.len();
                Domain::from_string_prefix(endpoint, name_len)
                    .map(|domain| domain.to_endpoint(port))
                    .map_err(|name| InvalidAddress::new(name, InvalidDomain))
            }
            Err(error) => Err(InvalidAddress::new(endpoint, error)),
        }
    }
}

impl TryFrom<Vec<u8>> for Endpoint {
    type Error = InvalidAddress<Vec<u8>>;

    /// Domain names are normalized to lowercase.
    fn try_from(endpoint: Vec<u8>) -> Result<Self, Self::Error> {
        match parse_port(endpoint.as_slice()) {
            Ok((name, port)) => {
                let name_len: usize = name.len();
                Domain::from_vec_prefix(endpoint, name_len)
                    .map(|domain| domain.to_endpoint(port))
                    .map_err(|name| InvalidAddress::new(name, InvalidDomain))
            }
            Err(error) => Err(InvalidAddress::new(endpoint, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidDomain, InvalidPort};
    use crate::{DomainRef, Endpoint, InvalidAddress, ParseError};
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
    fn try_from_slice() {
        let result: Result<Endpoint, ParseError> = Endpoint::try_from("LocalHost:80".as_bytes());
        let expected: Result<Endpoint, ParseError> = Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80));
        assert_eq!(result, expected);

        let result: Result<Endpoint, ParseError> = Endpoint::try_from(b"\xFF:80".as_slice());
        let expected: Result<Endpoint, ParseError> = Err(InvalidDomain);
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_string() {
        let test_cases: &[(&str, Result<Endpoint, &str>)] = &[
            ("localhost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("LocalHost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("Local!Host:80", Err("Local!Host:80")),
        ];

        for (input, expected) in test_cases {
            let result: Result<Endpoint, InvalidAddress<String>> = Endpoint::try_from(input.to_string());
            let result: Result<Endpoint, String> = result.map_err(|e| e.into_value());
            assert_eq!(result, expected.clone().map_err(String::from), "input={}", input);
        }
    }

    #[test]
    fn try_from_vec() {
        let test_cases: &[(&str, Result<Endpoint, &str>)] = &[
            ("localhost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("LocalHost:80", Ok(DomainRef::LOCALHOST.to_domain().to_endpoint(80))),
            ("Local!Host:80", Err("Local!Host:80")),
        ];

        for (input, expected) in test_cases {
            let result: Result<Endpoint, InvalidAddress<Vec<u8>>> = Endpoint::try_from(Vec::from(*input));
            let result: Result<Endpoint, Vec<u8>> = result.map_err(|e| e.into_value());
            assert_eq!(result, expected.clone().map_err(Vec::from), "input={}", input);
        }
    }
}
