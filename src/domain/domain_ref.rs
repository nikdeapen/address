use crate::{Domain, Labels};

/// A domain name reference.
///
/// See [`Domain`] for the domain name validation rules.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DomainRef<'a> {
    name: &'a str,
}

impl<'a> DomainRef<'a> {
    //! Special Domains

    /// The `localhost` domain reference.
    pub const LOCALHOST: Self = Self { name: "localhost" };

    /// The `example.com` domain reference.
    pub const EXAMPLE: Self = Self { name: "example.com" };
}

impl<'a> DomainRef<'a> {
    //! Construction

    /// Creates a new domain reference.
    ///
    /// # Safety
    /// The `name` must be valid and lowercase. Validity is a struct invariant that unsafe code may rely on.
    pub unsafe fn new_unchecked(name: &'a str) -> Self {
        debug_assert!(Domain::is_valid_name_str(name, false));

        Self { name }
    }
}

impl<'a> PartialEq<Domain> for DomainRef<'a> {
    fn eq(&self, other: &Domain) -> bool {
        *self == other.to_ref()
    }
}

impl<'a> PartialEq<&str> for DomainRef<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.name == *other
    }
}

impl<'a> PartialEq<DomainRef<'a>> for &str {
    fn eq(&self, other: &DomainRef<'a>) -> bool {
        *self == other.name
    }
}

impl<'a> DomainRef<'a> {
    //! Properties

    /// Gets the name.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Gets the labels.
    pub fn labels(self) -> Labels<'a> {
        Labels::new(self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef};

    #[test]
    fn specials() {
        assert_eq!(DomainRef::LOCALHOST.name, "localhost");
        assert_eq!(DomainRef::EXAMPLE.name, "example.com");
    }

    #[test]
    fn equality() {
        let owned: Domain = Domain::localhost();
        assert_eq!(DomainRef::LOCALHOST, owned);
        assert_ne!(DomainRef::EXAMPLE, owned);
        assert_eq!(DomainRef::LOCALHOST, "localhost");
        assert_ne!(DomainRef::LOCALHOST, "example.com");
        assert_eq!("localhost", DomainRef::LOCALHOST);
        assert_ne!("example.com", DomainRef::LOCALHOST);
    }

    #[test]
    fn properties() {
        let domain: DomainRef = DomainRef::LOCALHOST;
        assert_eq!(domain.name(), "localhost");
    }
}
