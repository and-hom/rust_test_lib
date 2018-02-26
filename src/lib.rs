pub mod memory;
pub mod disk;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod test;

#[macro_use]
extern crate log;

use std::rc::Rc;
use std::io;
use std::fmt;
use std::error;

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
pub trait Storage<TData> {
    /// Store data by key
    fn store(&mut self, id: &str, data: &TData) -> Result<(), StoreError>;
    /// Read value by key
    fn read(&self, id: &str) -> Result<Rc<TData>, ReadError>;
    /// Remove all data
    fn clear(&mut self);
}


#[derive(Debug)]
pub enum StoreError {
    IO(io::Error),
    INTERNAL(Box<error::Error>),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StoreError::IO(ref err) => err.fmt(f),
            StoreError::INTERNAL(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for StoreError {
    fn description(&self) -> &str {
        match *self {
            StoreError::IO(ref err) => err.description(),
            StoreError::INTERNAL(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            StoreError::IO(ref err) => Some(err),
            StoreError::INTERNAL(ref err) => Some(err.as_ref()),
        }
    }
}

impl From<io::Error> for StoreError {
    fn from(err: io::Error) -> StoreError {
        StoreError::IO(err)
    }
}


#[derive(Debug)]
pub enum ReadError {
    MISSING(String),
    IO(io::Error),
    INTERNAL(Box<error::Error>),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::MISSING(ref key) => write!(f, "Missing key {}", key),
            ReadError::IO(ref err) => err.fmt(f),
            ReadError::INTERNAL(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for ReadError {
    fn description(&self) -> &str {
        match *self {
            ReadError::MISSING(_) => "Missing key",
            ReadError::IO(ref err) => err.description(),
            ReadError::INTERNAL(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ReadError::MISSING(_) => None,
            ReadError::IO(ref err) => Some(err),
            ReadError::INTERNAL(ref err) => Some(err.as_ref()),
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::IO(err)
    }
}

