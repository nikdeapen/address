# Future

This document lists the planned work that is not started yet.

## Project

- Add support for `no_std` + `alloc`.
- Add `cargo fuzz` targets for the `parse` fns.

## Addresses

- Add `IPAddress::to_canonical` & `IPv6Address::to_canonical` to mirror `std`.
- Add `idna` support to the `Host`, `Endpoint`, and `Authority` types.
- Add DNS support via implementing `std::net::ToSocketAddrs` as well as async support.

## Parse

- Modify the IP `parse` fns with `as_ascii()` or std's `parse_ascii`.
    - This is blocked on rust#110998 or rust#101035.
