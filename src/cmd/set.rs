use bytes::Bytes;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "set: {} {:?}", self.key, self.value)
    }
}