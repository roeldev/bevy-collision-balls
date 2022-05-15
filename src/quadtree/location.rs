use bevy::prelude::{Color, Component};
use bevy::render::primitives::Aabb;
use bevy_prototype_debug_lines::DebugLines;

use crate::{Bounds, DebugDrawLines, Point};

#[derive(Component, Copy, Clone, Debug)]
pub enum Location {
    Point(Point),
    Area(Bounds),
}

impl Location {
    #[inline]
    pub fn from_center(center: Point, width: f32, height: f32) -> Self {
        if width == 0.0 && height == 0.0 {
            return Location::Point(center);
        }

        Location::Area(Bounds::from_center(
            center,
            Point::new(width * 0.5, height * 0.5),
        ))
    }
}

impl DebugDrawLines for Location {
    fn debug_draw_lines(self, draw: &mut DebugLines, color: Option<Color>) {
        let color = color.unwrap_or(Color::RED);
        match self {
            Location::Point(point) => { point.debug_draw_lines(draw, Some(color)) }
            Location::Area(bounds) => { bounds.debug_draw_lines(draw, Some(color)) }
        }
    }
}

impl From<Point> for Location {
    #[inline(always)]
    fn from(v: Point) -> Self {
        Self::Point(v)
    }
}

impl From<Bounds> for Location {
    #[inline(always)]
    fn from(v: Bounds) -> Self {
        Self::Area(v)
    }
}

impl From<Aabb> for Location {
    #[inline]
    fn from(v: Aabb) -> Self {
        if v.half_extents.x == 0.0 && v.half_extents.y == 0.0 {
            Location::Point(Point::from(v.center))
        } else {
            Location::Area(Bounds::from(v))
        }
    }
}
