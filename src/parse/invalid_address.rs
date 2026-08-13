use crate::ParseError;
use std::fmt::{Debug, Display, Formatter};

/// An error parsing an address from an owned value.
///
/// The invalid value can be recovered, like `std::string::FromUtf8Error`.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct InvalidAddress<T> {
    value: T,
    error: ParseError,
}

impl<T> InvalidAddress<T> {
    //! Construction

    /// Creates a new invalid address error.
    pub(crate) const fn new(value: T, error: ParseError) -> Self {
        Self { value, error }
    }
}

impl<T> InvalidAddress<T> {
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

impl<T> InvalidAddress<T> {
    //! Deconstruction

    /// Converts the error back into the invalid address value.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T> From<InvalidAddress<T>> for ParseError {
    fn from(error: InvalidAddress<T>) -> Self {
        error.error
    }
}

impl<T> Display for InvalidAddress<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl<T: Debug> std::error::Error for InvalidAddress<T> {}
