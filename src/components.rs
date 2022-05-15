use bevy::math::Vec2;
use bevy::prelude::*;

use crate::*;

#[derive(Component)]
pub struct Velocity(pub(crate) Vec2);

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub mass: f32,
}
