#![allow(unused)]
use std::any::type_name;

use hecs::{DynamicBundle, World};
use serde::{Deserialize, Serialize};
use sfml::system::Vector2i;

pub fn spawn_object<T>(world: &mut hecs::World, component: T, position: Vector2i) -> hecs::Entity
where
    T: hecs::Component,
{
    let texture_name = type_name::<T>().rsplit("::").next().unwrap().to_lowercase();

    world.spawn((component, TextureString(texture_name), TilePosition(position)))
}

pub fn spawn_object_with<T, B>(world: &mut hecs::World, component: T, position: Vector2i, bundle: B) -> hecs::Entity
where
    T: hecs::Component,
    B: hecs::DynamicBundle,
{
    let texture_name = type_name::<T>().rsplit("::").next().unwrap().to_lowercase();

    let entity = world.spawn((component, TextureString(texture_name), TilePosition(position)));
    world.spawn_at(entity, bundle);
    return entity;
}

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
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TilePosition(pub Vector2i);
