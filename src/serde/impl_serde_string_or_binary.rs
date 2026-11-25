use crate::{IPAddress, IPv4Address, IPv6Address};

/// Implements `Serialize` and `Deserialize` for an owned type that serializes as its `Display`
/// string in human-readable formats and as the `$bin` type in other formats, converting through
/// their `From` impls.
macro_rules! impl_serde_string_or_binary {
    ($ty:ident, $expecting:literal, $bin:ty) => {
        impl ::serde::Serialize for crate::$ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                if serializer.is_human_readable() {
                    serializer.collect_str(self)
                } else {
                    let binary: $bin = <$bin>::from(*self);
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
                    <$bin as ::serde::Deserialize>::deserialize(deserializer).map(crate::$ty::from)
                }
            }
        }
    };
}

impl_serde_string_or_binary!(IPv4Address, "an IPv4 address string", [u8; 4]);
impl_serde_string_or_binary!(IPv6Address, "an IPv6 address string", [u8; 16]);
impl_serde_string_or_binary!(SocketAddress, "a socket address string", (IPAddress, u16));
impl_serde_string_or_binary!(
    SocketAddressV4,
    "an IPv4 socket address string",
    (IPv4Address, u16)
);
impl_serde_string_or_binary!(
    SocketAddressV6,
    "an IPv6 socket address string",
    (IPv6Address, u16)
);

#[cfg(test)]
mod tests {
    use crate::serde::test_util::{assert_json, assert_postcard};
    use crate::{IPv4Address, IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use std::fmt::Debug;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    /// Asserts the `value` & the `std` value write the same JSON & read each other's JSON.
    fn assert_std_json<T, S>(value: T, std: S)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
        S: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let json: String = serde_json::to_string(&value).unwrap();
        let std_json: String = serde_json::to_string(&std).unwrap();
        assert_eq!(json, std_json, "value={:?}", value);

        let parsed: T = serde_json::from_str(std_json.as_str()).unwrap();
        assert_eq!(parsed, value, "value={:?}", value);

        let parsed: S = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(parsed, std, "value={:?}", value);
    }

    #[test]
    fn json() {
        assert_json(IPv4Address::UNSPECIFIED, "\"0.0.0.0\"");
        assert_json(IPv4Address::LOCALHOST, "\"127.0.0.1\"");
        assert_json(IPv6Address::LOCALHOST, "\"::1\"");
        assert_json(IPv4Address::LOCALHOST.to_socket(80), "\"127.0.0.1:80\"");
        assert_json(IPv6Address::LOCALHOST.to_socket(80), "\"[::1]:80\"");
        assert_json(
            IPv4Address::LOCALHOST.to_ip().to_socket(80),
            "\"127.0.0.1:80\"",
        );
        assert_json(
            IPv6Address::LOCALHOST.to_ip().to_socket(443),
            "\"[::1]:443\"",
        );
    }

    #[test]
    fn postcard() {
        assert_postcard(IPv4Address::BROADCAST);
        assert_postcard(IPv6Address::UNSPECIFIED);
        assert_postcard(IPv4Address::LOCALHOST.to_socket(65535));
        assert_postcard(IPv6Address::LOCALHOST.to_socket(0));
        assert_postcard(IPv6Address::LOCALHOST.to_ip().to_socket(443));
    }

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
        assert_eq!(
            assert_postcard(socket),
            postcard::to_allocvec(&SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80)).unwrap()
        );

        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        assert_eq!(
            assert_postcard(socket),
            postcard::to_allocvec(&SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0)).unwrap()
        );
    }

    #[test]
    fn diverges_from_std() {
        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        let bytes: Vec<u8> = assert_postcard(socket);
        let std_bytes: Vec<u8> = postcard::to_allocvec(&socket.to_std()).unwrap();
        assert_ne!(bytes, std_bytes);
        assert!(postcard::from_bytes::<SocketAddr>(&bytes).is_err());
        assert!(postcard::from_bytes::<SocketAddress>(&std_bytes).is_err());
    }

    #[test]
    fn matches_std_json() {
        let v4: IPv4Address = IPv4Address::LOCALHOST;
        let v6: IPv6Address = IPv6Address::LOCALHOST;

        assert_std_json(v4, v4.to_std());
        assert_std_json(v6, v6.to_std());
        assert_std_json(v4.to_socket(80), v4.to_socket(80).to_std());
        assert_std_json(v6.to_socket(80), v6.to_socket(80).to_std());
        assert_std_json(v4.to_ip().to_socket(80), v4.to_ip().to_socket(80).to_std());
        assert_std_json(v6.to_ip().to_socket(80), v6.to_ip().to_socket(80).to_std());
    }

    #[test]
    fn zone() {
        let expected: SocketAddressV6 =
            IPv6Address::from([0xFE80, 0, 0, 0, 0, 0, 0, 1]).to_socket(80);
        let std_json: String = serde_json::to_string(&expected.to_std_with(0, 7)).unwrap();
        assert_eq!(std_json, "\"[fe80::1%7]:80\"");

        let socket: SocketAddressV6 = serde_json::from_str(std_json.as_str()).unwrap();
        assert_eq!(socket, expected);
        assert_eq!(serde_json::to_string(&socket).unwrap(), "\"[fe80::1]:80\"");

        let socket: SocketAddress = serde_json::from_str(std_json.as_str()).unwrap();
        assert_eq!(socket, expected.to_socket());
        assert_eq!(serde_json::to_string(&socket).unwrap(), "\"[fe80::1]:80\"");
    }

    #[test]
    fn invalid_input() {
        assert!(serde_json::from_str::<IPv4Address>("\"256.0.0.1\"").is_err());
        assert!(serde_json::from_str::<IPv4Address>("\"::1\"").is_err());
        assert!(serde_json::from_str::<IPv4Address>("2130706433").is_err());
        assert!(serde_json::from_str::<IPv6Address>("\"127.0.0.1\"").is_err());
        assert!(serde_json::from_str::<IPv6Address>("\"[::1]\"").is_err());
        assert!(serde_json::from_str::<SocketAddress>("\"::1:80\"").is_err());
        assert!(serde_json::from_str::<SocketAddress>("\"127.0.0.1\"").is_err());
        assert!(serde_json::from_str::<SocketAddressV4>("\"[::1]:80\"").is_err());
        assert!(serde_json::from_str::<SocketAddressV6>("\"127.0.0.1:80\"").is_err());

        assert!(postcard::from_bytes::<IPv4Address>(&[127, 0, 0]).is_err());
        assert!(postcard::from_bytes::<IPv6Address>(&[0; 15]).is_err());
        assert!(postcard::from_bytes::<SocketAddressV4>(&[127, 0, 0, 1]).is_err());
    }
}
