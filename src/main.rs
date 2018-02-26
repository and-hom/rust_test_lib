extern crate libc;

use libc::c_int;

#[link(name = "test1")]
extern {
    fn my_test_func() -> c_int;
}

fn main() {
    let x = unsafe { my_test_func() };
    println!("максимальный размер сжатого буфера длиной 100 байт: {}", x);
}