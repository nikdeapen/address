use crate::HostRef;

/// A host reference with an associated port.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct AuthorityRef<'a> {
    host: HostRef<'a>,
    port: u16,
}

impl<'a> AuthorityRef<'a> {
    //! Construction

    /// Creates a new authority reference.
    pub const fn new(host: HostRef<'a>, port: u16) -> Self {
        Self { host, port }
    }
}

impl<'a> AuthorityRef<'a> {
    //! Properties

    /// Gets the host reference.
    pub fn host(&self) -> HostRef<'a> {
        self.host
    }

    /// Gets the port.
    pub const fn port(&self) -> u16 {
        self.port
    }
}

#[cfg(test)]
mod tests {
    use crate::{AuthorityRef, DomainRef, HostRef};

    #[test]
    fn new() {
        let authority: AuthorityRef = AuthorityRef::new(DomainRef::LOCALHOST.to_host(), 80);
        assert_eq!(authority.host, DomainRef::LOCALHOST.to_host());
        assert_eq!(authority.port, 80);
    }

    #[test]
    fn properties() {
        let authority: AuthorityRef = (DomainRef::LOCALHOST, 80).into();
        assert_eq!(authority.host(), HostRef::Name(DomainRef::LOCALHOST));
        assert_eq!(authority.port(), 80);
    }
}
