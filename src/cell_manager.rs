use std::collections::HashMap;

pub struct CellManager {
    // it is sorted
    ids: Vec<&'static str>,
    types: Vec<Vec<u8>>,
    pending: HashMap<usize, Vec<u8>>,
}

impl CellManager {
    pub fn get_by_id(&self, id: &'static str) -> Option<&[u8]> {
        let idx = self.ids.iter().position(|e| *e == id)?;
        Some(self.types.get(idx)?)
    }

    pub fn set_pending(&mut self, id: usize, data: Vec<u8>) {
        self.pending.insert(id, data);
    }

    pub fn commit(&mut self) {
        self.pending.drain().for_each(|(k, v)| {
            self.types.insert(k, v);
        });
    }

    pub fn new(data: Vec<(&'static str, Vec<u8>)>) -> Self {
        let mut data = data;
        data.sort_by_key(|d| d.0);
        let mut types = Vec::new();
        let mut ids = Vec::new();
        data.into_iter().for_each(|(id, d)| {
            types.push(d);
            ids.push(id);
        });

        CellManager {
            ids,
            types,
            pending: HashMap::new(),
        }
    }
}
