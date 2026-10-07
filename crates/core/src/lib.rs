use std::{collections::HashMap, net::TcpListener};

pub(crate) type Error = Box<dyn std::error::Error>;

pub struct FerriteKV {
    data: HashMap<String, String>,
    socket: Option<TcpListener>,
    port: u16,
}

impl Default for FerriteKV {
    fn default() -> Self {
        Self::new()
    }
}

impl FerriteKV {
    /// Creates a new instance of the KV store, without creating a socket.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            socket: None,
            port: 0,
        }
    }

    // Starts the TCP listener, changing the internal state of the KV store to add the TcpListener
    pub fn start(&mut self, host: &str) -> Result<(), Error> {
        let addr = format!("{}:0", host);

        let listener = TcpListener::bind(addr)?;
        let local_addr = listener.local_addr()?;

        self.socket = Some(listener);
        self.port = local_addr.port();

        Ok(())
    }

    /// Checks if the associated key already exists in the KV store
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Checks if the key does not exist, and sets the key to the data provided
    /// Later, we will add a feature for auto serde serialization
    pub fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        if self.contains(&key) {
            return Err("Key already exists".into());
        }

        self.data.insert(key, value);

        Ok(())
    }

    /// Gets the current value in the KV store, returns an error if the value does not exist
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_init() {
        let kv_store = FerriteKV::new();

        assert_eq!(kv_store.data, HashMap::new());
        assert_eq!(kv_store.port, 0);
        assert!(kv_store.socket.is_none());
    }

    #[test]
    fn start_listener() {
        let mut kv_store = FerriteKV::new();
        kv_store.start("127.0.0.1").unwrap();

        assert!(kv_store.socket.is_some());
        assert_ne!(kv_store.port, 0);
    }
}
