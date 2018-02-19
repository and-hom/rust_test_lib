extern crate bincode;
extern crate serde;

use ::Storage;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::{create_dir_all, remove_dir_all, remove_file};
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

impl<TData> Storage<TData> for FileStorage<TData> where TData: Serialize + DeserializeOwned {
    fn store(&mut self, id: &str, data: TData) {
        let path = self.path(id);
        let path_str = match path.to_str() {
            None => "unknown",
            Some(x) => x,
        };

        let mut file = match File::create(&path) {
            Err(why) => {
                panic!("couldn't open {}: {}", path_str, why.description())
            }
            Ok(file) => file,
        };

        let bytes = match bincode::serialize(&data) {
            Err(why) => panic!("couldn't serialize data: {}", why.description()),
            Ok(x) => x
        };

        match file.write_all(&bytes) {
            Err(why) => {
                panic!("couldn't write to {}: {}", path_str,
                       why.description())
            }
            Ok(_) => (),
        };
    }

    fn read(&self, id: &str, callback: &Fn(Option<&TData>)) {
        let path = self.path(id);
        let path_str = match path.to_str() {
            None => "unknown",
            Some(x) => x,
        };

        let found = match File::open(&path) {
            Err(why) => {
                println!("couldn't open {}: {}", path_str, why.description());
                None
            }
            Ok(mut file) => {
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Err(why) => {
                        println!("couldn't read data from {}: {}", path_str, why.description());
                        None
                    }
                    Ok(_) => {
                        let x: Result<TData, _> = bincode::deserialize(&buffer);
                        match x {
                            Err(why) => {
                                println!("couldn't deserialize data from {}: {}", path_str, why.description());
                                None
                            }
                            Ok(d) => {
                                Some(d)
                            }
                        }
                    }
                }
            }
        };
        match found {
            None => callback(None),
            Some(x) => callback(Some(&x))
        };
    }

    fn flush(&self) {}
}


pub fn new<TData>(base_dir: &str) -> Box<Storage<TData>>
    where TData: 'static + Serialize + DeserializeOwned {
    let path = PathBuf::from(base_dir);
    if !path.is_dir() {
        if path.exists() {
            panic!("Base dir {} exists but is not a directory", base_dir)
        } else {
            match create_dir_all(&path) {
                Err(why) => panic!("Can not mkdir {}: {}", base_dir, why.description()),
                Ok(_) => ()
            }
        }
    }
    Box::new(FileStorage {
        base_dir: path,
        phantom: PhantomData
    })
}
