# Issues

There are currently no issues aside from future work.

# Future Work

- Add `IPAddress::to_canonical` & `IPv6Address::to_canonical` to mirror `std`.
- Add support for `no_std` + `alloc`.
- Add `idna` support to the `Host`, `Endpoint`, and `Authority` types.
- Add DNS support via implementing `std::net::ToSocketAddrs` as well as async support.
- Modify the IP `parse` fns with `as_ascii()` or std's `parse_ascii`.
    - This is blocked on rust#110998 or rust#101035.
- Add `cargo fuzz` targets for the `parse` fns.
