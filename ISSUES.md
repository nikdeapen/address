# Issues

## Portability

- Add support for `no_std` + `alloc`.

## API

- Add `idna` support to the `Host`, `Endpoint`, and `Authority` types.
- Add DNS support via implementing `std::net::ToSocketAddrs` as well as async support.
- Add `TryFrom` impls mirroring the fallible conversions; only the infallible ones have `From`.

## Performance

- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char`
  (rust#110998).
