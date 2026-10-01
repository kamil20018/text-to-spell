use hecs::Entity;
use sfml::{
    cpp::FBox,
    graphics::{
        Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderTexture, Shape, Sprite, Transformable,
    },
    system::{Vector2f, Vector2i},
    window::Key,
};

use crate::states::game::constant;

pub mod components;
pub mod serialization;
pub mod spell_parser;
pub mod texture_atlas;
use spell_parser::*;
use texture_atlas::*;

/// Grid line thickness in pixels.
const GRID_LINE_THICKNESS: f32 = 1.0;
const CELL_WIDTH: f32 = 60.0;
const CELL_HEIGHT: f32 = 60.0;

pub struct World {
    pub ecs: hecs::World,
    texture_atlas: TextureAtlas,
    render_texture: FBox<RenderTexture>,
    entity_mappings: EntityMappings,
}

struct EntityMappings {
    player: Entity,
}

impl World {
    pub fn new() -> Self {
        World {
            ecs: hecs::World::new(),
            texture_atlas: TextureAtlas::new(),
            render_texture: RenderTexture::new(constant::SCREEN_W, constant::SCREEN_H).unwrap(),
            entity_mappings: EntityMappings {
                player: Entity::DANGLING,
            },
        }
    }

    pub fn init(&mut self) {
        self.texture_atlas.init();
        self.entity_mappings.player = components::spawn_object(&mut self.ecs, components::Player, Vector2i::new(15, 9));

        components::spawn_object(&mut self.ecs, components::Portal, Vector2i::new(15, 7));

        // components::spawn_object(&mut self.ecs, components::Rock, Vector2i::new(10, 8));
    }

    pub fn process_keystroke(&mut self, key: Key) {
        match key {
            Key::W => {
                self.move_player(Vector2i::new(0, -1));
            }
            Key::S => {
                self.move_player(Vector2i::new(0, 1));
            }
            Key::A => {
                self.move_player(Vector2i::new(-1, 0));
            }
            Key::D => {
                self.move_player(Vector2i::new(1, 0));
            }
            _ => {}
        }
    }

    pub fn move_player(&mut self, distance: Vector2i) {
        for (_, tile_position) in self
            .ecs
            .query_mut::<(&components::Player, &mut components::TilePosition)>()
            .into_iter()
        {
            tile_position.0 += distance;
        }
    }

    pub fn update(&mut self) {
        self.draw();
    }

    fn draw(&mut self) {
        self.render_texture.clear(Color::TRANSPARENT);
        self.draw_grid_lines(Vector2i::new(32, 18));
        self.draw_textures();
        self.render_texture.display();
    }

    fn draw_textures(&mut self) {
        for (tile_pos, texture_string) in self
            .ecs
            .query::<(&components::TilePosition, &components::TextureString)>()
            .iter()
        {
            let texture = self.texture_atlas.get(&texture_string.0).unwrap();
            let size = texture.size();
            let mut sprite = Sprite::with_texture(texture);

            sprite.set_scale((CELL_WIDTH / size.x as f32, CELL_HEIGHT / size.y as f32));

            sprite.set_position(Vector2f::new(
                CELL_WIDTH * tile_pos.0.x as f32,
                CELL_HEIGHT * tile_pos.0.y as f32,
            ));
            self.render_texture.draw(&sprite);
        }
    }

    fn draw_grid_lines(&mut self, grid_size: Vector2i) {
        let size = self.render_texture.size();
        let width = size.x as f32;
        let height = size.y as f32;
        let cell_width = width / grid_size.x as f32;
        let cell_height = height / grid_size.y as f32;

        let target = &mut *self.render_texture;

        let mut column = RectangleShape::with_size(Vector2f::new(GRID_LINE_THICKNESS, height));
        column.set_fill_color(Color::WHITE);
        for col in 1..grid_size.x {
            column.set_position(Vector2f::new(col as f32 * cell_width, 0.0));
            target.draw(&column);
        }

        let mut row = RectangleShape::with_size(Vector2f::new(width, GRID_LINE_THICKNESS));
        row.set_fill_color(Color::WHITE);
        for row_index in 1..grid_size.y {
            row.set_position(Vector2f::new(0.0, row_index as f32 * cell_height));
            target.draw(&row);
        }
    }

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
                    let dir_vec = self.get_vec_from_dir(dir);
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
                        if let Some(entity) = object.get_only_instance(&self.ecs) {
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
                    if let Some(entity) = object.get_only_instance(&self.ecs) {
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
                        self.move_entity(&entity, self.get_vec_from_dir(dir));
                        return Some((entity, object));
                    }
                }

                println!("move: wrong directional token");
                return None;
            }
        }
    }

    pub fn get_vec_from_dir(&self, dir: Dir) -> Vector2i {
        match dir {
            Dir::N => Vector2i::new(0, -1),
            Dir::S => Vector2i::new(0, 1),
            Dir::W => Vector2i::new(-1, 0),
            Dir::E => Vector2i::new(1, 0),
        }
    }

    pub fn spawn_object(&mut self, object: &Object, tile_position: Vector2i) -> Entity {
        match object {
            Object::Rock => self.ecs.spawn((
                components::Rock,
                components::TextureString("rock".to_string()),
                components::TilePosition(tile_position),
            )),
            Object::Lava => self.ecs.spawn((
                components::Lava,
                components::TextureString("lava".to_string()),
                components::TilePosition(tile_position),
            )),
            _ => Entity::DANGLING,
        }
    }

    pub fn heat_object(&mut self, entity: Entity, object: &Object) -> Option<(Entity, Token)> {
        let tile_position = *self.ecs.get::<&components::TilePosition>(entity).unwrap();
        self.ecs.despawn(entity).unwrap();
        let object = object.on_heat();
        let entity = self.spawn_object(&object.on_heat(), tile_position.0);
        Some((entity, Token::Object(object)))
    }

    pub fn move_entity(&mut self, entity: &Entity, vec: Vector2i) {
        self.ecs.get::<&mut components::TilePosition>(*entity).unwrap().0 += vec;
    }

    pub fn get_player_tile_pos(&self) -> Vector2i {
        for (_, tile_position) in self
            .ecs
            .query::<(&components::Player, &mut components::TilePosition)>()
            .into_iter()
        {
            return tile_position.0;
        }
        return Vector2i::new(-1, -1);
    }
}

impl Drawable for World {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut sprite = Sprite::with_texture(self.render_texture.texture());
        sprite.set_position(Vector2f::new(0.0, 0.0));
        target.draw_with_renderstates(&sprite, states);
    }
}
