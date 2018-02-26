//! Hash-map storage implementation
use ::Storage;
use ::ReadError;
use ::StoreError;
use ::RemoveError;
use std::collections::HashMap;
use std::rc::Rc;
use std::clone::Clone;

struct MemoryStorage<TData> where TData: Clone {
    storage: HashMap<String, Rc<TData>>
}

impl<TData> Storage<TData> for MemoryStorage<TData> where TData: Clone {
    fn store(&mut self, id: &str, data: &TData) -> Result<(), StoreError> {
        self.storage.insert(id.to_string(), Rc::new(data.clone()));
        Ok(())
    }

    fn read(&self, id: &str) -> Result<Rc<TData>, ReadError> {
        self.storage.get(id)
            .ok_or(ReadError::MISSING(id.to_string()))
            .map(|x| { Rc::clone(x) })
    }

    fn remove(&mut self, id: &str) -> Result<(), RemoveError> {
        self.storage.remove(id)
            .ok_or(RemoveError::MISSING(id.to_string()))
            .map(|_| { () })
    }

    fn clear(&mut self) {
        let s = &mut (self.storage);
        s.clear();
    }
}

/// Create in-memory storage instance
pub fn new<TData>() -> Box<Storage<TData>> where TData: 'static + Clone {
    Box::new(MemoryStorage {
        storage: HashMap::new()
    })
}
