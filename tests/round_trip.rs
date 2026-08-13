mod canonical;

use address::{
    Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef, IPAddress, IPv4Address,
    IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6,
};
use canonical::{
    AUTHORITIES, DOMAINS, ENDPOINTS, HOSTS, IP_ADDRESSES, IPV4_ADDRESSES, IPV6_ADDRESSES, SOCKET_ADDRESSES,
    SOCKET_ADDRESSES_V4, SOCKET_ADDRESSES_V6,
};
use std::fmt::{Debug, Display};
use std::str::FromStr;

/// Parses each canonical string, checks the value displays as the exact same string, then checks the displayed string
/// parses back to an equal value.
fn assert_round_trips<T>(canonical: &[&str])
where
    T: FromStr + Display + PartialEq + Debug,
    T::Err: Debug,
{
    for s in canonical {
        let parsed: T = match T::from_str(s) {
            Ok(parsed) => parsed,
            Err(error) => panic!("failed to parse {:?}: {:?}", s, error),
        };
        let displayed: String = parsed.to_string();
        assert_eq!(displayed.as_str(), *s, "display was not canonical for {:?}", s);

        let reparsed: T = T::from_str(displayed.as_str()).unwrap();
        assert_eq!(reparsed, parsed, "reparse changed the value for {:?}", s);
    }
}

/// Parses each canonical string as a reference type, checks the value displays as the exact same string, then checks
/// the displayed string parses back to an equal value.
macro_rules! assert_ref_round_trips {
    ($ty:ident, $canonical:expr) => {
        for s in $canonical {
            let value: $ty = $ty::try_from(*s).unwrap();

            let displayed: String = value.to_string();
            assert_eq!(displayed.as_str(), *s, "display was not canonical for {:?}", s);

            let reparsed: $ty = $ty::try_from(displayed.as_str()).unwrap();
            assert_eq!(reparsed, value, "reparse changed the value for {:?}", s);
        }
    };
}

/// Displays each value and checks the displayed string parses back to an equal value.
fn assert_display_parses<T>(values: &[T])
where
    T: FromStr + Display + PartialEq + Debug,
    T::Err: Debug,
{
    for value in values {
        let displayed: String = value.to_string();
        let reparsed: T = match T::from_str(displayed.as_str()) {
            Ok(reparsed) => reparsed,
            Err(error) => panic!("failed to reparse {:?} from {:?}: {:?}", value, displayed, error),
        };
        assert_eq!(&reparsed, value, "value={:?}", value);
    }
}

#[test]
fn ipv4() {
    assert_round_trips::<IPv4Address>(IPV4_ADDRESSES);
}

#[test]
fn ipv6() {
    assert_round_trips::<IPv6Address>(IPV6_ADDRESSES);
}

#[test]
fn ipv6_constructed() {
    assert_display_parses(&[
        IPv6Address::from(0u128),
        IPv6Address::from(1u128),
        IPv6Address::from(u128::MAX),
        IPv6Address::from([1, 0, 0, 1, 0, 0, 0, 0]),
        IPv6Address::from([0, 0, 1, 0, 0, 0, 0, 1]),
        IPv4Address::LOCALHOST.to_v6_compatible(),
        IPv4Address::LOCALHOST.to_v6_mapped(),
    ]);
}

#[test]
fn ip() {
    assert_round_trips::<IPAddress>(IP_ADDRESSES);
}

#[test]
fn socket_v4() {
    assert_round_trips::<SocketAddressV4>(SOCKET_ADDRESSES_V4);
}

#[test]
fn socket_v6() {
    assert_round_trips::<SocketAddressV6>(SOCKET_ADDRESSES_V6);
}

#[test]
fn socket() {
    assert_round_trips::<SocketAddress>(SOCKET_ADDRESSES);
}

#[test]
fn socket_constructed() {
    assert_display_parses(&[
        IPv6Address::from([0, 0, 1, 0, 0, 0, 0, 1]).to_socket(0),
        IPv6Address::from(u128::MAX).to_socket(65535),
    ]);
    assert_display_parses(&[
        IPv4Address::BROADCAST.to_socket(65535).to_socket(),
        IPv6Address::LOCALHOST.to_socket(1).to_socket(),
    ]);
}

#[test]
fn domain() {
    assert_round_trips::<Domain>(DOMAINS);
}

#[test]
fn domain_ref() {
    assert_ref_round_trips!(DomainRef, DOMAINS);
}

#[test]
fn endpoint() {
    assert_round_trips::<Endpoint>(ENDPOINTS);
}

#[test]
fn endpoint_ref() {
    assert_ref_round_trips!(EndpointRef, ENDPOINTS);
}

#[test]
fn host() {
    assert_round_trips::<Host>(HOSTS);
}

#[test]
fn host_ref() {
    assert_ref_round_trips!(HostRef, HOSTS);
}

#[test]
fn authority() {
    assert_round_trips::<Authority>(AUTHORITIES);
}

#[test]
fn authority_ref() {
    assert_ref_round_trips!(AuthorityRef, AUTHORITIES);
}
