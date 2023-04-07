use std::fmt;
use bytes::Bytes;

use crate::db::DB;

#[derive(Debug, Clone, PartialEq)]
pub struct Del {
    key: String,
}

impl Del {
    pub fn new(key: impl ToString) -> Del {
        Del {
            key: key.to_string(),
        }
    }

    pub fn apply(&self, db: &DB) -> Option<Bytes> {
        db.del(&self.key)
    }
}

impl fmt::Display for Del {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "del: {}", self.key)
    }
}