//! Hash-map storage implementation
use ::Storage;
use std::collections::HashMap;
use std::rc::Rc;
use std::clone::Clone;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum StoreError {}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Error for StoreError {
    fn description(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum ReadError {
    MISSING(String)
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::MISSING(_) => write!(f, "Missing key {}", self.description()),
        }
    }
}

impl Error for ReadError {
    fn description(&self) -> &str {
        match *self {
            ReadError::MISSING(_) => "Missing key",
        }
    }
}

struct MemoryStorage<TData> where TData: Clone {
    storage: HashMap<String, Rc<TData>>
}

impl<TData> Storage<TData, StoreError, ReadError> for MemoryStorage<TData> where TData: Clone {
    fn store(&mut self, id: &str, data: &TData) -> Result<(), StoreError> {
        self.storage.insert(id.to_string(), Rc::new(data.clone()));
        Ok(())
    }

    fn read(&self, id: &str) -> Result<Rc<TData>, ReadError> {
        self.storage.get(id)
            .ok_or(ReadError::MISSING(id.to_string()))
            .map(|x| { Rc::clone(x) })
    }

    fn clear(&mut self) {
        let s = &mut (self.storage);
        s.clear();
    }
}

/// Create in-memory storage instance
pub fn new<TData>() -> Box<Storage<TData, StoreError, ReadError>> where TData: 'static + Clone {
    Box::new(MemoryStorage {
        storage: HashMap::new()
    })
}
