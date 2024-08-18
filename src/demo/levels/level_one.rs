use bevy::{ecs::world::Command, prelude::*};
// TODO: figure out how to fix the import here if at all possible
use super::super::custom_entities::{get_entity_to_spawn, CustomEntities, CustomEntityComponent};
pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug)]
pub struct SpawnLevelOne;

impl Command for SpawnLevelOne {
    fn apply(self, world: &mut World) {
        world.commands().spawn_batch(create_level_entities());
        world
            .commands()
            .spawn(get_entity_to_spawn(CustomEntities::Block32Px));
    }
}

fn create_level_entities() -> Vec<CustomEntityComponent> {
    let entities: Vec<CustomEntityComponent> = vec![];

    entities
}

// should we query state and check if the level should be spawned?
// otherwise, levels will need to be spawned by the LevelOrchestrator
// ...
// On second thought, I don't think the level should be aware of what the 'current'
// level is. Levels should just take care of managing spawning all the pieces of
// their level... this might include enemies
// ...
// Maybe when I want to get fancy I can make a Level trait that a level implements.
