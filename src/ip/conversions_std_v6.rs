use crate::IPv6Address;
use std::net::Ipv6Addr;

impl IPv6Address {
    //! Standard Library Conversions

    /// Converts the address to a standard library address.
    #[must_use]
    pub const fn to_std(self) -> Ipv6Addr {
        Ipv6Addr::from_bits(u128::from_be_bytes(self.address()))
    }
}

impl From<Ipv6Addr> for IPv6Address {
    fn from(ip: Ipv6Addr) -> Self {
        Self::new(ip.octets())
    }
}

impl From<IPv6Address> for Ipv6Addr {
    fn from(ip: IPv6Address) -> Self {
        ip.to_std()
    }
}

#[cfg(test)]
mod tests {
    use crate::IPv6Address;
    use std::net::Ipv6Addr;

    #[test]
    fn to_std() {
        let ip: IPv6Address = IPv6Address::LOCALHOST;
        let expected: Ipv6Addr = Ipv6Addr::LOCALHOST;

        let result: Ipv6Addr = ip.to_std();
        assert_eq!(result, expected);

        let result: Ipv6Addr = ip.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn from() {
        let result: IPv6Address = Ipv6Addr::LOCALHOST.into();
        let expected: IPv6Address = IPv6Address::LOCALHOST;
        assert_eq!(result, expected);
    }
}
