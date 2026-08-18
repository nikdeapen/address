use crate::DomainRef;

/// A domain name.
#[must_use]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Domain {
    name: String,
}

impl Domain {
    //! Special Domains

    /// Creates the `localhost` domain.
    pub fn localhost() -> Self {
        DomainRef::LOCALHOST.to_domain()
    }

    /// Creates the `example.com` domain.
    pub fn example() -> Self {
        DomainRef::EXAMPLE.to_domain()
    }
}

impl Domain {
    //! Construction

    /// Creates a new [Domain].
    ///
    /// # Safety
    /// The `name` must be valid and lowercase.
    pub unsafe fn new_unchecked<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        let name: String = name.into();

        debug_assert!(Self::is_valid_name_str(name.as_str()));

        Self { name }
    }
}

impl From<Domain> for String {
    fn from(domain: Domain) -> Self {
        domain.name
    }
}

impl<'a> PartialEq<DomainRef<'a>> for Domain {
    fn eq(&self, other: &DomainRef<'a>) -> bool {
        self.to_ref() == *other
    }
}

impl PartialEq<&str> for Domain {
    /// Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal.
    fn eq(&self, other: &&str) -> bool {
        self.name == *other
    }
}

impl PartialEq<Domain> for &str {
    /// Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal.
    fn eq(&self, other: &Domain) -> bool {
        *self == other.name
    }
}

impl PartialEq<String> for Domain {
    /// Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal.
    fn eq(&self, other: &String) -> bool {
        self.name == *other
    }
}

impl PartialEq<Domain> for String {
    /// Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal.
    fn eq(&self, other: &Domain) -> bool {
        *self == other.name
    }
}

impl Domain {
    //! Properties

    /// Gets the name.
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef};

    #[test]
    fn specials() {
        assert_eq!(Domain::localhost().name, "localhost");
        assert_eq!(Domain::example().name, "example.com");
    }

    #[test]
    fn deconstruction() {
        let domain: Domain = Domain::localhost();
        let result: String = domain.into();
        let expected: &str = "localhost";
        assert_eq!(result, expected);
    }

    #[test]
    fn equality() {
        let domain: Domain = Domain::localhost();
        assert_eq!(domain, DomainRef::LOCALHOST);
        assert_ne!(domain, DomainRef::EXAMPLE);
        assert_eq!(domain, "localhost");
        assert_ne!(domain, "example.com");
        assert_eq!("localhost", domain);
        assert_ne!("example.com", domain);
        assert_eq!(domain, String::from("localhost"));
        assert_ne!(domain, String::from("example.com"));
        assert_eq!(String::from("localhost"), domain);
        assert_ne!(String::from("example.com"), domain);
    }

    #[test]
    fn properties() {
        let domain: Domain = Domain::localhost();
        assert_eq!(domain.name(), "localhost");
    }
}
