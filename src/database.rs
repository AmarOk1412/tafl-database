use crate::rewriter::Action;

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
    possibilities: Vec<Node>
}

impl Database {
    pub fn new(games : Vec<Vec<Action>>) {
        let mut db = Database {
            possibilities: Vec::new()
        };
        db.load();
        for game in games {
            db.add_game(game);
        }
        db.save()
    }

    fn add_game(&mut self, game: Vec<Action>) {
        let game = Database::to_tree(game);
        let position = self.possibilities.iter().position(|po| po.action == game.action);
        if position.is_none() {
            self.possibilities.push(game);
            return;
        }

        let mut current_game_node = &game;
        let mut current_node = &mut self.possibilities[position.unwrap()];

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
            possibility.serialize(&mut Serializer::new(&mut buf)).unwrap();
            let path = format!("data/oldtafl_{}_{}", possibility.action.from, possibility.action.dest);
            fs::write(path, buf);
        }
    }

    fn load(&mut self) {
        for file in fs::read_dir("data").unwrap() {
            let buf = fs::read(file.unwrap().path()).unwrap();
            let cur = Cursor::new(&buf[..]);
            let mut de = Deserializer::new(cur);
            self.possibilities.push(Deserialize::deserialize(&mut de).unwrap());
        }
    }
}