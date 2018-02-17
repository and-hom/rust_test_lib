extern crate bincode;
extern crate serde;

use ::Storage;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::marker::PhantomData;
//use self::bincode;
use self::serde::de::DeserializeOwned;
use self::serde::ser::Serialize;

struct FileStorage<TData> where TData: Serialize + DeserializeOwned {
    //    base_dir: &'a Path,
    phantom: PhantomData<TData>
}
//
//impl<'a, TData> FileStorage<'a, TData> {
//    fn path(&self, id: &str) -> &Path {
//        self.base_dir.with_file_name(id).as_path()
//    }
//}

impl<TData> Storage<TData> for FileStorage<TData> where TData: Serialize + DeserializeOwned {
    fn store(&mut self, id: &str, data: TData) {
        let path = Path::new("hello.txt");

        let mut file = match File::create(&path) {
            Err(why) => {
                let path_str = match path.to_str() {
                    None => "unknown",
                    Some(x) => x,
                };
                panic!("couldn't open {}: {}", path_str, why.description())
            }
            Ok(file) => file,
        };
    }

    fn read(&self, id: &str) -> Option<&TData> {
        let path = Path::new("hello.txt");
        match File::open(&path) {
            Err(why) => None,
            Ok(mut file) => {
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Err(why) => None,
                    Ok(_) => {
                        let x: Result<TData, _> = bincode::deserialize(&buffer);
                        match x {
                            Ok(d) => {
                                Some(&d)
                            },
                            Err(e) => None
                        }
                    }
                }
            }
        }
    }
}

pub fn new<TData>(base_dir: &str) -> Box<Storage<TData>>
    where TData: 'static + Serialize + DeserializeOwned {
    //    let p :&'a Path = Path::new(base_dir);
    Box::new(FileStorage {
        //        base_dir: p,
        phantom: PhantomData
    })
}