pub mod memory;
pub mod disk;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod test;

#[macro_use]
extern crate log;

use std::rc::Rc;

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
///     Some(x) => assert_eq!(Rc::deref(&x), &val1),
///     None => panic!("Should not")
/// }
/// ```
pub trait Storage<TData> {
    /// Store data by key
    fn store(&mut self, id: &str, data: &TData);
    /// Read value by key
    fn read(&self, id: &str) -> Option<Rc<TData>>;
    /// Remove all data
    fn clear(&mut self);
}
