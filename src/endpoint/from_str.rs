use std::str::FromStr;

use crate::from_str::extract_port;
use crate::{Domain, Endpoint, ParseError};

impl FromStr for Endpoint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, port) = extract_port(s)?;
        let domain: Domain = Domain::from_str(s)?;
        Ok(Endpoint::new(domain, port))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::ParseError::{InvalidDomain, InvalidPort};
    use crate::{Domain, Endpoint, ParseError};

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<Endpoint, ParseError>)] = &[
            ("", Err(InvalidPort)),
            (":", Err(InvalidPort)),
            (":invalid", Err(InvalidPort)),
            (":65536", Err(InvalidPort)),
            (":-1", Err(InvalidPort)),
            (":80", Err(InvalidDomain)),
            ("!invalid!:80", Err(InvalidDomain)),
            ("LocalHost:80", Ok(Domain::localhost().to_endpoint(80))),
            ("localhost:80", Ok(Domain::localhost().to_endpoint(80))),
        ];
        for (input, expected) in test_cases {
            let result: Result<Endpoint, ParseError> = Endpoint::from_str(*input);
            assert_eq!(result, *expected, "input={}", *input);
        }
    }
}
