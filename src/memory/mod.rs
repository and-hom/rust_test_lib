use ::Storage;
use std::collections::HashMap;

struct MemoryStorage<TData> {
    storage: HashMap<String, TData>
}

impl<TData> Storage<TData> for MemoryStorage<TData> {
    fn store(&mut self, id: &str, data: TData) {
        self.storage.insert(id.to_string(), data);
    }

    fn read(&self, id: &str) -> Option<&TData> {
        let link = self.storage.get(id);
    }
}

pub fn new<TData>() -> Box<Storage<TData>> where TData: 'static {
    Box::new(MemoryStorage {
        storage: HashMap::new()
    })
}
