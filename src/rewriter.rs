use crate::enums::{Player, Rotate, Variant};
use crate::game::Game;

use regex::Regex;
use std::mem;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

pub struct Rewriter {

}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Action {
    pub from: String,
    pub dest: String,
    pub take: String,
    pub won: Option<Player>
}

impl Action {
    pub fn null() -> Self {
        Action {
            from: String::new(),
            dest: String::new(),
            take: String::new(),
            won: None
        }
    }
}

impl Rewriter {
    pub fn new(game: &Game) -> Vec<Action> {
        let mut actions : Vec<&str> = game.moves.split('\n').collect();
        let mut result = Vec::new();
        if actions.len() == 0 {
            return result;
        }
        let rotation = Rewriter::rotation(actions.get(0).unwrap());
        for idx in 0..actions.len() {
            let mut act = Rewriter::parse(actions[0], &rotation);
            if idx== actions.len() - 1 {
                act.won = Some(game.winner.clone());
            }
            result.push(act);
        }
        result
    }

    fn parse(action: &str, rotation: &Rotate) -> Action {
        // TODO 3.json
        let re = Regex::new(r"([A-Z]+[0-9]+)-([A-Z]+[0-9]+)x?([A-Z]+[0-9]+)?(\+\+)?$").unwrap();
        let cap = re.captures(action).unwrap();
        let from = String::from(cap.get(1).map_or("", |m| m.as_str()));
        let from = Rewriter::rotate(from, &rotation);
        let dest = String::from(cap.get(2).map_or("", |m| m.as_str()));
        let dest = Rewriter::rotate(dest, &rotation);
        let mut take = String::from(cap.get(3).map_or("", |m| m.as_str()));
        if take.len() != 0 {
            take = Rewriter::rotate(take, &rotation);
        }
        //let won = cap.get(4).map_or("", |m| m.as_str()) == "++";
        Action {
            from,
            dest,
            take,
            won: None
        }
    }

    fn rotation(action: &str) -> Rotate {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let action = Rewriter::parse(action, &Rotate::Zero);
        let x = alphabet.find(action.from.chars().nth(0).unwrap()).unwrap() + 1;
        let y = action.from[1..].parse::<usize>().unwrap();
        let mid = 6; // TODO from variant
        // TODO G2_C2????
        if x <= mid && y < mid {
            return Rotate::Zero;
        } else if x > mid && y <= mid {
            return Rotate::Ninety;
        } else if x >= mid && y > mid {
            return Rotate::OneHundredEighty;
        }
        return Rotate::TwoHundredSeventy;
    }

    fn rotate(square: String, rotation: &Rotate) -> String {
        if rotation == &Rotate::Zero {
            return square;
        }
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut x = alphabet.find(square.chars().nth(0).unwrap()).unwrap() + 1;
        let mut y = square[1..].parse::<usize>().unwrap();

        let mut result = String::new();
        let mut size = 11; // TODO variant
        if rotation != &Rotate::OneHundredEighty {
            let fx = y;
            y = size - x;
            x = fx;
        }
        if rotation != &Rotate::Ninety {
            x = size + 1 - x;
            y = size + 1 - y;
        }
        result.push_str(&alphabet.chars().nth(x-1).unwrap().to_string());
        result.push_str(&y.to_string());
        return result;
    }
}
