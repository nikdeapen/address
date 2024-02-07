# address
This library aids in processing network addresses.

## Address Types
There are 6 primary address types. The IP and socket address types are `Copy` and are split into IPv4 and IPv6
variants. The other addresses possibly contain a variable length domain and cannot be `Copy`. These are all split into
owned and `Ref` types; so you can use them with or without allocation. 

- **IP Addresses**
  - Represents the bits of an IP address variant.
  - These are `Copy` split into three variants: `IPv4Address`, `IPv6Address`, and `IPAddress` (enum).
- **Socket Addresses**
  - These are IP address variants with an associate ports. 
  - These are also `Copy` and split into three variants: `SocketAddressV4`, `SocketAddressV6`, and `SocketAddress`.
- **Domains**
  - Wraps a validated domain name and is not `Copy`.
  - These are split into owned and reference variants: `Domain` and `DomainRef`.
- **Endpoints**
  - Represents a `Domain` with an associated port.
  - These are split into owned and reference variants: `Endpoint` and `EndpointRef`.
- **Hosts**
  - An enum that represents either an IP address or a domain name.
  - These are split into owned and reference variants: `Host` and `HostRef`.
- **Authorities**
  - Represents a `Host` with an associated port.
  - These are split into owned and reference variants: `Endpoint` and `EndpointRef`.
