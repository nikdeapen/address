use serde::de::{Error, Unexpected, Visitor};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;

/// A serde visitor that parses a string with `FromStr`.
pub(crate) struct FromStrVisitor<T> {
    expecting: &'static str,
    phantom: PhantomData<fn() -> T>,
}

impl<T> FromStrVisitor<T> {
    //! Construction

    /// Creates a new visitor with the `expecting` message.
    pub(crate) const fn new(expecting: &'static str) -> Self {
        Self {
            expecting,
            phantom: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for FromStrVisitor<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Value = T;

    fn expecting(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.expecting)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        T::from_str(v).map_err(E::custom)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match std::str::from_utf8(v) {
            Ok(s) => self.visit_str(s),
            Err(_) => Err(E::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IPv4Address;
    use crate::serde::FromStrVisitor;
    use serde::Deserializer;
    use serde::de::value::{BytesDeserializer, Error as ValueError};

    /// Formats that hand the visitor raw bytes take the `visit_bytes` path, which must reject non-UTF-8.
    #[test]
    fn visit_bytes() {
        let visitor: FromStrVisitor<IPv4Address> = FromStrVisitor::new("an IPv4 address string");
        let deserializer: BytesDeserializer<ValueError> = BytesDeserializer::new(b"127.0.0.1");
        assert_eq!(deserializer.deserialize_str(visitor).unwrap(), IPv4Address::LOCALHOST);

        let visitor: FromStrVisitor<IPv4Address> = FromStrVisitor::new("an IPv4 address string");
        let deserializer: BytesDeserializer<ValueError> = BytesDeserializer::new(b"\xFF");
        assert!(deserializer.deserialize_str(visitor).is_err());
    }
}
