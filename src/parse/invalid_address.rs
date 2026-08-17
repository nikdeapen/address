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

impl InvalidAddress<Vec<u8>> {
    //! String Deconstruction

    /// Converts the byte vector error into a string error, debug-asserting the value kept its original `len`.
    ///
    /// # Safety
    /// The value must be valid UTF-8.
    pub(crate) unsafe fn into_string_unchecked(self, len: usize) -> InvalidAddress<String> {
        debug_assert_eq!(self.value.len(), len);
        debug_assert!(std::str::from_utf8(self.value.as_slice()).is_ok());

        InvalidAddress::new(unsafe { String::from_utf8_unchecked(self.value) }, self.error)
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
