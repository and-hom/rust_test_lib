pub mod memory;
pub mod disk;
#[macro_use]
extern crate log;

pub trait Storage<TData> {
    fn store(&mut self, id: &str, data: TData);
    fn read(&self, id: &str, callback: &Fn(Option<&TData>));
    fn flush(&mut self);
}
