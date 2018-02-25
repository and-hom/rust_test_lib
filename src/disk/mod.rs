//! Filesystem storage implementation
//!
//! Every value stored in separate file
extern crate bincode;
pub extern crate serde;

use ::Storage;
use std::error::Error;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs;
use std::fmt;
use std::rc::Rc;
use std::marker::PhantomData;
use self::serde::de::DeserializeOwned;
use self::serde::ser::Serialize;

#[derive(Debug)]
pub enum StoreError {
    BINCODE(bincode::Error)
}

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

impl From<io::Error> for StoreError {
    fn from(err: io::Error) -> StoreError {
        StoreError::BINCODE(Box::new(bincode::ErrorKind::Io(err)))
    }
}

impl From<bincode::Error> for StoreError {
    fn from(err: bincode::Error) -> StoreError {
        StoreError::BINCODE(err)
    }
}


#[derive(Debug)]
pub enum ReadError {
    MISSING(String),
    BINCODE(bincode::Error)
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::MISSING(_) => write!(f, "Missing key {}", self.description()),
            ReadError::BINCODE(ref err) => err.fmt(f),
        }
    }
}

impl Error for ReadError {
    fn description(&self) -> &str {
        match *self {
            ReadError::MISSING(_) => "Missing key",
            ReadError::BINCODE(ref err) => err.description(),
        }
    }
}

impl From<bincode::Error> for ReadError {
    fn from(err: bincode::Error) -> ReadError {
        ReadError::BINCODE(err)
    }
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::BINCODE(Box::new(bincode::ErrorKind::Io(err)))
    }
}


struct FileStorage<TData> where TData: Serialize + DeserializeOwned {
    base_dir: PathBuf,
    phantom: PhantomData<TData>
}

impl<TData> FileStorage<TData> where TData: Serialize + DeserializeOwned {
    fn path(&self, id: &str) -> PathBuf {
        self.base_dir.join(id)
    }
}

macro_rules! try_or_panic {
    ($r:expr, $msg:expr) => {
        $r.expect($msg)
    };
    ($r:expr, $msg:expr, $($arg:tt)+) => {
        $r.expect(&format!($msg, $($arg)+))
    };
}

impl<TData> Storage<TData, StoreError, ReadError> for FileStorage<TData>
    where TData: Serialize + DeserializeOwned {
    fn store(&mut self, id: &str, data: &TData) -> Result<(), StoreError> {
        let path = self.path(id);
        let path_str = path.to_str().unwrap_or("unknown");

        let mut file = try!(File::create(&path));
        let bytes = try!(bincode::serialize(data));
        file.write_all(&bytes).map_err(StoreError::from)
    }

    fn read(&self, id: &str) -> Result<Rc<TData>, ReadError> {
        let path = self.path(id);
        let path_str = path.to_str().unwrap_or("unknown");

        let mut file = try!(File::open(&path));
        let mut buffer = Vec::new();
        try!(file.read_to_end(&mut buffer));
        let found = try!(bincode::deserialize(&buffer));

        Ok(Rc::new(found))
    }

    fn clear(&mut self) {
        if self.base_dir.is_dir() {
            delete_dir_contents(fs::read_dir(&(self.base_dir)));
        }
    }
}

/// Create filesystem storage instance
pub fn new<TData>(base_dir: &str) -> Box<Storage<TData, StoreError, ReadError>>
    where TData: 'static + Serialize + DeserializeOwned {
    let path = PathBuf::from(base_dir);
    if !path.is_dir() {
        if path.exists() {
            panic!("Base dir {} exists but is not a directory", base_dir)
        } else {
            fs::create_dir_all(&path)
                .expect(&format!("Can not mkdir {}", base_dir));
        }
    }
    Box::new(FileStorage {
        base_dir: path,
        phantom: PhantomData
    })
}

fn delete_dir_contents(read_dir_res: Result<fs::ReadDir, io::Error>) {
    if let Ok(dir) = read_dir_res {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
            };
        }
    };
}
