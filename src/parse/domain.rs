use crate::ParseError::InvalidDomain;
use crate::{Domain, InvalidDomainName, NameClass, ParseError, impl_parse};

impl_parse!(Domain, "The name is normalized to lowercase.");

impl Domain {
    //! Owned Parsing

    /// Creates a domain from the first `len` bytes of `name`, normalizing the name to lowercase.
    ///
    /// Returns the unmodified `name` if the prefix is not a valid domain name.
    pub(crate) fn from_string_prefix(name: String, len: usize) -> Result<Self, String> {
        match Self::classify_name(&name.as_bytes()[..len]) {
            NameClass::Invalid => Err(name),
            class => {
                let mut name: String = name;
                name.truncate(len);
                if class == NameClass::MixedCase {
                    name.make_ascii_lowercase();
                }
                Ok(unsafe { Self::new_unchecked(name) })
            }
        }
    }

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

    /// The name is normalized to lowercase.
    fn try_from(name: &[u8]) -> Result<Self, Self::Error> {
        match Self::classify_name(name) {
            NameClass::Lowercase => {
                let name: &str = unsafe { std::str::from_utf8_unchecked(name) };
                Ok(unsafe { Self::new_unchecked(name) })
            }
            NameClass::MixedCase => {
                let name: &str = unsafe { std::str::from_utf8_unchecked(name) };
                let name: String = name.to_ascii_lowercase();
                Ok(unsafe { Self::new_unchecked(name) })
            }
            NameClass::Invalid => Err(InvalidDomain),
        }
    }
}

impl TryFrom<String> for Domain {
    type Error = InvalidDomainName<String>;

    /// The name is normalized to lowercase.
    fn try_from(name: String) -> Result<Self, Self::Error> {
        let len: usize = name.len();
        Self::from_string_prefix(name, len).map_err(InvalidDomainName::new)
    }
}

impl TryFrom<Vec<u8>> for Domain {
    type Error = InvalidDomainName<Vec<u8>>;

    /// The name is normalized to lowercase.
    fn try_from(name: Vec<u8>) -> Result<Self, Self::Error> {
        let len: usize = name.len();
        Self::from_vec_prefix(name, len).map_err(InvalidDomainName::new)
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidDomain;
    use crate::{Domain, InvalidDomainName, ParseError};
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
        let test_cases: &[(&str, Result<Domain, &str>)] = &[
            ("localhost", Ok(Domain::localhost())),
            ("LocalHost", Ok(Domain::localhost())),
            ("Local!Host", Err("Local!Host")),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, InvalidDomainName<String>> = Domain::try_from(input.to_string());
            let result: Result<Domain, String> = result.map_err(|e| e.into_value());
            assert_eq!(result, expected.clone().map_err(String::from), "input={}", input);
        }
    }

    #[test]
    fn try_from_vec() {
        let test_cases: &[(&str, Result<Domain, &str>)] = &[
            ("localhost", Ok(Domain::localhost())),
            ("LocalHost", Ok(Domain::localhost())),
            ("Local!Host", Err("Local!Host")),
        ];

        for (input, expected) in test_cases {
            let result: Result<Domain, InvalidDomainName<Vec<u8>>> = Domain::try_from(Vec::from(*input));
            let result: Result<Domain, Vec<u8>> = result.map_err(|e| e.into_value());
            assert_eq!(result, expected.clone().map_err(Vec::from), "input={}", input);
        }
    }
}
