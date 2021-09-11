use crate::enums::{Player, Variant};
use serde_json::{Result, Value};
use std::fs;

pub struct Game {
    pub id: String,
    pub variant: Variant,
    pub white: String,
    pub black: String,
    pub winner: Option<Player>,
    pub moves: String
}

pub struct GameLoader {
    pub games: Vec<Game>
}

impl GameLoader {
    pub fn load() -> Self {
        let mut games = Vec::new();

        let mut idx = 0;

        for file in fs::read_dir("data/games").unwrap() {
            // TODO protect crash
            let data = fs::read(file.unwrap().path()).unwrap();
            let v: Value = serde_json::from_slice(&data).unwrap();
            games.push(GameLoader::from_json(v));
            idx += 1;
        }

        Self {
            games
        }
    }

    pub fn variant_from_string(variant: String) -> Variant {
        match &*variant {
            "oldhnefatafl" => Variant::OldHnefatafl,
            "historicalhnefatafl" => Variant::HistoricalHnefatafl,
            "legacyhnefatafl" => Variant::LegacyHnefatafl,
            "berserkhnefatafl" => Variant::BerserkHnefatafl,
            _ => Variant::UNKNOWN
        }
    }

    pub fn from_json(v: Value) -> Game {
        let variant = Self::variant_from_string(v["variant"].as_str().unwrap().to_string());

        let winner = match &*v["winner"].as_str().unwrap().to_string() {
            "black" => Some(Player::Black),
            "white" => Some(Player::White),
            _ => None
        };

        Game {
            id: String::new(), // TODO
            variant,
            white: v["white"].as_str().unwrap().to_string(),
            black: v["black"].as_str().unwrap().to_string(),
            winner,
            moves: v["game"].as_str().unwrap().to_string(),
        }
    }
}