use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;

use crate::*;

#[derive(Component)]
pub struct Velocity(pub(crate) Vec2);

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub mass: f32,
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub velocity: Velocity,

    #[bundle]
    pub shape_bundle: ShapeBundle,
}

impl BallBundle {
    pub fn new(color: Color, radius: f32, velocity: Vec2, position: Vec2) -> Self {
        Self {
            ball: Ball {
                radius,
                mass: radius * radius,
            },
            velocity: Velocity(velocity),
            shape_bundle: GeometryBuilder::build_as(
                &shapes::Circle {
                    radius,
                    ..default()
                },
                DrawMode::Fill(FillMode::color(color)),
                Transform::from_translation(Vec3::from((position, 0.))),
            ),
        }
    }
}