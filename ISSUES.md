# Issues

## Portability

- Add support for `no_std` + `alloc`.

## API

- Add `idna` support to the `Host`, `Endpoint`, and `Authority` types.
- Add DNS support via implementing `std::net::ToSocketAddrs` as well as async support.

## Testing

- Get rid of pedantic tests like testing display formatting.
- Consolidate each type's parse cases into one table looped through every entry point; they now
  differ per path.

## Performance

- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char`
  (rust#110998).
