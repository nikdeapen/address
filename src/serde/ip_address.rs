use crate::serde::FromStrVisitor;
use crate::{IPAddress, IPv4Address, IPv6Address};
use serde::de::{Error, Visitor};
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

struct IPAddressBytesVisitor;

impl<'de> Visitor<'de> for IPAddressBytesVisitor {
    type Value = IPAddress;

    fn expecting(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("a 4 or 16 byte IP address")
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
    use crate::serde::test_util::{assert_json, assert_postcard};
    use crate::{IPv4Address, IPv6Address};

    #[test]
    fn json() {
        assert_json(IPv4Address::LOCALHOST.to_ip(), "\"127.0.0.1\"");
        assert_json(IPv6Address::LOCALHOST.to_ip(), "\"::1\"");
        assert_json(IPv4Address::BROADCAST.to_ip(), "\"255.255.255.255\"");
    }

    #[test]
    fn postcard() {
        let bytes: Vec<u8> = assert_postcard(IPv4Address::LOCALHOST.to_ip());
        assert_eq!(bytes.len(), 5, "a length prefix plus 4 address bytes");

        let bytes: Vec<u8> = assert_postcard(IPv6Address::LOCALHOST.to_ip());
        assert_eq!(bytes.len(), 17, "a length prefix plus 16 address bytes");
    }
}
