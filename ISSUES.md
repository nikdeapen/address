# Issues

## API

- `Display` impls drop formatter flags (width/fill/alignment) because nested `write!` resets the spec. Fix by
  delegating with `inner.fmt(f)` or `f.pad` plus a pad helper; this also removes the duplicated socket formatting.
- `TryFrom<String>`/`TryFrom<Vec<u8>>` for `Domain` return tuple errors that implement neither `Display` nor
  `Error`. Replace with a dedicated value-recovering error struct like `std::string::FromUtf8Error`.

## Validation
  
- `Domain` allows an all-numeric final label (e.g. `999.1.1.1`), so malformed IPv4 strings parse as domains in
  `Host`/`Authority`. RFC 1123/3696 hostname rules forbid this; raw DNS allows it. Decide which profile to follow.
