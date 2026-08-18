use crate::ParseError::InvalidDomain;
use crate::{Domain, DomainRef, ParseError, impl_parse_ref};

impl<'a> DomainRef<'a> {
    //! Parse

    /// Dot-separated labels of ASCII letters, digits, & dashes. (see [`Domain::is_valid_name`])
    /// The name must already be in lowercase. Use [`Domain`](crate::Domain) to parse mixed-case input.
    pub fn parse_text(text: &'a [u8]) -> Result<Self, ParseError> {
        if Domain::is_valid_name(text) {
            let name: &str = unsafe { std::str::from_utf8_unchecked(text) };
            Ok(unsafe { Self::new_unchecked(name) })
        } else {
            Err(InvalidDomain)
        }
    }
}

impl_parse_ref!(
    DomainRef,
    "Dot-separated labels of ASCII letters, digits, & dashes. (see [`Domain::is_valid_name`])",
    "The name must already be in lowercase. Use [`Domain`](crate::Domain) to parse mixed-case input."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidDomain;
    use crate::{DomainRef, ParseError};

    #[test]
    fn try_from_str() {
        let result: Result<DomainRef, ParseError> = DomainRef::try_from("localhost");
        let expected: Result<DomainRef, ParseError> = Ok(DomainRef::LOCALHOST);
        assert_eq!(result, expected);

        let result: Result<DomainRef, ParseError> = DomainRef::try_from("LocalHost");
        let expected: Result<DomainRef, ParseError> = Err(InvalidDomain);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_text() {
        let test_cases: &[(&[u8], Result<DomainRef, ParseError>)] = &[
            ("localhost".as_bytes(), Ok(DomainRef::LOCALHOST)),
            ("LocalHost".as_bytes(), Err(InvalidDomain)),
            (b"\xFF".as_slice(), Err(InvalidDomain)),
            ("ü".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<DomainRef, ParseError> = DomainRef::parse_text(input);
            assert_eq!(result, *expected, "input={:?}", input);
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
        ];

        for input in canonical {
            let value: DomainRef = DomainRef::try_from(*input).unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
