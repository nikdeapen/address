use crate::ParseError::InvalidDomain;
use crate::{Domain, DomainRef, ParseError, impl_parse_ref};

impl<'a> DomainRef<'a> {
    //! Parse

    /// Parses a [DomainRef] from the `text`.
    pub fn parse(text: &'a [u8]) -> Result<Self, ParseError> {
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
    "Dot-separated ASCII labels. (see [`Domain::is_valid_name`])",
    "The name must already be lowercase; use [`Domain`](crate::Domain) for mixed-case input."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidDomain;
    use crate::{DomainRef, ParseError};

    type TestCase<'a> = (&'a [u8], Result<DomainRef<'a>, ParseError>);

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidDomain)),
            (b"localhost", Ok(DomainRef::LOCALHOST)),
            (b"example.com", Ok(DomainRef::EXAMPLE)),
            (b"LocalHost", Err(InvalidDomain)),
            (b"Local!Host", Err(InvalidDomain)),
            (b"127.0.0.1", Err(InvalidDomain)),
            (b"\xFF", Err(InvalidDomain)),
            ("ü".as_bytes(), Err(InvalidDomain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<DomainRef, ParseError> = DomainRef::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<DomainRef, ParseError> = DomainRef::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
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
        ];

        for input in canonical {
            let value: DomainRef = DomainRef::try_from(*input).unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
