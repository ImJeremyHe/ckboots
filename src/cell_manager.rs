use std::collections::HashMap;

pub struct CellManager {
    types: HashMap<&'static str, Vec<u8>>,
    pending: HashMap<&'static str, Vec<u8>>,
}

impl CellManager {
    pub fn get_by_id(&self, id: &'static str) -> Option<&[u8]> {
        Some(self.types.get(id)?)
    }

    pub fn set_pending(&mut self, id: &'static str, data: Vec<u8>) {
        self.pending.insert(id, data);
    }

    pub fn commit(&mut self) {
        self.pending.drain().for_each(|(k, v)| {
            self.types.insert(k, v);
        });
    }
}
