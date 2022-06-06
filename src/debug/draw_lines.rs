use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;

use crate::quadtree::{Bounds, Location};

use super::*;

pub trait DebugDrawLines {
    fn debug_draw_lines(self, draw: &mut DebugLines, color: Option<Color>);
}

impl DebugDrawLines for Vec2 {
    fn debug_draw_lines(self, draw: &mut DebugLines, color: Option<Color>) {
        let color = color.unwrap_or(Color::RED);
        draw.line_colored(Vec3::new(self.x - 1., self.y, 0.), Vec3::new(self.x + 1., self.y, 0.), 0., color);
        draw.line_colored(Vec3::new(self.x, self.y - 1., 0.), Vec3::new(self.x, self.y + 1., 0.), 0., color);
    }
}

impl DebugDrawLines for Bounds {
    fn debug_draw_lines(self, draw: &mut DebugLines, color: Option<Color>) {
        let color = color.unwrap_or(Color::GREEN);
        let tl = Vec3::from((self.top_left(), 0.));
        let tr = Vec3::from((self.top_right(), 0.));
        let bl = Vec3::from((self.bottom_left(), 0.));
        let br = Vec3::from((self.bottom_right(), 0.));

        draw.line_colored(tl, tr, 0., color); // top
        draw.line_colored(tr, br, 0., color); // right
        draw.line_colored(br, bl, 0., color); // bottom
        draw.line_colored(bl, tl, 0., color); // left
    }
}

impl DebugDrawLines for Location {
    fn debug_draw_lines(self, draw: &mut DebugLines, color: Option<Color>) {
        let color = color.unwrap_or(Color::RED);
        match self {
            Self::Point(point) => { point.debug_draw_lines(draw, Some(color)) }
            Self::Area(bounds) => { bounds.debug_draw_lines(draw, Some(color)) }
        }
    }
}