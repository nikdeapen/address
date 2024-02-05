use crate::DomainRef;

/// A domain.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Domain {
    name: String,
}

impl Domain {
    //! Special Domains

    /// Creates the localhost domain. (localhost)
    pub fn localhost() -> Self {
        DomainRef::LOCALHOST.to_domain()
    }

    /// Creates the example domain. (example.com)
    pub fn example() -> Self {
        DomainRef::EXAMPLE.to_domain()
    }
}

impl Domain {
    //! Construction

    /// Creates a new domain. The name is not validated.
    pub unsafe fn new_unchecked<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl TryFrom<String> for Domain {
    type Error = ();

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if Self::is_valid_name_str(name.as_str(), false) {
            Ok(Self { name })
        } else if Self::is_valid_name_str(name.as_str(), true) {
            let name: String = name.to_ascii_lowercase();
            Ok(Self { name })
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for Domain {
    type Error = ();

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if Self::is_valid_name_str(name, false) {
            Ok(Self {
                name: name.to_string(),
            })
        } else if Self::is_valid_name_str(name, true) {
            let name: String = name.to_ascii_lowercase();
            Ok(Self { name })
        } else {
            Err(())
        }
    }
}

impl TryFrom<Vec<u8>> for Domain {
    type Error = ();

    fn try_from(name: Vec<u8>) -> Result<Self, Self::Error> {
        if Self::is_valid_name(name.as_slice(), false) {
            let name: String = unsafe { String::from_utf8_unchecked(name) };
            Ok(Self { name })
        } else if Self::is_valid_name(name.as_slice(), true) {
            let name: String = unsafe { String::from_utf8_unchecked(name) };
            let name: String = name.to_ascii_lowercase();
            Ok(Self { name })
        } else {
            Err(())
        }
    }
}

impl TryFrom<&[u8]> for Domain {
    type Error = ();

    fn try_from(name: &[u8]) -> Result<Self, Self::Error> {
        if Self::is_valid_name(name, false) {
            let name: &str = unsafe { std::str::from_utf8_unchecked(name) };
            let name: String = name.to_string();
            Ok(Self { name })
        } else if Self::is_valid_name(name, true) {
            let name: &str = unsafe { std::str::from_utf8_unchecked(name) };
            let name: String = name.to_ascii_lowercase();
            Ok(Self { name })
        } else {
            Err(())
        }
    }
}

impl From<Domain> for String {
    fn from(domain: Domain) -> Self {
        domain.name
    }
}

impl Domain {
    //! Properties

    /// Gets the name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::Domain;

    #[test]
    fn specials() {
        assert_eq!(Domain::localhost().name, "localhost");
        assert_eq!(Domain::example().name, "example.com");
    }

    #[test]
    fn construction() {
        let result: Result<Domain, ()> = Domain::try_from("localhost".to_string());
        assert_eq!(result, Ok(Domain::localhost()));
        let result: Result<Domain, ()> = Domain::try_from("LocalHost".to_string());
        assert_eq!(result, Ok(Domain::localhost()));
        let result: Result<Domain, ()> = Domain::try_from("Local!Host".to_string());
        assert_eq!(result, Err(()));

        let result: Result<Domain, ()> = Domain::try_from("localhost");
        assert_eq!(result, Ok(Domain::localhost()));
        let result: Result<Domain, ()> = Domain::try_from("LocalHost");
        assert_eq!(result, Ok(Domain::localhost()));
        let result: Result<Domain, ()> = Domain::try_from("Local!Host");
        assert_eq!(result, Err(()));

        let result: Result<Domain, ()> = Domain::try_from(Vec::from("localhost"));
        assert_eq!(result, Ok(Domain::localhost()));
        let result: Result<Domain, ()> = Domain::try_from(Vec::from("LocalHost"));
        assert_eq!(result, Ok(Domain::localhost()));
        let result: Result<Domain, ()> = Domain::try_from(Vec::from("Local!Host"));
        assert_eq!(result, Err(()));

        let result: Result<Domain, ()> = Domain::try_from("localhost".as_bytes());
        assert_eq!(result, Ok(Domain::localhost()));
        let result: Result<Domain, ()> = Domain::try_from("LocalHost".as_bytes());
        assert_eq!(result, Ok(Domain::localhost()));
        let result: Result<Domain, ()> = Domain::try_from("Local!Host".as_bytes());
        assert_eq!(result, Err(()));
    }

    #[test]
    fn deconstruction() {
        let domain: Domain = Domain::localhost();
        let result: String = domain.into();
        let expected: &str = "localhost";
        assert_eq!(result, expected);
    }

    #[test]
    fn properties() {
        let domain: Domain = Domain::localhost();
        assert_eq!(domain.name(), "localhost");
    }
}
