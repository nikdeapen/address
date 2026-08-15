# address

[![Crates.io](https://img.shields.io/crates/v/address.svg)](https://crates.io/crates/address)
[![Docs.rs](https://docs.rs/address/badge.svg)](https://docs.rs/address)
[![License](https://img.shields.io/crates/l/address.svg)](https://github.com/nikdeapen/address/blob/master/LICENSE)

This library provides network address types — IP, socket, domain, endpoint, host, & authority — with strict
validation, owned & borrowed variants, and standard library conversions.

## Usage

```toml
address = "0.19.0"
```

## Example

```rust
use address::Authority;

// Parsing normalizes mixed-case domain names to lowercase.
let authority: Authority = "Example.com:443".parse().unwrap();
assert_eq!(authority.to_string(), "example.com:443");
assert!(authority.host().is_domain());
assert_eq!(authority.port(), 443);

// Hosts may also be IP addresses; socket addresses convert to & from the standard library types.
let authority: Authority = "[::1]:443".parse().unwrap();
let socket: std::net::SocketAddr = authority.to_socket().unwrap().to_std();
assert_eq!(socket, "[::1]:443".parse().unwrap());
```

## Features

This crate has no dependencies by default.

- `idna`: Adds `Domain::from_unicode` & `to_unicode` for international domain names. Uses the `idna` crate.
- `serde`: Adds `Serialize` & `Deserialize` implementations via the `serde` crate. Human-readable formats use the
  `Display` & `FromStr` strings. Binary formats use compact binary forms for the IP & socket address types. The
  version-specific types match the wire format of the standard library types; `IPAddress` & `SocketAddress` serialize
  the IP address as a byte string of 4 or 16 bytes instead of the standard library's enum encoding. The `Ref` types
  deserialize by borrowing from the input; domain names must already be lowercase & escaped input is an error, so
  use the owned types for mixed-case or escaped input.

## Address Types

There are 6 core address types:

- `IPAddress`: Either an IPv4 address or an IPv6 address.
    - Includes the `IPAddress` enum along with the `IPv4Address` & `IPv6Address` struct types.
- `SocketAddress`: An IP address with an associated port.
    - Includes the `SocketAddress`, `SocketAddressV4` & `SocketAddressV6` struct types.
- `Domain`: A domain name.
    - Includes the `Domain` & `DomainRef` struct types.
- `Endpoint`: A domain with an associated port.
    - Includes the `Endpoint` & `EndpointRef` struct types.
- `Host`: Either a domain or an IP address.
    - Includes the `Host` & `HostRef` enum types.
- `Authority`: A host with an associated port.
    - Includes the `Authority` & `AuthorityRef` struct types.

## Owned & Reference Types

Address types that are not `Copy` come in owned & reference pairs (example: `Domain` & `DomainRef`). The `Ref` types
borrow their text, so they parse & convert without allocating; each side converts to the other.

## Domain Names

Domain names are restricted to lowercase ASCII letters, digits, and dashes: dot-separated labels of up to 63 bytes
that do not start or end with a dash, with a total name length of up to 253 bytes. Mixed-case input is normalized to
lowercase when parsing owned types. Underscores, empty labels, and the trailing root dot are invalid. Unicode names
can be converted to their ASCII form with the `idna` feature.

## Standard Library Types

The IP & socket address types are separate from their standard library counterparts so the host & authority types can
compose them and the whole family behaves uniformly. They convert to & from the standard library types. IPv6 socket
addresses do not model `flow_info` or `scope_id`: converting from the standard library discards them, converting to it
zeroes them, & socket parsing accepts the numeric zone syntax the standard library accepts (`[fe80::1%1]:80`) while
ignoring the zone.
