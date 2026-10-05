use hecs::Entity;
use sfml::system::Vector2i;

use super::{
    World, components,
    spell_parser::{self, Action, Object, Token},
};

impl World {
    pub fn cast_spell(&mut self, spell_text: &String) {
        let verses: Vec<Vec<Token>> = spell_parser::parse_spell(spell_text);

        println!("{:?}", verses);

        let mut passed_object: Option<(Entity, Token)> = None;

        for verse in verses {
            passed_object = self.cast_verse(verse, passed_object);
        }
    }

    pub fn cast_verse(
        &mut self,
        mut verse: Vec<Token>,
        passed_object: Option<(Entity, Token)>,
    ) -> Option<(Entity, Token)> {
        let token = verse.remove(0);
        let Token::Action(action) = token else {
            panic!("Expected an Action token");
        };

        match action {
            Action::Conjure => {
                if verse.len() < 2 {
                    println!("conjure: missing tokens");
                    return None;
                }

                let conjure_type = verse.remove(0);
                let conjure_location = verse.remove(0);

                if verse.len() > 0 {
                    println!("conjure: too many tokens");
                    return None;
                }

                if let Token::Dir(dir) = conjure_location {
                    let dir_vec = dir.to_vec();
                    let tile_position = self.get_player_tile_pos() + dir_vec;
                    match conjure_type {
                        Token::GetFromPrev => {
                            if let Some((_, Token::Object(object))) = passed_object {
                                let entity = self.spawn_object(&object, tile_position);
                                return Some((entity, Token::Object(object)));
                            }
                            return None;
                        }
                        Token::Object(object) => {
                            let entity = self.spawn_object(&object, tile_position);
                            return Some((entity, Token::Object(object)));
                        }
                        _ => return None,
                    };
                }

                println!("conjure: wrong location token");

                return None;
            }
            Action::Heat => {
                if verse.len() < 1 {
                    println!("missing tokens in heat");
                    return None;
                }

                let heat_target = verse.remove(0);

                if verse.len() > 0 {
                    println!("heat: too many tokens");
                    return None;
                }

                match heat_target {
                    Token::Object(object) => {
                        if let Some(entity) = self.get_only_object_instance(&object) {
                            return self.heat_object(entity, &object);
                        } else {
                            println!("heat: there are less or more than 0 instances in the world")
                        }
                    }
                    Token::GetFromPrev => {
                        if let Some((entity, Token::Object(object))) = passed_object {
                            return self.heat_object(entity, &object);
                        }
                    }
                    _ => {
                        println!("heat: wrong target");
                    }
                }

                println!("heat requires piped entity");
                return None;
            }
            Action::Move => {
                if verse.len() < 2 {
                    println!("move: missing tokens");
                    return None;
                }

                let move_target = verse.remove(0);
                let dir = verse.remove(0);

                if verse.len() > 0 {
                    println!("move: too many tokens");
                    return None;
                }

                let mut entity_to_move: Option<Entity> = None;
                let mut return_object: Option<Token> = None;

                if let Token::GetFromPrev = move_target {
                    if let Some((entity, object)) = passed_object {
                        entity_to_move = Some(entity);
                        return_object = Some(object);
                    } else {
                        println!("move: missing passed_object");
                    }
                } else if let Token::Object(object) = move_target {
                    if let Some(entity) = self.get_only_object_instance(&object) {
                        entity_to_move = Some(entity);
                        return_object = Some(Token::Object(object));
                    } else {
                        println!("move: there are less or more than 0 instances in the world")
                    }
                } else {
                    println!("move: wrong move target");
                }

                if let Token::Dir(dir) = dir {
                    if let Some(entity) = entity_to_move
                        && let Some(object) = return_object
                    {
                        self.move_entity(&entity, dir.to_vec());
                        return Some((entity, object));
                    }
                }

                println!("move: wrong directional token");
                return None;
            }
        }
    }

    pub fn heat_object(&mut self, entity: Entity, object: &Object) -> Option<(Entity, Token)> {
        let tile_position = *self.ecs.get::<&components::TilePosition>(entity).unwrap();
        self.ecs.despawn(entity).unwrap();
        let object = object.on_heat();
        let entity = self.spawn_object(&object.on_heat(), tile_position.0);
        Some((entity, Token::Object(object)))
    }

    pub fn get_player_tile_pos(&self) -> Vector2i {
        for (_, tile_position) in self
            .ecs
            .query::<(&components::Player, &components::TilePosition)>()
            .into_iter()
        {
            return tile_position.0;
        }
        Vector2i::new(-1, -1)
    }
}
