#![allow(unused)]
use sfml::system::Vector2i;
pub struct Player;
pub struct Rock;
pub struct Name(pub String);
pub struct Age(pub i32);
pub struct TextureString(pub String);
#[derive(Clone, Copy)]
pub struct TilePosition(pub Vector2i);
