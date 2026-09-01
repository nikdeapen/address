use crate::ParseError::InvalidDomain;
use crate::{Domain, InvalidAddressError, NameClass, ParseError, impl_parse, impl_parse_string};

impl Domain {
    //! Parse

    /// Parses a [Domain] from the `text`.
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        match Self::classify_name(text) {
            NameClass::Invalid => Err(InvalidDomain),
            class => {
                let name: &str = unsafe { std::str::from_utf8_unchecked(text) };
                if class == NameClass::MixedCase {
                    Ok(unsafe { Self::new_unchecked(name.to_ascii_lowercase()) })
                } else {
                    Ok(unsafe { Self::new_unchecked(name) })
                }
            }
        }
    }

    /// Creates a domain from the `text`, normalizing the name to lowercase.
    ///
    /// The error holds the unmodified `text`, which `TryFrom<String>` soundly recovers as a string.
    pub(crate) fn parse_vec(text: Vec<u8>) -> Result<Self, InvalidAddressError<Vec<u8>>> {
        let len: usize = text.len();
        Self::parse_vec_prefix(text, len)
            .map_err(|text| InvalidAddressError::new(text, InvalidDomain))
    }

    /// Creates a domain from the first `len` bytes of `text`, normalizing the name to lowercase.
    ///
    /// Returns the unmodified `text` if the prefix is not a valid domain name.
    pub(crate) fn parse_vec_prefix(text: Vec<u8>, len: usize) -> Result<Self, Vec<u8>> {
        match Self::classify_name(&text[..len]) {
            NameClass::Invalid => Err(text),
            class => {
                let mut text: Vec<u8> = text;
                text.truncate(len);
                if class == NameClass::MixedCase {
                    text.make_ascii_lowercase();
                }
                let name: String = unsafe { String::from_utf8_unchecked(text) };
                Ok(unsafe { Self::new_unchecked(name) })
            }
        }
    }
}

impl_parse!(
    Domain,
    "Dot-separated ASCII labels. (see [`Domain::is_valid_name`])",
    "The name is normalized to lowercase."
);

impl_parse_string!(
    Domain,
    "Dot-separated ASCII labels. (see [`Domain::is_valid_name`])",
    "The name is normalized to lowercase."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidDomain;
    use crate::{Domain, InvalidAddressError, ParseError};
    use std::str::FromStr;

    type TestCase<'a> = (&'a [u8], Result<Domain, ParseError>);

    fn domain(name: &str) -> Domain {
        Domain::try_from(name).unwrap()
    }

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidDomain)),
            (b"localhost", Ok(Domain::localhost())),
            (b"LocalHost", Ok(Domain::localhost())),
            (b"WWW.Example.COM", Ok(domain("www.example.com"))),
            (b"A-B.C--D.EXAMPLE", Ok(domain("a-b.c--d.example"))),
            (b"123.EXAMPLE", Ok(domain("123.example"))),
            (b"Local!Host", Err(InvalidDomain)),
            (b"Local_Host", Err(InvalidDomain)),
            (b"127.0.0.1", Err(InvalidDomain)),
            (b"\xFF", Err(InvalidDomain)),
            ("ü".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, ParseError> = Domain::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<Domain, ParseError> = Domain::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<Domain, ParseError> = Domain::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);

            let result: Result<Domain, InvalidAddressError<String>> =
                Domain::try_from(text.to_string());
            let result: Result<Domain, ParseError> = result.map_err(|error| {
                assert_eq!(error.value().as_str(), text, "recovered input={}", text);
                error.error()
            });
            assert_eq!(result, *expected, "try_from(String) input={}", text);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &[
            "localhost",
            "example.com",
            "a-b.c--d.example",
            "xn--bcher-kva.example",
            "123.example",
            "a.b.c",
            "x",
        ];

        for input in canonical {
            let value: Domain = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
