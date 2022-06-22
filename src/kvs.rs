use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

#[derive(Default)] // will fill default value
pub struct KvStore {
    // directory for log and other data
    path: PathBuf,
    readers: HashMap<u64, ()>,
    writer: (),
    index: BTreeMap<String, CommandPos>,
    uncompacted: u64,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}