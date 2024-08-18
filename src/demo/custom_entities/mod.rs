use bevy::{prelude::*, ecs::world::Command};

use crate::assets::ImageHandles;

#[derive(Debug)]
pub enum CustomEntities {
    Block64Px,
    Block32Px,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
// what does this do? Find out
#[reflect(Component)]
pub struct CustomEntityComponent;

#[derive(Debug)]
pub struct SpawnCustomEntity {
    entity_to_spawn: CustomEntities
}

impl Command for SpawnCustomEntity {
    fn apply(self, world: &mut World) {
        // this code works but something is wrong with the imports, i think
        // TODO: fix
        world.run_system_once_with(self,spawn_entity);
    }
}

pub fn spawn_entity(
    entity: CustomEntities,
    texture_atlas_handle: Handle<TextureAtlasLayout>,
    image_handles: Res<ImageHandles>
) -> (SpriteBundle, TextureAtlas) {
    // use texture atlas (check player.rs for sample impl)
    // 32px / 1 col / 1 row / no padding / no offset
    // this needs to happen outside actually... we need to be given a handle
    //let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 1, 1, None, None);
    //let texture_atlas_layout = texture_atlas_layouts.add(layout);
    match entity {
        CustomEntities::Block64Px => {
            todo!()
        }
        CustomEntities::Block32Px => {
            (SpriteBundle {
                texture: image_handles[ImageHandles::PATH_TEXTUREMAP].clone_weak(),
                ..Default::default()

            },
            TextureAtlas {
                layout: texture_atlas_handle.clone(),
                index: 0,
            })
        }
    }
}
