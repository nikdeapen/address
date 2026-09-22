use crate::ParseError::InvalidDomain;
use crate::parse_port;
use crate::{Domain, Endpoint, InvalidAddressError, ParseError, impl_parse, impl_parse_string};

impl Endpoint {
    //! Parse

    /// Parses an [Endpoint] from the `text`.
    ///
    /// # Notes
    /// - The domain name is normalized to lowercase.
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        let (name, port): (&[u8], u16) = parse_port(text)?;
        let domain: Domain = Domain::parse(name)?;
        Ok(domain.to_endpoint(port))
    }

    /// Parses an [Endpoint] from the `text`.
    pub(crate) fn parse_string(text: String) -> Result<Self, InvalidAddressError<String>> {
        match parse_port(text.as_bytes()) {
            Ok((name, port)) => {
                let name_len: usize = name.len();
                Domain::parse_string_prefix(text, name_len)
                    .map(|domain| domain.to_endpoint(port))
                    .map_err(|text| InvalidAddressError::new(text, InvalidDomain))
            }
            Err(error) => Err(InvalidAddressError::new(text, error)),
        }
    }
}

impl_parse!(Endpoint);

impl_parse_string!(Endpoint);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidDomain, InvalidPort};
    use crate::{Domain, Endpoint, InvalidAddressError, ParseError};
    use std::str::FromStr;

    type TestCase<'a> = (&'a [u8], Result<Endpoint, ParseError>);

    fn endpoint(name: &str, port: u16) -> Endpoint {
        Domain::try_from(name).unwrap().to_endpoint(port)
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
            (b"LocalHost:80", Ok(endpoint("localhost", 80))),
            (b"WWW.Example.COM:443", Ok(endpoint("www.example.com", 443))),
            (b":80", Err(InvalidDomain)),
            (b"[localhost]:80", Err(InvalidDomain)),
            (b"Local_Host:80", Err(InvalidDomain)),
            (b"Local!Host:80", Err(InvalidDomain)),
            (b"127.0.0.1:80", Err(InvalidDomain)),
            (b"\xFF:80", Err(InvalidDomain)),
            ("ü:80".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Endpoint, ParseError> = Endpoint::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<Endpoint, ParseError> = Endpoint::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<Endpoint, ParseError> = Endpoint::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);

            let result: Result<Endpoint, InvalidAddressError<String>> =
                Endpoint::try_from(text.to_string());
            let result: Result<Endpoint, ParseError> = result.map_err(|error| {
                assert_eq!(error.value().as_str(), text, "recovered input={}", text);
                error.error()
            });
            assert_eq!(result, *expected, "try_from(String) input={}", text);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["localhost:80", "example.com:443", "a.b.c:65535", "x:0"];

        for input in canonical {
            let value: Endpoint = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
