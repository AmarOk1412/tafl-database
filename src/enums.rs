use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Variant {
    OldHnefatafl,
    HistoricalHnefatafl,
    BerserkHnefatafl,
    LegacyHnefatafl,
    UNKNOWN
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Player {
    White,
    Black
}

#[derive(Debug, PartialEq)]
pub enum Rotate {
    Zero,
    Ninety,
    OneHundredEighty,
    TwoHundredSeventy
}