use crate::{Host, HostRef};

impl Host {
    //! Conversions

    /// Converts the host to a host reference.
    pub fn to_ref(&self) -> HostRef<'_> {
        match self {
            Self::Name(domain) => HostRef::Name(domain.to_ref()),
            Self::Address(ip) => HostRef::Address(*ip),
        }
    }
}

impl<'a> HostRef<'a> {
    //! Conversions

    /// Converts the host reference to a host.
    pub fn to_host(&self) -> Host {
        match self {
            Self::Name(domain) => Host::Name(domain.to_domain()),
            Self::Address(ip) => Host::Address(*ip),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef, Host, HostRef, IPAddress, IPv4Address};

    #[test]
    fn host_to_ref() {
        let host: Host = Host::Name(Domain::localhost());
        let result: HostRef = host.to_ref();
        let expected: HostRef = HostRef::Name(DomainRef::LOCALHOST);
        assert_eq!(result, expected);

        let host: Host = Host::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: HostRef = host.to_ref();
        let expected: HostRef = HostRef::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_host() {
        let host: HostRef = HostRef::Name(DomainRef::LOCALHOST);
        let result: Host = host.to_host();
        let expected: Host = Host::Name(Domain::localhost());
        assert_eq!(result, expected);

        let host: HostRef = HostRef::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: Host = host.to_host();
        let expected: Host = Host::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }
}
