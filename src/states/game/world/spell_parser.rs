use std::{result::Result, str::FromStr};

use hecs::{Entity, World};

use crate::states::game::world::components;

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

#[derive(Debug, PartialEq)]
pub enum Object {
    Rock,
    Lava,
    Player,
}

#[derive(Debug, PartialEq)]
pub enum Dir {
    N,
    S,
    W,
    E,
}

impl Object {
    pub fn on_heat(&self) -> Self {
        match &self {
            Object::Rock => Object::Lava,
            Object::Lava => Object::Lava,
            Object::Player => Object::Player,
        }
    }

    pub fn get_instances(&self, world: &World) -> Vec<Entity> {
        match self {
            Self::Rock => get_instances::<components::Rock>(world),
            Self::Lava => get_instances::<components::Lava>(world),
            Self::Player => get_instances::<components::Player>(world),
        }
    }

    pub fn count_instances(&self, world: &World) -> usize {
        self.get_instances(world).len()
    }

    pub fn get_only_instance(&self, world: &World) -> Option<Entity> {
        let instances = self.get_instances(world);
        if instances.len() == 1 {
            return Some(instances[0]);
        }
        return None;
    }
}

fn get_instances<T: hecs::Component>(world: &World) -> Vec<Entity> {
    let entities: Vec<Entity> = world.query::<(Entity, &T)>().iter().map(|(entity, _)| entity).collect();
    return entities;
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

            "pipe" => Ok(Token::GetFromPrev),
            // "water" => Ok(Token::Water),
            // "steam" => Ok(Token::Steam),
            "n" => Ok(Token::Dir(Dir::N)),
            "s" => Ok(Token::Dir(Dir::S)),
            "w" => Ok(Token::Dir(Dir::W)),
            "e" => Ok(Token::Dir(Dir::E)),
            // "up" => Ok(Token::Up),
            // "down" => Ok(Token::Down),
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
