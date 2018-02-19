use ::Storage;
use std::collections::HashMap;

struct MemoryStorage<TData> {
    storage: HashMap<String, TData>
}

impl<TData> Storage<TData> for MemoryStorage<TData> {
    fn store(&mut self, id: &str, data: TData) {
        self.storage.insert(id.to_string(), data);
    }

    fn read(&self, id: &str, callback: &Fn(Option<&TData>)) {
        let found = self.storage.get(id);
        callback(found);
    }
    fn flush(&self) {}
}

pub fn new<TData>() -> Box<Storage<TData>> where TData: 'static {
    Box::new(MemoryStorage {
        storage: HashMap::new()
    })
}
