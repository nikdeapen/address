# Notes

## Safety

- The `unsafe` blocks have no safety comments by design. Each wraps an unchecked operation such as
  `from_utf8_unchecked` on input that is already known to be in the correct format.
- The `unsafe` functions such as `new_unchecked` skip runtime validation for performance. The rest
  of the crate may rely on the invariants they assume without the runtime cost of re-checking.

## Addresses

### Domains

- Domains have no equality with strings by design. Names are lowercase, so it is ambiguous
  whether `domain == "Example.com"` should hold. Callers compare `Domain::name()` and handle
  case themselves.
