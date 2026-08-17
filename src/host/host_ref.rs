use crate::{DomainRef, Host, IPAddress};

/// Either a [DomainRef] or an [IPAddress].
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HostRef<'a> {
    /// A [DomainRef].
    Name(DomainRef<'a>),

    /// An [IPAddress].
    Address(IPAddress),
}

impl<'a> PartialEq<Host> for HostRef<'a> {
    fn eq(&self, other: &Host) -> bool {
        *self == other.to_ref()
    }
}

impl<'a> HostRef<'a> {
    //! Matching

    /// Checks if the host is a domain.
    #[must_use]
    pub const fn is_domain(self) -> bool {
        matches!(self, Self::Name(_))
    }

    /// Checks if the host is an IP address.
    #[must_use]
    pub const fn is_ip(self) -> bool {
        matches!(self, Self::Address(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef, Host, HostRef, IPv4Address};

    #[test]
    fn equality() {
        let owned: Host = Domain::localhost().into();
        let host: HostRef = HostRef::Name(DomainRef::LOCALHOST);
        assert_eq!(host, owned);
        assert_ne!(IPv4Address::LOCALHOST.to_host_ref(), owned);
    }

    #[test]
    fn matching() {
        let host: HostRef = DomainRef::LOCALHOST.into();
        assert!(host.is_domain());
        assert!(!host.is_ip());

        let host: HostRef = IPv4Address::LOCALHOST.into();
        assert!(!host.is_domain());
        assert!(host.is_ip());
    }
}
