use crate::{DomainRef, Host, IPAddress};

/// Either a domain reference or an IP address.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HostRef<'a> {
    /// A domain reference.
    Name(DomainRef<'a>),

    /// An IP address.
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
        let test_cases: &[(HostRef, bool, bool)] = &[
            (DomainRef::LOCALHOST.into(), true, false),
            (IPv4Address::LOCALHOST.into(), false, true),
        ];

        for (host, is_domain, is_ip) in test_cases {
            assert_eq!(host.is_domain(), *is_domain, "host={:?}", host);
            assert_eq!(host.is_ip(), *is_ip, "host={:?}", host);
        }
    }
}
