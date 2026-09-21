# Issues

- Ambiguous conversions. Examples: `Host::from(1u32)` and `Authority::from((1u32, 80))`.
- Document on the `TryFrom<String>` impls that the string is returned in the error; only the
  `InvalidAddressError` doc says the value is preserved.
- Return `InvalidAuthority` for any unbracketed host with a colon; RFC 3986 allows none.
  Today it depends on the text before the last colon parsing as IPv6, so `2001:db8::1` & `::1`
  report `InvalidHost`. Checking for the colon in `HostForm::classify` also drops the IPv6 parse
  attempt from the domain path.
- Add an `exclude` list to `Cargo.toml`; the tarball ships `.github/`, `.idea/`, `.gitignore`,
  `ISSUES.md`, & `NOTES.md`.
- Mention `IPv4Address`, `IPv6Address`, `SocketAddressV4`, & `SocketAddressV6` in the README; the
  table only lists the six core types.
- Rename `is_valid_label_op_ignore_case` & `is_valid_name_op_ignore_case`; "op" reads as
  "operation".

## Parsing

- Guard the IP parse fns with is_ascii() + an unchecked conversion; the checked from_utf8
  costs up to 38%.

## Testing

- Test the width & precision handling of the `Display` impls; `EndpointRef` has a dedicated `pad`
  branch and `DomainRef` relies on `pad`, but `src/display/` has no tests.
- Test `ParseError`'s `Display` strings and `InvalidAddressError`'s `Display`, `into_value`, and
  `From` into `ParseError`; they are only exercised through `parse_unicode` under the `idna`
  feature.
- Add a nightly Miri job and `cargo fuzz` targets for the `parse` fns, plus a bench for the
  `is_ascii()` issue above; Miri passes the suite today.

# Future Work

- Add support for `no_std` + `alloc`.
- Add `idna` support to the `Host`, `Endpoint`, and `Authority` types.
- Add DNS support via implementing `std::net::ToSocketAddrs` as well as async support.
- Remove unsafe use `is_ascii()` in the `parse` fns. (blocked on rust#110998)
