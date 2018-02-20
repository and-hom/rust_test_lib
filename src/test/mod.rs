use ::Storage;
use ::memory;
use ::disk;

use std::fmt;
use std::cmp;
use std::clone;
use std::rc::Rc;
use std::ops::Deref;

const DISK_PATH: &str = "/tmp/test_disk_storage";

struct TestConf<TData> where TData: cmp::PartialEq<TData> + fmt::Debug + clone::Clone {
    var1: TData,
    var2: TData
}

struct StorageTest<TData> where TData: cmp::PartialEq<TData> + fmt::Debug + clone::Clone {
    storages: Vec<Box<Storage<TData>>>,
}

impl<TData> StorageTest<TData> where TData: cmp::PartialEq<TData> + fmt::Debug + clone::Clone {
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
        opt_eq(storage.read("key1"), None);
        storage.store("key1", &(conf.var1));
        opt_eq(storage.read("key1"), Some(&(conf.var1)));
    }

    fn test_overwrite(storage: &mut Box<Storage<TData>>, conf: &TestConf<TData>) {
        storage.clear();
        storage.store("key1", &(conf.var1));
        opt_eq(storage.read("key1"), Some(&(conf.var1)));
        storage.store("key1", &(conf.var2));
        opt_eq(storage.read("key1"), Some(&(conf.var2)));
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
        storages: vec![memory::new(), disk::new(DISK_PATH)]
    };
    test_mem_str.do_test(vec![TestConf {
        var1: "var1".to_string(),
        var2: "var2".to_string(),
    }]);
}

#[test]
fn test_i32() {
    let mut test_mem_str: StorageTest<i32> = StorageTest {
        storages: vec![memory::new(), disk::new(DISK_PATH)]
    };
    test_mem_str.do_test(vec![TestConf {
        var1: 1,
        var2: 2,
    }]);
}


fn opt_eq<T>(actual: Option<Rc<T>>, expected: Option<&T>) where T: fmt::Debug + cmp::PartialEq {
    match actual {
        None => match expected {
            None => (),
            Some(e) => panic!("Expected {:?} actual None", e)
        },
        Some(a) => match expected {
            None => panic!("Expected None actual {:?}", a),
            Some(e) => {
                let a_ref = Rc::deref(&a);
                assert_eq!(a_ref, e)
            }
        }
    };
}
