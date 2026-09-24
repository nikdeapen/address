use crate::{AuthorityRef, Host, HostRef};

/// A [Host] with an associated port.
#[must_use]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Authority {
    host: Host,
    port: u16,
}

impl Authority {
    //! Construction

    /// Creates a new [Authority].
    pub const fn new(host: Host, port: u16) -> Self {
        Self { host, port }
    }
}

impl<H: Into<Host>> From<(H, u16)> for Authority {
    fn from(tuple: (H, u16)) -> Self {
        Self::new(tuple.0.into(), tuple.1)
    }
}

impl From<Authority> for (Host, u16) {
    fn from(authority: Authority) -> Self {
        (authority.host, authority.port)
    }
}

impl<'a> PartialEq<AuthorityRef<'a>> for Authority {
    fn eq(&self, other: &AuthorityRef<'a>) -> bool {
        self.to_ref() == *other
    }
}

impl Authority {
    //! Properties

    /// Gets the host reference.
    pub fn host(&self) -> HostRef<'_> {
        self.host.to_ref()
    }

    /// Gets the port.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }
}

impl Authority {
    //! Matching

    /// Checks if the authority is an endpoint.
    #[must_use]
    pub const fn is_endpoint(&self) -> bool {
        self.host.is_domain()
    }

    /// Checks if the authority is a socket address.
    #[must_use]
    pub const fn is_socket(&self) -> bool {
        self.host.is_ip()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, AuthorityRef, Domain, DomainRef, Host, HostRef, IPv4Address};

    #[test]
    fn construction() {
        let authority: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        assert_eq!(authority.host, Host::Domain(Domain::localhost()));
        assert_eq!(authority.port, 80);

        let authority: Authority = (Domain::localhost(), 80).into();
        assert_eq!(authority.host, Host::Domain(Domain::localhost()));
        assert_eq!(authority.port, 80);

        let authority: Authority = (DomainRef::LOCALHOST, 80).into();
        assert_eq!(authority.host, Host::Domain(Domain::localhost()));
        assert_eq!(authority.port, 80);
    }

    #[test]
    fn deconstruction() {
        let authority: Authority = (Domain::localhost(), 80).into();
        let (host, port): (Host, u16) = authority.into();
        assert_eq!(host, Host::Domain(Domain::localhost()));
        assert_eq!(port, 80);
    }

    #[test]
    fn equality() {
        let authority: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        assert_eq!(
            authority,
            AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80)
        );
        assert_ne!(
            authority,
            AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 81)
        );
    }

    #[test]
    fn properties() {
        let authority: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        assert_eq!(authority.host(), HostRef::Domain(DomainRef::LOCALHOST));
        assert_eq!(authority.port(), 80);
    }

    #[test]
    fn matching() {
        let authority: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        assert!(authority.is_endpoint());
        assert!(!authority.is_socket());

        let authority: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);
        assert!(!authority.is_endpoint());
        assert!(authority.is_socket());
    }
}
