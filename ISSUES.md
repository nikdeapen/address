# Issues

## Portability

- Add support for `no_std` + `alloc`.

## API

- Add `idna` support to the `Host`, `Endpoint`, and `Authority` types.
- Add DNS support via implementing `std::net::ToSocketAddrs` as well as async support.
- Add `TryFrom` impls mirroring the fallible conversions; only the infallible ones have `From`.

## Testing

- Consolidate each type's parse cases into one table looped through every entry point.

## Documentation

- Document why the IP & socket types exist apart from `std`; it is crate-level, so the only rustdoc
  home is the front page, which is the README.

## Performance

- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char`
  (rust#110998).
