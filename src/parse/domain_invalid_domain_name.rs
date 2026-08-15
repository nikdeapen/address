use crate::ParseError;
use std::fmt::{Debug, Display, Formatter};

/// An error creating a domain from an invalid name.
///
/// The invalid name value can be recovered, like `std::string::FromUtf8Error`.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct InvalidDomainName<T> {
    value: T,
}

impl<T> InvalidDomainName<T> {
    //! Construction

    /// Creates a new invalid domain name error.
    pub(crate) const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> InvalidDomainName<T> {
    //! Properties

    /// Gets the invalid domain name value.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }
}

impl<T> InvalidDomainName<T> {
    //! Deconstruction

    /// Converts the error back into the invalid domain name value.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T> From<InvalidDomainName<T>> for ParseError {
    fn from(_: InvalidDomainName<T>) -> Self {
        Self::InvalidDomain
    }
}

impl<T> Display for InvalidDomainName<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad("invalid domain name")
    }
}

impl<T: Debug> std::error::Error for InvalidDomainName<T> {}
