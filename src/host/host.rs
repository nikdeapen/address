use crate::{Domain, HostRef, IPAddress};

/// Either a [Domain] or an [IPAddress].
#[must_use]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Host {
    /// A [Domain].
    Name(Domain),

    /// An [IPAddress].
    Address(IPAddress),
}

impl<'a> PartialEq<HostRef<'a>> for Host {
    fn eq(&self, other: &HostRef<'a>) -> bool {
        self.to_ref() == *other
    }
}

impl Host {
    //! Matching

    /// Checks if the host is a domain.
    #[must_use]
    pub const fn is_domain(&self) -> bool {
        matches!(self, Self::Name(_))
    }

    /// Checks if the host is an IP address.
    #[must_use]
    pub const fn is_ip(&self) -> bool {
        matches!(self, Self::Address(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef, Host, HostRef, IPv4Address};

    #[test]
    fn equality() {
        let host: Host = Domain::localhost().into();
        assert_eq!(host, HostRef::Name(DomainRef::LOCALHOST));
        assert_ne!(host, IPv4Address::LOCALHOST.to_host_ref());
    }

    #[test]
    fn matching() {
        let host: Host = Domain::localhost().into();
        assert!(host.is_domain());
        assert!(!host.is_ip());

        let host: Host = IPv4Address::LOCALHOST.into();
        assert!(!host.is_domain());
        assert!(host.is_ip());
    }
}
