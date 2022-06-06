#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::{Deref, RangeInclusive};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::math::*;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_prototype_lyon::prelude::*;
use rand::distributions::{Distribution, Uniform};

use crate::collision::*;
use crate::components::*;
use crate::debug::*;
use crate::quadtree::*;

mod collision;
mod components;
mod quadtree;
mod debug;

pub const WIDTH: f32 = 1024.;
pub const HEIGHT: f32 = 768.;

const BALLS: u64 = 1000;

// Min/max radius range of balls.
const BALL_RADIUS: RangeInclusive<f32> = 2.0..=16.0;

// Initial random speed of ball.
const BALL_INIT_SPEED: RangeInclusive<f32> = 10.0..=50.;

// Possible ball colors.
const BALL_COLORS: [Color; 36] = [
    Color::ALICE_BLUE,
    Color::ANTIQUE_WHITE,
    Color::AQUAMARINE,
    Color::AZURE,
    Color::BEIGE,
    Color::BISQUE,
    Color::BLUE,
    Color::CRIMSON,
    Color::CYAN,
    Color::DARK_GRAY,
    Color::DARK_GREEN,
    Color::FUCHSIA,
    Color::GOLD,
    Color::GRAY,
    Color::GREEN,
    Color::INDIGO,
    Color::LIME_GREEN,
    Color::MAROON,
    Color::MIDNIGHT_BLUE,
    Color::NAVY,
    Color::OLIVE,
    Color::ORANGE,
    Color::ORANGE_RED,
    Color::PINK,
    Color::PURPLE,
    Color::RED,
    Color::SALMON,
    Color::SEA_GREEN,
    Color::SILVER,
    Color::TEAL,
    Color::TOMATO,
    Color::TURQUOISE,
    Color::VIOLET,
    Color::WHITE,
    Color::YELLOW,
    Color::YELLOW_GREEN,
];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Balls".to_string(),
            width: WIDTH,
            height: HEIGHT,
            present_mode: PresentMode::Immediate,
            resizable: true,
            cursor_visible: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WindowTitleFpsPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(spawn_balls)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(check_collisions_quadtree.after(apply_velocity))
        // .add_system(check_collisions.after(apply_velocity))
        .add_system(apply_velocity)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn display_fps(
    mut windows: ResMut<Windows>,
    windescr: Res<WindowDescriptor>,
    diagnostics: Res<Diagnostics>,
) {
    if let Some(fps) = diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS) {
        let window = windows.primary_mut();
        window.set_title(format!("{}: {}", windescr.title, fps.value.to_string()));
    }
}

fn spawn_balls(mut cmd: Commands) {
    let rand_radius = Uniform::from(BALL_RADIUS);
    let rand_velocity = Uniform::from(BALL_INIT_SPEED);

    let edge = EdgeCollider::new(Bounds::new(Vec2::ZERO, WIDTH, HEIGHT));
    let rand_pos_x = Uniform::from(edge.range_x(*BALL_RADIUS.end()));
    let rand_pos_y = Uniform::from(edge.range_y(*BALL_RADIUS.end()));
    cmd.insert_resource(edge);

    let mut rng = rand::thread_rng();
    let mut ball_color_index: usize = 0;

    for _ in 0..BALLS {
        let radius = rand_radius.sample(&mut rng);
        let mut velocity = Vec2::new(
            rand_velocity.sample(&mut rng),
            rand_velocity.sample(&mut rng),
        );
        if rand::random() {
            velocity.x *= -1.;
        }
        if rand::random() {
            velocity.y *= -1.;
        }

        cmd.spawn_bundle(BallBundle::new(
            BALL_COLORS[ball_color_index],
            radius,
            velocity,
            Vec2::new(
                rand_pos_x.sample(&mut rng),
                rand_pos_y.sample(&mut rng),
            ),
        ));

        ball_color_index += 1;
        if ball_color_index == BALL_COLORS.len() {
            ball_color_index = 0;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // apply friction
        // velocity.0.x -= velocity.0.x * 0.03 * time.delta_seconds();
        // velocity.0.y -= velocity.0.y * 0.03 * time.delta_seconds();

        // apply velocity
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

#[allow(dead_code)]
// fn check_collisions(edge: Res<EdgeCollider>, mut query: Query<(Entity, &mut Transform, &mut Velocity, &Ball)>) {
//     for (_, mut transform, mut velocity, ball) in query.iter_mut() {
//         let transform = &mut *transform;
//         let velocity = &mut *velocity;
//
//         let _ = edge.check_left(ball, transform, velocity)
//             || edge.check_right(ball, transform, velocity);
//
//         let _ = edge.check_top(ball, transform, velocity)
//             || edge.check_bottom(ball, transform, velocity);
//     }
//
//     let mut collisions = BallCollisions::new(Some(BALLS as usize));
//
//     let mut combinations = query.iter_combinations_mut();
//     while let Some([a, b]) = combinations.fetch_next() {
//         let (a, mut transform_a, _, ball_a) = a;
//         let (b, mut transform_b, _, ball_b) = b;
//
//         collisions.check([
//             (a, &mut *transform_a, ball_a),
//             (b, &mut *transform_b, ball_b),
//         ]);
//     }
//
//     for balls in collisions {
//         let [
//         (_, transform_a, mut velocity_a, ball_a),
//         (_, transform_b, mut velocity_b, ball_b)
//         ] = query.many_mut(balls);
//
//         balls_bounce_after_collision([
//             (transform_a.deref(), &mut *velocity_a, ball_a),
//             (transform_b.deref(), &mut *velocity_b, ball_b),
//         ]);
//     }
// }
#[allow(dead_code)]
fn check_collisions_quadtree(
    edge: Res<EdgeCollider>,
    mut debug_lines: ResMut<DebugLines>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &Ball)>,
) {
    let debug_lines = &mut *debug_lines;
    edge.bounds.debug_draw_lines(debug_lines, Some(Color::WHITE));

    let mut tree = QuadTree::new(
        edge.bounds,
        Options {
            capacity: 4,
            min_size: Some(Vec2::splat(BALL_RADIUS.end() * 2.)),
            ..default()
        },
    );

    for (entity, mut transform, mut velocity, ball) in query.iter_mut() {
        let transform = &mut *transform;
        let velocity = &mut *velocity;

        let _ = edge.check_left(ball, transform, velocity)
            || edge.check_right(ball, transform, velocity);

        let _ = edge.check_top(ball, transform, velocity)
            || edge.check_bottom(ball, transform, velocity);

        if let Err(err) = tree.insert(
            Location::new(transform.translation.truncate(), ball.radius * 2., ball.radius * 2.),
            entity,
        ) {
            match err {
                ErrorKind::OutOfBounds(bounds, location) => {
                    println!("err: {}: {}, {:?} not in {:?}", entity.id(), err, location, bounds)
                }
            }
        }
    }

    // query.iter();
    // query.iter_combinations();
    // query.for_each(|(x, y, z)| {});
    // query.par_for_each(pool, 8, |(x, y, z)| {});

    for region in tree.regions() {
        region.bounds().debug_draw_lines(debug_lines, None);
        let elems = region.elements().unwrap();
        if elems.len() < 2 {
            continue;
        }

        let mut collisions = BallCollisions::new(Some(elems.capacity() * 2));
        for (_, a) in elems.clone() {
            for (_, b) in elems.clone() {
                if a == b {
                    continue;
                }

                let [
                (a, mut transform_a, _, ball_a),
                (b, mut transform_b, _, ball_b)
                ] = query.many_mut([a, b]);

                debug_lines.line(transform_a.translation, transform_b.translation, 0.);

                collisions.check([
                    (a, &mut *transform_a, ball_a),
                    (b, &mut *transform_b, ball_b),
                ]);
            }
        }

        for balls in collisions {
            let [
            (_, transform_a, mut velocity_a, ball_a),
            (_, transform_b, mut velocity_b, ball_b)
            ] = query.many_mut(balls);

            balls_bounce_after_collision([
                (transform_a.deref(), &mut *velocity_a, ball_a),
                (transform_b.deref(), &mut *velocity_b, ball_b),
            ]);
        }
    }

    // query.iter_combinations();
    // for (a, b) in tree.iter_combinations() {
    //     query.
    // }

    // for q in query.iter_mut() {}
    // for partitions in qt.iter() {
    //     let mut combinations = .iter_combinations_mut();
    //     for q in partitions.iter_combinations_mut() {}
    // }
    // print!("w:{}, h:{}, l:{}\n", qt.width(), qt.height(), qt.len())
}
