use hecs::{Entity, Query};
use sfml::{
    cpp::FBox,
    graphics::{
        Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderTexture, Shape, Sprite, Transformable,
    },
    system::{Vector2f, Vector2i},
    window::Key,
};

use crate::game::{constant, world::components::*};

pub mod components;
pub mod spell_parser;
pub mod texture_atlas;
use spell_parser::Token;
use texture_atlas::*;

/// Grid line thickness in pixels.
const GRID_LINE_THICKNESS: f32 = 1.0;
const CELL_WIDTH: f32 = 60.0;
const CELL_HEIGHT: f32 = 60.0;

pub struct World {
    ecs: hecs::World,
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
        let player = self.ecs.spawn((
            Player,
            TextureString("player".to_string()),
            TilePosition(Vector2i::new(5, 7)),
        ));
        self.entity_mappings.player = player;

        self.ecs.spawn((
            Rock,
            TextureString("stone".to_string()),
            TilePosition(Vector2i::new(10, 7)),
        ));
        self.ecs.spawn((
            Rock,
            TextureString("stone".to_string()),
            TilePosition(Vector2i::new(7, 2)),
        ));
        self.ecs.spawn((
            Rock,
            TextureString("stone".to_string()),
            TilePosition(Vector2i::new(3, 3)),
        ));
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
        for (_, tile_position) in self.ecs.query_mut::<(&Player, &mut TilePosition)>().into_iter() {
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
        for (tile_pos, texture_string) in self.ecs.query::<(&TilePosition, &TextureString)>().iter() {
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
        let mut sentences: Vec<Vec<Token>> = spell_parser::parse_spell(spell_text);

        println!("{:?}", sentences);
        for sentence in &mut sentences {
            while !sentence.is_empty() {
                let token = sentence.remove(0);
                match token {
                    Token::Conjure => {
                        let conjure_type = sentence.remove(0);
                        let conjure_location = sentence.remove(0);

                        let dir = match conjure_location {
                            Token::N => Vector2i::new(0, -1),
                            Token::S => Vector2i::new(0, 1),
                            Token::W => Vector2i::new(-1, 0),
                            Token::E => Vector2i::new(1,0 ),
                            _ => Vector2i::new(0,0 ),
                        };
                        match conjure_type {
                            Token::Rock => {
                                let _ = self.ecs.spawn((
                                    Rock,
                                    TextureString("stone".to_string()),
                                    TilePosition(self.get_player_tile_pos()+ dir),
                                ));
                            }
                            _ => {}
                        }
                    }
                    Token::Heat => {
                        println!("heat");
                    }
                    Token::Move => {
                        println!("move");
                    }
                    _ => {
                        println!("bad spell");
                        return;
                    }
                }
            }
        }
    }

    pub fn get_player_tile_pos(&self) -> Vector2i {
        for (_, tile_position) in self.ecs.query::<(&Player, &mut TilePosition)>().into_iter() {
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
