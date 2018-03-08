extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;
pub extern crate rust_test_lib;

use self::valico::json_dsl;
use self::rustless::server::status::StatusCode;
use self::rustless::{
    Application, Api, Nesting, Versioning, Client
};
use self::rustless::json::ToJson;
use self::rustless::JsonValue;
use std::error::Error;

pub type Storage = rust_test_lib::Storage<u8>;

pub struct Server {
    storage: Box<Storage>,
    iron: iron::Iron<Application>
}

impl Server {
    pub fn new(storage: Box<Storage>) -> Result<Server, ()> {
        let api = Api::build(|api| {
            api.version("v1", Versioning::Path);

            api.mount(Api::build(|storage_api| {
                storage_api.get(":key", |endpoint| {
                    // Add description
                    endpoint.desc("Get object");

                    // Valico settings for endpoint params
                    endpoint.params(|params| {
                        params.req_typed("key", json_dsl::string())
                    });

                    endpoint.handle(|client, params| {
                        match params.find("key") {
                            None => {
                                client.set_status(StatusCode::BadRequest);
                                Ok(client)
                            }
                            Some(x) => {
                                storage.read(x.as_str().unwrap());
                                Ok(client)
                            }
                        }
                    })
                });
            }));
        });

        let app = Application::new(api);
        let iron = iron::Iron::new(app);
        Ok(Server {
            storage: storage,
            iron: iron,
        })
    }

    pub fn start(self) -> Result<(), ApiError> {
        let _ = try!(self.iron.http("0.0.0.0:4000"));
        println!("Rustless server started!");
        Ok(())
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




