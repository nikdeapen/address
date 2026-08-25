# Issues

## Cargo

- Decide whether to declare an MSRV: add `rust-version` to `Cargo.toml`, measure it, and enforce it in CI.

## Portability

- Decide whether to support `no_std` + `alloc`: the crate uses nothing std-only.

## API

- Decide whether `TryFrom<Vec<u8>>` should become a named parser like `parse_text`.
- Decide whether to impl `std::net::ToSocketAddrs` for `Authority` & `AuthorityRef`.

## Validation

- Decide whether `Domain` should reject an all-numeric final label (e.g. `999.1.1.1`).
- Decide whether `Domain` should reject IPv4 text (e.g. `127.0.0.1`).

## Testing

- Decide whether to gate CI on `cargo miri test`; it passes today under `-Zmiri-strict-provenance`.
- Get rid of pedantic tests like testing display formatting.

## Performance

- Serialize `Domain`, `DomainRef`, & the `Host` `Name` variants with `serialize_str` instead of `collect_str`.
- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char` (rust#110998).
