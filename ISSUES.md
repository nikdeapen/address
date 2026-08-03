# Issues

## Future Work

- Add an optional `serde` feature serializing via `Display`/`FromStr`. Off by default to keep the crate
  dependency-free.
- Add `TryFrom<&'a str>` to `HostRef`, `EndpointRef`, and `AuthorityRef` so they can parse without
  allocating. `FromStr` cannot borrow from its input, which is why `DomainRef` already uses `TryFrom`.

## API

- `SocketAddress::to_std` requires `flow_info` and `scope_id` even for IPv4, where both are ignored.
  Consider an argless `to_std` alongside a `to_std_with` that takes them.
- Add narrowing conversions: `Host::to_domain`/`to_ip`, `Authority::to_endpoint`/`to_socket`, and the
  `HostRef` equivalents. `IPAddress` and `SocketAddress` already have theirs.
- Add borrowing conversions: `From<&Domain> for DomainRef` plus the `Host`, `Endpoint`, and `Authority`
  equivalents. `to_ref` covers direct use; `From` lets generic code accept `impl Into<DomainRef>`.
- Add `IPv6Address::to_v4_mapped` matching only `::ffff:a.b.c.d`, mirroring std's `to_ipv4_mapped`. The
  existing `to_v4` also matches the compatible form, so `::1` converts to `0.0.0.1`.
- Add cross-type `PartialEq` between the owned and ref types (`Domain`/`DomainRef`, `Host`/`HostRef`, etc.)
  so comparing them does not require a conversion.
- Add `AsRef<str>` and `Borrow<str>` to `Domain` so `HashMap<Domain, V>` can be looked up with a `&str`.
- `Display` impls drop formatter flags (width/fill/alignment) because nested `write!` resets the spec. Fix by
  delegating with `inner.fmt(f)` or `f.pad` plus a pad helper; this also removes the duplicated socket formatting.
- `TryFrom<String>`/`TryFrom<Vec<u8>>` for `Domain` return tuple errors that implement neither `Display` nor
  `Error`. Replace with a dedicated value-recovering error struct like `std::string::FromUtf8Error`.

## Documentation

- `lib.rs` has no crate-level docs so the docs.rs landing page is empty. Add `//!` docs or include the
  README via `#![doc = include_str!("../README.md")]`; consider `#![warn(missing_docs)]` as well.

## Testing

- Missing edge tests: `MAX_LABEL_LEN`/`MAX_NAME_LEN` boundaries, port overflow, and the IPv6/socket std `From`
  conversions. `parse_port` doc examples never run since it is `pub(crate)`; its edges are only covered indirectly.
- Add round-trip tests: `FromStr(Display(x)) == x` for the parseable types and parse-then-display for
  canonical strings, to cover the bracket and port edges systematically.

## Validation

- `Domain` allows an all-numeric final label (e.g. `999.1.1.1`), so malformed IPv4 strings parse as domains in
  `Host`/`Authority`. RFC 1123/3696 hostname rules forbid this; raw DNS allows it. Decide which profile to follow.
