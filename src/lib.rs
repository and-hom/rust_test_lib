pub mod memory;
pub mod disk;

#[macro_use]
extern crate log;

use std::rc::Rc;

pub trait Storage<TData> {
    fn store(&mut self, id: &str, data: TData);
    fn read(&self, id: &str) -> Option<Rc<TData>>;
    fn flush(&mut self);
}
