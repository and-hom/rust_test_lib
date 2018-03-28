extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;
pub extern crate rust_test_lib;

use self::rustless::server::status::StatusCode;
use self::rustless::{
    Application, Api, Nesting
};
use self::rustless::framework::Client;
use self::rustless::framework::client;
use self::rustless::framework::endpoint;
use self::rustless::errors;
use std::error::Error;
use rust_test_lib::{ReadError, StoreError, RemoveError};

use std::sync::Mutex;

pub struct Server {
    iron: iron::Iron<Application>
}

pub type Storage = rust_test_lib::Storage<u32>;

enum Method {
    GET,
    POST,
    DELETE,
}

impl Server {
    pub fn new(storage: Box<Storage>) -> Result<Server, ()> {
        let api = Api::build(|api| {
            api.mount(Api::build(|storage_api| {
                Server::on_key(storage_api, Method::GET, |key, client| {
                    let mut client = client;
                    let mut _storage: &Mutex<Box<Storage>> = client.app.storage();
                    match _storage.lock().unwrap().read(key) {
                        Err(e) => {
                            client.set_status(http_code_read(&e));
                            client.text(e.description().to_string())
                        }
                        Ok(x) => {
                            client.text(format!("{}", x))
                        }
                    }
                });

                Server::on_key(storage_api, Method::DELETE, |key, client| {
                    let mut client = client;
                    let mut _storage: &Mutex<Box<Storage>> = client.app.storage();
                    match _storage.lock().unwrap().remove(key) {
                        Err(e) => {
                            client.set_status(http_code_remove(&e));
                            client.text(e.description().to_string())
                        }
                        Ok(_) => {
                            client.set_status(StatusCode::Ok);
                            client.text("".to_string())
                        }
                    }
                });

                Server::on_key(storage_api, Method::POST, |key, client: Client| {
                    let mut client = client;
                    let mut _storage: &Mutex<Box<Storage>> = client.app.storage();

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
                                    client.set_status(StatusCode::Ok);
                                    client.text("".to_string())
                                }
                            }
                        }
                    }
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

    fn on_key<A>(api: &mut Api, method: Method, action: A) where
        A: for<'a> Fn(&str, Client<'a>) -> Result<Client<'a>, errors::ErrorResponse> + Send + Sync + 'static
    {
        let callback = |endpoint: &mut endpoint::Endpoint| {
            endpoint.handle(move |mut client, params| {
                match params.find("key") {
                    None => {
                        client.set_status(StatusCode::BadRequest);
                        client.text("key path param is required".to_string())
                    }
                    Some(key_opt) => {
                        let key = key_opt.as_str().unwrap();
                        action(key, client)
                    }
                }
            })
        };

        match method {
            Method::POST => { api.post(":key", callback); }
            Method::GET => { api.get(":key", callback); }
            Method::DELETE => { api.delete(":key", callback); }
        };
    }

    pub fn start(self) -> Result<(), ApiError> {
        let _ = try!(self.iron.http("0.0.0.0:4000"));
        println!("Rustless server started!");
        Ok(())
    }
}


trait Action<'a> {
    fn do_action(&self, key: &str, cliet: &mut Client<'a>) -> client::ClientResult<'a>;
}


fn read_body(c: &mut Client) -> String {
    match c.request.read_to_end() {
        Err(_) => {
            panic!("Can not read request body")
        }
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

fn http_code_remove(e: &RemoveError) -> StatusCode {
    match e {
        &RemoveError::MISSING(_) => StatusCode::NotFound,
        &RemoveError::IO(_) => StatusCode::InternalServerError,
        &RemoveError::INTERNAL(_) => StatusCode::InternalServerError,
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




