use std::collections::HashMap;
use std::ops;

pub struct CodeEntry {
    pub bits: u32,
    pub length: u8,
}

pub struct ByteMap(HashMap<u8, CodeEntry>);

impl ByteMap {
    pub fn new() -> Self {
        ByteMap(HashMap::new())
    }
}

impl ops::Deref for ByteMap {
    type Target = HashMap<u8, CodeEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ByteMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
