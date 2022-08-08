extern crate pest;
#[macro_use]
extern crate pest_derive;

use data_types::map::MapItem;
use data_types::vector::VectorItem;

use crate::config::Config;
use crate::data_types::item::Item;
use crate::data_types::primitives::link::Link;
use crate::data_types::primitives::path::PathToValue;
use crate::data_types::primitives::string::StringPrimitive;
use crate::data_types::primitives::Primitive;
use crate::errors::DBError;
use crate::storage::main::Storage;
use crate::tyson::de::Desereilize;
use crate::tyson::map::TySONMap;
use crate::tyson::primitive::TySONPrimitive;
use crate::tyson::vector::TySONVector;
use storage::transaction::Transaction;

mod config;
mod constants;
pub mod data_types;
mod errors;
pub mod query;
pub mod response;
pub mod storage;
pub mod tyson;

pub fn get_storage(path: String) -> Storage {
    Storage::new(path).unwrap()
}

pub fn run() {
    println!("Starting...");
    let config = Config::new();

    let mut storage = Storage::new(config.wh_path).unwrap(); // TODO fix this

    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder
        .bind(format!("tcp://0.0.0.0:{}", config.port).as_str())
        .is_ok());

    let mut msg = zmq::Message::new();

    println!("AnnaDB started at port: {}", config.port);
    loop {
        match responder.recv(&mut msg, 0) {
            Ok(_) => match msg.as_str() {
                Some(msg_value) => {
                    let res = storage.run(msg_value.to_string());
                    // println!("{:?}", &res);
                    responder.send(res.as_str(), 0).unwrap();
                }
                None => {}
            },
            Err(_) => responder.send("Receiving problem", 0).unwrap(),
        }
    }
}
