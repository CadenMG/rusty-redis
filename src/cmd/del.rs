use std::fmt;

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
}

impl fmt::Display for Del {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "del: {}", self.key)
    }
}