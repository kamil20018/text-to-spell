use std::{result::Result, str::FromStr};

#[derive(Debug)]
pub enum Token {
    Conjure,
    Move,
    Heat,

    Rock,
    Lava,
    Player,
    // Water,
    // Steam,
    N,
    S,
    W,
    E,
    // Up,
    // Down,
}



impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "conjure" => Ok(Token::Conjure),
            "move" => Ok(Token::Move),
            "heat" => Ok(Token::Heat),

            "rock" => Ok(Token::Rock),
            "lava" => Ok(Token::Lava),
            "player" => Ok(Token::Player),
            // "water" => Ok(Token::Water),
            // "steam" => Ok(Token::Steam),
            "n" => Ok(Token::N),
            "s" => Ok(Token::S),
            "w" => Ok(Token::W),
            "e" => Ok(Token::E),
            // "up" => Ok(Token::Up),
            // "down" => Ok(Token::Down),
            _ => Err(()),
        }
    }
}

impl Token {
    fn is_thing(&self) -> bool {
        matches!(&self, Token::Rock | Token::Lava)
    }
}

pub fn parse_spell(spell_text: &String) -> Vec<Vec<Token>> {
    let mut tokens: Vec<Vec<Token>> = Vec::new();

    for sentence in spell_text.split(",") {
        let mut sentence_vec: Vec<Token> = Vec::new();
        for word in sentence.split_whitespace() {
            if let Ok(token) = Token::from_str(word) {
                sentence_vec.push(token);
            }
        }
        tokens.push(sentence_vec);
    }

    tokens
}
