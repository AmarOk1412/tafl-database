mod database;
mod enums;
mod rewriter;

extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use crate::database::Database;
use crate::rewriter::Rewriter;
use crate::enums::{Variant, Player};

fn main() {
    let game = String::from("J6-J10\n\
H6-J6\n\
H1-I1\n\
J6-J1\n\
K4-J4\n\
G6-H6\n\
I1-H1\n\
G5-I5\n\
H1-I1xJ1\n\
H6-H1\n\
F2-J2\n\
F6-I6\n\
H11-I11\n\
I6-I9\n\
D11-D9\n\
E7-C7\n\
F10-B10\n\
I9-E9\n\
A8-C8\n\
E9-E10\n\
E11-C11\n\
E10-C10\n\
D9-C9\n\
C10-H10\n\
B6-C6xC7\n\
H10-H3\n\
D1-D3\n\
H3-K3\n\
J2-K2\n\
G7-G3\n\
F1-F2\n\
K3-K4\n\
K2-K3xK4++");
    let winner = Player::Black; // Amarok. other ardor
    let game = Rewriter::new(Variant::OldHnefatafl, game);
    let mut games = Vec::new();
    //games.push(game);
    Database::new(games);
}

