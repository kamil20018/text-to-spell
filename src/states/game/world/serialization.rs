use hecs::{
    EntityRef,
    serialize::row::{self, SerializeContext, try_serialize},
};
use serde::{Deserialize, Serialize, ser::SerializeMap};

use super::{World, components};

/// Generates the [`ComponentId`] tag enum and the [`SerializeContext`] impl for
/// `$context` from a single list of component types.
///
/// Add a component by adding its name to the invocation below (the type must
/// live in [`super::components`]); its `ComponentId` variant and its
/// `try_serialize` call are generated together, so the two cannot drift apart.
///
/// The macro is intentionally local to this file: it expands to imports that
/// only resolve here (the `components` module and the `hecs`/`serde` helpers).
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
    };
}

/// Tells `hecs` which components to write out for each entity.
struct SaveContext;

impl_serialization!(SaveContext;
    Player,
    Portal,
    Rock,
    Lava,
    TilePosition,
    TextureString,
);

/// Serializes every entity in `world` to a JSON string.
pub fn world_to_json(world: &World) -> String {
    let mut output = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut output);
    row::serialize(&world.ecs, &mut SaveContext, &mut serializer).expect("failed to serialize world");
    String::from_utf8(output).expect("serializer produced invalid UTF-8")
}
