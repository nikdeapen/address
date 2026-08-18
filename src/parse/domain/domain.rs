use crate::ParseError::InvalidDomain;
use crate::{Domain, InvalidAddressError, NameClass, ParseError, impl_parse, impl_parse_string};

impl Domain {
    //! Parse

    /// Dot-separated labels of ASCII letters, digits, & dashes. (see [`Domain::is_valid_name`])
    /// The name is normalized to lowercase.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
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
    "Dot-separated labels of ASCII letters, digits, & dashes. (see [`Domain::is_valid_name`])",
    "The name is normalized to lowercase."
);

impl_parse_string!(
    Domain,
    "Dot-separated labels of ASCII letters, digits, & dashes. (see [`Domain::is_valid_name`])",
    "The name is normalized to lowercase."
);

impl TryFrom<Vec<u8>> for Domain {
    type Error = InvalidAddressError<Vec<u8>>;

    /// Dot-separated labels of ASCII letters, digits, & dashes. (see [`Domain::is_valid_name`])
    /// The name is normalized to lowercase.
    /// The error contains the unmodified `text`, which `TryFrom<String>` soundly recovers as a string.
    fn try_from(text: Vec<u8>) -> Result<Self, Self::Error> {
        let len: usize = text.len();
        Self::parse_vec_prefix(text, len).map_err(|text| InvalidAddressError::new(text, InvalidDomain))
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidDomain;
    use crate::{Domain, InvalidAddressError, ParseError};
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<Domain, ParseError>)] = &[
            ("localhost", Ok(Domain::localhost())),
            ("LocalHost", Ok(Domain::localhost())),
            ("Local!Host", Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, ParseError> = Domain::from_str(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn try_from_str() {
        let test_cases: &[(&str, Result<Domain, ParseError>)] = &[
            ("localhost", Ok(Domain::localhost())),
            ("LocalHost", Ok(Domain::localhost())),
            ("Local!Host", Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, ParseError> = Domain::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn parse_text() {
        let test_cases: &[(&[u8], Result<Domain, ParseError>)] = &[
            ("localhost".as_bytes(), Ok(Domain::localhost())),
            ("LocalHost".as_bytes(), Ok(Domain::localhost())),
            (b"\xFF".as_slice(), Err(InvalidDomain)),
            ("ü".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, ParseError> = Domain::parse_text(input);
            assert_eq!(result, *expected, "input={:?}", input);
        }
    }

    #[test]
    fn try_from_string() {
        let test_cases: &[(&str, Result<Domain, ParseError>)] = &[
            ("localhost", Ok(Domain::localhost())),
            ("LocalHost", Ok(Domain::localhost())),
            ("Local!Host", Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, InvalidAddressError<String>> = Domain::try_from(input.to_string());
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
        let test_cases: &[(&str, Result<Domain, ParseError>)] = &[
            ("localhost", Ok(Domain::localhost())),
            ("LocalHost", Ok(Domain::localhost())),
            ("Local!Host", Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, InvalidAddressError<Vec<u8>>> = Domain::try_from(Vec::from(*input));
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

    /// Mixed-case input is lowercased on every owned parse path, dots and dashes intact.
    #[test]
    fn normalizes_case() {
        let test_cases: &[(&str, &str)] = &[
            ("LocalHost", "localhost"),
            ("WWW.Example.COM", "www.example.com"),
            ("A-B.C--D.EXAMPLE", "a-b.c--d.example"),
            ("123.EXAMPLE", "123.example"),
        ];

        for (input, expected) in test_cases {
            let domain: Domain = input.parse().unwrap();
            assert_eq!(domain, *expected, "from_str input={}", input);

            let domain: Domain = Domain::try_from(*input).unwrap();
            assert_eq!(domain, *expected, "try_from(&str) input={}", input);

            let domain: Domain = Domain::parse_text(input.as_bytes()).unwrap();
            assert_eq!(domain, *expected, "parse_text input={}", input);

            let domain: Domain = Domain::try_from(input.to_string()).unwrap();
            assert_eq!(domain, *expected, "try_from(String) input={}", input);

            let domain: Domain = Domain::try_from(Vec::from(*input)).unwrap();
            assert_eq!(domain, *expected, "try_from(Vec<u8>) input={}", input);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
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
