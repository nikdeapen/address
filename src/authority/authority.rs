use crate::{Host, HostRef};

/// A host with an associated port.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Authority {
    host: Host,
    port: u16,
}

impl Authority {
    //! Construction

    /// Creates a new authority.
    pub const fn new(host: Host, port: u16) -> Self {
        Self { host, port }
    }
}

impl From<Authority> for (Host, u16) {
    fn from(authority: Authority) -> Self {
        (authority.host, authority.port)
    }
}

impl Authority {
    //! Properties

    /// Gets the host reference.
    pub fn host(&self) -> HostRef {
        self.host.to_ref()
    }

    /// Gets the port.
    pub const fn port(&self) -> u16 {
        self.port
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, Domain, DomainRef, Host, HostRef};

    #[test]
    fn new() {
        let authority: Authority = Authority::new(Domain::localhost().to_host(), 80);
        assert_eq!(authority.host, Domain::localhost().to_host());
        assert_eq!(authority.port, 80);
    }

    #[test]
    fn tuple_from_authority() {
        let authority: Authority = (Domain::localhost(), 80).into();
        let (host, port) = authority.into();
        assert_eq!(host, Host::Name(Domain::localhost()));
        assert_eq!(port, 80);
    }

    #[test]
    fn properties() {
        let authority: Authority = (Domain::localhost(), 80).into();
        assert_eq!(authority.host(), HostRef::Name(DomainRef::LOCALHOST));
        assert_eq!(authority.port(), 80);
    }
}
