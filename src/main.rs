extern crate rust_test_lib;
extern crate serde;

use rust_test_lib::Storage;
use rust_test_lib::memory;
use rust_test_lib::disk;
use std::rc::Rc;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Display;

fn main() {
    let mut m = memory::new();
    let mut d = disk::new("/tmp/aaa");
    do_test(&mut *m, 1,2);
    do_test(&mut *d, "a".to_string(),"b".to_string());
}

fn do_test<T>(s: &mut Storage<T>, val1: T, val2: T) where T: Serialize + DeserializeOwned + Display {
    //    let mut s: Box<Storage<T>> = disk::new("/tmp/aaa");

    s.flush();
    println!("=============================");
    print_data(s.read("id"));
    s.store("id", val1);
    print_data(s.read("id"));
    s.flush();
    print_data(s.read("id"));
    s.store("id", val2);
    print_data(s.read("id"));
    println!("=============================");
}


fn print_data<Displ>(opt: Option<Rc<Displ>>) where Displ: Display {
    match opt {
        None => println!("Nothing found"),
        Some(x) => println!("{}", x)
    }
}