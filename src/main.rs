mod api;

use api::Server;
use api::rust_test_lib;
use api::rust_test_lib::disk;

fn main() {
    let storage: Box<api::Storage> = disk::new("/tmp/storage");
    let server = Server::new(storage).unwrap();
    server.start().unwrap();
}
