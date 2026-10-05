use std::{result::Result, str::FromStr};

use hecs::{Entity, World};
use sfml::system::Vector2i;

use crate::states::level::world::components;
#[derive(Debug, PartialEq)]
pub enum Token {
    Action(Action),
    Object(Object),
    Dir(Dir),
    GetFromPrev,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Conjure,
    Move,
    Heat,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
    Rock,
    Lava,
    Player,
    Wall,
    Portal,
}

#[derive(Debug, PartialEq)]
pub enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    pub fn to_vec(&self) -> Vector2i {
        match &self {
            Dir::N => Vector2i::new(0, -1),
            Dir::S => Vector2i::new(0, 1),
            Dir::W => Vector2i::new(-1, 0),
            Dir::E => Vector2i::new(1, 0),
        }
    }
}

impl Object {
    /// Name of the texture used to draw this object in the world.
    pub fn texture_name(&self) -> &'static str {
        match self {
            Object::Rock => "rock",
            Object::Lava => "lava",
            Object::Player => "player",
            Object::Wall => "wall",
            Object::Portal => "portal",
        }
    }

    pub fn on_heat(&self) -> Self {
        match &self {
            Object::Rock => Object::Lava,
            _ => *self,
        }
    }
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "conjure" => Ok(Token::Action(Action::Conjure)),
            "move" => Ok(Token::Action(Action::Move)),
            "heat" => Ok(Token::Action(Action::Heat)),

            "rock" => Ok(Token::Object(Object::Rock)),
            "lava" => Ok(Token::Object(Object::Lava)),
            "player" => Ok(Token::Object(Object::Player)),
            "wall" => Ok(Token::Object(Object::Wall)),

            "pipe" => Ok(Token::GetFromPrev),

            "n" => Ok(Token::Dir(Dir::N)),
            "s" => Ok(Token::Dir(Dir::S)),
            "w" => Ok(Token::Dir(Dir::W)),
            "e" => Ok(Token::Dir(Dir::E)),

            _ => Err(()),
        }
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
