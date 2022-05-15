use crate::*;
use bevy::prelude::{Entity, Transform};
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Collisions {
    store: Vec<[Entity; 2]>,
}

impl Collisions {
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

impl IntoIterator for Collisions {
    type Item = [Entity; 2];
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}

#[inline]
pub fn ball_edge_top_collision(
    ball: &Ball,
    transform: &mut Transform,
    velocity: &mut Velocity,
) -> bool {
    if transform.translation.y <= ball.radius {
        transform.translation.y = ball.radius + (ball.radius - transform.translation.y);
        velocity.0.y *= -1.0;
        return true;
    }
    return false;
}

#[inline]
pub fn ball_edge_bottom_collision(
    ball: &Ball,
    transform: &mut Transform,
    velocity: &mut Velocity,
) -> bool {
    let max_y = HEIGHT - ball.radius;
    if transform.translation.y >= max_y {
        transform.translation.y = max_y - (transform.translation.y - max_y);
        velocity.0.y *= -1.0;
        return true;
    }
    return false;
}

#[inline]
pub fn ball_edge_left_collision(
    ball: &Ball,
    transform: &mut Transform,
    velocity: &mut Velocity,
) -> bool {
    // transform.translation.x - ball.radius <= 0.0
    if transform.translation.x <= ball.radius {
        transform.translation.x = ball.radius + (ball.radius - transform.translation.x);
        velocity.0.x *= -1.0;
        return true;
    }
    return false;
}

#[inline]
pub fn ball_edge_right_collision(
    ball: &Ball,
    transform: &mut Transform,
    velocity: &mut Velocity,
) -> bool {
    let max_x = WIDTH - ball.radius;
    // transform.translation.x + self.radius >= WIDTH
    if transform.translation.x >= max_x {
        transform.translation.x = max_x - (transform.translation.x - max_x);
        velocity.0.x *= -1.0;
        return true;
    }
    return false;
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
