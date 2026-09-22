use crate::{Domain, Endpoint, EndpointRef};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;

impl Debug for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_ref(), f)
    }
}

impl<'a> Debug for EndpointRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for EndpointRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const MAX_LEN: usize = Domain::MAX_NAME_LEN + 1 + "65535".len();
        if f.width().is_none() && f.precision().is_none() {
            write!(f, "{}:{}", self.domain(), self.port())
        } else {
            let mut bytes: [u8; MAX_LEN] = [0; MAX_LEN];
            let mut cursor: &mut [u8] = &mut bytes;
            write!(cursor, "{}:{}", self.domain(), self.port()).map_err(|_| std::fmt::Error)?;
            let len: usize = MAX_LEN - cursor.len();
            f.pad(unsafe { std::str::from_utf8_unchecked(&bytes[..len]) })
        }
    }
}
