use std::vec::IntoIter;

use bevy::prelude::{Entity, Transform};

use crate::*;

#[derive(Debug)]
pub struct EdgeCollider {
    pub(crate) bounds: Bounds,
}

impl EdgeCollider {
    #[inline]
    pub fn new(bounds: Bounds) -> Self {
        Self { bounds }
    }

    #[inline]
    pub fn range_x(&self, padding: f32) -> RangeInclusive<f32> {
        (self.bounds.left() + padding)..=(self.bounds.right() - padding - padding)
    }

    #[inline]
    pub fn range_y(&self, padding: f32) -> RangeInclusive<f32> {
        (self.bounds.bottom() + padding)..=(self.bounds.top() - padding - padding)
    }

    #[inline]
    pub fn check_left(&self, ball: &Ball, transform: &mut Transform, velocity: &mut Velocity) -> bool {
        let min_x = self.bounds.left() + ball.radius;
        if transform.translation.x > min_x {
            return false;
        }

        transform.translation.x = min_x + (min_x - transform.translation.x);
        velocity.0.x *= -1.0;
        return true;
    }

    #[inline]
    pub fn check_right(&self, ball: &Ball, transform: &mut Transform, velocity: &mut Velocity) -> bool {
        let max_x = self.bounds.right() - ball.radius;
        if transform.translation.x < max_x {
            return false;
        }

        transform.translation.x = max_x - (transform.translation.x - max_x);
        velocity.0.x *= -1.0;
        return true;
    }

    #[inline]
    pub fn check_top(&self, ball: &Ball, transform: &mut Transform, velocity: &mut Velocity) -> bool {
        let max_y = self.bounds.top() - ball.radius;
        if transform.translation.y < max_y {
            return false;
        }

        transform.translation.y = max_y - (transform.translation.y - max_y);
        velocity.0.y *= -1.0;
        return true;
    }

    #[inline]
    pub fn check_bottom(&self, ball: &Ball, transform: &mut Transform, velocity: &mut Velocity) -> bool {
        let min_y = self.bounds.bottom() + ball.radius;
        if transform.translation.y > min_y {
            return false;
        }

        transform.translation.y = min_y + (min_y - transform.translation.y);
        velocity.0.y *= -1.0;
        return true;
    }
}

#[derive(Debug)]
pub struct BallCollisions {
    store: Vec<[Entity; 2]>,
}

impl BallCollisions {
    #[inline]
    pub fn new(capacity: Option<usize>) -> Self {
        Self {
            store: if let Some(c) = capacity {
                Vec::with_capacity(c)
            } else {
                Vec::new()
            },
        }
    }

    #[inline]
    pub fn check(&mut self, balls: [(Entity, &mut Transform, &Ball); 2]) {
        let [(a, transform_a, ball_a), (b, transform_b, ball_b)] = balls;

        let x = transform_a.translation.x - transform_b.translation.x;
        let y = transform_a.translation.y - transform_b.translation.y;
        let r = ball_a.radius + ball_b.radius;

        let mut distance = (x * x) + (y * y);
        if distance > (r * r) {
            return;
        }

        distance = f32::sqrt(distance);
        let overlap = (distance - r) * 0.5;

        transform_a.translation.x -= overlap * x / distance;
        transform_a.translation.y -= overlap * y / distance;
        transform_b.translation.x += overlap * x / distance;
        transform_b.translation.y += overlap * y / distance;
        self.store.push([a, b]);
    }
}

impl IntoIterator for BallCollisions {
    type Item = [Entity; 2];
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}

// Update velocity according to mass, after the balls bounce off of each other.
#[inline]
pub fn balls_bounce_after_collision(balls: [(&Transform, &mut Velocity, &Ball); 2]) {
    let [(transform_a, velocity_a, ball_a), (transform_b, velocity_b, ball_b)] = balls;

    let x = transform_a.translation.x - transform_b.translation.x;
    let y = transform_a.translation.y - transform_b.translation.y;

    let distance = f32::sqrt((x * x) + (y * y));

    let nx = (transform_b.translation.x - transform_a.translation.x) / distance;
    let ny = (transform_b.translation.y - transform_a.translation.y) / distance;
    let kx = velocity_a.0.x - velocity_b.0.x;
    let ky = velocity_a.0.y - velocity_b.0.y;

    let p = 2.0 * ((nx * kx) + (ny * ky)) / (ball_a.mass + ball_b.mass);

    velocity_a.0.x -= p * ball_b.mass * nx;
    velocity_a.0.y -= p * ball_b.mass * ny;
    velocity_b.0.x += p * ball_a.mass * nx;
    velocity_b.0.y += p * ball_a.mass * ny;
}
