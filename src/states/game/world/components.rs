#![allow(unused)]
use std::any::type_name;

use hecs::World;
use sfml::system::Vector2i;

pub fn spawn_object<T>(world: &mut hecs::World, component: T, position: Vector2i) -> hecs::Entity
where
    T: hecs::Component,
{
    let texture_name = type_name::<T>().rsplit("::").next().unwrap().to_lowercase();

    world.spawn((component, TextureString(texture_name), TilePosition(position)))
}

pub struct Player;
pub struct Portal;
pub struct Rock;
pub struct Lava;
pub struct Name(pub String);
pub struct Age(pub i32);
pub struct TextureString(pub String);
#[derive(Clone, Copy)]
pub struct TilePosition(pub Vector2i);
