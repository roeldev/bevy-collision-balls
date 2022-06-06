use bevy::prelude::*;

use crate::*;

pub struct QuadTreePlugin {}

impl Plugin for QuadTreePlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

pub struct EntityLocationCollector {}

// fn update_system(collector: ResMut<EntityLocationCollector>, entities: Query<(Entity, &Location)>) { // updated
//     for (entity, location) in entities.iter() {
//         collector.update(entity, location);
//     }
// }