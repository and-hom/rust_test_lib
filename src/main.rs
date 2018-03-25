mod api;

use api::Server;
use api::rust_test_lib;
use api::rust_test_lib::memory;

fn main() {
    let storage: Box<api::Storage> = memory::new();
    let server = Server::new(storage).unwrap();
    server.start().unwrap();
}
