use crate::ParseError::InvalidDomain;
use crate::{Domain, InvalidDomainName, ParseError, impl_parse};

impl_parse!(Domain, "The name is normalized to lowercase.");

impl TryFrom<&[u8]> for Domain {
    type Error = ParseError;

    /// The name is normalized to lowercase.
    fn try_from(name: &[u8]) -> Result<Self, Self::Error> {
        if Self::is_valid_name(name, false) {
            let name: &str = unsafe { std::str::from_utf8_unchecked(name) };
            Ok(unsafe { Self::new_unchecked(name) })
        } else if Self::is_valid_name(name, true) {
            let name: &str = unsafe { std::str::from_utf8_unchecked(name) };
            let name: String = name.to_ascii_lowercase();
            Ok(unsafe { Self::new_unchecked(name) })
        } else {
            Err(InvalidDomain)
        }
    }
}

impl TryFrom<String> for Domain {
    type Error = InvalidDomainName<String>;

    /// The name is normalized to lowercase.
    fn try_from(name: String) -> Result<Self, Self::Error> {
        if Self::is_valid_name_str(name.as_str(), false) {
            Ok(unsafe { Self::new_unchecked(name) })
        } else if Self::is_valid_name_str(name.as_str(), true) {
            let mut name: String = name;
            name.make_ascii_lowercase();
            Ok(unsafe { Self::new_unchecked(name) })
        } else {
            Err(InvalidDomainName::new(name))
        }
    }
}

impl TryFrom<Vec<u8>> for Domain {
    type Error = InvalidDomainName<Vec<u8>>;

    /// The name is normalized to lowercase.
    fn try_from(name: Vec<u8>) -> Result<Self, Self::Error> {
        if Self::is_valid_name(name.as_slice(), false) {
            let name: String = unsafe { String::from_utf8_unchecked(name) };
            Ok(unsafe { Self::new_unchecked(name) })
        } else if Self::is_valid_name(name.as_slice(), true) {
            let mut name: String = unsafe { String::from_utf8_unchecked(name) };
            name.make_ascii_lowercase();
            Ok(unsafe { Self::new_unchecked(name) })
        } else {
            Err(InvalidDomainName::new(name))
        }
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
