use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Set {
    key: String,
    value: Bytes,
}

impl Set {
    pub fn new(key: impl ToString, value: Bytes) -> Set {
        Set {
            key: key.to_string(),
            value,
        }
    }
}