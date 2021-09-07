pub enum Variant {
    OldHnefatafl
}

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