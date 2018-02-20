pub mod memory;
pub mod disk;

#[cfg(test)]
pub mod test;

#[macro_use]
extern crate log;

use std::rc::Rc;

pub trait Storage<TData> {
    fn store(&mut self, id: &str, data: &TData);
    fn read(&self, id: &str) -> Option<Rc<TData>>;
    fn clear(&mut self);
}
