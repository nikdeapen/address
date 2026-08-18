use crate::ParseError;
use std::fmt::{Debug, Display, Formatter};

/// An error parsing an address that preserves the owned `value`.
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

    /// Converts the error back into the invalid address value.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }
}

impl InvalidAddressError<Vec<u8>> {
    //! String Deconstruction

    /// Converts the byte vector error into a string error, debug-asserting the value kept its original `len`.
    ///
    /// # Safety
    /// The value must be valid UTF-8.
    pub(crate) unsafe fn into_string_unchecked(self, len: usize) -> InvalidAddressError<String> {
        debug_assert_eq!(self.value.len(), len);
        debug_assert!(std::str::from_utf8(self.value.as_slice()).is_ok());

        InvalidAddressError::new(unsafe { String::from_utf8_unchecked(self.value) }, self.error)
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
