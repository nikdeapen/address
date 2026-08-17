use crate::ParseError::InvalidDomain;
use crate::{
    Domain, InvalidAddress, NameClass, ParseError, doc_name_normalized, doc_recovers_value, impl_parse,
    impl_parse_string,
};

impl_parse!(Domain, doc_name_normalized!());
impl_parse_string!(Domain, doc_name_normalized!());

impl Domain {
    //! Owned Parsing

    /// Creates a domain from the first `len` bytes of `name`, normalizing the name to lowercase.
    ///
    /// Returns the unmodified `name` if the prefix is not a valid domain name.
    pub(crate) fn from_vec_prefix(name: Vec<u8>, len: usize) -> Result<Self, Vec<u8>> {
        match Self::classify_name(&name[..len]) {
            NameClass::Invalid => Err(name),
            class => {
                let mut name: Vec<u8> = name;
                name.truncate(len);
                if class == NameClass::MixedCase {
                    name.make_ascii_lowercase();
                }
                let name: String = unsafe { String::from_utf8_unchecked(name) };
                Ok(unsafe { Self::new_unchecked(name) })
            }
        }
    }
}

impl TryFrom<&[u8]> for Domain {
    type Error = ParseError;

    #[doc = doc_name_normalized!()]
    fn try_from(name: &[u8]) -> Result<Self, Self::Error> {
        match Self::classify_name(name) {
            NameClass::Invalid => Err(InvalidDomain),
            class => {
                let name: &str = unsafe { std::str::from_utf8_unchecked(name) };
                if class == NameClass::MixedCase {
                    Ok(unsafe { Self::new_unchecked(name.to_ascii_lowercase()) })
                } else {
                    Ok(unsafe { Self::new_unchecked(name) })
                }
            }
        }
    }
}

impl TryFrom<Vec<u8>> for Domain {
    type Error = InvalidAddress<Vec<u8>>;

    #[doc = doc_name_normalized!()]
    #[doc = doc_recovers_value!("name")]
    fn try_from(name: Vec<u8>) -> Result<Self, Self::Error> {
        let len: usize = name.len();
        Self::from_vec_prefix(name, len).map_err(|name| InvalidAddress::new(name, InvalidDomain))
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidDomain;
    use crate::{Domain, InvalidAddress, ParseError};
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
    fn try_from_slice() {
        let test_cases: &[(&[u8], Result<Domain, ParseError>)] = &[
            ("localhost".as_bytes(), Ok(Domain::localhost())),
            ("LocalHost".as_bytes(), Ok(Domain::localhost())),
            (b"\xFF".as_slice(), Err(InvalidDomain)),
            ("ü".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, ParseError> = Domain::try_from(*input);
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
            let result: Result<Domain, InvalidAddress<String>> = Domain::try_from(input.to_string());
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
            let result: Result<Domain, InvalidAddress<Vec<u8>>> = Domain::try_from(Vec::from(*input));
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

            let domain: Domain = Domain::try_from(input.as_bytes()).unwrap();
            assert_eq!(domain, *expected, "try_from(&[u8]) input={}", input);

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
