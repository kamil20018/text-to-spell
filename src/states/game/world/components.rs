#![allow(unused)]
use std::any::type_name;

use serde::{Deserialize, Serialize};
use hecs::World;
use sfml::system::Vector2i;

pub fn spawn_object<T>(world: &mut hecs::World, component: T, position: Vector2i) -> hecs::Entity
where
    T: hecs::Component,
{
    let texture_name = type_name::<T>().rsplit("::").next().unwrap().to_lowercase();

    world.spawn((component, TextureString(texture_name), TilePosition(position)))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Player;
#[derive(Debug, Serialize, Deserialize)]
pub struct Portal;
#[derive(Debug, Serialize, Deserialize)]
pub struct Rock;
#[derive(Debug, Serialize, Deserialize)]
pub struct Lava;
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureString(pub String);
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TilePosition(pub Vector2i);
