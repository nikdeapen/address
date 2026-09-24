use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddressV4};

impl IPv4Address {
    //! Conversions

    /// Converts the address to an IPv4-compatible IPv6 address. (::a.b.c.d)
    ///
    /// The compatible format is deprecated, prefer [`Self::to_v6_mapped`].
    /// See [RFC 4291](https://www.rfc-editor.org/rfc/rfc4291#section-2.5.5.1).
    pub const fn to_v6_compatible(self) -> IPv6Address {
        let (a, b, c, d) = self.bytes();
        IPv6Address::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, a, b, c, d])
    }

    /// Converts the address to an IPv4-mapped IPv6 address. (::ffff:a.b.c.d)
    pub const fn to_v6_mapped(self) -> IPv6Address {
        let (a, b, c, d) = self.bytes();
        IPv6Address::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, a, b, c, d])
    }

    /// Converts the address to an IP address.
    pub const fn to_ip(self) -> IPAddress {
        IPAddress::V4(self)
    }

    /// Converts the address to a socket address with the `port`.
    pub const fn to_socket(self, port: u16) -> SocketAddressV4 {
        SocketAddressV4::new(self, port)
    }

    /// Converts the address to a host.
    pub const fn to_host(self) -> Host {
        Host::IPAddress(self.to_ip())
    }

    /// Converts the address to a host reference.
    pub const fn to_host_ref(self) -> HostRef<'static> {
        HostRef::IPAddress(self.to_ip())
    }
}

impl TryFrom<IPAddress> for IPv4Address {
    type Error = IPAddress;

    fn try_from(ip: IPAddress) -> Result<Self, Self::Error> {
        ip.to_v4()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddressV4};

    #[test]
    fn to_v6() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;

        let result: IPv6Address = ip.to_v6_compatible();
        let expected: IPv6Address = IPv6Address::from([0, 0, 0, 0, 0, 0, 0x7F00, 1]);
        assert_eq!(result, expected);

        let result: IPv6Address = ip.to_v6_mapped();
        let expected: IPv6Address = IPv6Address::from([0, 0, 0, 0, 0, 0xFFFF, 0x7F00, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn to_ip() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;
        let result: IPAddress = ip.to_ip();
        let expected: IPAddress = IPAddress::V4(IPv4Address::LOCALHOST);
        assert_eq!(result, expected);
    }

    #[test]
    fn to_socket() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;
        let result: SocketAddressV4 = ip.to_socket(80);
        let expected: SocketAddressV4 = SocketAddressV4::new(IPv4Address::LOCALHOST, 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn to_host() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;

        let result: Host = ip.to_host();
        let expected: Host = Host::IPAddress(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);

        let result: HostRef = ip.to_host_ref();
        let expected: HostRef = HostRef::IPAddress(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();
        assert_eq!(IPv4Address::try_from(ip), Ok(IPv4Address::LOCALHOST));

        let ip: IPAddress = IPv6Address::LOCALHOST.to_ip();
        assert_eq!(IPv4Address::try_from(ip), Err(ip));
    }
}
