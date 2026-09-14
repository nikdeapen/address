# Wire Contract

The `serde` feature implements `Serialize` & `Deserialize` for every address type. The form depends
on the format's `is_human_readable()`, so a value must be read back with the same kind of format it
was written with.

## Human-Readable Formats

- Every type writes its `Display` string & reads a string with the same rules as `parse`.
- Owned types normalize domain names to lowercase & take the string in any form, borrowed or owned.
- The `Ref` types borrow from the input, so the text must reach them as a borrowed `&'de str`:
  domain names must already be lowercase, & the text cannot contain escape sequences. Escaped text
  fails with `invalid type: string ..., expected a borrowed ... string`; use the owned type for it.
- A byte string that is valid UTF-8 is accepted as a string.
- A numeric IPv6 zone in bracketed socket & authority text is accepted & dropped: `"[fe80::1%1]:80"`
  reads as `[fe80::1]:80`.

## Binary Formats

| Type              | Form                   | Standard Library                            |
|-------------------|------------------------|---------------------------------------------|
| `IPv4Address`     | `[u8; 4]`              | Byte-identical to `Ipv4Addr`.               |
| `IPv6Address`     | `[u8; 16]`             | Byte-identical to `Ipv6Addr`.               |
| `IPAddress`       | A 4- or 16-byte string | Differs; `IpAddr` writes a variant tag.     |
| `SocketAddressV4` | `(IPv4Address, u16)`   | Byte-identical to `SocketAddrV4`.           |
| `SocketAddressV6` | `(IPv6Address, u16)`   | Byte-identical to `SocketAddrV6`.           |
| `SocketAddress`   | `(IPAddress, u16)`     | Differs; `SocketAddr` writes a variant tag. |
| All other types   | The `Display` string   | No counterpart.                             |

- The `IPAddress` length selects the version. A sequence of 4 or 16 bytes is also accepted on read.
- Formats that length-prefix byte strings make an `IPAddress` larger than the concrete type: 5 bytes
  in postcard for an IPv4 address against 4 for an `IPv4Address`.
- The domain, endpoint, host, & authority types write & read the `Display` string in binary formats
  too.
- The two diverging types & their counterparts cannot read each other's bytes; the mismatch is a
  deserialization error, not a misread value.

## Standard Library Compatibility

- Human-readable: `IPAddress`, `IPv4Address`, `IPv6Address`, `SocketAddress`, `SocketAddressV4`, &
  `SocketAddressV6` match their `std::net` counterparts, since both sides write `Display`.
- Exception: `SocketAddrV6` writes a non-zero scope id, `"[fe80::1%7]:80"`. The crate reads it &
  drops the zone but never writes one, so the scope id does not survive a round trip. Flow info is
  never represented.
- Binary: see the table above.
