use crate::enums::{Player, Rotate, Variant};
use crate::game::Game;

use regex::Regex;
use std::fmt;
use std::mem;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

pub struct Rewriter {
    pub actions: Vec<Action>,
    pub rotation: Rotate,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Action {
    pub from: String,
    pub dest: String,
    pub take: Vec<String>,
    pub won: Option<Player>
}

impl Action {
    pub fn null() -> Self {
        Action {
            from: String::new(),
            dest: String::new(),
            take: Vec::new(),
            won: None
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut take = String::new();
        let mut won = String::new();
        if self.take.len() > 0 {
            take += "x";
            for i in 0..self.take.len() {
                if i != 0 {
                    take += "/";
                }
                take += &self.take[i];
            }
        }
        if self.won.is_some() {
            won = String::from("++");
        }
        write!(f, "{}-{}{}{}", self.from, self.dest, take, won)
    }
}


impl Rewriter {
    pub fn new(game: &Game) -> Self {
        let mut actions : Vec<&str> = game.moves.split('\n').collect();
        let mut result = Vec::new();
        if actions.len() == 0 || game.moves == "" {
            return Self {
                actions: result,
                rotation: Rotate::Zero
            }
        }
        let rotation = Rewriter::rotation(actions.get(0).unwrap());
        for idx in 0..actions.len() {
            let mut act = Rewriter::parse(actions[idx], &rotation);
            if idx == actions.len() - 1 {
                act.won = game.winner.clone();
            }
            result.push(act);
        }
        Self {
            actions: result,
            rotation
        }
    }

    pub fn rotate_action(action: &Action, rotation: &Rotate) -> Action {
        let mut result = action.clone();
        result.from = Self::rotate(result.from, &rotation);
        result.dest = Self::rotate(result.dest, &rotation);
        result
    }

    fn parse(action: &str, rotation: &Rotate) -> Action {
        let re = Regex::new(r"([A-Z]+[0-9]+)-([A-Z]+[0-9]+)x?([A-Z0-9/]*)(\+\+)?$").unwrap();
        let cap = re.captures(action).unwrap();
        let from = String::from(cap.get(1).map_or("", |m| m.as_str()));
        let from = Rewriter::rotate(from, &rotation);
        let dest = String::from(cap.get(2).map_or("", |m| m.as_str()));
        let dest = Rewriter::rotate(dest, &rotation);
        let mut take = Vec::new();
        let takes = String::from(cap.get(3).map_or("", |m| m.as_str()));
        if takes != "++" && takes != "" {
            take = takes.split('/').map(|s| Rewriter::rotate(s.to_string(), &rotation)).collect();
        }

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
            y = size + 1 - x;
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
