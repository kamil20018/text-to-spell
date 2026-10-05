#![allow(unused)]
use std::any::type_name;

use hecs::{DynamicBundle, World};
use serde::{Deserialize, Serialize};
use sfml::system::Vector2i;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player;
#[derive(Debug, Serialize, Deserialize)]
pub struct Impassable;
#[derive(Debug, Serialize, Deserialize)]
pub struct Wall;
#[derive(Debug, Serialize, Deserialize)]
pub struct Portal;
#[derive(Debug, Serialize, Deserialize)]
pub struct Rock;
#[derive(Debug, Serialize, Deserialize)]
pub struct Lava;
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureString(pub String);
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct TilePosition(pub Vector2i);
