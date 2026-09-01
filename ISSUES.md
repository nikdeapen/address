# Issues

## Portability

- Add support for `no_std` + `alloc`.

## API

- Add `idna` support to the `Host`, `Endpoint`, and `Authority` types.
- Add DNS support via implementing `std::net::ToSocketAddrs` as well as async support.

## Performance

- Guard the IP `parse` fns with `is_ascii()` + an unchecked conversion; the checked `from_utf8`
  costs up to 38%.
- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char`
  (rust#110998).
