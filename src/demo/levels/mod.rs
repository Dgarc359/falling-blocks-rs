use bevy::{ecs::world::Command, prelude::*};

mod level_one;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Levels>();
    app.init_state::<Levels>();
    app.enable_state_scoped_entities::<Levels>();

    app.add_plugins(level_one::plugin);
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
pub enum Levels {
    #[default]
    LevelOne,
}

#[derive(Debug)]
pub struct SpawnLevelOrchestrator;

impl Command for SpawnLevelOrchestrator {
    fn apply(self, world: &mut World) {
        println!("level orchestrator spawned");
        world.commands().add(level_one::SpawnLevelOne {});
    }
}

// Maybe some type of query on some level state can give us what we need to
// switch levels around, for now, we're starting with spawning level one...
// We'll need to make sure that it gets cleaned up correctly if we need to spawn
// another level
