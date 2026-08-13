//! The canonical string corpus shared by the integration tests: each string is the exact `Display` output of the
//! value it parses to.

pub const IPV4_ADDRESSES: &[&str] = &["0.0.0.0", "127.0.0.1", "1.2.3.4", "255.255.255.255"];

pub const IPV6_ADDRESSES: &[&str] = &[
    "::",
    "::1",
    "1::",
    "1::1",
    "1:0:0:1::",
    "1:2:3:4:5:6:7:8",
    "fe80::1",
    "::ffff:1.2.3.4",
    "2001:db8::8a2e:370:7334",
    "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
];

pub const IP_ADDRESSES: &[&str] = &["127.0.0.1", "255.255.255.255", "::1", "fe80::1"];

pub const SOCKET_ADDRESSES_V4: &[&str] = &["0.0.0.0:0", "127.0.0.1:80", "255.255.255.255:65535"];

pub const SOCKET_ADDRESSES_V6: &[&str] = &["[::]:0", "[::1]:80", "[::ffff:1.2.3.4]:443", "[fe80::1]:65535"];

pub const SOCKET_ADDRESSES: &[&str] = &["127.0.0.1:80", "[::1]:443", "[fe80::1]:0"];

pub const DOMAINS: &[&str] = &[
    "localhost",
    "example.com",
    "a-b.c--d.example",
    "xn--bcher-kva.example",
    "123.example",
];

pub const ENDPOINTS: &[&str] = &["localhost:80", "example.com:443", "a.b.c:65535", "x:0"];

pub const HOSTS: &[&str] = &["localhost", "example.com", "127.0.0.1", "::1", "fe80::1"];

pub const AUTHORITIES: &[&str] = &[
    "localhost:80",
    "example.com:443",
    "127.0.0.1:80",
    "[::1]:443",
    "[fe80::1]:0",
];
