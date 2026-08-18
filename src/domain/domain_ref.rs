use crate::Domain;

/// A [Domain] reference.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DomainRef<'a> {
    name: &'a str,
}

impl<'a> DomainRef<'a> {
    //! Special Domains

    /// The `localhost` domain.
    pub const LOCALHOST: Self = Self { name: "localhost" };

    /// The `example.com` domain.
    pub const EXAMPLE: Self = Self { name: "example.com" };
}

impl<'a> DomainRef<'a> {
    //! Construction

    /// Creates a new [DomainRef].
    ///
    /// # Safety
    /// The `name` must be valid and lowercase.
    pub unsafe fn new_unchecked(name: &'a str) -> Self {
        debug_assert!(Domain::is_valid_name_str(name));

        Self { name }
    }
}

impl<'a> PartialEq<Domain> for DomainRef<'a> {
    fn eq(&self, other: &Domain) -> bool {
        *self == other.to_ref()
    }
}

impl<'a> PartialEq<&str> for DomainRef<'a> {
    /// Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal.
    fn eq(&self, other: &&str) -> bool {
        self.name == *other
    }
}

impl<'a> PartialEq<DomainRef<'a>> for &str {
    /// Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal.
    fn eq(&self, other: &DomainRef<'a>) -> bool {
        *self == other.name
    }
}

impl<'a> PartialEq<String> for DomainRef<'a> {
    /// Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal.
    fn eq(&self, other: &String) -> bool {
        self.name == other.as_str()
    }
}

impl<'a> PartialEq<DomainRef<'a>> for String {
    /// Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal.
    fn eq(&self, other: &DomainRef<'a>) -> bool {
        self.as_str() == other.name
    }
}

impl<'a> DomainRef<'a> {
    //! Properties

    /// Gets the name.
    #[must_use]
    pub const fn name(self) -> &'a str {
        self.name
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
        assert_eq!(DomainRef::LOCALHOST, String::from("localhost"));
        assert_ne!(DomainRef::LOCALHOST, String::from("example.com"));
        assert_eq!(String::from("localhost"), DomainRef::LOCALHOST);
        assert_ne!(String::from("example.com"), DomainRef::LOCALHOST);
    }

    #[test]
    fn properties() {
        let domain: DomainRef = DomainRef::LOCALHOST;
        assert_eq!(domain.name(), "localhost");
    }
}
