use crate::DomainRef;

/// The exactness note for the string comparison impls.
macro_rules! doc_exact_comparison {
    () => {
        "Compares the name exactly; domain names are lowercase, so mixed-case strings are never equal."
    };
}

pub(crate) use doc_exact_comparison;

/// A domain name.
///
/// Domain names are lowercase ASCII letters, digits, and dashes: dot-separated labels that must not start or end
/// with a dash (see [`Domain::is_valid_name`]). Mixed-case input is normalized to lowercase when parsed.
#[must_use]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Domain {
    pub(super) name: String,
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

        debug_assert!(Self::is_valid_name_str(name.as_str(), false));

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
    #[doc = doc_exact_comparison!()]
    fn eq(&self, other: &&str) -> bool {
        self.name == *other
    }
}

impl PartialEq<Domain> for &str {
    #[doc = doc_exact_comparison!()]
    fn eq(&self, other: &Domain) -> bool {
        *self == other.name
    }
}

impl Domain {
    //! Properties

    /// Gets the name.
    #[must_use]
    pub fn name(&self) -> &str {
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
    }

    #[test]
    fn properties() {
        let domain: Domain = Domain::localhost();
        assert_eq!(domain.name(), "localhost");
    }
}
