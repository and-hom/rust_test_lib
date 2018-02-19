use ::Storage;
use std::collections::HashMap;
use std::rc::Rc;

struct MemoryStorage<TData> {
    storage: HashMap<String, Rc<TData>>
}

impl<TData> Storage<TData> for MemoryStorage<TData> {
    fn store(&mut self, id: &str, data: TData) {
        self.storage.insert(id.to_string(), Rc::new(data));
    }

    fn read(&self, id: &str) -> Option<Rc<TData>> {
        match self.storage.get(id) {
            None => None,
            Some(x) => Some(Rc::clone(&x))
        }
    }

    fn clear(&mut self) {
        let s = &mut (self.storage);
        s.clear();
    }
}

pub fn new<TData>() -> Box<Storage<TData>> where TData: 'static {
    Box::new(MemoryStorage {
        storage: HashMap::new()
    })
}
