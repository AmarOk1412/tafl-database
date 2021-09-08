use crate::rewriter::{Action, Rewriter};
use crate::game::{Game, GameLoader};
use crate::enums::{Variant};

use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::fs;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Node {
    next: Vec<Node>,
    action: Action,
}

pub struct Database {
    possibilities: Vec<(Variant, Node)>
}

impl Database {
    pub fn new(game_loader : GameLoader) -> Database {
        let mut db = Database {
            possibilities: Vec::new()
        };
        // TODO when API will be there, instead data/games, load from saved result db.load();
        for game in &game_loader.games {
            db.add_game(game);
        }
        db.save();
        db
    }

    fn add_game(&mut self, game: &Game) {
        let variant = game.variant.clone();
        let game = Rewriter::new(game);
        let game = Database::to_tree(game);
        let position = self.possibilities.iter().position(|po| po.0 == variant && po.1.action == game.action);
        if position.is_none() {
            self.possibilities.push((variant, game));
            return;
        }

        let mut current_game_node = &game;
        let mut current_node = &mut self.possibilities[position.unwrap()].1;

        loop {
            if current_game_node.next.len() == 0 {
                // TODO same game occurence!
                return;
            }
            let position = current_node.next.iter().position(|po| po.action == current_game_node.next[0].action);
            current_game_node = &current_game_node.next[0];
            if position.is_none() {
                current_node.next.push(current_game_node.clone());
                return;
            }
            current_node = &mut current_node.next[position.unwrap()];
        }
    }

    fn to_tree(game: Vec<Action>) -> Node {
        let mut result = Node {
            next: Vec::new(),
            action: Action::null(),
        };
        let mut current_node = &mut result;
        for i in 0..game.len() {
            current_node.action = game[i].clone();
            if i != game.len() - 1 {
                current_node.next.push(Node {
                    next: Vec::new(),
                    action: Action::null(),
                });
                current_node = &mut current_node.next[0];
            }
        }
        result
    }

    fn save(&self) {
        for possibility in &self.possibilities {
            let mut buf = Vec::new();
            possibility.1.serialize(&mut Serializer::new(&mut buf)).unwrap();
            let dir = format!("data/{}", possibility.0);
            let path = format!("{}/{}_{}.pk", dir, possibility.1.action.from, possibility.1.action.dest);
            fs::create_dir_all(dir);
            fs::write(path, buf);
        }
    }

    pub fn stats(&self, variant: Variant) -> String {
        let mut result = String::new();
        let mut coups: Vec<(usize, String)> = Vec::new();
        // TODO victories
        for possibility in &self.possibilities {
            if possibility.0 == variant {
                let used = Database::tree_size(&possibility.1);
                let id = format!("{}_{}", possibility.1.action.from, possibility.1.action.dest);
                coups.push((used, id));
            }
        }
        coups.sort_by_key(|k| k.0);
        result += &*format!("For variant: {}\n", variant);
        for coup in coups {
            result += &*format!("{}, occurence: {}\n", coup.1, coup.0);
        }
        result
    }

    fn tree_size(node: &Node) -> usize {
        if node.next.is_empty() {
            return 1;
        }
        let mut result = 0;
        for n in &node.next {
            result += Database::tree_size(n);
        }
        result
    }

    /*fn load(&mut self) {
        for file in fs::read_dir("data").unwrap() {
            let buf = fs::read(file.unwrap().path()).unwrap();
            let cur = Cursor::new(&buf[..]);
            let mut de = Deserializer::new(cur);
            self.possibilities.push(Deserialize::deserialize(&mut de).unwrap());
        }
    }*/
}