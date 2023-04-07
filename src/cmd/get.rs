use std::fmt;

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
}

impl fmt::Display for Get {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "get: {}", self.key)
    }
}