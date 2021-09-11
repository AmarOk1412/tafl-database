mod api;
mod database;
mod enums;
mod game;
mod rewriter;

extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use crate::database::Database;
use crate::enums::{Variant};
use crate::game::{GameLoader};
use crate::api::API;

use std::sync::{Arc, Mutex};

fn main() {
    let gl = GameLoader::load();
    let database = Database::new(gl);
    let stats = database.stats(Variant::OldHnefatafl);
    println!("{}", stats);
    let stats = database.stats(Variant::LegacyHnefatafl);
    println!("{}", stats);

    let mut api = API::new(Arc::new(Mutex::new(database)),
                           String::from("0.0.0.0:1412")
                        );
    api.start();
}