use crate::ParseError;
use std::fmt::{Debug, Display, Formatter};

/// An error parsing an address that preserves the owned `value`.
///
/// # Examples
/// ```
/// use address::{Domain, InvalidAddressError, ParseError};
///
/// let text: String = String::from("bad_name");
/// let error: InvalidAddressError<String> = Domain::try_from(text).unwrap_err();
/// assert_eq!(error.error(), ParseError::InvalidDomain);
/// assert_eq!(error.into_value(), "bad_name");
/// ```
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct InvalidAddressError<T> {
    value: T,
    error: ParseError,
}

impl<T> InvalidAddressError<T> {
    //! Construction

    /// Creates a new [InvalidAddressError].
    pub(crate) const fn new(value: T, error: ParseError) -> Self {
        Self { value, error }
    }
}

impl<T> InvalidAddressError<T> {
    //! Properties

    /// Gets the invalid address value.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Gets the parse error.
    #[must_use]
    pub const fn error(&self) -> ParseError {
        self.error
    }
}

impl<T> InvalidAddressError<T> {
    //! Deconstruction

    /// Converts the error back into the value.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T> From<InvalidAddressError<T>> for ParseError {
    fn from(error: InvalidAddressError<T>) -> Self {
        error.error
    }
}

impl<T> Display for InvalidAddressError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl<T: Debug> std::error::Error for InvalidAddressError<T> {}

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidDomain;
    use crate::{InvalidAddressError, ParseError};

    fn invalid_domain() -> InvalidAddressError<String> {
        InvalidAddressError::new(String::from("bad_name"), InvalidDomain)
    }

    #[test]
    fn properties() {
        let error: InvalidAddressError<String> = invalid_domain();
        assert_eq!(error.value().as_str(), "bad_name");
        assert_eq!(error.error(), InvalidDomain);
    }

    #[test]
    fn deconstruction() {
        let value: String = invalid_domain().into_value();
        assert_eq!(value, "bad_name");

        let error: ParseError = invalid_domain().into();
        assert_eq!(error, InvalidDomain);
    }

    #[test]
    fn display() {
        let error: InvalidAddressError<String> = invalid_domain();
        assert_eq!(error.to_string(), "invalid domain");
        assert_eq!(format!("{:>16}", error), "  invalid domain");
    }

    #[test]
    fn std_error() {
        let error: Box<dyn std::error::Error> = Box::new(invalid_domain());
        assert_eq!(error.to_string(), "invalid domain");
        assert!(error.source().is_none());
    }
}
