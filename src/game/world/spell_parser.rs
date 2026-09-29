use std::{result::Result, str::FromStr};

#[derive(Debug)]
enum Token {
    Conjure,
    Move,
    Heat,

    Rock,
    Lava,
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

fn tokenize(spell_text: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for word in spell_text.split_whitespace() {
        // println!("{:?}", Token::from_str(word));
        if let Ok(token) = Token::from_str(word) {
            tokens.push(token);
        }
    }
    tokens
}

pub fn parse_spell(spell_text: &String) {
    let mut tokens = tokenize(spell_text);

    while !tokens.is_empty() {
        let token = tokens.remove(0);
        match token {
            Token::Conjure => {}
            Token::Heat => {}
            Token::Move => {}
            _ => {
                println!("bad spell");
                return;
            }
        }
    }
}
