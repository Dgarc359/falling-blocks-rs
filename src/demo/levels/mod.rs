use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

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
