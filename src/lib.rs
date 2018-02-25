pub mod memory;
pub mod disk;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod test;

#[macro_use]
extern crate log;

use std::rc::Rc;
use std::error::Error;

/// Key-value storage. Key is allways ``&str``
///
/// # Examples
///
/// ```
/// use rust_test_lib;
/// use rust_test_lib::memory;
/// use std::rc::Rc;
/// use std::ops::Deref;
///
/// // here can be used disk::new("/path")
/// let mut storage = memory::new();
/// let val1 = 10;
/// let val2 = 20;
///
/// storage.store("1", &val1);
/// storage.store("2", &val2);
///
/// match storage.read("1") {
///     Ok(x) => assert_eq!(Rc::deref(&x), &val1),
///     Err(_) => panic!("Should not ever happen")
/// }
/// ```
pub trait Storage<TData, StoreError: Error, ReadError: Error> {
    /// Store data by key
    fn store(&mut self, id: &str, data: &TData) -> Result<(), StoreError>;
    /// Read value by key
    fn read(&self, id: &str) -> Result<Rc<TData>, ReadError>;
    /// Remove all data
    fn clear(&mut self);
}
