#![cfg(feature = "serde")]

mod canonical;

use address::{
    Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef, IPAddress, IPv4Address,
    IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6,
};
use canonical::{
    AUTHORITIES, DOMAINS, ENDPOINTS, HOSTS, IP_ADDRESSES, IPV4_ADDRESSES, IPV6_ADDRESSES, SOCKET_ADDRESSES,
    SOCKET_ADDRESSES_V4, SOCKET_ADDRESSES_V6,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::str::FromStr;

fn assert_round_trips<T>(canonical: &[&str])
where
    T: FromStr + Serialize + DeserializeOwned + PartialEq + Debug,
    T::Err: Debug,
{
    for s in canonical {
        let value: T = T::from_str(s).unwrap();

        let json: String = serde_json::to_string(&value).unwrap();
        assert_eq!(json, format!("\"{}\"", s), "json for {:?}", s);
        let parsed: T = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(parsed, value, "json round trip for {:?}", s);

        let bytes: Vec<u8> = postcard::to_allocvec(&value).unwrap();
        let parsed: T = postcard::from_bytes(bytes.as_slice()).unwrap();
        assert_eq!(parsed, value, "postcard round trip for {:?}", s);
    }
}

macro_rules! assert_ref_round_trips {
    ($ty:ident, $canonical:expr) => {
        for s in $canonical {
            let value: $ty = $ty::try_from(*s).unwrap();

            let json: String = serde_json::to_string(&value).unwrap();
            assert_eq!(json, format!("\"{}\"", s), "json for {:?}", s);
            let parsed: $ty = serde_json::from_str(json.as_str()).unwrap();
            assert_eq!(parsed, value, "json round trip for {:?}", s);

            let bytes: Vec<u8> = postcard::to_allocvec(&value).unwrap();
            let parsed: $ty = postcard::from_bytes(bytes.as_slice()).unwrap();
            assert_eq!(parsed, value, "postcard round trip for {:?}", s);
        }
    };
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

#[test]
fn escaped_input() {
    let domain: Domain = serde_json::from_str("\"local\\u0068ost\"").unwrap();
    assert_eq!(domain, Domain::localhost());

    let result: Result<DomainRef, serde_json::Error> = serde_json::from_str("\"local\\u0068ost\"");
    assert!(result.is_err());
}

#[test]
fn compact_sizes() {
    let test_cases: &[(Vec<u8>, usize, &str)] = &[
        (
            postcard::to_allocvec(&IPv4Address::LOCALHOST).unwrap(),
            4,
            "IPv4Address",
        ),
        (
            postcard::to_allocvec(&IPv6Address::LOCALHOST).unwrap(),
            16,
            "IPv6Address",
        ),
        (
            postcard::to_allocvec(&IPv4Address::LOCALHOST.to_ip()).unwrap(),
            5,
            "IPAddress::V4",
        ),
        (
            postcard::to_allocvec(&IPv6Address::LOCALHOST.to_ip()).unwrap(),
            17,
            "IPAddress::V6",
        ),
    ];

    for (bytes, expected, label) in test_cases {
        assert_eq!(bytes.len(), *expected, "type={}", label);
    }
}
