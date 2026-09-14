use serde::de::{Error, Unexpected, Visitor};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;

/// A serde visitor that parses a string with `FromStr`, reusing the buffer of an owned string
/// with the consuming `TryFrom<String>` conversion.
pub(in crate::serde) struct FromStringVisitor<T> {
    expecting: &'static str,
    phantom: PhantomData<fn() -> T>,
}

impl<T> FromStringVisitor<T> {
    //! Construction

    /// Creates a new visitor with the `expecting` message.
    pub(in crate::serde) const fn new(expecting: &'static str) -> Self {
        Self {
            expecting,
            phantom: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for FromStringVisitor<T>
where
    T: FromStr + TryFrom<String>,
    <T as FromStr>::Err: Display,
    <T as TryFrom<String>>::Error: Display,
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

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        T::try_from(v).map_err(E::custom)
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

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match String::from_utf8(v) {
            Ok(text) => self.visit_string(text),
            Err(error) => Err(E::invalid_value(Unexpected::Bytes(error.as_bytes()), &self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Domain;
    use crate::serde::FromStringVisitor;
    use serde::Deserializer;
    use serde::de::Visitor;
    use serde::de::value::{BytesDeserializer, Error as ValueError};

    #[test]
    fn visit_byte_buf() {
        let visitor: FromStringVisitor<Domain> = FromStringVisitor::new("a domain string");
        let result: Domain = visitor
            .visit_byte_buf::<ValueError>(Vec::from("LocalHost"))
            .unwrap();
        assert_eq!(result.name(), "localhost");

        let visitor: FromStringVisitor<Domain> = FromStringVisitor::new("a domain string");
        assert!(
            visitor
                .visit_byte_buf::<ValueError>(Vec::from("Local!Host"))
                .is_err()
        );
    }

    #[test]
    fn visit_bytes() {
        let visitor: FromStringVisitor<Domain> = FromStringVisitor::new("a domain string");
        let deserializer: BytesDeserializer<ValueError> = BytesDeserializer::new(b"LocalHost");
        let result: Domain = deserializer.deserialize_string(visitor).unwrap();
        assert_eq!(result.name(), "localhost");

        let visitor: FromStringVisitor<Domain> = FromStringVisitor::new("a domain string");
        let deserializer: BytesDeserializer<ValueError> = BytesDeserializer::new(b"\xFF");
        assert!(deserializer.deserialize_string(visitor).is_err());
    }
}
