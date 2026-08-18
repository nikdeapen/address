# Issues

## Cargo

- Decide whether to declare an MSRV: add `rust-version` to `Cargo.toml`, measure it, and enforce it in CI.

## Portability

- Decide whether to support `no_std` + `alloc`: the crate uses nothing std-only. `core::net` (1.77) covers the IP &
  socket types with their `FromStr` & `Display` impls, `core::error::Error` (1.81) is the same trait, & `String` &
  `Vec` come from `alloc`; none of it raises the MSRV past the 1.88 the let-chains already require. The lib builds
  with `--all-features` under `#![no_std]` after mechanical `std::` -> `core::`/`alloc::` rewrites; what remains is
  `alloc` imports in each `#[cfg(test)] mod tests` block & building `idna` with
  `default-features = false, features = ["alloc", "compiled_data"]`, which its manifest supports.

## API

- Decide whether `TryFrom<Vec<u8>>` should become a named parser like `parse_text`: the `&[u8]` half already did,
  leaving the byte-vector impls as the only byte API still spelled as a trait. `FromStringVisitor` is bound on it for
  the `visit_byte_buf` buffer reuse, & an inherent `parse_text` cannot be used generically, so the visitor needs
  another mechanism first.
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

- The domain-bearing types serialize as `Display` strings in every format, & `impl_serde_string!` writes them with
  `collect_str`, which postcard implements by formatting the value twice: once to count the length for the varint
  prefix, then again to write it. `Domain` & `DomainRef` can use `serialize_str(self.name())` instead, & `Host` &
  `HostRef` can do so for their `Name` variants; the output is byte-identical & nothing allocates. `Endpoint`,
  `Authority` & their refs have to build the string, so switching them would trade the double pass for an allocation
  on the streaming JSON path.
- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char` (rust#110998)
  stabilizes; they use `from_utf8_unchecked` behind an ASCII check that `as_ascii` would make safe.
