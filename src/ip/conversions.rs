use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddress};

impl IPAddress {
    //! Conversions

    /// Converts the address to an IPv4 address.
    pub const fn to_v4(self) -> Result<IPv4Address, Self> {
        if let Self::V4(ip) = self {
            Ok(ip)
        } else {
            Err(self)
        }
    }

    /// Converts the address to an IPv6 address.
    pub const fn to_v6(self) -> Result<IPv6Address, Self> {
        if let Self::V6(ip) = self {
            Ok(ip)
        } else {
            Err(self)
        }
    }

    /// Converts the address to a socket address with the `port`.
    pub const fn to_socket(self, port: u16) -> SocketAddress {
        SocketAddress::new(self, port)
    }

    /// Converts the address to a host.
    pub const fn to_host(self) -> Host {
        Host::IP(self)
    }

    /// Converts the address to a host reference.
    pub const fn to_host_ref(self) -> HostRef<'static> {
        HostRef::IP(self)
    }
}

impl From<IPv4Address> for IPAddress {
    fn from(v4: IPv4Address) -> Self {
        v4.to_ip()
    }
}

impl From<IPv6Address> for IPAddress {
    fn from(v6: IPv6Address) -> Self {
        v6.to_ip()
    }
}

impl TryFrom<Host> for IPAddress {
    type Error = Host;

    fn try_from(host: Host) -> Result<Self, Self::Error> {
        host.to_ip()
    }
}

impl<'a> TryFrom<HostRef<'a>> for IPAddress {
    type Error = HostRef<'a>;

    fn try_from(host: HostRef<'a>) -> Result<Self, Self::Error> {
        host.to_ip()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Domain, DomainRef, Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddress,
    };

    #[test]
    fn ip_to_v4() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();
        let result: Result<IPv4Address, IPAddress> = ip.to_v4();
        let expected: Result<IPv4Address, IPAddress> = Ok(IPv4Address::LOCALHOST);
        assert_eq!(result, expected);

        let ip: IPAddress = IPv6Address::LOCALHOST.to_ip();
        let result: Result<IPv4Address, IPAddress> = ip.to_v4();
        let expected: Result<IPv4Address, IPAddress> = Err(IPv6Address::LOCALHOST.to_ip());
        assert_eq!(result, expected);
    }

    #[test]
    fn ip_to_v6() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();
        let result: Result<IPv6Address, IPAddress> = ip.to_v6();
        let expected: Result<IPv6Address, IPAddress> = Err(IPv4Address::LOCALHOST.to_ip());
        assert_eq!(result, expected);

        let ip: IPAddress = IPv6Address::LOCALHOST.to_ip();
        let result: Result<IPv6Address, IPAddress> = ip.to_v6();
        let expected: Result<IPv6Address, IPAddress> = Ok(IPv6Address::LOCALHOST);
        assert_eq!(result, expected);
    }

    #[test]
    fn ip_to_socket() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();
        let result: SocketAddress = ip.to_socket(80);
        let expected: SocketAddress = SocketAddress::new(IPAddress::V4(IPv4Address::LOCALHOST), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ip_to_host() {
        let ip: IPAddress = IPAddress::V4(IPv4Address::LOCALHOST);

        let result: Host = ip.to_host();
        let expected: Host = Host::IP(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);

        let result: HostRef = ip.to_host_ref();
        let expected: HostRef = HostRef::IP(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }

    #[test]
    fn ip_from() {
        let result: IPAddress = IPv4Address::LOCALHOST.into();
        let expected: IPAddress = IPAddress::V4(IPv4Address::LOCALHOST);
        assert_eq!(result, expected);

        let result: IPAddress = IPv6Address::LOCALHOST.into();
        let expected: IPAddress = IPAddress::V6(IPv6Address::LOCALHOST);
        assert_eq!(result, expected);
    }

    #[test]
    fn ip_try_from() {
        let host: Host = IPv4Address::LOCALHOST.to_host();
        let expected: IPAddress = IPv4Address::LOCALHOST.to_ip();
        assert_eq!(IPAddress::try_from(host), Ok(expected));

        let host: Host = Domain::localhost().to_host();
        assert_eq!(IPAddress::try_from(host.clone()), Err(host));

        let host: HostRef = IPv4Address::LOCALHOST.to_host_ref();
        assert_eq!(IPAddress::try_from(host), Ok(expected));

        let host: HostRef = DomainRef::LOCALHOST.to_host_ref();
        assert_eq!(IPAddress::try_from(host), Err(host));
    }
}
