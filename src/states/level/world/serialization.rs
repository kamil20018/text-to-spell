use hecs::{
    EntityBuilder, EntityRef,
    serialize::row::{self, DeserializeContext, SerializeContext, try_serialize},
};
use serde::{Deserialize, Serialize, de::MapAccess, ser::SerializeMap};

use super::{World, components};

/// Generates the [`ComponentId`] tag enum plus the [`SerializeContext`] and
/// [`DeserializeContext`] impls for `$context` from a single list of component
/// types.
///
/// Add a component by adding its name to the invocation below (the type must
/// live in [`super::components`]); its `ComponentId` variant, its save entry and
/// its load entry are generated together, so the three cannot drift apart.
///
/// The macro is intentionally local to this file: it expands to names that only
/// resolve here (the `components` module and the `hecs`/`serde` helpers).
macro_rules! impl_serialization {
    ($context:ty; $($component:ident),+ $(,)?) => {
        /// Tag written next to every component so a reader can tell them apart.
        #[derive(Debug, Serialize, Deserialize)]
        enum ComponentId {
            $($component,)+
        }

        impl SerializeContext for $context {
            fn serialize_entity<S>(
                &mut self,
                entity: EntityRef<'_>,
                mut map: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: SerializeMap,
            {
                $(
                    try_serialize::<components::$component, _, _>(
                        &entity,
                        &ComponentId::$component,
                        &mut map,
                    )?;
                )+
                map.end()
            }
        }

        impl DeserializeContext for $context {
            fn deserialize_entity<'de, M>(
                &mut self,
                mut map: M,
                entity: &mut EntityBuilder,
            ) -> Result<(), M::Error>
            where
                M: MapAccess<'de>,
            {
                while let Some(key) = map.next_key::<ComponentId>()? {
                    match key {
                        $(
                            ComponentId::$component => {
                                entity.add::<components::$component>(map.next_value()?);
                            }
                        )+
                    }
                }
                Ok(())
            }
        }
    };
}

/// Reads and writes the world's entities in `hecs`'s human-friendly row format.
struct WorldContext;

impl_serialization!(WorldContext;
    Player,
    Portal,
    Rock,
    Lava,
    TilePosition,
    TextureString,
    Impassable,
    Wall,
);

/// Serializes every entity in `world` to a JSON string.
pub fn world_to_json(world: &World) -> String {
    let mut output = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut output);
    row::serialize(&world.ecs, &mut WorldContext, &mut serializer).expect("failed to serialize world");
    String::from_utf8(output).expect("serializer produced invalid UTF-8")
}

/// Deserializes a `hecs` world from a JSON string in the format produced by
/// [`world_to_json`].
pub fn world_from_json(json: &str) -> Result<hecs::World, LoadError> {
    let mut deserializer = serde_json::Deserializer::from_str(json);
    Ok(row::deserialize(&mut WorldContext, &mut deserializer)?)
}

/// Reads `path` and deserializes a `hecs` world from it.
pub fn world_from_file(path: &str) -> Result<hecs::World, LoadError> {
    let json = std::fs::read_to_string(path)?;
    world_from_json(&json)
}

/// Errors that can happen while loading a saved world.
#[derive(Debug)]
pub enum LoadError {
    /// The save file could not be read.
    Io(std::io::Error),
    /// The save file was not valid JSON for the expected world shape.
    Json(serde_json::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(error) => write!(f, "could not read save file: {error}"),
            LoadError::Json(error) => write!(f, "could not parse save file: {error}"),
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::Io(error) => Some(error),
            LoadError::Json(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for LoadError {
    fn from(error: std::io::Error) -> Self {
        LoadError::Io(error)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(error: serde_json::Error) -> Self {
        LoadError::Json(error)
    }
}
