# address

[![Build](https://github.com/nikdeapen/address/actions/workflows/build.yml/badge.svg)](https://github.com/nikdeapen/address/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/address.svg)](https://crates.io/crates/address)
[![Docs.rs](https://docs.rs/address/badge.svg)](https://docs.rs/address)
[![License](https://img.shields.io/crates/l/address.svg)](https://github.com/nikdeapen/address/blob/master/LICENSE)

This library aids in processing network addresses. It adds the domain, endpoint, host, & authority
types that `std` lacks. The types have strict validation, owned & borrowed variants, and standard
library conversions.

## Usage

```toml
address = "0.23.0"
```

## Features

This crate has no dependencies by default. The following optional features add dependencies:

- `idna`: Adds `Domain::parse_unicode` & `to_unicode` for international domain names.
- `serde`: Adds the `Serialize` & `Deserialize` implementations for all address types.

## Address Types

There are 6 core address types:

| Host       | Without a port         | With a port                  |
|------------|------------------------|------------------------------|
| IP Address | `IPAddress`            | `SocketAddress`              |
| Domain     | `Domain` & `DomainRef` | `Endpoint` & `EndpointRef`   |
| (Either)   | `Host` & `HostRef`     | `Authority` & `AuthorityRef` |

Types that are not inherently `Copy` come in owned & reference pairs; the IP & socket address types
are `Copy`, so they have no reference form. The `Ref` types borrow their text, so they can parse &
convert without allocation.

## Example

```rust
use address::{Authority, SocketAddress};

// Parsing validates and normalizes the address types.
let authority: Authority = "Example.com:443".parse().unwrap();
assert_eq!(authority.to_string(), "example.com:443");
assert_eq!(authority.port(), 443);

// You can easily convert between address types.
let authority: Authority = "127.0.0.1:80".parse().unwrap();
let socket: SocketAddress = authority.to_socket().unwrap();
assert_eq!(socket.to_string(), "127.0.0.1:80");

// You can even convert to the addresses provided in the standard library.
use std::net::SocketAddr;
let socket: SocketAddr = socket.to_std();
assert_eq!(socket, "127.0.0.1:80".parse().unwrap());
```
