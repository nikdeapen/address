use crate::Domain;

/// A domain reference.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct DomainRef<'a> {
    name: &'a str,
}

impl<'a> DomainRef<'a> {
    //! Special Domains

    /// The localhost domain reference. (localhost)
    pub const LOCALHOST: Self = unsafe { Self::new_unchecked("localhost") };

    /// The example domain reference. (example.com)
    pub const EXAMPLE: Self = unsafe { Self::new_unchecked("example.com") };
}

impl<'a> DomainRef<'a> {
    //! Construction

    /// Creates a new domain reference. The name is not validated.
    pub const unsafe fn new_unchecked(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> TryFrom<&'a str> for DomainRef<'a> {
    type Error = ();

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        if Domain::is_valid_name_str(name, false) {
            Ok(Self { name })
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for DomainRef<'a> {
    type Error = ();

    fn try_from(name: &'a [u8]) -> Result<Self, Self::Error> {
        if Domain::is_valid_name(name, false) {
            let name: &str = unsafe { std::str::from_utf8_unchecked(name) };
            Ok(Self { name })
        } else {
            Err(())
        }
    }
}

impl<'a> DomainRef<'a> {
    //! Properties

    /// Gets the name.
    pub const fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::DomainRef;

    #[test]
    fn specials() {
        assert_eq!(DomainRef::LOCALHOST.name, "localhost");
        assert_eq!(DomainRef::EXAMPLE.name, "example.com");
    }

    #[test]
    fn construction() {
        let result: Result<DomainRef, ()> = DomainRef::try_from("localhost");
        assert_eq!(result, Ok(DomainRef::LOCALHOST));
        let result: Result<DomainRef, ()> = DomainRef::try_from("LocalHost");
        assert_eq!(result, Err(()));

        let result: Result<DomainRef, ()> = DomainRef::try_from("localhost".as_bytes());
        assert_eq!(result, Ok(DomainRef::LOCALHOST));
        let result: Result<DomainRef, ()> = DomainRef::try_from("LocalHost".as_bytes());
        assert_eq!(result, Err(()));
    }

    #[test]
    fn properties() {
        let domain: DomainRef = DomainRef::LOCALHOST;
        assert_eq!(domain.name(), "localhost");
    }
}
