use crate::IPv4Address;
use std::net::Ipv4Addr;

impl IPv4Address {
    //! Standard Library Conversions

    /// Converts the address to a standard library address.
    #[must_use]
    pub const fn to_std(self) -> Ipv4Addr {
        let (a, b, c, d) = self.bytes();
        Ipv4Addr::new(a, b, c, d)
    }
}

impl From<Ipv4Addr> for IPv4Address {
    fn from(ip: Ipv4Addr) -> Self {
        Self::new(ip.octets())
    }
}

impl From<IPv4Address> for Ipv4Addr {
    fn from(ip: IPv4Address) -> Self {
        ip.to_std()
    }
}

#[cfg(test)]
mod tests {
    use crate::IPv4Address;
    use std::net::Ipv4Addr;

    #[test]
    fn to_std() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;
        let expected: Ipv4Addr = Ipv4Addr::LOCALHOST;

        let result: Ipv4Addr = ip.to_std();
        assert_eq!(result, expected);

        let result: Ipv4Addr = ip.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn from() {
        let result: IPv4Address = Ipv4Addr::LOCALHOST.into();
        let expected: IPv4Address = IPv4Address::LOCALHOST;
        assert_eq!(result, expected);
    }
}
