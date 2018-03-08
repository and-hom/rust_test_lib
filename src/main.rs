mod api;

use api::Server;
use api::rust_test_lib;
use api::rust_test_lib::memory;

fn main() {
    let storage: Box<rust_test_lib::Storage<u8>> = memory::new();
    let server = Server::new(storage).unwrap();
    server.start().unwrap();
}
