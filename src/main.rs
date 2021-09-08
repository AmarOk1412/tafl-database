mod database;
mod enums;
mod game;
mod rewriter;

extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use crate::database::Database;
use crate::enums::{Variant};
use crate::game::{GameLoader};

fn main() {
    let gl = GameLoader::load();
    let database = Database::new(gl);
    let stats = database.stats(Variant::OldHnefatafl);
    println!("{}", stats);
    let stats = database.stats(Variant::LegacyHnefatafl);
    println!("{}", stats);
}