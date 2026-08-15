# Issues

## Cargo

- Decide whether to declare an MSRV: add `rust-version` to `Cargo.toml`, measure it, and enforce it in CI.

## API

- Decide whether to impl `std::net::ToSocketAddrs` for `Authority` & `AuthorityRef`: it enables
  `TcpStream::connect(&authority)` but puts blocking DNS resolution in a pure-data crate.

## Validation

- `Domain` allows an all-numeric final label (e.g. `999.1.1.1`), so malformed IPv4 strings parse as domains in
  `Host`/`Authority`. RFC 1123/3696 hostname rules forbid this; raw DNS allows it. Decide which profile to follow.
- Socket parsing rejects the zone syntax std accepts (`[fe80::1%1]:80` parses via `SocketAddrV6::from_str`), since
  scope ids are not modeled. Decide whether to document or support it.

## Parsing

- Compose the owned `Endpoint`/`Host`/`Authority` parsers from owned components directly, replacing the triplicated
  Ref-parse-then-lowercase-rescue blocks.
- Consolidate `Domain`'s triple validate-then-lowercase ladder into a single classifier & route all construction
  through `new_unchecked`.

## Testing

- Refactor the `tests/` folder: rework the `tests/canonical/` corpus (free consts in `mod.rs`, frozen composite
  subsets) & dedupe the duplicated round-trip/serde macros & wiring.
- Coverage is representative, not exhaustive: most items get a single happy-path case. Edge cases could be much
  deeper across the crate.
- Test the README claim that version-specific types match the std wire format: assert binary-serialized
  `IPv4Address`/`IPv6Address`/`SocketAddressV4`/`SocketAddressV6` bytes equal their std counterparts.

## Performance

- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char` (rust#110998)
  stabilizes; safe `from_utf8` currently relies on its ASCII fast path being equivalent.
