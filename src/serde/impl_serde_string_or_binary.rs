use crate::{IPAddress, IPv4Address, IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6};

/// Implements `Serialize` and `Deserialize` for an owned type that serializes as its `Display` string in
/// human-readable formats and as the binary type in other formats.
macro_rules! impl_serde_string_or_binary {
    ($ty:ident, $expecting:literal, $bin:ty, $to_bin:expr, $from_bin:expr) => {
        impl ::serde::Serialize for crate::$ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                if serializer.is_human_readable() {
                    serializer.collect_str(self)
                } else {
                    let to_bin = $to_bin;
                    let binary: $bin = to_bin(*self);
                    ::serde::Serialize::serialize(&binary, serializer)
                }
            }
        }

        impl<'de> ::serde::Deserialize<'de> for crate::$ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                if deserializer.is_human_readable() {
                    deserializer.deserialize_str(crate::serde::FromStrVisitor::new($expecting))
                } else {
                    <$bin as ::serde::Deserialize>::deserialize(deserializer).map($from_bin)
                }
            }
        }
    };
}

impl_serde_string_or_binary!(
    IPv4Address,
    "an IPv4 address string",
    [u8; 4],
    IPv4Address::address,
    IPv4Address::new
);

impl_serde_string_or_binary!(
    IPv6Address,
    "an IPv6 address string",
    [u8; 16],
    IPv6Address::address,
    IPv6Address::new
);

impl_serde_string_or_binary!(
    SocketAddress,
    "a socket address string",
    (IPAddress, u16),
    |socket: SocketAddress| (socket.ip(), socket.port()),
    |(ip, port)| SocketAddress::new(ip, port)
);

impl_serde_string_or_binary!(
    SocketAddressV4,
    "an IPv4 socket address string",
    (IPv4Address, u16),
    |socket: SocketAddressV4| (socket.ip(), socket.port()),
    |(ip, port)| SocketAddressV4::new(ip, port)
);

impl_serde_string_or_binary!(
    SocketAddressV6,
    "an IPv6 socket address string",
    (IPv6Address, u16),
    |socket: SocketAddressV6| (socket.ip(), socket.port()),
    |(ip, port)| SocketAddressV6::new(ip, port)
);

#[cfg(test)]
mod tests {
    use crate::serde::test_util::{assert_json, assert_postcard};
    use crate::{IPv4Address, IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6};
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

    #[test]
    fn json() {
        assert_json(IPv4Address::UNSPECIFIED, "\"0.0.0.0\"");
        assert_json(IPv4Address::LOCALHOST, "\"127.0.0.1\"");
        assert_json(IPv6Address::LOCALHOST, "\"::1\"");
        assert_json(IPv4Address::LOCALHOST.to_socket(80), "\"127.0.0.1:80\"");
        assert_json(IPv6Address::LOCALHOST.to_socket(80), "\"[::1]:80\"");
        assert_json(IPv4Address::LOCALHOST.to_ip().to_socket(80), "\"127.0.0.1:80\"");
        assert_json(IPv6Address::LOCALHOST.to_ip().to_socket(443), "\"[::1]:443\"");
    }

    #[test]
    fn postcard() {
        assert_postcard(IPv4Address::BROADCAST);
        assert_postcard(IPv6Address::UNSPECIFIED);
        assert_postcard(IPv4Address::LOCALHOST.to_socket(65535));
        assert_postcard(IPv6Address::LOCALHOST.to_socket(0));
        assert_postcard(IPv6Address::LOCALHOST.to_ip().to_socket(443));
    }

    /// The binary form is the raw address bytes, with no tag or length prefix.
    #[test]
    fn compact_sizes() {
        let test_cases: &[(Vec<u8>, usize, &str)] = &[
            (assert_postcard(IPv4Address::LOCALHOST), 4, "IPv4Address"),
            (assert_postcard(IPv6Address::LOCALHOST), 16, "IPv6Address"),
            (
                assert_postcard(IPv4Address::LOCALHOST.to_socket(80)),
                5,
                "SocketAddressV4",
            ),
            (
                assert_postcard(IPv6Address::LOCALHOST.to_socket(80)),
                17,
                "SocketAddressV6",
            ),
        ];

        for (bytes, expected, label) in test_cases {
            assert_eq!(bytes.len(), *expected, "type={}", label);
        }
    }

    /// The README claims the version-specific types match the standard library wire format.
    #[test]
    fn matches_std_wire_format() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;
        assert_eq!(
            assert_postcard(ip),
            postcard::to_allocvec(&Ipv4Addr::LOCALHOST).unwrap()
        );

        let ip: IPv6Address = IPv6Address::LOCALHOST;
        assert_eq!(
            assert_postcard(ip),
            postcard::to_allocvec(&Ipv6Addr::LOCALHOST).unwrap()
        );

        let socket: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let std: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80);
        assert_eq!(assert_postcard(socket), postcard::to_allocvec(&std).unwrap());

        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let std: SocketAddrV6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0);
        assert_eq!(assert_postcard(socket), postcard::to_allocvec(&std).unwrap());
    }

    /// `SocketAddress` deliberately diverges: it encodes the IP as a byte string, not the standard library's enum.
    #[test]
    fn socket_address_diverges_from_std() {
        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        let bytes: Vec<u8> = assert_postcard(socket);
        assert_eq!(bytes.len(), 6, "4 address bytes with a length prefix, plus the port");
    }
}
