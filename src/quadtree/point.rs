use std::ops::{Add, Sub};

use bevy::math::{Vec3, Vec3A, XY};
use bevy::prelude::Color;
use bevy_prototype_debug_lines::DebugLines;

use crate::DebugDrawLines;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

impl DebugDrawLines for Point {
    fn debug_draw_lines(self, draw: &mut DebugLines, color: Option<Color>) {
        let color = color.unwrap_or(Color::RED);
        draw.line_colored(Vec3::new(self.x - 1.0, self.y, 0.0), Vec3::new(self.x + 1.0, self.y, 0.0), 0.0, color);
        draw.line_colored(Vec3::new(self.x, self.y - 1.0, 0.0), Vec3::new(self.x, self.y + 1.0, 0.0), 0.0, color);
    }
}

impl Add for Point {
    type Output = Point;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<(f32, f32)> for Point {
    fn from(v: (f32, f32)) -> Self { Self::new(v.0, v.1) }
}

impl From<Point> for (f32, f32) {
    #[inline(always)]
    fn from(v: Point) -> Self { (v.x, v.y) }
}

impl From<XY<f32>> for Point {
    #[inline(always)]
    fn from(v: XY<f32>) -> Self { Self::new(v.x, v.y) }
}

impl From<Point> for XY<f32> {
    #[inline(always)]
    fn from(v: Point) -> Self { Self { x: v.x, y: v.y } }
}

impl From<Vec3> for Point {
    #[inline(always)]
    fn from(v: Vec3) -> Self { Self::new(v.x, v.y) }
}

impl From<Point> for Vec3 {
    #[inline(always)]
    fn from(v: Point) -> Self { Self::new(v.x, v.y, 0.0) }
}

impl From<Vec3A> for Point {
    #[inline(always)]
    fn from(v: Vec3A) -> Self { Self::new(v.x, v.y) }
}

impl From<Point> for Vec3A {
    #[inline(always)]
    fn from(v: Point) -> Self { Self::new(v.x, v.y, 0.0) }
}