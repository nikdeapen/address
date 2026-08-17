use crate::{Authority, HostRef};

/// An [Authority] reference.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AuthorityRef<'a> {
    host: HostRef<'a>,
    port: u16,
}

impl<'a> AuthorityRef<'a> {
    //! Construction

    /// Creates a new [AuthorityRef].
    pub const fn new(host: HostRef<'a>, port: u16) -> Self {
        Self { host, port }
    }
}

impl<'a, H: Into<HostRef<'a>>> From<(H, u16)> for AuthorityRef<'a> {
    fn from(tuple: (H, u16)) -> Self {
        Self::new(tuple.0.into(), tuple.1)
    }
}

impl<'a> From<AuthorityRef<'a>> for (HostRef<'a>, u16) {
    fn from(authority: AuthorityRef<'a>) -> Self {
        (authority.host, authority.port)
    }
}

impl<'a> PartialEq<Authority> for AuthorityRef<'a> {
    fn eq(&self, other: &Authority) -> bool {
        *self == other.to_ref()
    }
}

impl<'a> AuthorityRef<'a> {
    //! Properties

    /// Gets the host reference.
    pub const fn host(self) -> HostRef<'a> {
        self.host
    }

    /// Gets the port.
    #[must_use]
    pub const fn port(self) -> u16 {
        self.port
    }
}

impl<'a> AuthorityRef<'a> {
    //! Matching

    /// Checks if the authority is an endpoint.
    #[must_use]
    pub const fn is_endpoint(self) -> bool {
        self.host.is_domain()
    }

    /// Checks if the authority is a socket address.
    #[must_use]
    pub const fn is_socket(self) -> bool {
        self.host.is_ip()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, AuthorityRef, Domain, DomainRef, Host, HostRef, IPv4Address};

    #[test]
    fn construction() {
        let authority: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        assert_eq!(authority.host, HostRef::Name(DomainRef::LOCALHOST));
        assert_eq!(authority.port, 80);

        let authority: AuthorityRef = (DomainRef::LOCALHOST, 80).into();
        assert_eq!(authority.host, HostRef::Name(DomainRef::LOCALHOST));
        assert_eq!(authority.port, 80);
    }

    #[test]
    fn deconstruction() {
        let authority: AuthorityRef = (DomainRef::LOCALHOST, 80).into();
        let (host, port): (HostRef, u16) = authority.into();
        assert_eq!(host, HostRef::Name(DomainRef::LOCALHOST));
        assert_eq!(port, 80);
    }

    #[test]
    fn equality() {
        let eighty: Authority = Authority::new(Host::Name(Domain::localhost()), 80);
        assert_eq!(eighty.to_ref(), eighty);

        let eighty_one: Authority = Authority::new(Host::Name(Domain::localhost()), 81);
        assert_ne!(eighty.to_ref(), eighty_one);
    }

    #[test]
    fn properties() {
        let authority: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        assert_eq!(authority.host(), HostRef::Name(DomainRef::LOCALHOST));
        assert_eq!(authority.port(), 80);
    }

    #[test]
    fn matching() {
        let authority: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        assert!(authority.is_endpoint());
        assert!(!authority.is_socket());

        let authority: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);
        assert!(!authority.is_endpoint());
        assert!(authority.is_socket());
    }
}
