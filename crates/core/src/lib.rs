mod kv_store;
mod packets;

use std::io::ErrorKind;

pub(crate) type Error = Box<dyn std::error::Error>;

pub const PORT_FILE: &str = "/tmp/ferrite.port";

pub fn is_disconnect_error(error: &(dyn std::error::Error + 'static)) -> bool {
    error
        .downcast_ref::<std::io::Error>()
        .is_some_and(|io_error| {
            matches!(
                io_error.kind(),
                ErrorKind::UnexpectedEof
                    | ErrorKind::ConnectionReset
                    | ErrorKind::BrokenPipe
                    | ErrorKind::ConnectionAborted
            )
        })
}

pub use kv_store::FerriteKV;
pub use packets::{Packet, Request, Response};
pub use packets::{read, send};
