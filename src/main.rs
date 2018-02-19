extern crate rust_test_lib;

use rust_test_lib::Storage;
//use rust_test_lib::memory;
use rust_test_lib::disk;

fn main() {
    let mut s: Box<Storage<String>> = disk::new("/tmp/aaa");
//    let mut s: Box<Storage<i32>> = memory::new();

    let print_data = |ptr: Option<&String>| {
        match ptr {
            None => println!("Nothing found"),
            Some(x) => println!("{}", x)
        }
    };

    s.flush();
    println!("=============================");
    s.read("id", &print_data);
    s.store("id", "aaaa".to_string());
    s.read("id", &print_data);
    s.flush();
    s.read("id", &print_data);
    s.store("id", "bbbb".to_string());
    s.read("id", &print_data);
    println!("=============================");
}
