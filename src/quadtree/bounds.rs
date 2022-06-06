use std::fmt;
use std::fmt::Formatter;

use crate::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Bounds {
    pub(crate) center: Vec2,
    half_extents: Vec2,
}

#[allow(dead_code)]
impl Bounds {
    #[inline(always)]
    pub fn new(center: Vec2, width: f32, height: f32) -> Self {
        Self {
            center,
            half_extents: Vec2::new(width * 0.5, height * 0.5),
        }
    }

    #[inline]
    pub fn from_corners(min: Vec2, max: Vec2) -> Self {
        assert!(min.x < max.x);
        let mut center = Vec2::ZERO;
        let mut half_extents = Vec2::new((max.x - min.x) * 0.5, 0.0);

        if min.y > max.y {
            // min = top left
            half_extents.y = (min.y - max.y) * 0.5;
            center.y = min.y - half_extents.y;
        } else {
            // min = bottom left
            half_extents.y = (max.y - min.y) * 0.5;
            center.y = min.y + half_extents.y;
        }

        center.x = min.x + half_extents.x;
        Self { center, half_extents }
    }

    #[inline(always)]
    pub fn width(&self) -> f32 { self.half_extents.x * 2.0 }

    #[inline(always)]
    pub fn height(&self) -> f32 { self.half_extents.y * 2.0 }

    #[inline(always)]
    pub fn top(&self) -> f32 { self.center.y + self.half_extents.y }

    #[inline(always)]
    pub fn bottom(&self) -> f32 { self.center.y - self.half_extents.y }

    #[inline(always)]
    pub fn left(&self) -> f32 { self.center.x - self.half_extents.x }

    #[inline(always)]
    pub fn right(&self) -> f32 { self.center.x + self.half_extents.x }

    #[inline(always)]
    pub fn center(&self) -> Vec2 { self.center }

    #[inline(always)]
    pub fn min(&self) -> Vec2 { self.bottom_left() }

    #[inline(always)]
    pub fn max(&self) -> Vec2 { self.top_right() }

    #[inline]
    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.left(), self.top())
    }

    #[inline]
    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.right(), self.top())
    }

    #[inline]
    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.left(), self.bottom())
    }

    #[inline]
    pub fn bottom_right(&self) -> Vec2 {
        Vec2::new(self.right(), self.bottom())
    }

    #[inline]
    pub fn intersects(&self, area: Bounds) -> bool {
        self.contains(area.top_left())
            || self.contains(area.top_right())
            || self.contains(area.bottom_left())
            || self.contains(area.bottom_right())
    }

    #[inline]
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y <= self.top()
            && point.y >= self.bottom()
    }
}

// impl From<Aabb> for Bounds {
//     #[inline]
//     fn from(v: Aabb) -> Self {
//         Self {
//             center: v.center.truncate(),
//             half_extents: v.half_extents.truncate(),
//         }
//     }
// }

impl fmt::Debug for Bounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bounds")
            .field("center", &self.center)
            .field("top_left", &self.top_left())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_from_corners() {
        let bounds = Bounds::new(Vec2::new(5.0, 5.0), 10.0, 10.0);
        assert_eq!(Bounds::from_corners(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0)), bounds);
        assert_eq!(Bounds::from_corners(Vec2::new(0.0, 10.0), Vec2::new(10.0, 0.0)), bounds);
    }

    #[test]
    fn bounds_contains_point() {
        let bounds = Bounds::new(Vec2::ZERO, 4.0, 4.0);
        assert!(bounds.contains(Vec2::new(2.0, 2.0)));
        assert!(bounds.contains(Vec2::new(-2.0, -2.0)));
        assert!(!bounds.contains(Vec2::new(10.0, 10.0)))
    }
}
