use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct DB {
    state: Arc<Mutex<State>>
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Entry>
}

#[derive(Debug)]
struct Entry {
    id: u64,
    data: Bytes, 
}

impl DB {
    pub fn new() -> DB {
        let state = State {
            entries: HashMap::new(),
        };
        DB { state: Arc::new(Mutex::new(state)) }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        self.state.lock().unwrap().entries.get(key).map(|entry| entry.data.clone())
    }

    pub fn del(&self, key: &str) -> Option<Bytes> {
        self.state.lock().unwrap().entries.remove(key).map(|entry| entry.data.clone())
    }

    pub fn set(&self, key: String, val: Bytes) -> Option<Bytes> {
        let mut state = self.state.lock().unwrap();
        state.entries.insert(
            key,
            Entry {
                id: 0,
                data: val,
            },
        ).map(|entry| entry.data.clone())
    }
}