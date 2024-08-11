//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod animation;
pub mod level;
mod levels;
mod movement;
mod physics;
mod player;

pub(super) fn plugin(app: &mut App) {
    // set timestep rate for fixed timestep subsystems
    app.insert_resource(Time::<Fixed>::from_hz(1000.0));

    app.add_plugins((
        animation::plugin,
        //movement::plugin,
        player::plugin,
        level::plugin,
        physics::plugin,
    ));
}
