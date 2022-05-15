use bevy::prelude::Color;
use bevy_prototype_debug_lines::DebugLines;

pub trait DebugDrawLines {
    fn debug_draw_lines(self, draw: &mut DebugLines, color: Option<Color>);
}