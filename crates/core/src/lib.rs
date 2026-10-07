use std::collections::HashMap;

pub(crate) type Error = Box<dyn std::error::Error>;

pub const PORT_FILE: &str = "/tmp/ferrite.port";

pub struct FerriteKV {
    data: HashMap<String, String>,
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
        }
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
    }
}
