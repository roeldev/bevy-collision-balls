use bevy::ecs::component::Component;

use crate::*;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub enum Location {
    Point(Vec2),
    Area(Bounds),
}

impl Location {
    #[inline]
    pub fn new(center: Vec2, width: f32, height: f32) -> Self {
        if width == 0.0 && height == 0.0 {
            Self::Point(center)
        } else {
            Self::Area(Bounds::new(center, width, height))
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn set_center(&mut self, center: Vec2) {
        match self {
            Self::Point(point) => {
                point.x = center.x;
                point.y = center.y;
            }
            Self::Area(bounds) => {
                bounds.center.x = center.x;
                bounds.center.y = center.y;
            }
        }
    }
}

impl From<Vec2> for Location {
    #[inline(always)]
    fn from(v: Vec2) -> Self { Self::Point(v) }
}

impl From<Bounds> for Location {
    #[inline(always)]
    fn from(v: Bounds) -> Self { Self::Area(v) }
}