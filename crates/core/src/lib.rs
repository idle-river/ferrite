mod kv_store;
mod packets;

pub(crate) type Error = Box<dyn std::error::Error>;

pub const PORT_FILE: &str = "/tmp/ferrite.port";

pub use kv_store::FerriteKV;
pub use packets::{Packet, Request, Response};
pub use packets::{read, send};
