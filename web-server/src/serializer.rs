use std::error::Error;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Serializer {
    type Error: Error;

    fn serialize<T: Serialize>(&self, v: &T) -> Result<String, Self::Error>;

    fn deserialize<T: DeserializeOwned + Default>(&self, s: &str) -> Result<T, Self::Error>;
}

pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    type Error = serde_json::error::Error;

    fn serialize<T: Serialize>(&self, v: &T) -> Result<String, Self::Error> {
        serde_json::to_string(v)
    }

    fn deserialize<T: DeserializeOwned + Default>(&self, s: &str) -> Result<T, Self::Error> {
        if s.len() == 0 {
            return Ok(Default::default());
        }

        match serde_json::from_str::<T>(s) {
            Ok(t) => Ok(t),
            Err(e) => Err(Self::Error::from(e)),
        }
    }
}
