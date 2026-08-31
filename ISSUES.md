# Issues

## Portability

- Add support for `no_std` + `alloc`.

## API

- Add `idna` support to the `Host`, `Endpoint`, and `Authority` types.
- Add DNS support via implementing `std::net::ToSocketAddrs` as well as async support.
- Conform the fallible conversions to `Result`; most still return `Option`.
- Add `TryFrom` impls mirroring the fallible conversions; only the infallible ones have `From`.
- Add `#[must_use]` to `ParseError` & `InvalidAddressError`; the only public types without it.

## Parsing

- Extract the host classification duplicated between `Authority::parse_text` & `parse_vec`.

## Testing

- Get rid of pedantic tests like testing display formatting.
- Consolidate each type's parse cases into one table looped through every entry point; they now
  differ per path.

## Documentation

- Document the serde wire contract & the standard library divergences in rustdoc, not the README;
  `src/serde/` is private, so the docs must go on the public impls.

## Performance

- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char`
  (rust#110998).
