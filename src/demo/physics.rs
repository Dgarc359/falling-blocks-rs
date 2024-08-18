use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Physics;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Physics>();
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_systems(
        // meant to be called at consistent timesteps
        FixedUpdate,
        (
            update_system,
            //read_result_system
        ),
    );
}

fn update_system(_time: Res<Time>, mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        // add gravity to kinematic body
        controller.translation = Some(Vec2::new(0.0, -2.0));
    }
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (_entity, _output) in controllers.iter() {
        /*
        dbg!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            entity,
            output.effective_translation,
            output.grounded
        );
        */
    }
}
