//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

use crate::{
    assets::ImageHandles,
    demo::{
        animation::PlayerAnimation,
        movement::ScreenWrap,
    },
    screens::Screen,
};
use bevy_rapier2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    /*
    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSet::RecordInput),
    );
    */
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// A command to spawn the player character.
#[derive(Debug)]
pub struct SpawnPlayer {
    pub _max_speed: f32,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

fn spawn_player(
    In(_config): In<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple
    // sprites. By attaching it to a [`SpriteBundle`] and providing an index, we
    // can specify which section of the image we want to see. We will use this
    // to animate our player character. You can learn more about texture atlases in
    // this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands
        .spawn((
            Name::new("Player"),
            Player,
            SpriteBundle {
                texture: image_handles[ImageHandles::PATH_DUCKY].clone_weak(),
                transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_animation.get_atlas_index(),
            },
            /*
            MovementController {
                max_speed: config.max_speed,
                ..default()
            },
            */
            ScreenWrap,
            player_animation,
            StateScoped(Screen::Playing),
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default());
}
