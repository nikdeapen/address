#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "serde", doc = include_str!("serde/WIRE_CONTRACT.md"))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg))]

pub use authority::*;
pub use domain::*;
pub use endpoint::*;
pub use host::*;
pub use ip::*;
pub use parse::*;
pub use socket::*;

mod authority;
mod domain;
mod endpoint;
mod host;
mod ip;
mod parse;
mod socket;

mod display;

#[cfg(feature = "serde")]
mod serde;
