use ::Storage;
use std::collections::HashMap;
use std::rc::Rc;
use std::clone::Clone;

struct MemoryStorage<TData> where TData: Clone {
    storage: HashMap<String, Rc<TData>>
}

impl<TData> Storage<TData> for MemoryStorage<TData> where TData: Clone {
    fn store(&mut self, id: &str, data: &TData) {
        let cloned_data = data.clone();
        self.storage.insert(id.to_string(), Rc::new(cloned_data));
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

pub fn new<TData>() -> Box<Storage<TData>> where TData: 'static + Clone {
    Box::new(MemoryStorage {
        storage: HashMap::new()
    })
}
