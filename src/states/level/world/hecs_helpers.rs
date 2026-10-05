use std::any::type_name;

use hecs::Entity;
use sfml::system::Vector2i;

use super::{World, components, spell_parser::Object};

impl World {
    pub fn spawn_object(&mut self, object: &Object, tile_position: Vector2i) -> Entity {
        match object {
            Object::Rock => self.spawn_entity_with(components::Rock, tile_position, (components::Impassable,)),
            Object::Lava => self.spawn_entity(components::Lava, tile_position),
            Object::Player => self.spawn_entity(components::Player, tile_position),
            Object::Wall => self.spawn_entity_with(components::Wall, tile_position, (components::Impassable,)),
            Object::Portal => self.spawn_entity(components::Portal, tile_position),
        }
    }

    pub fn spawn_entity<T>(&mut self, component: T, position: Vector2i) -> hecs::Entity
    where
        T: hecs::Component,
    {
        let texture_name = type_name::<T>().rsplit("::").next().unwrap().to_lowercase();

        self.ecs.spawn((
            component,
            components::TextureString(texture_name),
            components::TilePosition(position),
        ))
    }

    pub fn spawn_entity_with<T, B>(&mut self, component: T, position: Vector2i, bundle: B) -> hecs::Entity
    where
        T: hecs::Component,
        B: hecs::DynamicBundle,
    {
        let texture_name = type_name::<T>().rsplit("::").next().unwrap().to_lowercase();

        let entity = self.ecs.spawn((
            component,
            components::TextureString(texture_name),
            components::TilePosition(position),
        ));
        self.ecs.insert(entity, bundle).unwrap();
        return entity;
    }

    pub fn despawn_at(&mut self, tile: Vector2i) {
        let entities: Vec<Entity> = self
            .ecs
            .query::<(Entity, &components::TilePosition)>()
            .iter()
            .filter(|(_, position)| position.0 == tile)
            .map(|(entity, _)| entity)
            .collect();

        for entity in entities {
            self.ecs.despawn(entity).unwrap();
        }
    }

    pub fn get_only_object_instance(&self, object: &Object) -> Option<Entity> {
        let instances = self.get_object_instances(object);
        if instances.len() == 1 {
            return Some(instances[0]);
        }
        return None;
    }

    pub fn get_object_instances(&self, object: &Object) -> Vec<Entity> {
        match object {
            Object::Rock => self.get_instances_with::<components::Rock>(),
            Object::Lava => self.get_instances_with::<components::Lava>(),
            Object::Player => self.get_instances_with::<components::Player>(),
            Object::Wall => self.get_instances_with::<components::Wall>(),
            Object::Portal => self.get_instances_with::<components::Portal>(),
        }
    }

    pub fn get_instances_with<T: hecs::Component>(&self) -> Vec<Entity> {
        let entities: Vec<Entity> = self
            .ecs
            .query::<(Entity, &T)>()
            .iter()
            .map(|(entity, _)| entity)
            .collect();
        return entities;
    }

    pub fn move_entity(&mut self, entity: &Entity, vec: Vector2i) {
        self.ecs.get::<&mut components::TilePosition>(*entity).unwrap().0 += vec;
    }
}
