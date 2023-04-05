#[derive(Debug, Clone)]
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