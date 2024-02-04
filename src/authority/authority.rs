use crate::Host;

/// A host with an associated port.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Authority {
    host: Host,
    port: u16,
}

impl Authority {
    //! Construction

    /// Creates a new authority.
    pub const fn new(host: Host, port: u16) -> Self {
        Self { host, port }
    }
}

impl<H: Into<Host>> From<(H, u16)> for Authority {
    fn from(tuple: (H, u16)) -> Self {
        Self {
            host: tuple.0.into(),
            port: tuple.1,
        }
    }
}

impl Authority {
    //! Properties

    /// Gets the host.
    pub fn host(&self) -> &Host {
        &self.host
    }

    /// Gets the port.
    pub const fn port(&self) -> u16 {
        self.port
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, Domain, Host};

    #[test]
    fn properties() {
        let authority: Authority = (Domain::localhost(), 80).into();
        assert_eq!(authority.host(), &Host::Name(Domain::localhost()));
        assert_eq!(authority.port(), 80);
    }
}
