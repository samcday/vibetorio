//! Save/load and serialization helpers.

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveArtifact<T> {
    pub version: u32,
    pub payload: T,
}

impl<T> SaveArtifact<T> {
    pub const fn new(version: u32, payload: T) -> Self {
        Self { version, payload }
    }
}

pub fn encode_to_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

pub fn decode_from_json<T: DeserializeOwned>(json: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_and_decode_round_trips() {
        let artifact = SaveArtifact::new(1, vec![1u32, 2u32, 3u32]);

        let json = encode_to_json(&artifact).expect("artifact should serialize");
        let decoded =
            decode_from_json::<SaveArtifact<Vec<u32>>>(&json).expect("artifact should deserialize");

        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.payload.len(), 3);
        assert_eq!(decoded.payload[0], 1);
        assert_eq!(decoded.payload[2], 3);
    }

    #[test]
    fn decode_rejects_invalid_json() {
        let result = decode_from_json::<SaveArtifact<Vec<u32>>>("not json");
        assert!(result.is_err());
    }
}
