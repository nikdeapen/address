use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddress};

impl IPAddress {
    //! Conversions

    /// Converts the address to an optional IPv4 address.
    #[must_use]
    pub const fn to_v4(self) -> Option<IPv4Address> {
        if let Self::V4(ip) = self { Some(ip) } else { None }
    }

    /// Converts the address to an optional IPv6 address.
    #[must_use]
    pub const fn to_v6(self) -> Option<IPv6Address> {
        if let Self::V6(ip) = self { Some(ip) } else { None }
    }

    /// Converts the address to a socket address with the `port`.
    pub const fn to_socket(self, port: u16) -> SocketAddress {
        SocketAddress::new(self, port)
    }

    /// Converts the address to a host.
    pub const fn to_host(self) -> Host {
        Host::Address(self)
    }

    /// Converts the address to a host reference.
    pub const fn to_host_ref(self) -> HostRef<'static> {
        HostRef::Address(self)
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

#[cfg(test)]
mod tests {
    use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddress};

    #[test]
    fn ip_to_v4() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();
        let result: Option<IPv4Address> = ip.to_v4();
        let expected: Option<IPv4Address> = Some(IPv4Address::LOCALHOST);
        assert_eq!(result, expected);

        let ip: IPAddress = IPv6Address::LOCALHOST.to_ip();
        let result: Option<IPv4Address> = ip.to_v4();
        let expected: Option<IPv4Address> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn ip_to_v6() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();
        let result: Option<IPv6Address> = ip.to_v6();
        let expected: Option<IPv6Address> = None;
        assert_eq!(result, expected);

        let ip: IPAddress = IPv6Address::LOCALHOST.to_ip();
        let result: Option<IPv6Address> = ip.to_v6();
        let expected: Option<IPv6Address> = Some(IPv6Address::LOCALHOST);
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
        let expected: Host = Host::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);

        let result: HostRef = ip.to_host_ref();
        let expected: HostRef = HostRef::Address(IPAddress::V4(IPv4Address::LOCALHOST));
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
}
