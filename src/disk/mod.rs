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
use std::rc::Rc;
use std::marker::PhantomData;
use self::serde::de::DeserializeOwned;
use self::serde::ser::Serialize;

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

macro_rules! try_or_none {
    ($r:expr, $msg:expr, $($arg:tt)+) => {{
        match $r {
            Err(why) => {
                let msg_str = format!($msg, $($arg)+);
                error!("{}: {:?}", msg_str, why.description());
                return None;
            },
            Ok(x) => x,
        }
    }};
}

impl<TData> Storage<TData> for FileStorage<TData> where TData: Serialize + DeserializeOwned {
    fn store(&mut self, id: &str, data: &TData) {
        let path = self.path(id);
        let path_str = path.to_str().unwrap_or("unknown");

        let mut file = try_or_panic!(File::create(&path), "couldn't open {}", path_str);
        let bytes = try_or_panic!(bincode::serialize(data), "couldn't serialize data");
        try_or_panic!(file.write_all(&bytes) ,"couldn't write to {}", path_str);
    }

    fn read(&self, id: &str) -> Option<Rc<TData>> {
        let path = self.path(id);
        let path_str = path.to_str().unwrap_or("unknown");

        let mut file = try_or_none!(File::open(&path), "couldn't open {}", path_str);
        let mut buffer = Vec::new();
        try_or_none!(file.read_to_end(&mut buffer),"couldn't read data from {}", path_str);
        let found = try_or_none!(bincode::deserialize(&buffer), "couldn't deserialize data from {}", path_str);

        Some(Rc::new(found))
    }

    fn clear(&mut self) {
        if self.base_dir.is_dir() {
            delete_dir_contents(fs::read_dir(&(self.base_dir)));
        }
    }
}

/// Create filesystem storage instance
pub fn new<TData>(base_dir: &str) -> Box<Storage<TData>>
    where TData: 'static + Serialize + DeserializeOwned {
    let path = PathBuf::from(base_dir);
    if !path.is_dir() {
        if path.exists() {
            panic!("Base dir {} exists but is not a directory", base_dir)
        } else {
            fs::create_dir_all(&path).expect(&format!("Can not mkdir {}", base_dir));
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
