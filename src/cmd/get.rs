use std::fmt;
use bytes::Bytes;

use crate::db::DB;

#[derive(Debug, Clone, PartialEq)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn new(key: impl ToString) -> Get {
        Get {
            key: key.to_string(),
        }
    }

    pub fn apply(&self, db: &DB) -> Option<Bytes> {
        db.get(&self.key)
    }
}

impl fmt::Display for Get {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "get: {}", self.key)
    }
}