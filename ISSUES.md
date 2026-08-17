# Issues

## Cargo

- Decide whether to declare an MSRV: add `rust-version` to `Cargo.toml`, measure it, and enforce it in CI.

## API

- Decide whether to impl `std::net::ToSocketAddrs` for `Authority` & `AuthorityRef`: it enables
  `TcpStream::connect(&authority)` but puts blocking DNS resolution in a pure-data crate.

## Validation

- `Domain` allows an all-numeric final label (e.g. `999.1.1.1`), so malformed IPv4 strings parse as domains in
  `Host`/`Authority`. RFC 1123/3696 hostname rules forbid this; raw DNS allows it. Decide which profile to follow.

## Testing

- Coverage is representative, not exhaustive: most items get a single happy-path case. Edge cases could be much
  deeper across the crate.
- Run `cargo miri test` on the `from_utf8_unchecked` sites & decide whether to gate CI on it: a regressed ASCII
  guard is undefined behavior that the tests still pass.

## Performance

- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char` (rust#110998)
  stabilizes; they use `from_utf8_unchecked` behind an ASCII check that `as_ascii` would make safe.
