use crate::rewriter::{Action, Rewriter};
use crate::game::{Game, GameLoader};
use crate::enums::{Player, Rotate, Variant};

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

    pub fn add_game(&mut self, game: &Game) {
        let variant = game.variant.clone();
        let rw = Rewriter::new(game);
        let game = Database::to_tree(rw.actions);
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

    pub fn save(&self) {
        for possibility in &self.possibilities {
            let mut buf = Vec::new();
            possibility.1.serialize(&mut Serializer::new(&mut buf)).unwrap();
            let dir = format!("data/out/{}", possibility.0);
            let path = format!("{}/{}_{}.pk", dir, possibility.1.action.from, possibility.1.action.dest);
            fs::create_dir_all(dir);
            fs::write(path, buf);
        }
    }

    pub fn search(&mut self, variant: Variant, moves: String) -> String {
        let game = Game {
            id: String::new(),
            variant: variant.clone(),
            white: String::new(),
            black: String::new(),
            winner: None,
            moves
        };
        let rw = Rewriter::new(&game);
        let invert_rotate = match rw.rotation {
            Rotate::Zero => Rotate::Zero,
            Rotate::Ninety => Rotate::TwoHundredSeventy,
            Rotate::TwoHundredSeventy => Rotate::Ninety,
            Rotate::OneHundredEighty => Rotate::OneHundredEighty,
        };
        let mut game = Database::to_tree(rw.actions);

        for possibility in &mut self.possibilities {
            if possibility.0 == variant && possibility.1.action == game.action {
                let mut current_node = &mut game;
                let mut current_search_node = &mut possibility.1;
                loop {
                    let mut continue_search = false;
                    if current_node.next.len() == 0 {
                        break;
                    }
                    for n in 0..current_search_node.next.len() {
                        if current_search_node.next[n].action == current_node.next[0].action {
                            continue_search = true;
                            current_search_node = &mut current_search_node.next[n];
                            current_node = &mut current_node.next[0];
                            break;
                        }
                    }
                    if !continue_search {
                        break;
                    }
                }

                if current_node.next.len() == 0 {
                    // Search ended
                    //TODO construct json object via serde?
                    let mut result = String::from("[");
                    for next_coup in &current_search_node.next {
                        let stats = Database::tree_stats(&next_coup);
                        let used = stats.len();
                        let white_victory = stats.iter().filter(|&p| *p == Player::White).count();
                        let black_victory = stats.iter().filter(|&p| *p == Player::Black).count();
                        let percentw = white_victory as f32 / used as f32;
                        let percentb = black_victory as f32 / used as f32;
                        let rotated = Rewriter::rotate_action(&next_coup.action, &invert_rotate);
                        let id = format!("{}-{}", rotated.from, rotated.dest);
                        result += &*format!("{{\"id\":\"{}\",\"count\":{},\"whiteWin\":{},\"blackWin\":{} }},", id, used, percentw, percentb);
                    }
                    if result != "[" {
                        result.pop();
                    }
                    result += "]";
                    return result;
                }
            }
        }
        String::from("{}")
    }

    pub fn stats(&self, variant: Variant) -> String {
        let mut result = String::new();
        let mut coups: Vec<(usize, String, f32, f32)> = Vec::new();
        // TODO victories
        for possibility in &self.possibilities {
            if possibility.0 == variant {
                let stats = Database::tree_stats(&possibility.1);
                let used = stats.len();
                let white_victory = stats.iter().filter(|&p| *p == Player::White).count();
                let black_victory = stats.iter().filter(|&p| *p == Player::Black).count();
                let percentw = white_victory as f32 / used as f32;
                let percentb = black_victory as f32 / used as f32;
                let id = format!("{}_{}", possibility.1.action.from, possibility.1.action.dest);
                coups.push((used, id, percentw, percentb));
            }
        }
        coups.sort_by_key(|k| k.0);
        result += &*format!("For variant: {}\n", variant);
        for coup in coups {
            result += &*format!("{}, occurence: {} (victory: white {} / black {})\n", coup.1, coup.0, coup.2, coup.3);
        }
        result
    }

    fn tree_stats(node: &Node) -> Vec<Player> {
        let mut result : Vec<Player> = Vec::new();
        if node.next.is_empty() {
            if node.action.won.is_some(){
                result.push(node.action.won.clone().unwrap());
            }
            return result;
        }
        for n in &node.next {
            result.append(&mut Database::tree_stats(n));
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