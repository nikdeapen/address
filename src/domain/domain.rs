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
    /// The `name` must be valid and lowercase. See [`Domain::is_valid_name`].
    pub unsafe fn new_unchecked<S: Into<String>>(name: S) -> Self {
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
    fn construction() {
        let domain: Domain = unsafe { Domain::new_unchecked("localhost") };
        assert_eq!(domain.name, "localhost");
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
    }

    #[test]
    fn properties() {
        let domain: Domain = Domain::localhost();
        assert_eq!(domain.name(), "localhost");
    }
}
