use std::collections::HashMap;

const NONE_VALUE: Option<String> = None;

pub struct FerriteKV {
    data: HashMap<String, Option<String>>,
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
    pub fn set(&mut self, key: String, value: String) -> Result<(), crate::Error> {
        if self.contains(&key) {
            return Err("Key already exists".into());
        }

        self.data.insert(key, Some(value));

        Ok(())
    }

    /// Gets the current value in the KV store.
    pub fn get(&self, key: &str) -> &Option<String> {
        self.data.get(key).unwrap_or(&NONE_VALUE)
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
