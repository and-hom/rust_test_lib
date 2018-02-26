use ::Storage;
use ::memory;
use ::disk;

use std::fmt;
use std::cmp;
use std::clone;
use std::rc::Rc;
use std::ops::Deref;
use std::thread;

fn disk_path() -> String {
    let current_thread = thread::current();
    format!("/tmp/test_disk_storage/{:?}", current_thread.id())
}

struct TestConf<TData> where TData: cmp::PartialEq<TData> + fmt::Debug + clone::Clone {
    var1: TData,
    var2: TData
}

struct StorageTest<TData> where
    TData: cmp::PartialEq<TData> + fmt::Debug + clone::Clone
{
    storages: Vec<Box<Storage<TData>>>,
}

impl<TData> StorageTest<TData> where
    TData: cmp::PartialEq<TData> + fmt::Debug + clone::Clone
{
    fn do_test(&mut self, params: Vec<TestConf<TData>>) {
        for storage in self.storages.iter_mut() {
            for data in params.iter() {
                StorageTest::test_store(storage, data);
                StorageTest::test_overwrite(storage, data);
            }
        }
    }

    fn test_store(storage: &mut Box<Storage<TData>>, conf: &TestConf<TData>) {
        storage.clear();
        res_eq(storage.read("key1"), None);
        storage.store("key1", &(conf.var1)).unwrap();
        res_eq(storage.read("key1"), Some(&(conf.var1)));
    }

    fn test_overwrite(storage: &mut Box<Storage<TData>>, conf: &TestConf<TData>) {
        storage.clear();
        storage.store("key1", &(conf.var1)).unwrap();
        res_eq(storage.read("key1"), Some(&(conf.var1)));
        storage.store("key1", &(conf.var2)).unwrap();
        res_eq(storage.read("key1"), Some(&(conf.var2)));
    }
}

#[test]
fn test_str_mem() {
    let mut test_mem_str: StorageTest<&str> = StorageTest {
        storages: vec![memory::new()]
    };
    test_mem_str.do_test(vec![TestConf {
        var1: "var1",
        var2: "var2",
    }]);
}

#[test]
fn test_string() {
    let mut test_mem_str: StorageTest<String> = StorageTest {
        storages: vec![memory::new(), disk::new(&*disk_path())]
    };
    test_mem_str.do_test(vec![TestConf {
        var1: "var1".to_string(),
        var2: "var2".to_string(),
    }]);
}

#[test]
fn test_i32() {
    let mut test_mem_str: StorageTest<i32> = StorageTest {
        storages: vec![memory::new(), disk::new(&*disk_path())]
    };
    test_mem_str.do_test(vec![TestConf {
        var1: 1,
        var2: 2,
    }]);
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
struct CustomStruct {
    x: String,
    y: bool,
    z: f64,
}

#[test]
fn test_custom_struct() {
    let mut test_mem_str: StorageTest<CustomStruct> = StorageTest {
        storages: vec![memory::new(), disk::new(&*disk_path())]
    };
    test_mem_str.do_test(vec![TestConf {
        var1: CustomStruct {
            x: "aaa".to_string(),
            y: true,
            z: 0.256,
        },
        var2: CustomStruct {
            x: "bbb".to_string(),
            y: false,
            z: 128.,
        },
    }]);
}


fn res_eq<T, E>(actual: Result<Rc<T>, E>, expected: Option<&T>) where
    T: fmt::Debug + cmp::PartialEq,
    E: fmt::Debug
{
    match actual {
        Err(err) => match expected {
            None => (),
            Some(expected_val) => panic!("Expected {:?} actual error: {:?}", expected_val, err)
        },
        Ok(actual_val) => match expected {
            None => panic!("Expected None actual {:?}", actual_val),
            Some(e) => {
                let a_ref = Rc::deref(&actual_val);
                assert_eq!(a_ref, e)
            }
        }
    };
}
