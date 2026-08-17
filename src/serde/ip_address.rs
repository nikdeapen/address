use crate::serde::FromStrVisitor;
use crate::{IPAddress, IPv4Address, IPv6Address};
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

impl Serialize for IPAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            serializer.serialize_bytes(self.address())
        }
    }
}

/// A serde visitor that matches a byte string's length: 4 bytes for an IPv4 address, 16 bytes for an IPv6 address.
struct IPAddressBytesVisitor;

impl<'de> Visitor<'de> for IPAddressBytesVisitor {
    type Value = IPAddress;

    fn expecting(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("4 or 16 IP address bytes")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(address) = <[u8; 4]>::try_from(v) {
            Ok(IPv4Address::new(address).to_ip())
        } else if let Ok(address) = <[u8; 16]>::try_from(v) {
            Ok(IPv6Address::new(address).to_ip())
        } else {
            Err(E::invalid_length(v.len(), &self))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut address: [u8; 16] = [0; 16];
        let mut len: usize = 0;
        while let Some(byte) = seq.next_element::<u8>()? {
            if len == address.len() {
                return Err(A::Error::invalid_length(len + 1, &self));
            }
            address[len] = byte;
            len += 1;
        }
        self.visit_bytes(&address[..len])
    }
}

impl<'de> Deserialize<'de> for IPAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(FromStrVisitor::new("an IP address string"))
        } else {
            deserializer.deserialize_bytes(IPAddressBytesVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serde::ip_address::IPAddressBytesVisitor;
    use crate::serde::test_util::{assert_json, assert_postcard};
    use crate::{IPAddress, IPv4Address, IPv6Address};
    use serde::Deserializer;
    use serde::de::value::{Error as ValueError, SeqDeserializer};

    #[test]
    fn json() {
        assert_json(IPv4Address::LOCALHOST.to_ip(), "\"127.0.0.1\"");
        assert_json(IPv6Address::LOCALHOST.to_ip(), "\"::1\"");
        assert_json(IPv4Address::BROADCAST.to_ip(), "\"255.255.255.255\"");
    }

    /// The binary form is a byte string whose length selects the version, not the standard library's enum tag.
    #[test]
    fn postcard() {
        let bytes: Vec<u8> = assert_postcard(IPv4Address::LOCALHOST.to_ip());
        assert_eq!(bytes.len(), 5, "a length prefix plus 4 address bytes");

        let bytes: Vec<u8> = assert_postcard(IPv6Address::LOCALHOST.to_ip());
        assert_eq!(bytes.len(), 17, "a length prefix plus 16 address bytes");
    }

    /// Formats that present a byte string as a sequence take the `visit_seq` path.
    #[test]
    fn visit_seq() {
        let test_cases: &[(&[u8], Option<IPAddress>)] = &[
            (&[127, 0, 0, 1], Some(IPv4Address::LOCALHOST.to_ip())),
            (&[0; 16], Some(IPv6Address::UNSPECIFIED.to_ip())),
            (&[], None),
            (&[127, 0, 0], None),
            (&[0; 5], None),
            (&[0; 17], None),
        ];

        for (input, expected) in test_cases {
            let seq: SeqDeserializer<_, ValueError> = SeqDeserializer::new(input.iter().copied());
            let result: Option<IPAddress> = seq.deserialize_seq(IPAddressBytesVisitor).ok();
            assert_eq!(result, *expected, "input={:?}", input);
        }
    }
}
