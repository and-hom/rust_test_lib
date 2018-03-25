extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;
pub extern crate rust_test_lib;

use self::valico::json_dsl;
use self::rustless::server::status::StatusCode;
use self::rustless::{
    Application, Api, Nesting
};
use std::error::Error;
use rust_test_lib::{StoreError, ReadError};

use std::sync::Mutex;

use self::rustless::framework::Client;

pub type Storage = rust_test_lib::Storage<u32>;

pub struct Server {
    iron: iron::Iron<Application>
}

impl Server {
    pub fn new(storage: Box<Storage>) -> Result<Server, ()> {
        let api = Api::build(|api| {
            //            api.version("v1", Versioning::Path);

            api.mount(Api::build(|storage_api| {
                storage_api.get(":key", |endpoint| {
                    // Add description
                    endpoint.desc("Get object");

                    // Valico settings for endpoint params
                    endpoint.params(|params| {
                        params.req_typed("key", json_dsl::string())
                    });

                    endpoint.handle(|mut client, params| {
                        match params.find("key") {
                            None => {
                                client.set_status(StatusCode::BadRequest);
                                Ok(client)
                            }
                            Some(x) => {
                                let _storage: &Mutex<Box<Storage>> = client.app.storage();
                                let key = x.as_str().unwrap();
                                match _storage.lock().unwrap().read(key) {
                                    Err(e) => {
                                        client.set_status(http_code_read(&e));
                                        client.text(e.description().to_string())
                                    }
                                    Ok(x) => {
                                        client.text(format!("{}", x))
                                    }
                                }
                            }
                        }
                    })
                });
                storage_api.post(":key", |endpoint| {
                    // Add description
                    endpoint.desc("Put object");

                    // Valico settings for endpoint params
                    endpoint.params(|params| {
                        params.req_typed("key", json_dsl::string())
                    });

                    endpoint.handle(|mut client, params| {
                        match params.find("key") {
                            None => {
                                client.set_status(StatusCode::BadRequest);
                                Ok(client)
                            }
                            Some(x) => {
                                let mut _storage: &Mutex<Box<Storage>> = client.app.storage();
                                let key = x.as_str().unwrap();

                                let body_str = read_body(&mut client);
                                match u32::from_str_radix(body_str.as_str(), 10) {
                                    Err(e) => {
                                        client.set_status(StatusCode::BadRequest);
                                        client.text(e.description().to_string())
                                    }
                                    Ok(value) => {
                                        match _storage.lock().unwrap().store(key, &value) {
                                            Err(e) => {
                                                client.set_status(http_code_store(&e));
                                                client.text(e.description().to_string())
                                            }
                                            Ok(_) => {
                                                Ok(client)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })
                });
            }));
        });

        let mut app = Application::new(api);
        app.ext.insert::<AppStorage>(Mutex::new(storage));
        let iron = iron::Iron::new(app);
        Ok(Server {
            iron: iron,
        })
    }

    pub fn start(self) -> Result<(), ApiError> {
        let _ = try!(self.iron.http("0.0.0.0:4000"));
        println!("Rustless server started!");
        Ok(())
    }
}

fn read_body(c: &mut Client) -> String {
    match c.request.read_to_end() {
        Err(_) => {
            panic!("Can not read request body")
        },
        Ok(x) => x.unwrap_or("".to_string())
    }
}


pub struct AppStorage;

impl iron::typemap::Key for AppStorage {
    type Value = Mutex<Box<Storage>>;
}

pub trait StorageExt: rustless::Extensible {
    fn storage(&self) -> &Mutex<Box<Storage>>;
}

impl StorageExt for rustless::Application {
    fn storage(&self) -> &Mutex<Box<Storage>> {
        self.ext.get::<AppStorage>().unwrap()
    }
}

#[derive(Debug)]
pub struct ApiError {
    desc: String
}

impl From<hyper::Error> for ApiError {
    fn from(e: hyper::Error) -> Self {
        ApiError {
            desc: e.description().to_string()
        }
    }
}

fn http_code_read(e: &ReadError) -> StatusCode {
    match e {
        &ReadError::MISSING(_) => StatusCode::NotFound,
        &ReadError::IO(_) => StatusCode::InternalServerError,
        &ReadError::INTERNAL(_) => StatusCode::InternalServerError,
    }
}

fn http_code_store(e: &StoreError) -> StatusCode {
    match e {
        &StoreError::IO(_) => StatusCode::InternalServerError,
        &StoreError::INTERNAL(_) => StatusCode::InternalServerError,
    }
}




