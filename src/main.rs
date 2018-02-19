extern crate rust_test_lib;

use rust_test_lib::Storage;
//use rust_test_lib::memory;
use rust_test_lib::disk;

fn main() {
    let mut s: Box<Storage<i32>> = disk::new("/tmp/aaa");
    let print_data = |ptr: Option<&i32>| {
        match ptr {
            None => println!("Nothing found"),
            Some(x) => println!("{}", x)
        }
    };

    println!("=============================");
    s.read("id", &print_data);
    s.store("id", 1);
    s.read("id", &print_data);
    s.store("id", 1);
    s.read("id", &print_data);
    println!("=============================");
}
