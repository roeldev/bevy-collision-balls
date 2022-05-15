use std::fmt;

use bevy::render::primitives::Aabb;

use crate::*;

#[derive(Clone, Copy)]
pub struct Bounds {
    center: Point,
    half_extents: Point,
}

#[allow(dead_code)]
impl Bounds {
    #[inline]
    pub fn new(top_left: Point, width: f32, height: f32) -> Self {
        let half_extents = Point::new(width * 0.5, height * 0.5);
        Self::from_center(top_left + half_extents, half_extents)
    }

    #[inline(always)]
    pub fn from_center(center: Point, half_extents: Point) -> Self {
        Self {
            center: Point::from(center),
            half_extents: Point::from(half_extents),
        }
    }

    #[inline]
    pub fn from_corners(top_left: Point, bottom_right: Point) -> Self {
        Self::new(
            top_left,
            bottom_right.x - top_left.x,
            bottom_right.y - top_left.y,
        )
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.half_extents.x * 2.0
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.half_extents.y * 2.0
    }

    #[inline(always)]
    pub fn top(&self) -> f32 {
        self.center.y - self.half_extents.y
    }

    #[inline(always)]
    pub fn bottom(&self) -> f32 {
        self.center.y + self.half_extents.y
    }

    #[inline(always)]
    pub fn left(&self) -> f32 {
        self.center.x - self.half_extents.x
    }

    #[inline(always)]
    pub fn right(&self) -> f32 {
        self.center.x + self.half_extents.x
    }

    #[inline]
    pub fn center(&self) -> Point {
        self.center
    }

    #[inline]
    pub fn top_left(&self) -> Point {
        Point::new(self.left(), self.top())
    }

    #[inline]
    pub fn top_right(&self) -> Point {
        Point::new(self.right(), self.top())
    }

    #[inline]
    pub fn bottom_left(&self) -> Point {
        Point::new(self.left(), self.bottom())
    }

    #[inline]
    pub fn bottom_right(&self) -> Point {
        Point::new(self.right(), self.bottom())
    }

    pub fn intersects(&self, area: Bounds) -> bool {
        self.contains(area.top_left())
            || self.contains(area.top_right())
            || self.contains(area.bottom_left())
            || self.contains(area.bottom_right())
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.top()
            && point.y <= self.bottom()
    }
}

impl DebugDrawLines for Bounds {
    fn debug_draw_lines(self, draw: &mut DebugLines, color: Option<Color>) {
        let color = color.unwrap_or(Color::GREEN);
        draw.line_colored(self.top_left().into(), self.top_right().into(), 0.0, color);
        draw.line_colored(self.top_right().into(), self.bottom_right().into(), 0.0, color);
        draw.line_colored(self.bottom_right().into(), self.bottom_left().into(), 0.0, color);
        draw.line_colored(self.bottom_left().into(), self.top_left().into(), 0.0, color);
    }
}

impl From<Aabb> for Bounds {
    #[inline]
    fn from(v: Aabb) -> Self {
        Self::from_center(Point::from(v.center), Point::from(v.half_extents))
    }
}

impl fmt::Debug for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bounds")
            .field("center", &self.center)
            .field("top_left", &self.top_left())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}