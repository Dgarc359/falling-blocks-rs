use bevy::prelude::Component;

pub enum CustomEntities {
    Block64Px,
    Block32Px
}

#[derive(Component)]
pub struct CustomEntityComponent;


pub fn get_entity_to_spawn(entity: CustomEntities) -> CustomEntityComponent  {
    match entity {
        CustomEntities::Block64Px => { todo!() },
        CustomEntities::Block32Px => { todo!() },
    }

}
